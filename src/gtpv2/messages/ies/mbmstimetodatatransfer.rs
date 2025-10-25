// MBMS Time to Data Transfer IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Time to Data Transfer IE Type

pub const MBMSTIMETODATATRNSF: u8 = 153;
pub const MBMSTIMETODATATRNSF_LENGTH: usize = 1;

// MBMS Time to Data Transfer IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsTimeToDataTransfer {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mbms_time_to_data: u8,
}

impl Default for MbmsTimeToDataTransfer {
    fn default() -> Self {
        MbmsTimeToDataTransfer {
            t: MBMSTIMETODATATRNSF,
            length: MBMSTIMETODATATRNSF_LENGTH as u16,
            ins: 0,
            mbms_time_to_data: 0,
        }
    }
}

impl From<MbmsTimeToDataTransfer> for InformationElement {
    fn from(i: MbmsTimeToDataTransfer) -> Self {
        InformationElement::MbmsTimeToDataTransfer(i)
    }
}

impl IEs for MbmsTimeToDataTransfer {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMSTIMETODATATRNSF);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.mbms_time_to_data);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + MBMSTIMETODATATRNSF_LENGTH {
            let data = MbmsTimeToDataTransfer {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                mbms_time_to_data: buffer[4],
                ..MbmsTimeToDataTransfer::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMSTIMETODATATRNSF))
        }
    }

    fn len(&self) -> usize {
        MBMSTIMETODATATRNSF_LENGTH + MIN_IE_SIZE
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
fn mbmstimetodatatransfer_ie_marshal_test() {
    let encoded: [u8; 5] = [0x99, 0x00, 0x01, 0x00, 0xff];
    let decoded = MbmsTimeToDataTransfer {
        mbms_time_to_data: 0xff,
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mbmstimetodatatransfer_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x99, 0x00, 0x01, 0x00, 0xff];
    let decoded = MbmsTimeToDataTransfer {
        mbms_time_to_data: 0xff,
        ..Default::default()
    };
    assert_eq!(
        MbmsTimeToDataTransfer::unmarshal(&encoded).unwrap(),
        decoded
    );
}
