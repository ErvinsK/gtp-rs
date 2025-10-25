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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum IndicationValues {
    #[default]
    NoAction,
    DeactivationIndication,
    PagingIndication,
    PagingStopIndication,
    Spare,
}

impl From<&IndicationValues> for u8 {
    fn from(i: &IndicationValues) -> u8 {
        match i {
            IndicationValues::NoAction => 0,
            IndicationValues::DeactivationIndication => 1,
            IndicationValues::PagingIndication => 2,
            IndicationValues::PagingStopIndication => 3,
            IndicationValues::Spare => 4,
        }
    }
}

impl From<u8> for IndicationValues {
    fn from(i: u8) -> IndicationValues {
        match i {
            0 => IndicationValues::NoAction,
            1 => IndicationValues::DeactivationIndication,
            2 => IndicationValues::PagingIndication,
            3 => IndicationValues::PagingStopIndication,
            4..=7 => IndicationValues::Spare,
            _ => IndicationValues::NoAction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionIndication {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub indication: IndicationValues,
}

impl Default for ActionIndication {
    fn default() -> ActionIndication {
        ActionIndication {
            t: ACTION_IND,
            length: ACTION_IND_LENGTH as u16,
            ins: 0,
            indication: IndicationValues::NoAction,
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
        buffer_ie.push(u8::from(&self.indication));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= ACTION_IND_LENGTH + MIN_IE_SIZE {
            let data = ActionIndication {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                indication: (buffer[4] & 0x07).into(),
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
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn action_indication_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xa8, 0x00, 0x01, 0x00, 0x02];
    let decoded = ActionIndication {
        t: ACTION_IND,
        length: ACTION_IND_LENGTH as u16,
        ins: 0,
        indication: IndicationValues::PagingIndication,
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
        indication: IndicationValues::PagingIndication,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
