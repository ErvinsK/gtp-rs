// Change to Report Flags IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Change to Report Flags IE TV

pub const CHNG_TO_RPRT_FLAGS: u8 = 167;
pub const CHNG_TO_RPRT_FLAGS_LENGTH: usize = 1;

// Change to Report Flags IE implementation

// TZCR (Time Zone Change to Report): When set to 1 (true), this bit indicates that a UE Time Zone change still needs to be reported to the SGW/PGW.
// SNCR (Serving Network Change to Report): When set to 1 (true), this bit indicates that a Serving Network change still need to be reported to the SGW/PGW.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeToReportFlags {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub tzcr: bool,
    pub sncr: bool,
}

impl Default for ChangeToReportFlags {
    fn default() -> ChangeToReportFlags {
        ChangeToReportFlags {
            t: CHNG_TO_RPRT_FLAGS,
            length: CHNG_TO_RPRT_FLAGS_LENGTH as u16,
            ins: 0,
            tzcr: false,
            sncr: false,
        }
    }
}

impl From<ChangeToReportFlags> for InformationElement {
    fn from(i: ChangeToReportFlags) -> Self {
        InformationElement::ChangeToReportFlags(i)
    }
}

impl IEs for ChangeToReportFlags {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CHNG_TO_RPRT_FLAGS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.tzcr, self.sncr) {
            (false, false) => buffer_ie.push(0x00),
            (false, true) => buffer_ie.push(0x01),
            (true, false) => buffer_ie.push(0x02),
            (true, true) => buffer_ie.push(0x03),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= CHNG_TO_RPRT_FLAGS_LENGTH + MIN_IE_SIZE {
            let mut data = ChangeToReportFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..ChangeToReportFlags::default()
            };
            match buffer[4] & 0x03 {
                0 => (data.tzcr, data.sncr) = (false, false),
                1 => (data.tzcr, data.sncr) = (false, true),
                2 => (data.tzcr, data.sncr) = (true, false),
                3 => (data.tzcr, data.sncr) = (true, true),
                _ => return Err(GTPV2Error::IEIncorrect(CHNG_TO_RPRT_FLAGS)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CHNG_TO_RPRT_FLAGS))
        }
    }

    fn len(&self) -> usize {
        CHNG_TO_RPRT_FLAGS_LENGTH + MIN_IE_SIZE
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
fn change_to_report_flags_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xa7, 0x00, 0x01, 0x00, 0x02];
    let decoded = ChangeToReportFlags {
        t: CHNG_TO_RPRT_FLAGS,
        length: CHNG_TO_RPRT_FLAGS_LENGTH as u16,
        ins: 0,
        tzcr: true,
        sncr: false,
    };
    let i = ChangeToReportFlags::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn change_to_report_flags_ie_marshal_test() {
    let encoded: [u8; 5] = [0xa7, 0x00, 0x01, 0x00, 0x02];
    let decoded = ChangeToReportFlags {
        t: CHNG_TO_RPRT_FLAGS,
        length: CHNG_TO_RPRT_FLAGS_LENGTH as u16,
        ins: 0,
        tzcr: true,
        sncr: false,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
