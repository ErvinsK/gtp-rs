// Local Distinguished Name (LDN) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// LDN IE Type

pub const LDN: u8 = 151;

// LDN IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ldn {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub name: String,
}

impl Default for Ldn {
    fn default() -> Self {
        Ldn {
            t: LDN,
            length: 0,
            ins: 0,
            name: "".to_string(),
        }
    }
}

impl From<Ldn> for InformationElement {
    fn from(i: Ldn) -> Self {
        InformationElement::Ldn(i)
    }
}

impl IEs for Ldn {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(LDN);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(self.name.as_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Ldn {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Ldn::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                let donor: Vec<u8> =
                    buffer[MIN_IE_SIZE..(MIN_IE_SIZE + data.length as usize)].to_vec();
                data.name = donor.iter().map(|x| *x as char).collect();
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(LDN))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(LDN))
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
fn ldn_ie_marshal_test() {
    let encoded: [u8; 56] = [
        0x97, 0x00, 0x34, 0x00, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x2e, 0x6e, 0x6f, 0x64, 0x65, 0x73,
        0x2e, 0x70, 0x67, 0x77, 0x2e, 0x73, 0x65, 0x2e, 0x65, 0x70, 0x63, 0x2e, 0x6d, 0x6e, 0x63,
        0x30, 0x35, 0x2e, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x2e, 0x33, 0x67, 0x70, 0x70, 0x6e,
        0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    ];

    let decoded = Ldn {
        t: LDN,
        length: 52,
        ins: 0,
        name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ldn_ie_unmarshal_test() {
    let encoded: [u8; 56] = [
        0x97, 0x00, 0x34, 0x00, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x2e, 0x6e, 0x6f, 0x64, 0x65, 0x73,
        0x2e, 0x70, 0x67, 0x77, 0x2e, 0x73, 0x65, 0x2e, 0x65, 0x70, 0x63, 0x2e, 0x6d, 0x6e, 0x63,
        0x30, 0x35, 0x2e, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x2e, 0x33, 0x67, 0x70, 0x70, 0x6e,
        0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    ];

    let decoded = Ldn {
        t: LDN,
        length: 52,
        ins: 0,
        name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
    };
    assert_eq!(Ldn::unmarshal(&encoded).unwrap(), decoded);
}
