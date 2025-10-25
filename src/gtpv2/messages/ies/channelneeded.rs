// Channel needed ID (PTI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Channel needed IE Type

pub const CHNL_NEEDED: u8 = 133;
pub const CHNL_NEEDED_LENGTH: usize = 1;

// Channel needed IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelNeeded {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub chnl_needed: u8,
}

impl Default for ChannelNeeded {
    fn default() -> Self {
        ChannelNeeded {
            t: CHNL_NEEDED,
            length: CHNL_NEEDED_LENGTH as u16,
            ins: 0,
            chnl_needed: 0,
        }
    }
}

impl From<ChannelNeeded> for InformationElement {
    fn from(i: ChannelNeeded) -> Self {
        InformationElement::ChannelNeeded(i)
    }
}

impl IEs for ChannelNeeded {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CHNL_NEEDED);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.chnl_needed);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + CHNL_NEEDED_LENGTH {
            let data = ChannelNeeded {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                chnl_needed: buffer[4],
                ..ChannelNeeded::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CHNL_NEEDED))
        }
    }

    fn len(&self) -> usize {
        CHNL_NEEDED_LENGTH + MIN_IE_SIZE
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
fn channel_needed_ie_marshal_test() {
    let encoded: [u8; 5] = [0x85, 0x00, 0x01, 0x00, 0xff];
    let decoded = ChannelNeeded {
        t: CHNL_NEEDED,
        length: CHNL_NEEDED_LENGTH as u16,
        ins: 0,
        chnl_needed: 0xff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn channel_needed_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x85, 0x00, 0x01, 0x00, 0xff];
    let decoded = ChannelNeeded {
        t: CHNL_NEEDED,
        length: CHNL_NEEDED_LENGTH as u16,
        ins: 0,
        chnl_needed: 0xff,
    };
    assert_eq!(ChannelNeeded::unmarshal(&encoded).unwrap(), decoded);
}
