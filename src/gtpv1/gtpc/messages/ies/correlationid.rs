// Correlation-ID IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Correlation-ID IE TV

pub const CORRELATIONID: u8 = 183;
pub const CORRELATIONID_LENGTH: u16 = 1;

// Correlation-ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorrelationId {
    pub t: u8,
    pub length: u16,
    pub correlation_id: u8,
}

impl Default for CorrelationId {
    fn default() -> CorrelationId {
        CorrelationId {
            t: CORRELATIONID,
            length: CORRELATIONID_LENGTH,
            correlation_id: 0,
        }
    }
}

impl IEs for CorrelationId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.correlation_id);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= (CORRELATIONID_LENGTH + 3) as usize {
            let data = CorrelationId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                correlation_id: buffer[3],
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        CORRELATIONID_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn correlationid_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xb7, 0x00, 0x01, 0xff];
    let test_struct = CorrelationId {
        t: CORRELATIONID,
        length: CORRELATIONID_LENGTH,
        correlation_id: 0xff,
    };
    let i = CorrelationId::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn correlationid_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xb7, 0x00, 0x01, 0xff];
    let test_struct = CorrelationId {
        t: CORRELATIONID,
        length: CORRELATIONID_LENGTH,
        correlation_id: 0xff,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
