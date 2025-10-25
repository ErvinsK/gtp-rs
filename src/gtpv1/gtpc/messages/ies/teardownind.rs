// Teardown Ind IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Teardown Ind IE TV

pub const TEARDOWN_IND: u8 = 19;
pub const TEARDOWN_IND_LENGTH: u16 = 1;

// Teardown Ind IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeardownInd {
    pub t: u8,
    pub teardown: bool, // Teardown Ind
}

impl Default for TeardownInd {
    fn default() -> TeardownInd {
        TeardownInd {
            t: TEARDOWN_IND,
            teardown: true,
        }
    }
}

impl IEs for TeardownInd {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        match self.teardown {
            false => buffer.push(0xfe),
            true => buffer.push(0xff),
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= (TEARDOWN_IND_LENGTH + 1) as usize {
            let data = TeardownInd {
                teardown: match buffer[1] {
                    0xfe => false,
                    0xff => true,
                    _ => return Err(GTPV1Error::IEIncorrect),
                },
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        TEARDOWN_IND_LENGTH as usize + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn teardown_ind_ie_unmarshal_test() {
    let encoded_ie: [u8; 2] = [0x13, 0xff];
    let test_struct = TeardownInd {
        t: TEARDOWN_IND,
        teardown: true,
    };
    let i = TeardownInd::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn teardown_ind_ie_marshal_test() {
    let encoded_ie: [u8; 2] = [0x13, 0xfe];
    let test_struct = TeardownInd {
        t: TEARDOWN_IND,
        teardown: false,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
