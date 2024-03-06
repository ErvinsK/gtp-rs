// Unknown IE - for internal message handling purposes

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Unknown IE implementation

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Unknown {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub value: Vec<u8>,
}

impl From<Unknown> for InformationElement {
    fn from(i: Unknown) -> Self {
        InformationElement::Unknown(i)
    }
}

impl IEs for Unknown {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.value.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Unknown {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            if check_tliv_ie_buffer(data.length, buffer) {
                data.value
                    .extend_from_slice(&buffer[4..(data.length + 4) as usize]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(data.t))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(buffer[0]))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
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
fn unknown_ie_marshal_test() {
    let decoded = Unknown {
        t: 0,
        length: 3,
        ins: 0,
        value: vec![0x00, 0x0f, 0xff],
    };
    let encoded: [u8; 7] = [0x00, 0x00, 0x03, 0x00, 0x00, 0x0f, 0xff];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn unknown_ie_unmarshal_test() {
    let decoded = Unknown {
        t: 0,
        length: 3,
        ins: 0,
        value: vec![0x00, 0x0f, 0xff],
    };
    let encoded: [u8; 7] = [0x00, 0x00, 0x03, 0x00, 0x00, 0x0f, 0xff];
    assert_eq!(Unknown::unmarshal(&encoded).unwrap(), decoded);
}
