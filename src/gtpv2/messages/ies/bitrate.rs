// Bit Rate IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Bit Rate IE Type

pub const BITRATE: u8 = 211;
pub const BITRATE_LENGTH: usize = 4;

// Bit Rate IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitRate {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub bitrate: u32, // The bit rate field is encoded as kilobits per second (1 kbps = 1000 bps) in binary value.
}

impl Default for BitRate {
    fn default() -> Self {
        BitRate {
            t: BITRATE,
            length: BITRATE_LENGTH as u16,
            ins: 0,
            bitrate: 0,
        }
    }
}

impl From<BitRate> for InformationElement {
    fn from(i: BitRate) -> Self {
        InformationElement::BitRate(i)
    }
}

impl IEs for BitRate {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(BITRATE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.bitrate.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + BITRATE_LENGTH {
            let data = BitRate {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                bitrate: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..BitRate::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(BITRATE))
        }
    }

    fn len(&self) -> usize {
        BITRATE_LENGTH + MIN_IE_SIZE
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
fn bitrate_ie_marshal_test() {
    let encoded: [u8; 8] = [0xd3, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff];
    let decoded = BitRate {
        t: BITRATE,
        length: BITRATE_LENGTH as u16,
        ins: 0,
        bitrate: 0xffffffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn bitrate_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0xd3, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff];
    let decoded = BitRate {
        t: BITRATE,
        length: BITRATE_LENGTH as u16,
        ins: 0,
        bitrate: 0xffffffff,
    };
    assert_eq!(BitRate::unmarshal(&encoded).unwrap(), decoded);
}
