// Cause IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Cause IE TL

pub const CAUSE: u8 = 2;
pub const SHORT_CAUSE_LENGTH: usize = 2;
pub const LONG_CAUSE_LENGTH: usize = 6;

// Cause IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cause {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub value: u8,
    pub pce: bool,
    pub bce: bool,
    pub cs: bool,
    pub offend_ie_type: Option<u8>,
}

impl Default for Cause {
    fn default() -> Self {
        Cause {
            t: CAUSE,
            length: SHORT_CAUSE_LENGTH as u16,
            ins: 0,
            value: 0,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        }
    }
}

impl From<Cause> for InformationElement {
    fn from(i: Cause) -> Self {
        InformationElement::Cause(i)
    }
}

impl IEs for Cause {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CAUSE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.value);
        let from_bool = |i: bool| -> u8 {
            if i {
                1
            } else {
                0
            }
        };
        let flags = from_bool(self.pce) << 2 | from_bool(self.bce) << 1 | from_bool(self.cs);
        buffer_ie.push(flags);
        if let Some(i) = self.offend_ie_type {
            buffer_ie.push(i);
            buffer_ie.extend_from_slice(&[0x00; 3]);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let to_bool = |i: u8| -> bool { i == 1 };
            let mut data = Cause {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Cause::default()
            };
            match data.length as usize {
                SHORT_CAUSE_LENGTH => {
                    if buffer.len() >= (SHORT_CAUSE_LENGTH + MIN_IE_SIZE) {
                        data.value = buffer[4];
                        data.cs = to_bool(buffer[5] & 1);
                        data.bce = to_bool((buffer[5] >> 1) & 1);
                        data.pce = to_bool((buffer[5] >> 2) & 1);
                        Ok(data)
                    } else {
                        Err(GTPV2Error::IEIncorrect(CAUSE))
                    }
                }
                LONG_CAUSE_LENGTH => {
                    if buffer.len() >= (LONG_CAUSE_LENGTH + MIN_IE_SIZE) {
                        data.value = buffer[4];
                        data.cs = to_bool(buffer[5] & 1);
                        data.bce = to_bool((buffer[5] >> 1) & 1);
                        data.pce = to_bool((buffer[5] >> 2) & 1);
                        data.offend_ie_type = Some(buffer[6]);
                        Ok(data)
                    } else {
                        Err(GTPV2Error::IEIncorrect(CAUSE))
                    }
                }
                _ => Err(GTPV2Error::IEIncorrect(CAUSE)),
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(CAUSE))
        }
    }

    fn len(&self) -> usize {
        (self.length + 4) as usize
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
fn cause_ie_short_marshal_test() {
    let decoded = Cause {
        t: CAUSE,
        length: 2,
        ins: 0,
        value: 16,
        pce: false,
        bce: false,
        cs: false,
        offend_ie_type: None,
    };
    let encoded: [u8; 6] = [0x02, 0x00, 0x02, 0x00, 0x10, 0x00];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn cause_ie_short_unmarshal_test() {
    let decoded = Cause {
        t: CAUSE,
        length: 2,
        ins: 0,
        value: 16,
        pce: false,
        bce: false,
        cs: false,
        offend_ie_type: None,
    };
    let encoded: [u8; 6] = [0x02, 0x00, 0x02, 0x00, 0x10, 0x00];
    assert_eq!(Cause::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn cause_ie_long_marshal_test() {
    let decoded = Cause {
        t: CAUSE,
        length: 6,
        ins: 0,
        value: 16,
        pce: false,
        bce: true,
        cs: false,
        offend_ie_type: Some(0x0f),
    };
    let encoded: [u8; 10] = [0x02, 0x00, 0x06, 0x00, 0x10, 0x02, 0x0f, 0x00, 0x00, 0x00];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn cause_ie_long_unmarshal_test() {
    let decoded = Cause {
        t: CAUSE,
        length: 6,
        ins: 0,
        value: 16,
        pce: false,
        bce: true,
        cs: false,
        offend_ie_type: Some(0x0f),
    };
    let encoded: [u8; 10] = [0x02, 0x00, 0x06, 0x00, 0x10, 0x02, 0x0f, 0x00, 0x00, 0x00];
    assert_eq!(Cause::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn cause_ie_incorrect_length_unmarshal_test() {
    let encoded: [u8; 12] = [
        0x02, 0x00, 0x08, 0x00, 0x10, 0x02, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    assert_eq!(
        Cause::unmarshal(&encoded),
        Err(GTPV2Error::IEIncorrect(CAUSE))
    );
}

#[test]
fn cause_ie_too_incorrect_length_too_small_unmarshal_test() {
    let encoded: [u8; 10] = [0x02, 0x00, 0x08, 0x00, 0x10, 0x02, 0x0f, 0x00, 0x00, 0x00];
    assert_eq!(
        Cause::unmarshal(&encoded),
        Err(GTPV2Error::IEIncorrect(CAUSE))
    );
}
