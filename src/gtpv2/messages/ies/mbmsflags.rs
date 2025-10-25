// MBMB Flags IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MBMS Flags IE TL

pub const MBMS_FLAGS: u8 = 171;
pub const MBMS_FLAGS_LENGTH: usize = 1;

// MBMS Flags IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsFlags {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub msri: bool, // MSRI (MBMS Session Re-establishment Indication): if set to 1, this flag indicates that the MBMS Session Start Request message is used to re-establish an MBMS session.
    pub lmri: bool, // LMRI (Local MBMS Bearer Context Release Indication): if set to 1, this flag indicates that the MBMS Session Stop Request message is used to release the MBMS Bearer Context locally in the MME/SGSN (see 3GPP TS 23.007 [13]).
}

impl Default for MbmsFlags {
    fn default() -> Self {
        MbmsFlags {
            t: MBMS_FLAGS,
            length: MBMS_FLAGS_LENGTH as u16,
            ins: 0,
            msri: false,
            lmri: false,
        }
    }
}

impl From<MbmsFlags> for InformationElement {
    fn from(i: MbmsFlags) -> Self {
        InformationElement::MbmsFlags(i)
    }
}

impl IEs for MbmsFlags {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMS_FLAGS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags = match (self.lmri, self.msri) {
            (false, false) => 0b00,
            (true, false) => 0b10,
            (false, true) => 0b01,
            (true, true) => 0b11,
        };
        buffer_ie.push(flags);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MBMS_FLAGS_LENGTH + MIN_IE_SIZE {
            let mut data = MbmsFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..MbmsFlags::default()
            };
            match buffer[4] & 0b11 {
                0b00 => {
                    data.lmri = false;
                    data.msri = false;
                }
                0b10 => {
                    data.lmri = true;
                    data.msri = false;
                }
                0b01 => {
                    data.lmri = false;
                    data.msri = true;
                }
                0b11 => {
                    data.lmri = true;
                    data.msri = true;
                }
                _ => {
                    return Err(GTPV2Error::IEIncorrect(MBMS_FLAGS));
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMS_FLAGS))
        }
    }

    fn len(&self) -> usize {
        MBMS_FLAGS_LENGTH + MIN_IE_SIZE
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
fn mbms_flags_ie_marshal_test() {
    let encoded: [u8; 5] = [0xab, 0x00, 0x01, 0x00, 0x01];
    let decoded = MbmsFlags {
        t: MBMS_FLAGS,
        length: MBMS_FLAGS_LENGTH as u16,
        ins: 0,
        msri: true,
        lmri: false,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mbms_flags_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xab, 0x00, 0x01, 0x00, 0x01];
    let decoded = MbmsFlags {
        t: MBMS_FLAGS,
        length: MBMS_FLAGS_LENGTH as u16,
        ins: 0,
        msri: true,
        lmri: false,
    };
    assert_eq!(MbmsFlags::unmarshal(&encoded).unwrap(), decoded);
}
