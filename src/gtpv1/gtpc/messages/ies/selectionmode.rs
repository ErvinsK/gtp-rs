// Selection Mode IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Selection Mode IE TL

pub const SELECTION_MODE: u8 = 15;
pub const SELECTION_MODE_LENGTH: usize = 1;

// Selection Mode IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionMode {
    pub t: u8,
    pub value: u8,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode {
            t: SELECTION_MODE,
            value: 0,
        }
    }
}

impl IEs for SelectionMode {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(0b11111100 | self.value);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() > SELECTION_MODE_LENGTH {
            let data = SelectionMode {
                value: buffer[1] & 0b11,
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        SELECTION_MODE_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn selectionmode_ie_marshal_test() {
    let ie_to_marshal = SelectionMode {
        t: SELECTION_MODE,
        value: 2,
    };
    let ie_marshalled: [u8; 2] = [0x0f, 0xfe];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn selectionmode_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 2] = [0x0f, 0xfc];
    let ie_unmarshalled = SelectionMode {
        t: SELECTION_MODE,
        value: 0,
    };
    assert_eq!(
        SelectionMode::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
