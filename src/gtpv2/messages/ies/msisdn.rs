// MSISDN IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MSISDN IE Type

pub const MSISDN: u8 = 76;

// MSISDN IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msisdn {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub msisdn: String,
}

impl Default for Msisdn {
    fn default() -> Msisdn {
        Msisdn {
            t: MSISDN,
            length: 0,
            ins: 0,
            msisdn: "0".to_string(),
        }
    }
}

impl From<Msisdn> for InformationElement {
    fn from(i: Msisdn) -> Self {
        InformationElement::Msisdn(i)
    }
}

impl IEs for Msisdn {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MSISDN);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend(tbcd_encode(&self.msisdn));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Msisdn, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Msisdn {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Msisdn::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4..(data.length + 4) as usize].try_into() {
                    Ok(i) => data.msisdn = tbcd_decode(i),
                    Err(_) => return Err(GTPV2Error::IEIncorrect(MSISDN)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(MSISDN))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(MSISDN))
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
fn msisdn_ie_unmarshal_test() {
    let encoded_ie: [u8; 12] = [
        0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2,
    ];
    let test_struct = Msisdn {
        t: MSISDN,
        length: 8,
        ins: 0,
        msisdn: "882285100125112".to_string(),
    };
    let i = Msisdn::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn msisdn_ie_marshal_test() {
    let encoded_ie: [u8; 12] = [
        0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2,
    ];
    let test_struct = Msisdn {
        t: MSISDN,
        length: 8,
        ins: 0,
        msisdn: "882285100125112".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
