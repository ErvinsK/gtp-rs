// Millisecond Timestamp IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Millisecond Timestamp IE Type

pub const MS_TIMESTAMP: u8 = 188;
pub const MS_TIMESTAMP_LENGTH: usize = 6;

// Millisecond Timestamp IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilliSecondTimeStamp {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub timestamp: u64, //  Timestamp represents a 48 bit unsigned integer in network order format and are encoded as the number of milliseconds since 00:00:00 January 1, 1900 00:00 UTC, i.e. as the rounded value of 1000 x the value of the 64-bit timestamp (Seconds  + (Fraction / (1<<32))) defined in section 6 of IETF RFC 5905 [53].
}

impl Default for MilliSecondTimeStamp {
    fn default() -> MilliSecondTimeStamp {
        MilliSecondTimeStamp {
            t: MS_TIMESTAMP,
            length: MS_TIMESTAMP_LENGTH as u16,
            ins: 0,
            timestamp: 0,
        }
    }
}

impl From<MilliSecondTimeStamp> for InformationElement {
    fn from(i: MilliSecondTimeStamp) -> Self {
        InformationElement::MilliSecondTimeStamp(i)
    }
}

impl IEs for MilliSecondTimeStamp {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MS_TIMESTAMP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&u64::to_be_bytes(self.timestamp)[2..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MS_TIMESTAMP_LENGTH + MIN_IE_SIZE {
            let data = MilliSecondTimeStamp {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                timestamp: u64::from_be_bytes([
                    0x00, 0x00, buffer[4], buffer[5], buffer[6], buffer[7], buffer[8], buffer[9],
                ]),
                ..MilliSecondTimeStamp::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MS_TIMESTAMP))
        }
    }

    fn len(&self) -> usize {
        MS_TIMESTAMP_LENGTH + MIN_IE_SIZE
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
fn ms_timestamp_ie_unmarshal_test() {
    let encoded: [u8; 10] = [0xbc, 0x00, 0x06, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let decoded = MilliSecondTimeStamp {
        t: MS_TIMESTAMP,
        length: MS_TIMESTAMP_LENGTH as u16,
        ins: 0,
        timestamp: 0xffffffffffff,
    };
    let i = MilliSecondTimeStamp::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn ms_timestamp_ie_marshal_test() {
    let encoded: [u8; 10] = [0xbc, 0x00, 0x06, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let decoded = MilliSecondTimeStamp {
        t: MS_TIMESTAMP,
        length: MS_TIMESTAMP_LENGTH as u16,
        ins: 0,
        timestamp: 0xffffffffffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
