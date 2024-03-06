// Aggregate Maximum Bit Rate (AMBR) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// AMBR IE TL

pub const AMBR: u8 = 72;
pub const AMBR_LENGTH: u16 = 8;

// AMBR IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ambr {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ambr_ul: u32,
    pub ambr_dl: u32,
}

impl Default for Ambr {
    fn default() -> Self {
        Ambr {
            t: AMBR,
            length: AMBR_LENGTH,
            ins: 0,
            ambr_ul: 0,
            ambr_dl: 0,
        }
    }
}

impl From<Ambr> for InformationElement {
    fn from(i: Ambr) -> Self {
        InformationElement::ApnAmbr(i)
    }
}

impl IEs for Ambr {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(AMBR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.ambr_ul.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ambr_dl.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= AMBR_LENGTH as usize + MIN_IE_SIZE {
            let data = Ambr {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ambr_ul: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ambr_dl: u32::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
                ..Ambr::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(AMBR))
        }
    }

    fn len(&self) -> usize {
        AMBR_LENGTH as usize + 4
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
fn apnambr_ie_marshal_test() {
    let ie_marshalled: [u8; 12] = [
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40,
    ];
    let ie_to_marshal = Ambr {
        t: AMBR,
        length: AMBR_LENGTH,
        ins: 0,
        ambr_ul: 2000,
        ambr_dl: 8000,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn apnambr_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 12] = [
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40,
    ];
    let ie_unmarshalled = Ambr {
        t: AMBR,
        length: AMBR_LENGTH,
        ins: 0,
        ambr_ul: 2000,
        ambr_dl: 8000,
    };
    assert_eq!(Ambr::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}
