// Evolved Allocation/Retention I IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Evolved Allocation/Retention I IE TVL

pub const EVOLVEDALLOCRETENTIONI: u8 = 191;
pub const EVOLVEDALLOCRETENTIONI_LENGTH: u16 = 1;

// Evolved Allocation/Retention I IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolvedAllocationRetentionI {
    pub t: u8,
    pub length: u16,
    pub pre_emption_vulnerability: u8,
    pub priority_level: u8,
    pub pre_emption_capability: u8,
}

impl Default for EvolvedAllocationRetentionI {
    fn default() -> EvolvedAllocationRetentionI {
        EvolvedAllocationRetentionI {
            t: EVOLVEDALLOCRETENTIONI,
            length: EVOLVEDALLOCRETENTIONI_LENGTH,
            pre_emption_vulnerability: 0,
            priority_level: 0,
            pre_emption_capability: 0,
        }
    }
}

impl IEs for EvolvedAllocationRetentionI {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(
            (self.pre_emption_capability << 6)
                | (self.priority_level << 2)
                | self.pre_emption_vulnerability,
        );
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= EVOLVEDALLOCRETENTIONI_LENGTH as usize + 3 {
            let data = EvolvedAllocationRetentionI {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                pre_emption_capability: buffer[3] >> 6 & 0x01,
                priority_level: buffer[3] >> 2 & 0x0f,
                pre_emption_vulnerability: buffer[3] & 0x01,
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        EVOLVEDALLOCRETENTIONI_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn evolvedallocretentioni_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xbf, 0x00, 0x01, 0x64];
    let test_struct = EvolvedAllocationRetentionI {
        t: EVOLVEDALLOCRETENTIONI,
        length: EVOLVEDALLOCRETENTIONI_LENGTH,
        pre_emption_capability: 1,
        priority_level: 9,
        pre_emption_vulnerability: 0,
    };
    let i = EvolvedAllocationRetentionI::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn evolvedallocretentioni_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xbf, 0x00, 0x01, 0x64];
    let test_struct = EvolvedAllocationRetentionI {
        t: EVOLVEDALLOCRETENTIONI,
        length: EVOLVEDALLOCRETENTIONI_LENGTH,
        pre_emption_capability: 1,
        priority_level: 9,
        pre_emption_vulnerability: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
