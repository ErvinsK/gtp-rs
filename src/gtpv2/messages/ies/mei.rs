// Mobile Equipment Identity (MEI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MEI IE TV

pub const MEI: u8 = 75;
pub const MEI_LENGTH: usize = 8;

// MEI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mei {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mei: String,
}

impl Default for Mei {
    fn default() -> Mei {
        Mei {
            t: MEI,
            length: MEI_LENGTH as u16,
            ins: 0,
            mei: "0".to_string(),
        }
    }
}

impl From<Mei> for InformationElement {
    fn from(i: Mei) -> Self {
        InformationElement::Mei(i)
    }
}

impl IEs for Mei {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MEI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend(tbcd_encode(&self.mei));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Mei, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Mei {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Mei::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4..(data.length + 4) as usize].try_into() {
                    Ok(i) => data.mei = tbcd_decode(i),
                    Err(_) => return Err(GTPV2Error::IEIncorrect(MEI)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(MEI))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(MEI))
        }
    }

    fn len(&self) -> usize {
        MEI_LENGTH + MIN_IE_SIZE
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
fn mei_ie_unmarshal_test() {
    let encoded_ie: [u8; 12] = [
        0x4b, 0x00, 0x08, 0x00, 0x68, 0x67, 0x84, 0x40, 0x10, 0x23, 0x03, 0x30,
    ];
    let test_struct = Mei {
        t: MEI,
        length: MEI_LENGTH as u16,
        ins: 0,
        mei: "8676480401323003".to_string(),
    };
    let i = Mei::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn mei_ie_marshal_test() {
    let encoded_ie: [u8; 12] = [
        0x4b, 0x00, 0x08, 0x00, 0x68, 0x67, 0x84, 0x40, 0x10, 0x23, 0x03, 0x30,
    ];
    let test_struct = Mei {
        t: MEI,
        length: MEI_LENGTH as u16,
        ins: 0,
        mei: "8676480401323003".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
