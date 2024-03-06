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
    pub time_zone: i8, // Negative value means UTC- and positive UTC+
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
            let tz = (self.time_zone as u8) << 2;
            let b: u8 = ((tz - (tz % 10)) / 10) << 4;
            let a = tz % 10;
            buffer_ie.push(b >> 4 | a << 4);
        } else {
            let tz = self.time_zone.unsigned_abs() << 2;
            let b: u8 = (((tz - (tz % 10)) / 10) << 4) | 0x80;
            let a = tz % 10;
            buffer_ie.push(b >> 4 | a << 4);
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
            let bcd = (buffer[4] >> 4) | (buffer[4] << 4);
            match bcd >> 7 {
                0 => {
                    data.time_zone = ((((bcd & 0x7f) >> 4) * 10 + ((bcd & 0x7f) & 0xf)) >> 2) as i8
                }
                1 => {
                    data.time_zone =
                        -(((((bcd & 0x7f) >> 4) * 10 + ((bcd & 0x7f) & 0xf)) >> 2) as i8)
                }
                _ => data.time_zone = 0,
            }
            data.dst = buffer[5] & 0x07;
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
    let encoded: [u8; 6] = [0x72, 0x00, 0x02, 0x00, 0x80, 0x01];
    let decoded = UeTimeZone {
        t: UETIMEZONE,
        length: UETIMEZONE_LENGTH as u16,
        ins: 0,
        time_zone: 2,
        dst: 1,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn uetimezone_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x72, 0x00, 0x02, 0x00, 0x80, 0x01];
    let decoded = UeTimeZone {
        t: UETIMEZONE,
        length: UETIMEZONE_LENGTH as u16,
        ins: 0,
        time_zone: 2,
        dst: 1,
    };
    assert_eq!(UeTimeZone::unmarshal(&encoded).unwrap(), decoded);
}
