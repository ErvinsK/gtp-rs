// RAT Type IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// RAT Type IE Type

pub const RATTYPE: u8 = 151;
pub const RATTYPE_LENGTH: u16 = 1;

// RAT Type enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rat {
    Utran,
    Geran,
    Wlan,
    Gan,
    Hspaevo,
    Eutran,
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
        }
    }
    fn value_to_enum(i: u8) -> Result<Rat, GTPV1Error> {
        match i {
            1 => Ok(Rat::Utran),
            2 => Ok(Rat::Geran),
            3 => Ok(Rat::Wlan),
            4 => Ok(Rat::Gan),
            5 => Ok(Rat::Hspaevo),
            6 => Ok(Rat::Eutran),
            _ => Err(GTPV1Error::IEIncorrect),
        }
    }
}

// RAT Type IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatType {
    pub t: u8,
    pub length: u16,
    pub rat_type: Rat,
}

impl Default for RatType {
    fn default() -> Self {
        RatType {
            t: RATTYPE,
            length: RATTYPE_LENGTH,
            rat_type: Rat::Geran,
        }
    }
}

impl IEs for RatType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(Rat::enum_to_value(&self.rat_type));
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 4 {
            let mut data = RatType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match Rat::value_to_enum(buffer[3]) {
                Ok(i) => data.rat_type = i,
                Err(j) => return Err(j),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (RATTYPE_LENGTH + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn rattype_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0x97, 0x00, 0x01, 0x02];
    let ie_to_marshal = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH,
        rat_type: Rat::Geran,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn rattype_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0x97, 0x00, 0x01, 0x02];
    let ie_unmarshalled = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH,
        rat_type: Rat::Geran,
    };
    assert_eq!(
        RatType::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
