// Absolute Time of MBMS Data Transfer IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Absolute Time of MBMS Data Transfer IE TL

pub const ABSTIME_MBMSDATATRNSFR: u8 = 164;
pub const ABSTIME_MBMSDATATRNSFR_LENGTH: usize = 8;

// Absolute Time of MBMS Data Transfer IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsoluteTimeMbmsDataTransfer {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub seconds: u64, // The time in seconds relative to 00:00:00 on 1 January  1900 (calculated as continuous time without leap seconds and traceable to a common time reference) where binary encoding of the integer part is in the 32 most significant bits and binary encoding of the fraction part in the 32 least significant bits. The fraction part is expressed with a granularity of 1 /2**32 second.
}

impl Default for AbsoluteTimeMbmsDataTransfer {
    fn default() -> Self {
        AbsoluteTimeMbmsDataTransfer {
            t: ABSTIME_MBMSDATATRNSFR,
            length: ABSTIME_MBMSDATATRNSFR_LENGTH as u16,
            ins: 0,
            seconds: 0,
        }
    }
}

impl From<AbsoluteTimeMbmsDataTransfer> for InformationElement {
    fn from(i: AbsoluteTimeMbmsDataTransfer) -> Self {
        InformationElement::AbsoluteTimeMbmsDataTransfer(i)
    }
}

impl IEs for AbsoluteTimeMbmsDataTransfer {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ABSTIME_MBMSDATATRNSFR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.seconds.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= ABSTIME_MBMSDATATRNSFR_LENGTH + MIN_IE_SIZE {
            let data = AbsoluteTimeMbmsDataTransfer {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                seconds: u64::from_be_bytes([
                    buffer[4], buffer[5], buffer[6], buffer[7], buffer[8], buffer[9], buffer[10],
                    buffer[11],
                ]),
                ..AbsoluteTimeMbmsDataTransfer::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ABSTIME_MBMSDATATRNSFR))
        }
    }

    fn len(&self) -> usize {
        ABSTIME_MBMSDATATRNSFR_LENGTH + MIN_IE_SIZE
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
fn abstime_mbmsdatatransfer_ie_marshal_test() {
    let encoded: [u8; 12] = [
        0xa4, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
    ];
    let decoded = AbsoluteTimeMbmsDataTransfer {
        t: ABSTIME_MBMSDATATRNSFR,
        length: ABSTIME_MBMSDATATRNSFR_LENGTH as u16,
        ins: 0,
        seconds: 0xffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn abstime_mbmsdatatransfer_ie_unmarshal_test() {
    let encoded: [u8; 12] = [
        0xa4, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
    ];
    let decoded = AbsoluteTimeMbmsDataTransfer {
        t: ABSTIME_MBMSDATATRNSFR,
        length: ABSTIME_MBMSDATATRNSFR_LENGTH as u16,
        ins: 0,
        seconds: 0xffff,
    };
    assert_eq!(
        AbsoluteTimeMbmsDataTransfer::unmarshal(&encoded).unwrap(),
        decoded
    );
}
