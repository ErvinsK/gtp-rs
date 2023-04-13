// RAT Type IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// RAT Type IE Type

pub const RATTYPE: u8 = 82;
pub const RATTYPE_LENGTH: usize = 1;

// RAT Type enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rat {
    Utran,
    Geran,
    Wlan,
    Gan,
    Hspaevo,
    Eutran,
    Virtual,
    Nbiot,
    Ltem,
    Nr,
}

impl Rat {
    fn enum_to_value(i: &Rat) -> u8 {
        match i {
            Rat::Utran => 1,
            Rat::Geran => 2,
            Rat::Wlan => 3,
            Rat::Gan => 4,
            Rat::Hspaevo => 5,
            Rat::Eutran => 6,
            Rat::Virtual => 7,
            Rat::Nbiot => 8,
            Rat::Ltem => 9,
            Rat::Nr => 10,
        }
    }
    fn value_to_enum(i: u8) -> Result<Rat, GTPV2Error> {
        match i {
            1 => Ok(Rat::Utran),
            2 => Ok(Rat::Geran),
            3 => Ok(Rat::Wlan),
            4 => Ok(Rat::Gan),
            5 => Ok(Rat::Hspaevo),
            6 => Ok(Rat::Eutran),
            7 => Ok(Rat::Virtual),
            8 => Ok(Rat::Nbiot),
            9 => Ok(Rat::Ltem),
            10 => Ok(Rat::Nr),
            _ => Err(GTPV2Error::IEIncorrect(RATTYPE)),
        }
    }
}

// RAT Type IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatType {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub rat_type: Rat,
}

impl Default for RatType {
    fn default() -> Self {
        RatType {
            t: RATTYPE,
            length: RATTYPE_LENGTH as u16,
            ins: 0,
            rat_type: Rat::Eutran,
        }
    }
}

impl From<RatType> for InformationElement {
    fn from(i: RatType) -> Self {
        InformationElement::RatType(i)
    }
}

impl IEs for RatType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(Rat::enum_to_value(&self.rat_type));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + RATTYPE_LENGTH {
            let mut data = RatType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            match Rat::value_to_enum(buffer[4]) {
                Ok(i) => data.rat_type = i,
                Err(j) => return Err(j),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RATTYPE))
        }
    }

    fn len(&self) -> usize {
        RATTYPE_LENGTH + 4
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn rattype_ie_marshal_test() {
    let encoded: [u8; 5] = [0x52, 0x00, 0x01, 0x00, 0x06];
    let decoded = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH as u16,
        ins: 0,
        rat_type: Rat::Eutran,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn rattype_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x52, 0x00, 0x01, 0x00, 0x06];
    let decoded = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH as u16,
        ins: 0,
        rat_type: Rat::Eutran,
    };
    assert_eq!(RatType::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn rattype_ie_unknown_rattype_unmarshal_test() {
    let encoded: [u8; 5] = [0x52, 0x00, 0x01, 0x00, 0x0f];
    assert_eq!(
        RatType::unmarshal(&encoded),
        Err(GTPV2Error::IEIncorrect(RATTYPE))
    );
}
