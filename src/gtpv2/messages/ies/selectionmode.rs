// Selection Mode IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Selection Mode IE Type

pub const SELECTION_MODE: u8 = 128;
pub const SELECTION_MODE_LENGTH: usize = 1;

// Selection Mode IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionMode {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mode: u8,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode {
            t: SELECTION_MODE,
            length: SELECTION_MODE_LENGTH as u16,
            ins: 0,
            mode: 0,
        }
    }
}

impl From<SelectionMode> for InformationElement {
    fn from(i: SelectionMode) -> Self {
        InformationElement::SelectionMode(i)
    }
}

impl IEs for SelectionMode {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SELECTION_MODE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.mode);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + SELECTION_MODE_LENGTH {
            let data = SelectionMode {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                mode: buffer[4] & 0x03,
                ..SelectionMode::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SELECTION_MODE))
        }
    }

    fn len(&self) -> usize {
        SELECTION_MODE_LENGTH + MIN_IE_SIZE
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
fn selection_mode_ie_marshal_test() {
    let encoded: [u8; 5] = [0x80, 0x00, 0x01, 0x00, 0x00];
    let decoded = SelectionMode {
        t: SELECTION_MODE,
        length: SELECTION_MODE_LENGTH as u16,
        ins: 0,
        mode: 0x00,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn chargingchar_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x80, 0x00, 0x01, 0x00, 0x00];
    let decoded = SelectionMode {
        t: SELECTION_MODE,
        length: SELECTION_MODE_LENGTH as u16,
        ins: 0,
        mode: 0x00,
    };
    assert_eq!(SelectionMode::unmarshal(&encoded).unwrap(), decoded);
}
