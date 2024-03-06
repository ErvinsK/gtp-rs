// Trusted WLAN Mode Indication (TWMI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// TWMI IE TV

pub const TWMI: u8 = 174;
pub const TWMI_LENGTH: usize = 1;

// TWMI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Twmi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcm: bool, // MCM (Multiple-connection mode Indication): if this bit is set to 1, it indicates that the Multiple-connection mode is used.
    pub scm: bool, // SCM (Single-connection mode Indication): if this bit is set to 1, it indicates that the Single-connection mode is used.
}

impl Default for Twmi {
    fn default() -> Self {
        Twmi {
            t: TWMI,
            length: TWMI_LENGTH as u16,
            ins: 0,
            mcm: false,
            scm: false,
        }
    }
}

impl From<Twmi> for InformationElement {
    fn from(i: Twmi) -> Self {
        InformationElement::Twmi(i)
    }
}

impl IEs for Twmi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TWMI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.mcm, self.scm) {
            (false, false) => buffer_ie.push(0x00),
            (false, true) => buffer_ie.push(0x01),
            (true, false) => buffer_ie.push(0x02),
            (true, true) => buffer_ie.push(0x03),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= TWMI_LENGTH + MIN_IE_SIZE {
            let mut data = Twmi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Twmi::default()
            };
            match buffer[4] & 0x03 {
                0 => (data.mcm, data.scm) = (false, false),
                1 => (data.mcm, data.scm) = (false, true),
                2 => (data.mcm, data.scm) = (true, false),
                3 => (data.mcm, data.scm) = (true, true),
                _ => return Err(GTPV2Error::IEIncorrect(TWMI)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TWMI))
        }
    }

    fn len(&self) -> usize {
        TWMI_LENGTH + MIN_IE_SIZE
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
fn twmi_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xae, 0x00, 0x01, 0x00, 0x02];
    let decoded = Twmi {
        t: TWMI,
        length: TWMI_LENGTH as u16,
        ins: 0,
        mcm: true,
        scm: false,
    };
    let i = Twmi::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn twmi_ie_marshal_test() {
    let encoded: [u8; 5] = [0xae, 0x00, 0x01, 0x00, 0x02];
    let decoded = Twmi {
        t: TWMI,
        length: TWMI_LENGTH as u16,
        ins: 0,
        mcm: true,
        scm: false,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
