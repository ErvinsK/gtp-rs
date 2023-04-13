// TEID IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// TEID IE TL

pub const TEID_DATA: u8 = 16;
pub const TEID_CONTROL: u8 = 17;
pub const TEID_LENGTH: usize = 4;

// TEID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Teid {
    pub t: u8,
    pub teid: u32,
}

impl Default for Teid {
    fn default() -> Teid {
        Teid {
            t: TEID_DATA,
            teid: 0,
        }
    }
}

impl IEs for Teid {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.teid.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Teid, GTPV1Error> {
        if buffer.len() > TEID_LENGTH {
            let data = Teid {
                t: buffer[0],
                teid: u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]),
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        TEID_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn teid_ie_marshal_test() {
    let ie_marshalled: [u8; 5] = [0x10, 0x63, 0x41, 0xaf, 0xd7];
    let ie_to_marshal = Teid {
        t: TEID_DATA,
        teid: 0x6341afd7,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn teid_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 5] = [0x11, 0x63, 0x41, 0xaf, 0xd7];
    let ie_unmarshalled = Teid {
        t: TEID_CONTROL,
        teid: 0x6341afd7,
    };
    assert_eq!(Teid::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}
