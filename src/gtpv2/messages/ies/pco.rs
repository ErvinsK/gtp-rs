// PCO IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// PCO IE Type

pub const PCO: u8 = 78;

// PCO IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pco {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pco: Vec<u8>,
}

impl Default for Pco {
    fn default() -> Self {
        Pco {
            t: PCO,
            length: 0,
            ins: 0,
            pco: vec![],
        }
    }
}

impl From<Pco> for InformationElement {
    fn from(i: Pco) -> Self {
        InformationElement::Pco(i)
    }
}

impl IEs for Pco {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PCO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.pco[..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Pco {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Pco::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.pco
                    .extend_from_slice(&buffer[4..(data.length + 4) as usize]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(PCO))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(PCO))
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
fn pco_ie_marshal_test() {
    let encoded: [u8; 24] = [
        0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Pco {
        t: PCO,
        length: 20,
        ins: 0,
        pco: vec![
            0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
            0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pco_ie_unmarshal_test() {
    let encoded: [u8; 24] = [
        0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Pco {
        t: PCO,
        length: 20,
        ins: 0,
        pco: vec![
            0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
            0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    assert_eq!(Pco::unmarshal(&encoded).unwrap(), decoded);
}
