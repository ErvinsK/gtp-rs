// Cause IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Cause IE TL

pub const CAUSE:u8 = 1;
pub const CAUSE_LENGTH:usize = 1;

// Cause IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct Cause {
    pub t:u8,
    pub value:u8,
}

impl Default for Cause {
    fn default() -> Self {
        Cause { t: CAUSE, value: 234 }
    }
}

impl IEs for Cause {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.value);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len()>=CAUSE_LENGTH+1 {
            let mut data=Cause::default();
            data.value = buffer[1];
            Ok(data) 
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
        CAUSE_LENGTH+1
    }
}

#[test]
fn cause_ie_marshal_test() {
    let ie_to_marshal=Cause{ t: CAUSE, value:128};
    let ie_marshalled:[u8;2]=[0x01, 0x80];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn cause_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;2]=[0x01, 0x80];
    let ie_unmarshalled = Cause { t: CAUSE, value:128};
    assert_eq!(Cause::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}