// Trace Type IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::gtpc::ies::commons::*;

// Trace Type TL

pub const TRACE_TYPE:u8 = 28;
pub const TRACE_TYPE_LENGTH:usize = 2;

// Trace Type IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct TraceType {
    pub t:u8,
    pub value:u16, 
}

impl Default for TraceType {
    fn default() -> Self {
        TraceType { t: TRACE_TYPE, value: 0 }
    }
}

impl IEs for TraceType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.value.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=TRACE_TYPE_LENGTH+1 {
            let mut data=TraceType::default();
            data.value = u16::from_be_bytes([buffer[1],buffer[2]]);
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        TRACE_TYPE_LENGTH+1
    }
}

#[test]
fn trace_type_ie_marshal_test() {
    let ie_to_marshal=TraceType{ t: TRACE_TYPE, value:2};
    let ie_marshalled:[u8;3]=[0x1c, 0x00, 0x02];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn trace_type_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;3]=[0x1c, 0x00, 0x02];
    let ie_unmarshalled = TraceType { t: TRACE_TYPE, value:2};
    assert_eq!(TraceType::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}