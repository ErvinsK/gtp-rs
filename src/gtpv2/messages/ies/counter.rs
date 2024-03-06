// Counter IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Counter IE Type

pub const COUNTER: u8 = 199;
pub const COUNTER_LENGTH: usize = 5;

// Counter IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub timestamp: u32, //  Epoch Era 0 - 00:00:00 on January 1, 1900
    pub counter: u8,
}

impl Default for Counter {
    fn default() -> Self {
        Counter {
            t: COUNTER,
            length: COUNTER_LENGTH as u16,
            ins: 0,
            timestamp: 0,
            counter: 0,
        }
    }
}

impl From<Counter> for InformationElement {
    fn from(i: Counter) -> Self {
        InformationElement::Counter(i)
    }
}

impl IEs for Counter {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(COUNTER);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.timestamp.to_be_bytes());
        buffer_ie.push(self.counter);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= COUNTER_LENGTH + MIN_IE_SIZE {
            let data = Counter {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                timestamp: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                counter: buffer[8],
                ..Counter::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(COUNTER))
        }
    }

    fn len(&self) -> usize {
        COUNTER_LENGTH + MIN_IE_SIZE
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
fn counter_ie_unmarshal_test() {
    let encoded: [u8; 9] = [0xc7, 0x00, 0x05, 0x00, 0xee, 0x6b, 0x28, 0x00, 0x09];
    let decoded = Counter {
        t: COUNTER,
        length: COUNTER_LENGTH as u16,
        ins: 0,
        timestamp: 4000000000,
        counter: 9,
    };
    let i = Counter::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn counter_ie_marshal_test() {
    let encoded: [u8; 9] = [0xc7, 0x00, 0x05, 0x00, 0xee, 0x6b, 0x28, 0x00, 0x09];
    let decoded = Counter {
        t: COUNTER,
        length: COUNTER_LENGTH as u16,
        ins: 0,
        timestamp: 4000000000,
        counter: 9,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
