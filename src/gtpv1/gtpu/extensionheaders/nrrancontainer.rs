use crate::gtpv1::{errors::GTPV1Error, gtpu::extensionheaders::commons::*};

pub const NR_RAN_CONTAINER: u8 = 0x84;
pub const NR_RAN_CONTAINER_LENGTH: u8 = 1;

// Struct for NR RAN Container Extension Header

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NrRanContainer {
    pub extension_header_type: u8,
    pub length: u8,
    pub container: Vec<u8>,
}

impl Default for NrRanContainer {
    fn default() -> NrRanContainer {
        NrRanContainer {
            extension_header_type: NR_RAN_CONTAINER,
            length: NR_RAN_CONTAINER_LENGTH,
            container: vec![],
        }
    }
}

impl ExtensionHeaders for NrRanContainer {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.append(&mut self.container.clone());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = NrRanContainer {
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.container
                .extend_from_slice(&buffer[2..((data.length * 4) as usize)]);
            Ok(data)
        } else {
            Err(GTPV1Error::ExtHeaderInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length * 4) as usize
    }

    fn is_empty(&self) -> bool {
        self.container.is_empty()
    }
}

#[test]
fn nr_ran_container_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 8] = [0x84, 0x02, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let test_struct = NrRanContainer {
        extension_header_type: NR_RAN_CONTAINER,
        length: 2,
        container: vec![0, 1, 2, 3, 4, 5],
    };
    let i = NrRanContainer::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn nr_ran_container_exthdr_marshal_test() {
    let encoded_ie: [u8; 8] = [0x84, 0x02, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let test_struct = NrRanContainer {
        extension_header_type: NR_RAN_CONTAINER,
        length: 2,
        container: vec![0, 1, 2, 3, 4, 5],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
