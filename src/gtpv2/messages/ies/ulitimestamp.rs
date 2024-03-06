// ULI Timestamp IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// ULI Timestamp IE Type

pub const ULI_TIMESTAMP: u8 = 170;
pub const ULI_TIMESTAMP_LENGTH: usize = 4;

// ULI Timestamp IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UliTimestamp {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub timestamp: u32, //  Epoch Era 0 - 00:00:00 on January 1, 1900
}

impl Default for UliTimestamp {
    fn default() -> UliTimestamp {
        UliTimestamp {
            t: ULI_TIMESTAMP,
            length: ULI_TIMESTAMP_LENGTH as u16,
            ins: 0,
            timestamp: 0,
        }
    }
}

impl From<UliTimestamp> for InformationElement {
    fn from(i: UliTimestamp) -> Self {
        InformationElement::UliTimestamp(i)
    }
}

impl IEs for UliTimestamp {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ULI_TIMESTAMP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&u32::to_be_bytes(self.timestamp));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= ULI_TIMESTAMP_LENGTH + MIN_IE_SIZE {
            let data = UliTimestamp {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                timestamp: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..UliTimestamp::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ULI_TIMESTAMP))
        }
    }

    fn len(&self) -> usize {
        ULI_TIMESTAMP_LENGTH + MIN_IE_SIZE
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
fn uli_timestamp_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0xaa, 0x00, 0x04, 0x00, 0xee, 0x6b, 0x28, 0x00];
    let decoded = UliTimestamp {
        t: ULI_TIMESTAMP,
        length: ULI_TIMESTAMP_LENGTH as u16,
        ins: 0,
        timestamp: 4000000000,
    };
    let i = UliTimestamp::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn uli_timestamp_ie_marshal_test() {
    let encoded: [u8; 8] = [0xaa, 0x00, 0x04, 0x00, 0xee, 0x6b, 0x28, 0x00];
    let decoded = UliTimestamp {
        t: ULI_TIMESTAMP,
        length: ULI_TIMESTAMP_LENGTH as u16,
        ins: 0,
        timestamp: 4000000000,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
