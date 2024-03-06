// Private Extension IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Private Extension IE Type

pub const PRIVATE_EXT: u8 = 255;

// Private Extension IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateExtension {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub enterprise_id: u16,
    pub value: Vec<u8>,
}

impl Default for PrivateExtension {
    fn default() -> Self {
        PrivateExtension {
            t: PRIVATE_EXT,
            length: 0,
            ins: 0,
            enterprise_id: 0,
            value: vec![],
        }
    }
}

impl From<PrivateExtension> for InformationElement {
    fn from(i: PrivateExtension) -> Self {
        InformationElement::PrivateExtension(i)
    }
}

impl IEs for PrivateExtension {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PRIVATE_EXT);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.enterprise_id.to_be_bytes());
        buffer_ie.append(&mut self.value.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 3 {
            let mut data = PrivateExtension {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..PrivateExtension::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.enterprise_id = u16::from_be_bytes([buffer[4], buffer[5]]);
                data.value
                    .extend_from_slice(&buffer[6..(data.length + 4) as usize]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(PRIVATE_EXT))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(PRIVATE_EXT))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn private_ext_ie_marshal_test() {
    let encoded: [u8; 7] = [0xff, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00];
    let decoded = PrivateExtension {
        t: PRIVATE_EXT,
        length: 3,
        ins: 0,
        enterprise_id: 0xaff,
        value: vec![0x00],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn private_ext_ie_unmarshal_test() {
    let encoded: [u8; 7] = [0xff, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00];
    let decoded = PrivateExtension {
        t: PRIVATE_EXT,
        length: 3,
        ins: 0,
        enterprise_id: 0xaff,
        value: vec![0x00],
    };
    assert_eq!(PrivateExtension::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn private_ext_ie_wrong_size_unmarshal_test() {
    let encoded: [u8; 6] = [0xff, 0x00, 0x02, 0x00, 0x0a, 0x00];
    assert_eq!(
        PrivateExtension::unmarshal(&encoded),
        Err(GTPV2Error::IEInvalidLength(PRIVATE_EXT))
    );
}
