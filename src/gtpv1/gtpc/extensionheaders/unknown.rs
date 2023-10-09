use crate::gtpv1::{errors::GTPV1Error, gtpc::extensionheaders::commons::*};

// Struct for Unknow Extension Headers

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unknown {
    pub extension_header_type: u8,
    pub length: u8,
    pub value: Vec<u8>,
}

impl Default for Unknown {
    fn default() -> Unknown {
        Unknown {
            extension_header_type: 0xff,
            length: 0,
            value: vec![],
        }
    }
}

impl ExtensionHeaders for Unknown {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.append(&mut self.value.clone());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = Unknown {
            extension_header_type: buffer[0],
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.value
                .extend_from_slice(&buffer[2..(data.length * 4) as usize]);
            Ok(data)
        } else {
            Err(GTPV1Error::ExtHeaderInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length * 4) as usize
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn unknown_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xfa, 0x01, 0xff, 0xff];
    let test_struct = Unknown {
        extension_header_type: 0xfa,
        length: 1,
        value: vec![0xff, 0xff],
    };
    let i = Unknown::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn unknown_ind_exthdr_marshal_test() {
    let encoded_ie: [u8; 4] = [0xfa, 0x01, 0xff, 0xff];
    let test_struct = Unknown {
        extension_header_type: 0xfa,
        length: 1,
        value: vec![0xff, 0xff],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
