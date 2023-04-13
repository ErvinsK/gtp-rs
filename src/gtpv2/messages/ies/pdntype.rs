// PDN Type IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

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
}

impl Pdn {
    fn enum_to_value(i: &Pdn) -> u8 {
        match i {
            Pdn::Ipv4 => 1,
            Pdn::Ipv6 => 2,
            Pdn::Ipv46 => 3,
            Pdn::NonIp => 4,
        }
    }
    fn value_to_enum(i: u8) -> Result<Pdn, GTPV2Error> {
        match i {
            1 => Ok(Pdn::Ipv4),
            2 => Ok(Pdn::Ipv6),
            3 => Ok(Pdn::Ipv46),
            4 => Ok(Pdn::NonIp),
            _ => Err(GTPV2Error::IEIncorrect(PDNTYPE)),
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
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(Pdn::enum_to_value(&self.pdn_type));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PDNTYPE_LENGTH {
            let mut data = PdnType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            match Pdn::value_to_enum(buffer[4]) {
                Ok(i) => data.pdn_type = i,
                Err(j) => return Err(j),
            }
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
    let encoded: [u8; 5] = [0x63, 0x00, 0x01, 0x00, 0x05];
    assert_eq!(
        PdnType::unmarshal(&encoded),
        Err(GTPV2Error::IEIncorrect(PDNTYPE))
    );
}
