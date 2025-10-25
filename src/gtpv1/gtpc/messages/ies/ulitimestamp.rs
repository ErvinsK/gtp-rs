// ULI Timestamp IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// ULI Timestamp IE TL

pub const ULI_TIMESTAMP: u8 = 214;
pub const ULI_TIMESTAMP_LENGTH: u16 = 4;

// ULI Timestamp IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UliTimestamp {
    pub t: u8,
    pub length: u16,
    pub timestamp: u32, //  Epoch Era 0 - 00:00:00 on January 1, 1900
}

impl Default for UliTimestamp {
    fn default() -> UliTimestamp {
        UliTimestamp {
            t: ULI_TIMESTAMP,
            length: ULI_TIMESTAMP_LENGTH,
            timestamp: 0,
        }
    }
}

impl IEs for UliTimestamp {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.extend_from_slice(&u32::to_be_bytes(self.timestamp));
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= (ULI_TIMESTAMP_LENGTH + 3) as usize {
            let data = UliTimestamp {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                timestamp: u32::from_be_bytes([buffer[3], buffer[4], buffer[5], buffer[6]]),
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        ULI_TIMESTAMP_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn uli_timestamp_ie_unmarshal_test() {
    let encoded_ie: [u8; 7] = [0xd6, 0x00, 0x04, 0xee, 0x6b, 0x28, 0x00];
    let test_struct = UliTimestamp {
        t: ULI_TIMESTAMP,
        length: ULI_TIMESTAMP_LENGTH,
        timestamp: 4000000000,
    };
    let i = UliTimestamp::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn uli_timestamp_ie_marshal_test() {
    let encoded_ie: [u8; 7] = [0xd6, 0x00, 0x04, 0xee, 0x6b, 0x28, 0x00];
    let test_struct = UliTimestamp {
        t: ULI_TIMESTAMP,
        length: ULI_TIMESTAMP_LENGTH,
        timestamp: 4000000000,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
