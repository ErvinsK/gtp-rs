// MBMB Distribution Acknowledge IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Distribution Acknowledge IE TL

pub const MBMS_DISTRACK: u8 = 143;
pub const MBMS_DISTRACK_LENGTH: usize = 1;

// MBMS Flow Identifier IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsDistributionAck {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub distr_id: u8,
}

impl Default for MbmsDistributionAck {
    fn default() -> Self {
        MbmsDistributionAck {
            t: MBMS_DISTRACK,
            length: MBMS_DISTRACK_LENGTH as u16,
            ins: 0,
            distr_id: 0,
        }
    }
}

impl From<MbmsDistributionAck> for InformationElement {
    fn from(i: MbmsDistributionAck) -> Self {
        InformationElement::MbmsDistributionAck(i)
    }
}

impl IEs for MbmsDistributionAck {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMS_DISTRACK);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.distr_id & 0x03);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MBMS_DISTRACK_LENGTH + MIN_IE_SIZE {
            let data = MbmsDistributionAck {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                distr_id: buffer[4] & 0x03,
                ..MbmsDistributionAck::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMS_DISTRACK))
        }
    }

    fn len(&self) -> usize {
        MBMS_DISTRACK_LENGTH + MIN_IE_SIZE
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
fn mbms_distrack_ie_marshal_test() {
    let encoded: [u8; 5] = [0x8f, 0x00, 0x01, 0x00, 0x02];
    let decoded = MbmsDistributionAck {
        t: MBMS_DISTRACK,
        length: MBMS_DISTRACK_LENGTH as u16,
        ins: 0,
        distr_id: 0x02,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mbms_distrack_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x8f, 0x00, 0x01, 0x00, 0x02];
    let decoded = MbmsDistributionAck {
        t: MBMS_DISTRACK,
        length: MBMS_DISTRACK_LENGTH as u16,
        ins: 0,
        distr_id: 0x02,
    };
    assert_eq!(MbmsDistributionAck::unmarshal(&encoded).unwrap(), decoded);
}
