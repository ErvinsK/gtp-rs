// STN-SR IE - according to 3GPP TS 29.280 V16.0.0 (2020-07)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// STN-SR IE Type

pub const STNSR: u8 = 51;

// STN-SR IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StnSr {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub nanpi: u8, // Nature of Address and Numbering Plan Identity
    pub msisdn: String,
}

impl Default for StnSr {
    fn default() -> StnSr {
        StnSr {
            t: STNSR,
            length: 0,
            ins: 0,
            nanpi: 0,
            msisdn: "0".to_string(),
        }
    }
}

impl From<StnSr> for InformationElement {
    fn from(i: StnSr) -> Self {
        InformationElement::StnSr(i)
    }
}

impl IEs for StnSr {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(STNSR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.nanpi);
        buffer_ie.extend(tbcd_encode(&self.msisdn));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = StnSr {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Default::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.nanpi = buffer[4];
                match buffer[5..(data.length + 4) as usize].try_into() {
                    Ok(i) => data.msisdn = tbcd_decode(i),
                    Err(_) => return Err(GTPV2Error::IEInvalidLength(STNSR)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(STNSR))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(STNSR))
        }
    }

    fn len(&self) -> usize {
        MIN_IE_SIZE + self.length as usize
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
fn stnsr_ie_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x33, 0x00, 0x09, 0x00, 0x04, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2,
    ];
    let decoded = StnSr {
        length: 9,
        nanpi: 0x04,
        msisdn: "882285100125112".to_string(),
        ..Default::default()
    };
    assert_eq!(StnSr::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn stnsr_ie_marshal_test() {
    let encoded: [u8; 13] = [
        0x33, 0x00, 0x09, 0x00, 0x04, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2,
    ];
    let decoded = StnSr {
        length: 9,
        nanpi: 0x04,
        msisdn: "882285100125112".to_string(),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
