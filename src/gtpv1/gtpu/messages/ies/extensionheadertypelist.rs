// Extension Header Type List IE - according to 3GPP TS 29.281 V16.0.0 (2019-12)

use crate::gtpv1::{errors::GTPV1Error, gtpu::messages::ies::commons::*};

// Extension Header Type List IE Type

pub const EXTENSION_HEADER_TYPE_LIST: u8 = 141;

// Extension Header Type List IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionHeaderTypeList {
    pub t: u8,
    pub length: u8,
    pub list: Vec<u8>,
}

impl Default for ExtensionHeaderTypeList {
    fn default() -> ExtensionHeaderTypeList {
        ExtensionHeaderTypeList {
            t: EXTENSION_HEADER_TYPE_LIST,
            length: 0,
            list: vec![],
        }
    }
}

impl IEs for ExtensionHeaderTypeList {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.push(self.length);
        buffer_ie.append(&mut self.list.clone());
        buffer_ie[1] = (buffer_ie.len() - 2) as u8;
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<ExtensionHeaderTypeList, GTPV1Error> {
        let mut data = ExtensionHeaderTypeList {
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length + 2) as usize <= buffer.len() {
            data.list.extend_from_slice(&buffer[2..]);
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 2) as usize
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

#[test]
fn extension_header_type_list_ie_marshal_test() {
    let ie_to_marshal = ExtensionHeaderTypeList {
        t: EXTENSION_HEADER_TYPE_LIST,
        length: 5,
        list: vec![0x00, 0x01, 0x02, 0x03, 0x04],
    };
    let ie_marshalled: [u8; 7] = [0x8d, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn extension_header_type_list_ie_unmarshal_test() {
    let ie_unmarshalled = ExtensionHeaderTypeList {
        t: EXTENSION_HEADER_TYPE_LIST,
        length: 5,
        list: vec![0x00, 0x01, 0x02, 0x03, 0x04],
    };
    let ie_to_unmarshal: [u8; 7] = [0x8d, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04];
    assert_eq!(
        ExtensionHeaderTypeList::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
