// APN Aggregate Maximum Bit Rate (AMBR) IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// APN-AMBR IE TL

pub const APNAMBR: u8 = 72;
pub const APNAMBR_LENGTH: u16 = 8;

// APN-AMBR IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApnAmbr {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ambr_ul: u32,
    pub ambr_dl: u32,
}

impl Default for ApnAmbr {
    fn default() -> Self {
        ApnAmbr {
            t: APNAMBR,
            length: APNAMBR_LENGTH,
            ins: 0,
            ambr_ul: 0,
            ambr_dl: 0,
        }
    }
}

impl From<ApnAmbr> for InformationElement {
    fn from(i: ApnAmbr) -> Self {
        InformationElement::ApnAmbr(i)
    }
}

impl IEs for ApnAmbr {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(APNAMBR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.ambr_ul.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ambr_dl.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= APNAMBR_LENGTH as usize + MIN_IE_SIZE {
            let data = ApnAmbr {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ambr_ul: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ambr_dl: u32::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
                ..ApnAmbr::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(APNAMBR))
        }
    }

    fn len(&self) -> usize {
        APNAMBR_LENGTH as usize + 4
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn apnambr_ie_marshal_test() {
    let ie_marshalled: [u8; 12] = [
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40,
    ];
    let ie_to_marshal = ApnAmbr {
        t: APNAMBR,
        length: APNAMBR_LENGTH,
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
    let ie_unmarshalled = ApnAmbr {
        t: APNAMBR,
        length: APNAMBR_LENGTH,
        ins: 0,
        ambr_ul: 2000,
        ambr_dl: 8000,
    };
    assert_eq!(
        ApnAmbr::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
