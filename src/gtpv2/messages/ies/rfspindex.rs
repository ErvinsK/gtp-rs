// RFSP IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// RFSP IE Type

pub const RFSP: u8 = 144;
pub const RFSP_LENGTH: usize = 2;

// RFSP IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RfspIndex {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub spid: u16, // "Subscriber Profile ID for RAT/Frequency Priority (SPID)" = integer between 1 and 256
}

impl Default for RfspIndex {
    fn default() -> Self {
        RfspIndex {
            t: RFSP,
            length: RFSP_LENGTH as u16,
            ins: 0,
            spid: 0,
        }
    }
}

impl From<RfspIndex> for InformationElement {
    fn from(i: RfspIndex) -> Self {
        InformationElement::RfspIndex(i)
    }
}

impl IEs for RfspIndex {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(RFSP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.spid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + RFSP_LENGTH {
            let data = RfspIndex {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                spid: u16::from_be_bytes([buffer[4], buffer[5]]),
                ..RfspIndex::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RFSP))
        }
    }

    fn len(&self) -> usize {
        RFSP_LENGTH + MIN_IE_SIZE
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
fn rfsp_index_ie_marshal_test() {
    let encoded: [u8; 6] = [0x90, 0x00, 0x02, 0x00, 0x01, 0x00];
    let decoded = RfspIndex {
        t: RFSP,
        length: RFSP_LENGTH as u16,
        ins: 0,
        spid: 0x100,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn rfsp_index_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x90, 0x00, 0x02, 0x00, 0x01, 0x00];
    let decoded = RfspIndex {
        t: RFSP,
        length: RFSP_LENGTH as u16,
        ins: 0,
        spid: 0x100,
    };
    assert_eq!(RfspIndex::unmarshal(&encoded).unwrap(), decoded);
}
