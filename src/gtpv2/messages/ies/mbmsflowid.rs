// MBMB Flow Identifier IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Flow Identifier IE TL

pub const MBMS_FLOWID: u8 = 141;
pub const MBMS_FLOWID_LENGTH: usize = 2;

// MBMS Flow Identifier IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsFlowId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mbms_flowid: u16,
}

impl Default for MbmsFlowId {
    fn default() -> Self {
        MbmsFlowId {
            t: MBMS_FLOWID,
            length: MBMS_FLOWID_LENGTH as u16,
            ins: 0,
            mbms_flowid: 0,
        }
    }
}

impl From<MbmsFlowId> for InformationElement {
    fn from(i: MbmsFlowId) -> Self {
        InformationElement::MbmsFlowId(i)
    }
}

impl IEs for MbmsFlowId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMS_FLOWID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.mbms_flowid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MBMS_FLOWID_LENGTH + MIN_IE_SIZE {
            let data = MbmsFlowId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                mbms_flowid: u16::from_be_bytes([buffer[4], buffer[5]]),
                ..MbmsFlowId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMS_FLOWID))
        }
    }

    fn len(&self) -> usize {
        MBMS_FLOWID_LENGTH + MIN_IE_SIZE
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
fn mbms_flowid_ie_marshal_test() {
    let encoded: [u8; 6] = [0x8d, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = MbmsFlowId {
        t: MBMS_FLOWID,
        length: MBMS_FLOWID_LENGTH as u16,
        ins: 0,
        mbms_flowid: 0xffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mbms_flowid_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x8d, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = MbmsFlowId {
        t: MBMS_FLOWID,
        length: MBMS_FLOWID_LENGTH as u16,
        ins: 0,
        mbms_flowid: 0xffff,
    };
    assert_eq!(MbmsFlowId::unmarshal(&encoded).unwrap(), decoded);
}
