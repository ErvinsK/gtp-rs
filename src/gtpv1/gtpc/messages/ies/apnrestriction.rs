// APN Restriction IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// APN Restriction IE Type

pub const APNRESTRICTION: u8 = 149;
pub const APNRESTRICTION_LENGTH: u16 = 1;

// APN Restriction Enum and Values as per 3GPP 23.060 V16.0.0 (2019-03)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Restriction {
    NoApnRestriction,
    Public1,
    Public2,
    Private1,
    Private2,
}

impl Restriction {
    fn enum_to_value(i: &Restriction) -> u8 {
        match i {
            Restriction::NoApnRestriction => 0,
            Restriction::Public1 => 1,
            Restriction::Public2 => 2,
            Restriction::Private1 => 3,
            Restriction::Private2 => 4,
        }
    }
    fn value_to_enum(i: u8) -> Result<Restriction, GTPV1Error> {
        match i {
            0 => Ok(Restriction::NoApnRestriction),
            1 => Ok(Restriction::Public1),
            2 => Ok(Restriction::Public2),
            3 => Ok(Restriction::Private1),
            4 => Ok(Restriction::Private2),
            _ => Err(GTPV1Error::IEIncorrect),
        }
    }
}

// APN Restriction IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApnRestriction {
    pub t: u8,
    pub length: u16,
    pub restriction_type: Restriction,
}

impl Default for ApnRestriction {
    fn default() -> Self {
        ApnRestriction {
            t: APNRESTRICTION,
            length: APNRESTRICTION_LENGTH,
            restriction_type: Restriction::NoApnRestriction,
        }
    }
}

impl IEs for ApnRestriction {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(Restriction::enum_to_value(&self.restriction_type));
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 4 {
            let mut data = ApnRestriction {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match Restriction::value_to_enum(buffer[3]) {
                Ok(i) => data.restriction_type = i,
                Err(j) => return Err(j),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (APNRESTRICTION_LENGTH + 1) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn apnrestriction_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0x95, 0x00, 0x01, 0x00];
    let ie_to_marshal = ApnRestriction {
        t: APNRESTRICTION,
        length: APNRESTRICTION_LENGTH,
        restriction_type: Restriction::NoApnRestriction,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn apnrestriciton_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0x95, 0x00, 0x01, 0x00];
    let ie_unmarshalled = ApnRestriction {
        t: APNRESTRICTION,
        length: APNRESTRICTION_LENGTH,
        restriction_type: Restriction::NoApnRestriction,
    };
    assert_eq!(
        ApnRestriction::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
