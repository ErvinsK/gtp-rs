// Reordering Required IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Reordering Required IE TL

pub const REORDERING_REQUIRED: u8 = 8;
pub const REORDERING_REQUIRED_LENGTH: usize = 1;

// Reordering Required IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct ReorderingRequired {
    pub t: u8,
    pub req: bool,
}

impl Default for ReorderingRequired {
    fn default() -> Self {
        ReorderingRequired {
            t: REORDERING_REQUIRED,
            req: false,
        }
    }
}

impl IEs for ReorderingRequired {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        match self.req {
            true => buffer.push(0xff),
            false => buffer.push(0xfe),
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() > REORDERING_REQUIRED_LENGTH {
            let data = ReorderingRequired {
                req: !matches!(buffer[1] & 1, 0),
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        REORDERING_REQUIRED_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn reordering_required_ie_marshal_test() {
    let ie_to_marshal = ReorderingRequired {
        t: REORDERING_REQUIRED,
        req: true,
    };
    let ie_marshalled: [u8; 2] = [0x08, 0xff];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn reordering_required_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 2] = [0x08, 0xff];
    let ie_unmarshalled = ReorderingRequired {
        t: REORDERING_REQUIRED,
        req: true,
    };
    assert_eq!(
        ReorderingRequired::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
