// Allocation/Retention Priority (ARP) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// ARP IE TV

pub const ARP: u8 = 155;
pub const ARP_LENGTH: usize = 1;

// ARP IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arp {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pci: bool, // PCI - Pre-emption Capability as per 3GPP TS 29.212
    pub pl: u8,    // PL - Priority Level as per 3GPP TS 29.212
    pub pvi: bool, // PVI - Pre-emption Vulnerability as per 3GPP TS 29.212
}

impl Default for Arp {
    fn default() -> Arp {
        Arp {
            t: ARP,
            length: ARP_LENGTH as u16,
            ins: 0,
            pci: false,
            pl: 0,
            pvi: false,
        }
    }
}

impl From<Arp> for InformationElement {
    fn from(i: Arp) -> Self {
        InformationElement::Arp(i)
    }
}

impl IEs for Arp {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ARP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.pci, self.pvi) {
            (false, false) => buffer_ie.push(self.pl << 2),
            (false, true) => buffer_ie.push((self.pl << 2) | 0x01),
            (true, false) => buffer_ie.push(0x40 | (self.pl << 2)),
            (true, true) => buffer_ie.push(0x40 | (self.pl << 2) | 0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= ARP_LENGTH + MIN_IE_SIZE {
            let mut data = Arp {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Arp::default()
            };
            match buffer[4] >> 6 {
                0 => data.pci = false,
                1 => data.pci = true,
                _ => return Err(GTPV2Error::IEIncorrect(ARP)),
            }
            data.pl = (buffer[4] >> 2) & 0x0f;
            match buffer[4] & 0x01 {
                0 => data.pvi = false,
                1 => data.pvi = true,
                _ => return Err(GTPV2Error::IEIncorrect(ARP)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ARP))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn arp_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x9b, 0x00, 0x01, 0x00, 0x54];
    let decoded = Arp {
        t: ARP,
        length: ARP_LENGTH as u16,
        ins: 0,
        pci: true,
        pl: 5,
        pvi: false,
    };
    let i = Arp::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn arp_ie_marshal_test() {
    let encoded: [u8; 5] = [0x9b, 0x00, 0x01, 0x00, 0x54];
    let decoded = Arp {
        t: ARP,
        length: ARP_LENGTH as u16,
        ins: 0,
        pci: true,
        pl: 5,
        pvi: false,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
