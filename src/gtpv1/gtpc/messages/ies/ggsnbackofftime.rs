// GGSN Back-Off Time IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// GGSN Back-Off Time IE Type

pub const GGSN_BACKOFF: u8 = 202;
pub const GGSN_BACKOFF_LENGTH: u16 = 1;

// Extended Common Flags II IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GGSNBackOffTime {
    pub t: u8,
    pub length: u16,
    pub timer_unit: u8, // 0 - 2 seconds, 1 - 1 minute, 2 - 10 minutes, 3 - 1 hour, 4 - 10 hours, 7 - infinite
    pub timer_value: u8, // Timer value to be incremented by Timer unit to calculate GGSN back-off time. If both Timer Unit and Timer Value set to zero = timer is stopped.
}

impl Default for GGSNBackOffTime {
    fn default() -> Self {
        GGSNBackOffTime {
            t: GGSN_BACKOFF,
            length: GGSN_BACKOFF_LENGTH,
            timer_unit: 0,
            timer_value: 0,
        }
    }
}

impl IEs for GGSNBackOffTime {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        match self.timer_value {
            i if i < 0x7f => buffer_ie.push((self.timer_unit << 5) | self.timer_value),
            _ => buffer_ie.push((self.timer_unit << 5) | 0x7f),
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= (GGSN_BACKOFF_LENGTH + 1) as usize {
            let mut data = GGSNBackOffTime {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match buffer[3] >> 5 {
                i if i <= 4 => data.timer_unit = buffer[3] >> 5,
                7 => data.timer_unit = 7,
                _ => data.timer_unit = 1,
            }
            data.timer_value = buffer[3] & 0x1f;
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn ggsn_backoff_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0xca, 0x00, 0x01, 0x7f];
    let ie_to_marshal = GGSNBackOffTime {
        t: GGSN_BACKOFF,
        length: GGSN_BACKOFF_LENGTH,
        timer_unit: 3,
        timer_value: 31,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn ggsn_backoff_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0xca, 0x00, 0x01, 0x7f];
    let ie_unmarshalled = GGSNBackOffTime {
        t: GGSN_BACKOFF,
        length: GGSN_BACKOFF_LENGTH,
        timer_unit: 3,
        timer_value: 31,
    };
    assert_eq!(
        GGSNBackOffTime::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
