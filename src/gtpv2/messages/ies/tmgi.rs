// Temporary Mobile Group Identity (TMGI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Temporary Mobile Group Identity (TMGI) IE Type

pub const TMGI: u8 = 158;
pub const TMGI_LENGTH: usize = 6;

// Temporary Mobile Group Identity (TMGI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tmgi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub tmgi: [u8; 6],
}

impl Default for Tmgi {
    fn default() -> Self {
        Tmgi {
            t: TMGI,
            length: TMGI_LENGTH as u16,
            ins: 0,
            tmgi: [0; 6],
        }
    }
}

impl From<Tmgi> for InformationElement {
    fn from(i: Tmgi) -> Self {
        InformationElement::Tmgi(i)
    }
}

impl IEs for Tmgi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TMGI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.tmgi);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + TMGI_LENGTH {
            let mut data = Tmgi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                ..Default::default()
            };
            if let Ok(tmgi) = buffer[4..MIN_IE_SIZE + TMGI_LENGTH].try_into() {
                data.tmgi = tmgi;
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(TMGI))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(TMGI))
        }
    }

    fn len(&self) -> usize {
        TMGI_LENGTH + MIN_IE_SIZE
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
fn tmgi_ie_marshal_test() {
    let encoded: [u8; 10] = [0x9E, 0x00, 0x06, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let decoded = Tmgi {
        tmgi: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        ..Tmgi::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn tmgi_ie_unmarshal_test() {
    let encoded: [u8; 10] = [0x9E, 0x00, 0x06, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let decoded = Tmgi {
        tmgi: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        ..Tmgi::default()
    };
    assert_eq!(Tmgi::unmarshal(&encoded).unwrap(), decoded);
}
