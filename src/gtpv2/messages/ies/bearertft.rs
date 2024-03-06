// EPS Bearer Level Traffic Flow Template (TFT) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// EPS Bearer Level Traffic Flow Template (TFT) IE Type

pub const BEARERTFT: u8 = 84;

// EPS Bearer Level Traffic Flow Template (TFT) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerTft {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub tft: Vec<u8>,
}

impl Default for BearerTft {
    fn default() -> Self {
        BearerTft {
            t: BEARERTFT,
            length: 0,
            ins: 0,
            tft: vec![],
        }
    }
}

impl From<BearerTft> for InformationElement {
    fn from(i: BearerTft) -> Self {
        InformationElement::BearerTft(i)
    }
}

impl IEs for BearerTft {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(BEARERTFT);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.tft.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = BearerTft {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..BearerTft::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.tft.extend_from_slice(
                    &buffer[MIN_IE_SIZE..(MIN_IE_SIZE + (data.length as usize))],
                );
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(BEARERTFT))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(BEARERTFT))
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
fn bearertft_ie_marshal_test() {
    let encoded: [u8; 8] = [0x54, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = BearerTft {
        t: BEARERTFT,
        length: 4,
        ins: 0,
        tft: vec![0, 0, 0, 0],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn bearertft_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x54, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = BearerTft {
        t: BEARERTFT,
        length: 4,
        ins: 0,
        tft: vec![0, 0, 0, 0],
    };
    assert_eq!(BearerTft::unmarshal(&encoded).unwrap(), decoded);
}
