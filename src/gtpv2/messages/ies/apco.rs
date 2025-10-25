// Additional PCO IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
// Additional Protocol Configuration Options information element is specified in 3GPP TS 29.275

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Additional PCO IE Type

pub const APCO: u8 = 163;

// Additional PCO IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Apco {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub apco: Vec<u8>,
}

impl Default for Apco {
    fn default() -> Self {
        Apco {
            t: APCO,
            length: 0,
            ins: 0,
            apco: vec![],
        }
    }
}

impl From<Apco> for InformationElement {
    fn from(i: Apco) -> Self {
        InformationElement::Apco(i)
    }
}

impl IEs for Apco {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(APCO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.apco.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Apco {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Apco::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.apco
                    .extend_from_slice(&buffer[MIN_IE_SIZE..MIN_IE_SIZE + data.length as usize]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(APCO))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(APCO))
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
fn apco_ie_marshal_test() {
    let encoded: [u8; 24] = [
        0xa3, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Apco {
        t: APCO,
        length: 20,
        ins: 0,
        apco: vec![
            0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
            0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn apco_ie_unmarshal_test() {
    let encoded: [u8; 24] = [
        0xa3, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Apco {
        t: APCO,
        length: 20,
        ins: 0,
        apco: vec![
            0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
            0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    assert_eq!(Apco::unmarshal(&encoded).unwrap(), decoded);
}
