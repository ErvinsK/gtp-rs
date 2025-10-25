// Throttling IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Throttling IE Type

pub const THROTTLING: u8 = 154;
pub const THROTTLING_LENGTH: usize = 2;

// Throttling IE implementation
// Delay value represent the binary coded timer value.
// Delay unit defines the timer unit for the timer as follows:
//
// 0 0 0  value is incremented in multiples of 2 seconds
// 0 0 1  value is incremented in multiples of 1 minute
// 0 1 0  value is incremented in multiples of 10 minutes
// 0 1 1  value is incremented in multiples of 1 hour
// 1 0 0  value is incremented in multiples of 10 hours
// 1 1 1  value indicates that the timer is deactivated.
//
// Other values shall be interpreted as multiples of 1 minute.
//
// Factor = Throttling Factor indicates a percentage and may take binary coded integer values from and including 0 up to and including 100. Other values shall be considered as 0.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Throttling {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub delay_unit: u8,
    pub delay_value: u8,
    pub factor: u8,
}

impl Default for Throttling {
    fn default() -> Self {
        Throttling {
            t: THROTTLING,
            length: THROTTLING_LENGTH as u16,
            ins: 0,
            delay_unit: 0,
            delay_value: 0,
            factor: 0,
        }
    }
}

impl From<Throttling> for InformationElement {
    fn from(i: Throttling) -> Self {
        InformationElement::Throttling(i)
    }
}

impl IEs for Throttling {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(THROTTLING);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push((self.delay_unit << 5) | self.delay_value);
        buffer_ie.push(self.factor);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + THROTTLING_LENGTH {
            let mut data = Throttling {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Throttling::default()
            };
            match buffer[4] >> 5 {
                i if i < 5 => data.delay_unit = buffer[4] >> 5,
                7 => data.delay_unit = 0,
                _ => data.delay_unit = 1,
            }
            data.delay_value = buffer[4] & 0x1f;
            match buffer[5] {
                i if i < 101 => data.factor = buffer[5],
                _ => data.factor = 0,
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(THROTTLING))
        }
    }

    fn len(&self) -> usize {
        THROTTLING_LENGTH + MIN_IE_SIZE
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
fn throttling_ie_marshal_test() {
    let encoded: [u8; 6] = [0x9a, 0x00, 0x02, 0x00, 0x65, 0x64];
    let decoded = Throttling {
        t: THROTTLING,
        length: THROTTLING_LENGTH as u16,
        ins: 0,
        delay_unit: 3,
        delay_value: 5,
        factor: 100,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn throttling_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x9a, 0x00, 0x02, 0x00, 0x65, 0x64];
    let decoded = Throttling {
        t: THROTTLING,
        length: THROTTLING_LENGTH as u16,
        ins: 0,
        delay_unit: 3,
        delay_value: 5,
        factor: 100,
    };
    assert_eq!(Throttling::unmarshal(&encoded).unwrap(), decoded);
}
