// RAN NAS Cause IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// RAN NAC Cause IE TL

pub const RAN_NAS_CAUSE: u8 = 172;

// S1AP Cause Types

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S1APCause {
    RadioLayer(u8),
    TransportLayer(u8),
    Nas(u8),
    Protocol(u8),
    Misc(u8),
    Spare(u8),
}

impl S1APCause {
    fn to_u8(&self) -> [u8; 2] {
        match self {
            S1APCause::RadioLayer(i) => [0x10, *i],
            S1APCause::TransportLayer(i) => [0x11, *i],
            S1APCause::Nas(i) => [0x12, *i],
            S1APCause::Protocol(i) => [0x13, *i],
            S1APCause::Misc(i) => [0x14, *i],
            S1APCause::Spare(i) => [0x15, *i],
        }
    }
    fn from_u8(i: u8, j: u8) -> Self {
        match i {
            0 => S1APCause::RadioLayer(j),
            1 => S1APCause::TransportLayer(j),
            2 => S1APCause::Nas(j),
            3 => S1APCause::Protocol(j),
            4 => S1APCause::Misc(j),
            _ => S1APCause::Spare(j),
        }
    }
}

// Enum of RAN NAS Causes

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CauseValue {
    S1ap(S1APCause),
    Emm(u8),
    Esm(u8),
    Diameter(u16),
    Ikev2(u16),
    Spare,
}

// RAN NAS Cause IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RanNasCause {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub cause: CauseValue,
}

impl Default for RanNasCause {
    fn default() -> Self {
        RanNasCause {
            t: RAN_NAS_CAUSE,
            length: 0,
            ins: 0,
            cause: CauseValue::Spare,
        }
    }
}

impl From<RanNasCause> for InformationElement {
    fn from(i: RanNasCause) -> Self {
        InformationElement::RanNasCause(i)
    }
}

impl IEs for RanNasCause {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(RAN_NAS_CAUSE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match &self.cause {
            CauseValue::S1ap(i) => buffer_ie.extend_from_slice(&i.to_u8()),
            CauseValue::Emm(i) => {
                buffer_ie.push(0x20);
                buffer_ie.push(*i);
            }
            CauseValue::Esm(i) => {
                buffer_ie.push(0x30);
                buffer_ie.push(*i);
            }
            CauseValue::Diameter(i) => {
                buffer_ie.push(0x40);
                buffer_ie.extend_from_slice(&i.to_be_bytes());
            }
            CauseValue::Ikev2(i) => {
                buffer_ie.push(0x50);
                buffer_ie.extend_from_slice(&i.to_be_bytes());
            }
            CauseValue::Spare => {
                buffer_ie.push(0x60);
                buffer_ie.push(0);
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 2 {
            let mut data = RanNasCause {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..RanNasCause::default()
            };
            if !check_tliv_ie_buffer(data.length, buffer) {
                return Err(GTPV2Error::IEInvalidLength(RAN_NAS_CAUSE));
            }
            match buffer[4] >> 4 {
                1 => {
                    let c = S1APCause::from_u8(buffer[4] & 0x0f, buffer[5]);
                    data.cause = CauseValue::S1ap(c);
                }
                2 => data.cause = CauseValue::Emm(buffer[5]),
                3 => data.cause = CauseValue::Esm(buffer[5]),
                4 => {
                    if buffer.len() >= MIN_IE_SIZE + 3 {
                        data.cause =
                            CauseValue::Diameter(u16::from_be_bytes([buffer[5], buffer[6]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(RAN_NAS_CAUSE));
                    }
                }
                5 => {
                    if buffer.len() >= MIN_IE_SIZE + 3 {
                        data.cause = CauseValue::Ikev2(u16::from_be_bytes([buffer[5], buffer[6]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(RAN_NAS_CAUSE));
                    }
                }
                _ => data.cause = CauseValue::Spare,
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RAN_NAS_CAUSE))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn ran_nas_cause_ie_short_marshal_test() {
    let decoded = RanNasCause {
        t: RAN_NAS_CAUSE,
        length: 2,
        ins: 0,
        cause: CauseValue::S1ap(S1APCause::RadioLayer(26)),
    };
    let encoded: [u8; 6] = [0xac, 0x00, 0x02, 0x00, 0x10, 0x1a];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ran_nas_cause_ie_short_unmarshal_test() {
    let decoded = RanNasCause {
        t: RAN_NAS_CAUSE,
        length: 2,
        ins: 0,
        cause: CauseValue::S1ap(S1APCause::RadioLayer(26)),
    };
    let encoded: [u8; 6] = [0xac, 0x00, 0x02, 0x00, 0x10, 0x1a];
    assert_eq!(RanNasCause::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn cause_ie_incorrect_length_unmarshal_test() {
    let encoded: [u8; 5] = [0xac, 0x00, 0x01, 0x00, 0x10];
    assert_eq!(
        RanNasCause::unmarshal(&encoded),
        Err(GTPV2Error::IEInvalidLength(RAN_NAS_CAUSE))
    );
}
