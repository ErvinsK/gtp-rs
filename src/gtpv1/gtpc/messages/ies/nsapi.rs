// NSAPI IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// NSAPI IE TL

pub const NSAPI: u8 = 20;
pub const NSAPI_LENGTH: usize = 1;

// NSAPI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Nsapi {
    pub t: u8,
    pub value: u8,
}

impl Default for Nsapi {
    fn default() -> Self {
        Nsapi { t: NSAPI, value: 0 }
    }
}

impl IEs for Nsapi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.value);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() > NSAPI_LENGTH {
            let data = Nsapi {
                value: buffer[1] & 0b1111,
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        NSAPI_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn nsapi_ie_marshal_test() {
    let ie_to_marshal = Nsapi { t: NSAPI, value: 5 };
    let ie_marshalled: [u8; 2] = [0x14, 0x05];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn nsapi_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 2] = [0x14, 0x05];
    let ie_unmarshalled = Nsapi { t: NSAPI, value: 5 };
    assert_eq!(Nsapi::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}
