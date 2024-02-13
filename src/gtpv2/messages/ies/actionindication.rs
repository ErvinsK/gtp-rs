// Action Indication IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Action Indication IE TV

pub const ACTION_IND: u8 = 168;
pub const ACTION_IND_LENGTH: usize = 1;

// Action Indication IE implementation

//     Indication               Values (Decimal)
//      No Action                    0
//  Deactivation Indication          1
//   Paging Indication               2
// Paging Stop Indication            3
//       <spare>                   4 to 7

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionIndication {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub indication: u8,
}

impl Default for ActionIndication {
    fn default() -> ActionIndication {
        ActionIndication {
            t: ACTION_IND,
            length: ACTION_IND_LENGTH as u16,
            ins: 0,
            indication: 0,
        }
    }
}

impl From<ActionIndication> for InformationElement {
    fn from(i: ActionIndication) -> Self {
        InformationElement::ActionIndication(i)
    }
}

impl IEs for ActionIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ACTION_IND);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.indication);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= ACTION_IND_LENGTH + MIN_IE_SIZE {
            let data = ActionIndication {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                indication: buffer[4] & 0x07,
                ..ActionIndication::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ACTION_IND))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn action_indication_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xa8, 0x00, 0x01, 0x00, 0x02];
    let decoded = ActionIndication {
        t: ACTION_IND,
        length: ACTION_IND_LENGTH as u16,
        ins: 0,
        indication: 0x02,
    };
    let i = ActionIndication::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn action_indication_ie_marshal_test() {
    let encoded: [u8; 5] = [0xa8, 0x00, 0x01, 0x00, 0x02];
    let decoded = ActionIndication {
        t: ACTION_IND,
        length: ACTION_IND_LENGTH as u16,
        ins: 0,
        indication: 0x02,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
