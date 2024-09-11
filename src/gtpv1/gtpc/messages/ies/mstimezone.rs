// MS Time Zone IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// MS Time Zone IE Type

pub const MSTIMEZONETYPE: u8 = 153;
pub const MSTIMEZONE_LENGTH: u16 = 2;

// MS Time Zone IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsTimeZone {
    pub t: u8,
    pub length: u16,
    pub time_zone: i8, // Negative value means UTC- and positive UTC+
    pub dst: u8,
}

impl Default for MsTimeZone {
    fn default() -> Self {
        MsTimeZone {
            t: MSTIMEZONETYPE,
            length: MSTIMEZONE_LENGTH,
            time_zone: 0x00,
            dst: 0x00,
        }
    }
}

impl IEs for MsTimeZone {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        if self.time_zone >= 0 {
            buffer_ie.push(self.time_zone as u8);
        } else {
            buffer_ie.push(0x80 | self.time_zone.unsigned_abs());
        }
        buffer_ie.push(self.dst);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 5 {
            let mut data = MsTimeZone {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match buffer[3] >> 7 {
                0 => data.time_zone = (buffer[3] & 0x7f) as i8,
                1 => data.time_zone = -((buffer[3] & 0x7f) as i8),
                _ => data.time_zone = 0,
            }
            data.dst = buffer[4] & 0x03;
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (MSTIMEZONE_LENGTH + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn mstimezone_ie_marshal_test() {
    let ie_marshalled: [u8; 5] = [0x99, 0x00, 0x02, 0x04, 0x00];
    let ie_to_marshal = MsTimeZone {
        t: MSTIMEZONETYPE,
        length: MSTIMEZONE_LENGTH,
        time_zone: 4,
        dst: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn mstimezone_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 5] = [0x99, 0x00, 0x02, 0x84, 0x00];
    let ie_unmarshalled = MsTimeZone {
        t: MSTIMEZONETYPE,
        length: MSTIMEZONE_LENGTH,
        time_zone: -4,
        dst: 0,
    };
    assert_eq!(
        MsTimeZone::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
