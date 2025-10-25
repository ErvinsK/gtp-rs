// TWAN Identifier Timestamp IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// TWAN Identifier Timestamp IE Type

pub const TWAN_ID_TIMESTAMP: u8 = 179;
pub const TWAN_ID_TIMESTAMP_LENGTH: usize = 4;

// TWAN Identifier Timestamp IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwanIdTimeStamp {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub timestamp: u32, //  Epoch Era 0 - 00:00:00 on January 1, 1900
}

impl Default for TwanIdTimeStamp {
    fn default() -> TwanIdTimeStamp {
        TwanIdTimeStamp {
            t: TWAN_ID_TIMESTAMP,
            length: TWAN_ID_TIMESTAMP_LENGTH as u16,
            ins: 0,
            timestamp: 0,
        }
    }
}

impl From<TwanIdTimeStamp> for InformationElement {
    fn from(i: TwanIdTimeStamp) -> Self {
        InformationElement::TwanIdTimeStamp(i)
    }
}

impl IEs for TwanIdTimeStamp {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TWAN_ID_TIMESTAMP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&u32::to_be_bytes(self.timestamp));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= TWAN_ID_TIMESTAMP_LENGTH + MIN_IE_SIZE {
            let data = TwanIdTimeStamp {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                timestamp: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..TwanIdTimeStamp::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TWAN_ID_TIMESTAMP))
        }
    }

    fn len(&self) -> usize {
        TWAN_ID_TIMESTAMP_LENGTH + MIN_IE_SIZE
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
fn twan_id_timestamp_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0xb3, 0x00, 0x04, 0x00, 0xee, 0x6b, 0x28, 0x00];
    let decoded = TwanIdTimeStamp {
        t: TWAN_ID_TIMESTAMP,
        length: TWAN_ID_TIMESTAMP_LENGTH as u16,
        ins: 0,
        timestamp: 4000000000,
    };
    let i = TwanIdTimeStamp::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn twan_id_timestamp_ie_marshal_test() {
    let encoded: [u8; 8] = [0xb3, 0x00, 0x04, 0x00, 0xee, 0x6b, 0x28, 0x00];
    let decoded = TwanIdTimeStamp {
        t: TWAN_ID_TIMESTAMP,
        length: TWAN_ID_TIMESTAMP_LENGTH as u16,
        ins: 0,
        timestamp: 4000000000,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
