// Transaction Identifier (TI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Transaction Identifier (TI) IE TL

pub const TRANSACT_ID: u8 = 137;

// Transaction Identifier (TI) struct according to 3GPP TS 24.007

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionId {
    pub flag: bool, // Flag "false" = 0 for connection initiated by the sender, "true" = 1 for connection initiated by the receiver
    pub id: u8, // For Ids < 7 - encoded as u8, but for ids > 7 - endoded within extension field according to 3GPP TS 24.007 11.2.3.1.3
}

// Transaction Identfier (TI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionIdentifier {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub tid: TransactionId,
}

impl Default for TransactionIdentifier {
    fn default() -> Self {
        TransactionIdentifier {
            t: TRANSACT_ID,
            length: 0,
            ins: 0,
            tid: TransactionId { flag: false, id: 0 },
        }
    }
}

impl From<TransactionIdentifier> for InformationElement {
    fn from(i: TransactionIdentifier) -> Self {
        InformationElement::TransactionIdentifier(i)
    }
}

impl IEs for TransactionIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TRANSACT_ID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let x = match self.tid.flag {
            false => 0x00,
            true => 0x80,
        };
        if self.tid.id < 7 {
            buffer_ie.push(x | ((self.tid.id << 4) & 0x70));
        } else {
            buffer_ie.push(x);
            buffer_ie.push(self.tid.id);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= (MIN_IE_SIZE + 1) {
            let mut data = TransactionIdentifier {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..TransactionIdentifier::default()
            };
            data.tid.flag = matches!(buffer[4] >> 7, 1);
            if data.length > 1 {
                if buffer.len() >= 0x06 {
                    data.tid.id = buffer[5];
                } else {
                    return Err(GTPV2Error::IEInvalidLength(TRANSACT_ID));
                }
            } else {
                data.tid.id = (buffer[4] >> 4) & 0x07;
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TRANSACT_ID))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn transaction_id_ie_marshal_test() {
    let decoded = TransactionIdentifier {
        t: TRANSACT_ID,
        length: 2,
        ins: 0,
        tid: TransactionId { flag: true, id: 7 },
    };
    let encoded: [u8; 6] = [0x89, 0x00, 0x02, 0x00, 0x80, 0x07];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn transaction_id_ie_unmarshal_test() {
    let decoded = TransactionIdentifier {
        t: TRANSACT_ID,
        length: 2,
        ins: 0,
        tid: TransactionId { flag: true, id: 7 },
    };
    let encoded: [u8; 6] = [0x89, 0x00, 0x02, 0x00, 0x80, 0x07];
    assert_eq!(TransactionIdentifier::unmarshal(&encoded).unwrap(), decoded);
}
