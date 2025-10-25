// P-TMSI Signature IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// P-TMSI Signature Type

pub const PTMSI_SIG: u8 = 112;
pub const PTMSI_SIG_LENGTH: usize = 4;

// P-TMSI Signature IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtmsiSignature {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ptmsi_sig: u32,
}

impl Default for PtmsiSignature {
    fn default() -> Self {
        PtmsiSignature {
            t: PTMSI_SIG,
            length: PTMSI_SIG_LENGTH as u16,
            ins: 0,
            ptmsi_sig: 0,
        }
    }
}

impl From<PtmsiSignature> for InformationElement {
    fn from(i: PtmsiSignature) -> Self {
        InformationElement::PtmsiSignature(i)
    }
}

impl IEs for PtmsiSignature {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PTMSI_SIG);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.ptmsi_sig.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PTMSI_SIG_LENGTH {
            let data = PtmsiSignature {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ptmsi_sig: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..PtmsiSignature::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PTMSI_SIG))
        }
    }

    fn len(&self) -> usize {
        PTMSI_SIG_LENGTH + MIN_IE_SIZE
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
fn ptmsi_sig_ie_marshal_test() {
    let encoded: [u8; 8] = [0x70, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff];
    let decoded = PtmsiSignature {
        t: PTMSI_SIG,
        length: PTMSI_SIG_LENGTH as u16,
        ins: 0,
        ptmsi_sig: 0xffffffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ptmsi_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x70, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff];
    let decoded = PtmsiSignature {
        t: PTMSI_SIG,
        length: PTMSI_SIG_LENGTH as u16,
        ins: 0,
        ptmsi_sig: 0xffffffff,
    };
    assert_eq!(PtmsiSignature::unmarshal(&encoded).unwrap(), decoded);
}
