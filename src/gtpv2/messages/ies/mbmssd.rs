// MBMS Session Duration IE - according to 3GPP TS 29.274 V17.10.0 (2023-12), 3GPP TS 29.061 V15.3.0 ()

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Session Duration IE TL

pub const MBMSSD: u8 = 138;
pub const MBMSSD_LENGTH: usize = 3;

// MBMS Session Duration IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsSessionDuration {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub seconds: u32,
    pub days: u8,
}

impl Default for MbmsSessionDuration {
    fn default() -> MbmsSessionDuration {
        MbmsSessionDuration {
            t: MBMSSD,
            length: MBMSSD_LENGTH as u16,
            ins: 0,
            seconds: 0,
            days: 0,
        }
    }
}

impl From<MbmsSessionDuration> for InformationElement {
    fn from(i: MbmsSessionDuration) -> Self {
        InformationElement::MbmsSd(i)
    }
}

impl IEs for MbmsSessionDuration {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMSSD);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let i = (self.seconds << 7) | (self.days as u32);
        buffer_ie.extend_from_slice(&i.to_be_bytes()[1..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MBMSSD_LENGTH + MIN_IE_SIZE {
            let data = MbmsSessionDuration {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                seconds: (u32::from_be_bytes([0x00, buffer[4], buffer[5], buffer[6]])) >> 7,
                days: buffer[6] & 0x7f,
                ..MbmsSessionDuration::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMSSD))
        }
    }

    fn len(&self) -> usize {
        MBMSSD_LENGTH + MIN_IE_SIZE
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
fn mbms_sd_ie_unmarshal_test() {
    let encoded_ie: [u8; 7] = [0x8a, 0x00, 0x03, 0x00, 0x00, 0xc8, 0x0a];
    let test_struct = MbmsSessionDuration {
        t: MBMSSD,
        length: MBMSSD_LENGTH as u16,
        ins: 0,
        seconds: 400,
        days: 10,
    };
    let i = MbmsSessionDuration::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn mbms_sd_ie_marshal_test() {
    let encoded_ie: [u8; 7] = [0x8a, 0x00, 0x03, 0x00, 0x00, 0xc8, 0x0a];
    let test_struct = MbmsSessionDuration {
        t: MBMSSD,
        length: MBMSSD_LENGTH as u16,
        ins: 0,
        seconds: 400,
        days: 10,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
