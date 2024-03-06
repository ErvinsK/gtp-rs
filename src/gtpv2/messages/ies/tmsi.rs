// TMSI IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// TMSI Type

pub const TMSI: u8 = 88;
pub const TMSI_LENGTH: usize = 4;

// TMSI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tmsi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub tmsi: u32,
}

impl Default for Tmsi {
    fn default() -> Self {
        Tmsi {
            t: TMSI,
            length: TMSI_LENGTH as u16,
            ins: 0,
            tmsi: 0,
        }
    }
}

impl From<Tmsi> for InformationElement {
    fn from(i: Tmsi) -> Self {
        InformationElement::Tmsi(i)
    }
}

impl IEs for Tmsi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TMSI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.tmsi.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + TMSI_LENGTH {
            let data = Tmsi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                tmsi: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..Tmsi::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TMSI))
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
fn tmsi_ie_marshal_test() {
    let encoded: [u8; 8] = [0x58, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xfa];
    let decoded = Tmsi {
        t: TMSI,
        length: TMSI_LENGTH as u16,
        ins: 0,
        tmsi: 0xfffffffa,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn tmsi_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x58, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xfa];
    let decoded = Tmsi {
        t: TMSI,
        length: TMSI_LENGTH as u16,
        ins: 0,
        tmsi: 0xfffffffa,
    };
    assert_eq!(Tmsi::unmarshal(&encoded).unwrap(), decoded);
}
