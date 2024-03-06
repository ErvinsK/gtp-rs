// Packet TMSI (P-TMSI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// P-TMSI Type

pub const PTMSI: u8 = 111;
pub const PTMSI_LENGTH: usize = 4;

// P-TMSI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ptmsi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ptmsi: u32,
}

impl Default for Ptmsi {
    fn default() -> Self {
        Ptmsi {
            t: PTMSI,
            length: PTMSI_LENGTH as u16,
            ins: 0,
            ptmsi: 0,
        }
    }
}

impl From<Ptmsi> for InformationElement {
    fn from(i: Ptmsi) -> Self {
        InformationElement::Ptmsi(i)
    }
}

impl IEs for Ptmsi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PTMSI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.ptmsi.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PTMSI_LENGTH {
            let mut data = Ptmsi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ptmsi: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..Ptmsi::default()
            };
            data.ins = buffer[3];
            data.ptmsi = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PTMSI))
        }
    }

    fn len(&self) -> usize {
        PTMSI_LENGTH + MIN_IE_SIZE
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
fn ptmsi_ie_marshal_test() {
    let encoded: [u8; 8] = [0x6f, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff];
    let decoded = Ptmsi {
        t: PTMSI,
        length: PTMSI_LENGTH as u16,
        ins: 0,
        ptmsi: 0xffffffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ptmsi_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x6f, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff];
    let decoded = Ptmsi {
        t: PTMSI,
        length: PTMSI_LENGTH as u16,
        ins: 0,
        ptmsi: 0xffffffff,
    };
    assert_eq!(Ptmsi::unmarshal(&encoded).unwrap(), decoded);
}
