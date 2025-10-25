// Additional Flags for SRVCC IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Additional Flags for SRVCC IE TL

pub const ADDFLAGS_SRVCC: u8 = 160;
pub const ADDFLAGS_SRVCC_LENGTH: usize = 1;

// MBMS Flags IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalFlagsSrvcc {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ics: bool, // ICS (IMS Centralized Service): This flag indicates that UE supports ICS specific service as specified in 3GPP TS 23.292 [47]
    pub vf: bool, // VF (vSRVCC Flag): This flag indicates that the user is subscribed to the vSRVCC.
}

impl Default for AdditionalFlagsSrvcc {
    fn default() -> Self {
        AdditionalFlagsSrvcc {
            t: ADDFLAGS_SRVCC,
            length: ADDFLAGS_SRVCC_LENGTH as u16,
            ins: 0,
            ics: false,
            vf: false,
        }
    }
}

impl From<AdditionalFlagsSrvcc> for InformationElement {
    fn from(i: AdditionalFlagsSrvcc) -> Self {
        InformationElement::AdditionalFlagsSrvcc(i)
    }
}

impl IEs for AdditionalFlagsSrvcc {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ADDFLAGS_SRVCC);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags = match (self.vf, self.ics) {
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
        if buffer.len() >= ADDFLAGS_SRVCC_LENGTH + MIN_IE_SIZE {
            let mut data = AdditionalFlagsSrvcc {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..AdditionalFlagsSrvcc::default()
            };
            match buffer[4] & 0b11 {
                0b00 => {
                    data.vf = false;
                    data.ics = false;
                }
                0b10 => {
                    data.vf = true;
                    data.ics = false;
                }
                0b01 => {
                    data.vf = false;
                    data.ics = true;
                }
                0b11 => {
                    data.vf = true;
                    data.ics = true;
                }
                _ => {
                    return Err(GTPV2Error::IEIncorrect(ADDFLAGS_SRVCC));
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ADDFLAGS_SRVCC))
        }
    }

    fn len(&self) -> usize {
        ADDFLAGS_SRVCC_LENGTH + MIN_IE_SIZE
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
fn additionalflags_srvcc_ie_marshal_test() {
    let encoded: [u8; 5] = [0xa0, 0x00, 0x01, 0x00, 0x02];
    let decoded = AdditionalFlagsSrvcc {
        t: ADDFLAGS_SRVCC,
        length: ADDFLAGS_SRVCC_LENGTH as u16,
        ins: 0,
        ics: false,
        vf: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn additionalflags_srvcc_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xa0, 0x00, 0x01, 0x00, 0x02];
    let decoded = AdditionalFlagsSrvcc {
        t: ADDFLAGS_SRVCC,
        length: ADDFLAGS_SRVCC_LENGTH as u16,
        ins: 0,
        ics: false,
        vf: true,
    };
    assert_eq!(AdditionalFlagsSrvcc::unmarshal(&encoded).unwrap(), decoded);
}
