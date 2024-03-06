// APN Restriction IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// APN Restriction IE Type

pub const APNRESTRICTION: u8 = 127;
pub const APNRESTRICTION_LENGTH: usize = 1;

// APN Restriction Enum and Values as per 3GPP 23.060 V16.0.0 (2019-03)

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Restriction {
    #[default]
    NoApnRestriction,
    Public1,
    Public2,
    Private1,
    Private2,
}

impl From<&Restriction> for u8 {
    fn from(i: &Restriction) -> u8 {
        match i {
            Restriction::NoApnRestriction => 0,
            Restriction::Public1 => 1,
            Restriction::Public2 => 2,
            Restriction::Private1 => 3,
            Restriction::Private2 => 4,
        }
    }
}

impl From<u8> for Restriction {
    fn from(i: u8) -> Restriction {
        match i {
            0 => Restriction::NoApnRestriction,
            1 => Restriction::Public1,
            2 => Restriction::Public2,
            3 => Restriction::Private1,
            4 => Restriction::Private2,
            _ => Restriction::NoApnRestriction,
        }
    }
}

// APN Restriction IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApnRestriction {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub restriction_type: Restriction,
}

impl Default for ApnRestriction {
    fn default() -> Self {
        ApnRestriction {
            t: APNRESTRICTION,
            length: APNRESTRICTION_LENGTH as u16,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }
    }
}

impl From<ApnRestriction> for InformationElement {
    fn from(i: ApnRestriction) -> Self {
        InformationElement::ApnRestriction(i)
    }
}

impl IEs for ApnRestriction {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(APNRESTRICTION);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push((&self.restriction_type).into());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= APNRESTRICTION_LENGTH + MIN_IE_SIZE {
            let data = ApnRestriction {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                restriction_type: (buffer[4]).into(),
                ..ApnRestriction::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(APNRESTRICTION))
        }
    }

    fn len(&self) -> usize {
        APNRESTRICTION_LENGTH + MIN_IE_SIZE
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
fn apnrestriction_ie_marshal_test() {
    let encoded: [u8; 5] = [0x7f, 0x00, 0x01, 0x00, 0x00];
    let decoded = ApnRestriction {
        t: APNRESTRICTION,
        length: APNRESTRICTION_LENGTH as u16,
        ins: 0,
        restriction_type: Restriction::NoApnRestriction,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn apnrestriciton_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x7f, 0x00, 0x01, 0x00, 0x00];
    let decoded = ApnRestriction {
        t: APNRESTRICTION,
        length: APNRESTRICTION_LENGTH as u16,
        ins: 0,
        restriction_type: Restriction::NoApnRestriction,
    };
    assert_eq!(ApnRestriction::unmarshal(&encoded).unwrap(), decoded);
}
