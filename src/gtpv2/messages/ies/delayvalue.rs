// Delay Value IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Delay Value IE Type

pub const DELAY_VALUE: u8 = 92;
pub const DELAY_VALUE_LENGTH: usize = 1;

// Delay Value IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayValue {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub value: u8, // Delay Value in integer multiples of 50 millisecs, or zero
}

impl Default for DelayValue {
    fn default() -> Self {
        DelayValue {
            t: DELAY_VALUE,
            length: DELAY_VALUE_LENGTH as u16,
            ins: 0,
            value: 0,
        }
    }
}

impl From<DelayValue> for InformationElement {
    fn from(i: DelayValue) -> Self {
        InformationElement::DelayValue(i)
    }
}

impl IEs for DelayValue {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(DELAY_VALUE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.value);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + DELAY_VALUE_LENGTH {
            let data = DelayValue {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                value: buffer[4],
                ..DelayValue::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(DELAY_VALUE))
        }
    }

    fn len(&self) -> usize {
        DELAY_VALUE_LENGTH + MIN_IE_SIZE
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
fn delay_value_ie_marshal_test() {
    let encoded: [u8; 5] = [0x5c, 0x00, 0x01, 0x00, 0xff];
    let decoded = DelayValue {
        t: DELAY_VALUE,
        length: DELAY_VALUE_LENGTH as u16,
        ins: 0,
        value: 0xff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn delay_value_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x5c, 0x00, 0x01, 0x00, 0xff];
    let decoded = DelayValue {
        t: DELAY_VALUE,
        length: DELAY_VALUE_LENGTH as u16,
        ins: 0,
        value: 0xff,
    };
    assert_eq!(DelayValue::unmarshal(&encoded).unwrap(), decoded);
}
