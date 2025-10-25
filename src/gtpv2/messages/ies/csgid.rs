// CSG ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// CSG ID Type

pub const CSGID: u8 = 147;
pub const CSGID_LENGTH: usize = 4;

// CSG ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsgId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub csgid: u32,
}

impl Default for CsgId {
    fn default() -> Self {
        CsgId {
            t: CSGID,
            length: CSGID_LENGTH as u16,
            ins: 0,
            csgid: 0,
        }
    }
}

impl From<CsgId> for InformationElement {
    fn from(i: CsgId) -> Self {
        InformationElement::CsgId(i)
    }
}

impl IEs for CsgId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CSGID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.csgid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + CSGID_LENGTH {
            let data = CsgId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                csgid: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..CsgId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CSGID))
        }
    }

    fn len(&self) -> usize {
        CSGID_LENGTH + MIN_IE_SIZE
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
fn csgid_ie_marshal_test() {
    let encoded: [u8; 8] = [0x93, 0x00, 0x04, 0x00, 0x07, 0xff, 0xff, 0xff];
    let decoded = CsgId {
        t: CSGID,
        length: CSGID_LENGTH as u16,
        ins: 0,
        csgid: 0x7ffffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn csgid_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x93, 0x00, 0x04, 0x00, 0x07, 0xff, 0xff, 0xff];
    let decoded = CsgId {
        t: CSGID,
        length: CSGID_LENGTH as u16,
        ins: 0,
        csgid: 0x7ffffff,
    };
    assert_eq!(CsgId::unmarshal(&encoded).unwrap(), decoded);
}
