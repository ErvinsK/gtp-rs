// PDN Type IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// PDN Type IE Type

pub const PDNTYPE: u8 = 99;
pub const PDNTYPE_LENGTH: usize = 1;

// PDN Type enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pdn {
    Ipv4,
    Ipv6,
    Ipv46,
    NonIp,
    Ethernet,
    Reserved,
}

impl From<Pdn> for u8 {
    fn from(value: Pdn) -> Self {
        match value {
            Pdn::Ipv4 => 1,
            Pdn::Ipv6 => 2,
            Pdn::Ipv46 => 3,
            Pdn::NonIp => 4,
            Pdn::Ethernet => 5,
            Pdn::Reserved => 6,
        }
    }
}

impl From<u8> for Pdn {
    fn from(value: u8) -> Self {
        match value {
            1 => Pdn::Ipv4,
            2 => Pdn::Ipv6,
            3 => Pdn::Ipv46,
            4 => Pdn::NonIp,
            5 => Pdn::Ethernet,
            6 => Pdn::Reserved,
            _ => Pdn::Reserved,
        }
    }
}

// PDN Type IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdnType {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pdn_type: Pdn,
}

impl Default for PdnType {
    fn default() -> Self {
        PdnType {
            t: PDNTYPE,
            length: PDNTYPE_LENGTH as u16,
            ins: 0,
            pdn_type: Pdn::Ipv4,
        }
    }
}

impl From<PdnType> for InformationElement {
    fn from(i: PdnType) -> Self {
        InformationElement::PdnType(i)
    }
}

impl IEs for PdnType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PDNTYPE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.pdn_type.clone().into());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PDNTYPE_LENGTH {
            let data = PdnType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                pdn_type: buffer[4].into(),
                ..PdnType::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PDNTYPE))
        }
    }

    fn len(&self) -> usize {
        PDNTYPE_LENGTH + MIN_IE_SIZE
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
fn pdntype_ie_marshal_test() {
    let encoded: [u8; 5] = [0x63, 0x00, 0x01, 0x00, 0x01];
    let decoded = PdnType {
        t: PDNTYPE,
        length: PDNTYPE_LENGTH as u16,
        ins: 0,
        pdn_type: Pdn::Ipv4,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pdntype_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x63, 0x00, 0x01, 0x00, 0x01];
    let decoded = PdnType {
        t: PDNTYPE,
        length: PDNTYPE_LENGTH as u16,
        ins: 0,
        pdn_type: Pdn::Ipv4,
    };
    assert_eq!(PdnType::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn pdntype_ie_unknown_rattype_unmarshal_test() {
    let encoded: [u8; 5] = [0x63, 0x00, 0x01, 0x00, 0x07];
    let decoded = PdnType {
        t: PDNTYPE,
        length: PDNTYPE_LENGTH as u16,
        ins: 0,
        pdn_type: Pdn::Reserved,
    };
    assert_eq!(PdnType::unmarshal(&encoded).unwrap(), decoded);
}
