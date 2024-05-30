// MBMS Service Area IE - according to 3GPP TS 29.274 V17.10.0 (2023-12), 3GPP TS 29.061 V15.3.0 ()

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Service Area IE Type

pub const MBMSSA: u8 = 139;

// MBMS Service Area IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsServiceArea {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mbms_sa: Vec<u16>,
}

impl Default for MbmsServiceArea {
    fn default() -> MbmsServiceArea {
        MbmsServiceArea {
            t: MBMSSA,
            length: 0,
            ins: 0,
            mbms_sa: vec![0],
        }
    }
}

impl From<MbmsServiceArea> for InformationElement {
    fn from(i: MbmsServiceArea) -> Self {
        InformationElement::MbmsSa(i)
    }
}

impl IEs for MbmsServiceArea {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMSSA);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.mbms_sa.len() {
            0 => {
                buffer_ie.push(0x00);
                buffer_ie.push(0x00);
                buffer_ie.push(0x00);
            }
            _ => {
                buffer_ie.push((self.mbms_sa.len() - 1) as u8);
                buffer_ie.extend(self.mbms_sa.iter().flat_map(|&i| i.to_be_bytes().to_vec()));
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<MbmsServiceArea, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = MbmsServiceArea {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..MbmsServiceArea::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                if buffer.len() >= 5 + (buffer[4] as usize + 1) * 2 {
                    data.mbms_sa = buffer[5..(5 + (buffer[4] as usize + 1) * 2)]
                        .chunks_exact(2)
                        .map(|x| u16::from_be_bytes([x[0], x[1]]))
                        .collect();
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MBMSSA));
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(MBMSSA))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMSSA))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
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
fn mbmssa_ie_unmarshal_test() {
    let encoded_ie: [u8; 9] = [0x8b, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0xff, 0xff];
    let test_struct = MbmsServiceArea {
        t: MBMSSA,
        length: 5,
        ins: 0,
        mbms_sa: vec![0, 0xffff],
    };
    let i = MbmsServiceArea::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn mbmssa_ie_marshal_test() {
    let encoded_ie: [u8; 9] = [0x8b, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0xff, 0xff];
    let test_struct = MbmsServiceArea {
        t: MBMSSA,
        length: 5,
        ins: 0,
        mbms_sa: vec![0, 0xffff],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn mbmssa_ie_wrong_msmssa_length() {
    let encoded_ie: [u8; 6] = [0x8b, 0x00, 0x03, 0x00, 0x00, 0x00];
    let i = MbmsServiceArea::unmarshal(&encoded_ie);
    assert_eq!(i, Err(GTPV2Error::IEInvalidLength(MBMSSA)));
}
