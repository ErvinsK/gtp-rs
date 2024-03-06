// MBMB Session Identifier IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Session Identifier IE TL

pub const MBMS_SESSIONID: u8 = 140;
pub const MBMS_SESSIONID_LENGTH: usize = 1;

// MBMS Flow Identifier IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsSessionId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mbms_sessionid: u8,
}

impl Default for MbmsSessionId {
    fn default() -> Self {
        MbmsSessionId {
            t: MBMS_SESSIONID,
            length: MBMS_SESSIONID_LENGTH as u16,
            ins: 0,
            mbms_sessionid: 0,
        }
    }
}

impl From<MbmsSessionId> for InformationElement {
    fn from(i: MbmsSessionId) -> Self {
        InformationElement::MbmsSessionId(i)
    }
}

impl IEs for MbmsSessionId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMS_SESSIONID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.mbms_sessionid);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MBMS_SESSIONID_LENGTH + MIN_IE_SIZE {
            let data = MbmsSessionId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                mbms_sessionid: buffer[4],
                ..MbmsSessionId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMS_SESSIONID))
        }
    }

    fn len(&self) -> usize {
        MBMS_SESSIONID_LENGTH + MIN_IE_SIZE
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
fn mbms_sessionid_ie_marshal_test() {
    let encoded: [u8; 5] = [0x8c, 0x00, 0x01, 0x00, 0x0a];
    let decoded = MbmsSessionId {
        t: MBMS_SESSIONID,
        length: MBMS_SESSIONID_LENGTH as u16,
        ins: 0,
        mbms_sessionid: 0x0a,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mbms_sessionid_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x8c, 0x00, 0x01, 0x00, 0x0a];
    let decoded = MbmsSessionId {
        t: MBMS_SESSIONID,
        length: MBMS_SESSIONID_LENGTH as u16,
        ins: 0,
        mbms_sessionid: 0x0a,
    };
    assert_eq!(MbmsSessionId::unmarshal(&encoded).unwrap(), decoded);
}
