// EPC Timer IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// EPC Timer IE TL

pub const EPC_TIMER: u8 = 156;
pub const EPC_TIMER_LENGTH: usize = 1;

// EPC Timer IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpcTimer {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub timer_unit: u8, // 0 - 2 seconds, 1 - 1 minute, 2 - 10 minutes, 3 - 1 hour, 4 - 10 hours, 7 - infinite
    pub timer_value: u8, // Timer value to be incremented by Timer unit to calculate EPC timer. If both Timer Unit and Timer Value set to zero = timer is stopped.
}

impl Default for EpcTimer {
    fn default() -> Self {
        EpcTimer {
            t: EPC_TIMER,
            length: EPC_TIMER_LENGTH as u16,
            ins: 0,
            timer_unit: 0,
            timer_value: 0,
        }
    }
}

impl From<EpcTimer> for InformationElement {
    fn from(i: EpcTimer) -> Self {
        InformationElement::EpcTimer(i)
    }
}

impl IEs for EpcTimer {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(EPC_TIMER);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.timer_value {
            i if i < 0x7f => buffer_ie.push((self.timer_unit << 5) | self.timer_value),
            _ => buffer_ie.push((self.timer_unit << 5) | 0x7f),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= EPC_TIMER_LENGTH + MIN_IE_SIZE {
            let data = EpcTimer {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                timer_unit: match buffer[4] >> 5 {
                    i if i <= 4 => buffer[4] >> 5,
                    7 => 7,
                    _ => 1,
                },
                timer_value: buffer[4] & 0x1f,
                ..EpcTimer::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(EPC_TIMER))
        }
    }

    fn len(&self) -> usize {
        EPC_TIMER_LENGTH + MIN_IE_SIZE
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
fn epctimer_ie_marshal_test() {
    let encoded: [u8; 5] = [0x9c, 0x00, 0x01, 0x00, 0x7f];
    let decoded = EpcTimer {
        t: EPC_TIMER,
        length: EPC_TIMER_LENGTH as u16,
        ins: 0,
        timer_unit: 3,
        timer_value: 31,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn epctimer_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x9c, 0x00, 0x01, 0x00, 0x7f];
    let decoded = EpcTimer {
        t: EPC_TIMER,
        length: EPC_TIMER_LENGTH as u16,
        ins: 0,
        timer_unit: 3,
        timer_value: 31,
    };
    assert_eq!(EpcTimer::unmarshal(&encoded).unwrap(), decoded);
}
