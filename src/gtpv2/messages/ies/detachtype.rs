// Detach Type IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Detach Type IE Type

pub const DETACHTYPE: u8 = 150;
pub const DETACHTYPE_LENGTH: usize = 1;

// Detach Type IE implementation

//     Detach Type           Values (Decimal)
//     <reserved>                 0
//      PS Detach                 1
//  Combined PS/CS Detach         2
//      <spare>                 3-255

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DetachTypeValue {
    Reserved,
    #[default]
    PsDetach,
    CombinedPsCsDetach,
    Spare,
}

impl From<&DetachTypeValue> for u8 {
    fn from(i: &DetachTypeValue) -> u8 {
        match i {
            DetachTypeValue::Reserved => 0,
            DetachTypeValue::PsDetach => 1,
            DetachTypeValue::CombinedPsCsDetach => 2,
            DetachTypeValue::Spare => 3,
        }
    }
}

impl From<u8> for DetachTypeValue {
    fn from(i: u8) -> DetachTypeValue {
        match i {
            0 => DetachTypeValue::Reserved,
            1 => DetachTypeValue::PsDetach,
            2 => DetachTypeValue::CombinedPsCsDetach,
            _ => DetachTypeValue::Spare,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetachType {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub detach_type: DetachTypeValue,
}

impl Default for DetachType {
    fn default() -> Self {
        DetachType {
            t: DETACHTYPE,
            length: DETACHTYPE_LENGTH as u16,
            ins: 0,
            detach_type: DetachTypeValue::default(),
        }
    }
}

impl From<DetachType> for InformationElement {
    fn from(i: DetachType) -> Self {
        InformationElement::DetachType(i)
    }
}

impl IEs for DetachType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(DETACHTYPE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.detach_type));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + DETACHTYPE_LENGTH {
            let data = DetachType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                detach_type: buffer[4].into(),
                ..DetachType::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(DETACHTYPE))
        }
    }

    fn len(&self) -> usize {
        DETACHTYPE_LENGTH + MIN_IE_SIZE
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
fn detach_type_ie_marshal_test() {
    let encoded: [u8; 5] = [0x96, 0x00, 0x01, 0x00, 0x02];
    let decoded = DetachType {
        t: DETACHTYPE,
        length: DETACHTYPE_LENGTH as u16,
        ins: 0,
        detach_type: DetachTypeValue::CombinedPsCsDetach,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn detach_type_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x96, 0x00, 0x01, 0x00, 0x02];
    let decoded = DetachType {
        t: DETACHTYPE,
        length: DETACHTYPE_LENGTH as u16,
        ins: 0,
        detach_type: DetachTypeValue::CombinedPsCsDetach,
    };
    assert_eq!(DetachType::unmarshal(&encoded).unwrap(), decoded);
}
