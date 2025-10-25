// Alternative IMSI IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Alternative IMSI IE Type

pub const ALT_IMSI: u8 = 219;

// Alternative IMSI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeImsi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub imsi: String,
}

impl Default for AlternativeImsi {
    fn default() -> AlternativeImsi {
        AlternativeImsi {
            t: ALT_IMSI,
            length: 0,
            ins: 0,
            imsi: "".to_string(),
        }
    }
}

impl From<AlternativeImsi> for InformationElement {
    fn from(i: AlternativeImsi) -> Self {
        InformationElement::AlternativeImsi(i)
    }
}

impl IEs for AlternativeImsi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ALT_IMSI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend(tbcd_encode(&self.imsi));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = AlternativeImsi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..AlternativeImsi::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4..=(data.length as usize) + 3].try_into() {
                    Ok(i) => data.imsi = tbcd_decode(i),
                    Err(_) => return Err(GTPV2Error::IEIncorrect(ALT_IMSI)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(ALT_IMSI))
            }
        } else {
            Err(GTPV2Error::IEIncorrect(ALT_IMSI))
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
fn alt_imsi_ie_unmarshal_test() {
    let encoded_ie: [u8; 12] = [
        0xdb, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let test_struct = AlternativeImsi {
        length: 0x08,
        imsi: "901405101961873".to_string(),
        ..AlternativeImsi::default()
    };
    let i = AlternativeImsi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn alt_imsi_ie_marshal_test() {
    let encoded_ie: [u8; 12] = [
        0xdb, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let test_struct = AlternativeImsi {
        length: 0x08,
        imsi: "901405101961873".to_string(),
        ..AlternativeImsi::default()
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn imsi_ie_unmarshal_buffer_test() {
    let encoded_ie: [u8; 12] = [
        0xdb, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let test_struct = AlternativeImsi {
        length: 0x08,
        imsi: "901405101961873".to_string(),
        ..AlternativeImsi::default()
    };
    let i = AlternativeImsi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}
