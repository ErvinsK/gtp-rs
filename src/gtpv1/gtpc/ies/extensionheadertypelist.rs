// Extension Header Type List IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::gtpc::ies::commons::*;

// Extension Header Type List IE Type

pub const EXTENSION_HEADER_TYPE_LIST:u8 = 141;

// Extension Header Type List IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionHeaderTypeList {
    pub t:u8,
    pub length:u8,
    pub list:Vec<u8>,
}

impl Default for ExtensionHeaderTypeList {
    fn default() -> ExtensionHeaderTypeList {
        ExtensionHeaderTypeList {
            t:EXTENSION_HEADER_TYPE_LIST,
            length:0,
            list:vec!(),
        }
    }
}

impl IEs for ExtensionHeaderTypeList {

    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.length);
        for i in self.list.iter() {
            buffer.push(*i);
        }
    }

    fn unmarshal(buffer: &[u8]) -> Option<ExtensionHeaderTypeList> {
        if buffer.len()>=2 {
            let mut data = ExtensionHeaderTypeList::default();
            data.length = buffer[1];
            if data.length*3+2>(buffer.len()-2) as u8 {
                return None;
            }
            data.list.extend_from_slice(&buffer[2..]);
            Some(data)
        } else {
            None
        }
                
    }

    fn len(&self) -> usize {
        (self.length+2) as usize
    }
}

#[test]
fn extension_header_type_list_ie_marshal_test() {
    let ie_to_marshal = ExtensionHeaderTypeList { t: EXTENSION_HEADER_TYPE_LIST, length:1, list: vec![ 0x00, 0x01, 0x02, 0x03, 0x04]};
    let ie_marshalled:[u8;7] = [0x8d, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn extension_header_type_list_ie_unmarshal_test() {
    let ie_unmarshalled = ExtensionHeaderTypeList { t: EXTENSION_HEADER_TYPE_LIST, length:1, list: vec![ 0x00, 0x01, 0x02, 0x03, 0x04]};
    let ie_to_unmarshal:[u8;7] = [0x8d, 0x01, 0x00, 0x01, 0x02, 0x03, 0x04];
    assert_eq!(ExtensionHeaderTypeList::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}