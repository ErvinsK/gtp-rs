// Recovery IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpu::messages::ies::commons::*};

// Recovery IE TV

pub const RECOVERY_LENGTH: usize = 1;
pub const RECOVERY: u8 = 14;

// Recovery IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recovery {
    pub t: u8,
    pub value: u8,
}

impl Default for Recovery {
    fn default() -> Recovery {
        Recovery {
            t: RECOVERY,
            value: 0,
        }
    }
}

impl IEs for Recovery {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(RECOVERY);
        buffer.push(self.value);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Recovery, GTPV1Error> {
        if buffer.len() > RECOVERY_LENGTH {
            Ok(Recovery {
                t: buffer[0],
                value: buffer[1],
            })
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        RECOVERY_LENGTH + 1
    }

    fn is_empty(&self) -> bool {
        self.value == 0
    }
}

#[test]
fn recovery_ie_marshal_test() {
    let ie_marshalled: [u8; 2] = [0x0e, 0x63];
    let ie_to_marshal = Recovery {
        t: RECOVERY,
        value: 0x63,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn recovery_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 2] = [0x0e, 0x63];
    let ie_unmarshalled = Recovery {
        t: RECOVERY,
        value: 0x63,
    };
    assert_eq!(
        Recovery::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
