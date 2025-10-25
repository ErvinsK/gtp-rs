// UE Time Zone IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// UE Time Zone IE Type

pub const UETIMEZONE: u8 = 114;
pub const UETIMEZONE_LENGTH: usize = 2;

// UE Time Zone IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeTimeZone {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub time_zone: i8, // Negative value means UTC- and positive UTC+, and given with 15 minutes resolution (i.e. UTC+1 = 4, UTC-1 = -4)
    pub dst: u8,
}

impl Default for UeTimeZone {
    fn default() -> Self {
        UeTimeZone {
            t: UETIMEZONE,
            length: UETIMEZONE_LENGTH as u16,
            ins: 0,
            time_zone: 0x00,
            dst: 0x00,
        }
    }
}

impl From<UeTimeZone> for InformationElement {
    fn from(i: UeTimeZone) -> Self {
        InformationElement::UeTimeZone(i)
    }
}

impl IEs for UeTimeZone {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(UETIMEZONE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        if self.time_zone >= 0 {
            buffer_ie.push(self.time_zone as u8);
        } else {
            buffer_ie.push(0x80 | self.time_zone.unsigned_abs());
        }
        buffer_ie.push(self.dst);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + UETIMEZONE_LENGTH {
            let mut data = UeTimeZone {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..UeTimeZone::default()
            };
            match buffer[4] >> 7 {
                0 => data.time_zone = (buffer[4] & 0x7f) as i8,
                1 => data.time_zone = -((buffer[4] & 0x7f) as i8),
                _ => data.time_zone = 0,
            }
            data.dst = buffer[5] & 0x03;
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(UETIMEZONE))
        }
    }

    fn len(&self) -> usize {
        UETIMEZONE_LENGTH + MIN_IE_SIZE
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
fn uetimezone_ie_marshal_test() {
    let encoded: [u8; 6] = [0x72, 0x00, 0x02, 0x00, 0x04, 0x01];
    let decoded = UeTimeZone {
        t: UETIMEZONE,
        length: UETIMEZONE_LENGTH as u16,
        ins: 0,
        time_zone: 4,
        dst: 1,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn uetimezone_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x72, 0x00, 0x02, 0x00, 0x04, 0x01];
    let decoded = UeTimeZone {
        t: UETIMEZONE,
        length: UETIMEZONE_LENGTH as u16,
        ins: 0,
        time_zone: 4,
        dst: 1,
    };
    assert_eq!(UeTimeZone::unmarshal(&encoded).unwrap(), decoded);
}
