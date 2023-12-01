use crate::gtpv1::{errors::GTPV1Error, gtpu::extensionheaders::commons::*};

pub const SCI: u8 = 0x20;
pub const SCI_LENGTH: u8 = 1;

// Struct for Service Class Indicator Extension Header

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sci {
    pub extension_header_type: u8,
    pub length: u8,
    pub sci: u8,
}

impl Default for Sci {
    fn default() -> Sci {
        Sci {
            extension_header_type: SCI,
            length: SCI_LENGTH,
            sci: 0,
        }
    }
}

impl ExtensionHeaders for Sci {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.push(self.sci);
        buffer.push(0x00);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = Sci {
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.sci = buffer[2];
            Ok(data)
        } else {
            Err(GTPV1Error::ExtHeaderInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length * 4) as usize
    }

    fn is_empty(&self) -> bool {
        self.sci == 0
    }
}

#[test]
fn sci_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0x20, 0x01, 0x09, 0x00];
    let test_struct = Sci {
        extension_header_type: SCI,
        length: SCI_LENGTH,
        sci: 9,
    };
    let i = Sci::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn sci_exthdr_marshal_test() {
    let encoded_ie: [u8; 4] = [0x20, 0x01, 0x09, 0x00];
    let test_struct = Sci {
        extension_header_type: SCI,
        length: SCI_LENGTH,
        sci: 9,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
