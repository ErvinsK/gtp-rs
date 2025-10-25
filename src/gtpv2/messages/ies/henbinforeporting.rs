// H(e)NB Information Reporting IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// H(e)NB Information Reporting IE TV

pub const HENB_INFO: u8 = 165;
pub const HENB_INFO_LENGTH: usize = 1;

// H(e)NB Information Reporting IE implementation

// FTI: When set to "1" (true), shall indicate to start reporting H(e)NB local IP address and UDP port number information change when the UE moves from (e)NB to H(e)NB,
// from H(e)NB to another H(e)NB with a fixed network backhaul change, or from H(e)NB to (e)NB.
// FTI shall be set to 0 (false) to stop reporting H(e)NB local IP address and UDP port number information change.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HenbInfoReporting {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub fti: bool,
}

impl Default for HenbInfoReporting {
    fn default() -> HenbInfoReporting {
        HenbInfoReporting {
            t: HENB_INFO,
            length: HENB_INFO_LENGTH as u16,
            ins: 0,
            fti: false,
        }
    }
}

impl From<HenbInfoReporting> for InformationElement {
    fn from(i: HenbInfoReporting) -> Self {
        InformationElement::HenbInfoReporting(i)
    }
}

impl IEs for HenbInfoReporting {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(HENB_INFO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.fti {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= HENB_INFO_LENGTH + MIN_IE_SIZE {
            let data = HenbInfoReporting {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                fti: match buffer[4] {
                    0 => false,
                    1 => true,
                    _ => return Err(GTPV2Error::IEIncorrect(HENB_INFO)),
                },
                ..HenbInfoReporting::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(HENB_INFO))
        }
    }

    fn len(&self) -> usize {
        HENB_INFO_LENGTH + MIN_IE_SIZE
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
fn henb_info_reporting_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xa5, 0x00, 0x01, 0x00, 0x01];
    let decoded = HenbInfoReporting {
        t: HENB_INFO,
        length: HENB_INFO_LENGTH as u16,
        ins: 0,
        fti: true,
    };
    let i = HenbInfoReporting::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn upfsif_ie_marshal_test() {
    let encoded: [u8; 5] = [0xa5, 0x00, 0x01, 0x00, 0x01];
    let decoded = HenbInfoReporting {
        t: HENB_INFO,
        length: HENB_INFO_LENGTH as u16,
        ins: 0,
        fti: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
