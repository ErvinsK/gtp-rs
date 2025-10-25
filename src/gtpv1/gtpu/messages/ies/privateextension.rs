// Private Extension IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpu::messages::ies::commons::*, utils::*};

// Private Extension IE type

pub const PRIVATE_EXTENSION: u8 = 255;

// Private Extension IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateExtension {
    pub t: u8,
    pub length: u16,
    pub extension_id: u16,
    pub extension_value: Vec<u8>,
}

impl Default for PrivateExtension {
    fn default() -> PrivateExtension {
        PrivateExtension {
            t: PRIVATE_EXTENSION,
            length: 0,
            extension_id: 0,
            extension_value: vec![],
        }
    }
}

impl IEs for PrivateExtension {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.extend_from_slice(&self.extension_id.to_be_bytes());
        buffer_ie.append(&mut self.extension_value.clone());
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<PrivateExtension, GTPV1Error> {
        if buffer.len() >= 3 {
            let mut data = PrivateExtension {
                length: match u16::from_be_bytes([buffer[1], buffer[2]]) {
                    0 => return Err(GTPV1Error::IEInvalidLength),
                    _ => u16::from_be_bytes([buffer[1], buffer[2]]),
                },
                ..Default::default()
            };
            if check_tlv_ie_buffer(data.length, buffer) {
                data.extension_id = u16::from_be_bytes([buffer[3], buffer[4]]);
                data.extension_value.extend_from_slice(&buffer[5..]);
                Ok(data)
            } else {
                Err(GTPV1Error::IEInvalidLength)
            }
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 3) as usize
    }

    fn is_empty(&self) -> bool {
        self.extension_value.is_empty()
    }
}

#[test]
fn private_extension_ie_marshal_test() {
    let ie_to_marshal = PrivateExtension {
        t: PRIVATE_EXTENSION,
        length: 5,
        extension_id: 8,
        extension_value: vec![1, 2, 3],
    };
    let ie_marshalled: [u8; 8] = [0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn private_extension_ie_unmarshal_test() {
    let ie_unmarshalled = PrivateExtension {
        t: PRIVATE_EXTENSION,
        length: 5,
        extension_id: 8,
        extension_value: vec![1, 2, 3],
    };
    let ie_to_unmarshal: [u8; 8] = [0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    assert_eq!(
        PrivateExtension::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
