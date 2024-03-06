// Change Reporting Action IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Change Reporting Action Type

pub const CHANGE_RPRT: u8 = 131;
pub const CHANGE_RPRT_LENGTH: usize = 1;

// Change Reporting Action implementation

//  Action                                                          Value (Decimal)
//  Stop Reporting                                                        0
//  Start Reporting CGI/SAI                                               1
//  Start Reporting RAI                                                   2
//  Start Reporting TAI                                                   3
//  Start Reporting ECGI                                                  4
//  Start Reporting CGI/SAI and RAI                                       5
//  Start Reporting TAI and ECGI                                          6
//  Start Reporting Macro eNodeB ID and Extended Macro eNodeB ID          7
//  Start Reporting TAI, Macro eNodeB ID and Extended Macro eNodeB ID     8
//  <spare>                                                             9-255

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeReportingAction {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub action: u8,
}

impl Default for ChangeReportingAction {
    fn default() -> Self {
        ChangeReportingAction {
            t: CHANGE_RPRT,
            length: CHANGE_RPRT_LENGTH as u16,
            ins: 0,
            action: 0,
        }
    }
}

impl From<ChangeReportingAction> for InformationElement {
    fn from(i: ChangeReportingAction) -> Self {
        InformationElement::ChangeReportingAction(i)
    }
}

impl IEs for ChangeReportingAction {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CHANGE_RPRT);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.action);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + CHANGE_RPRT_LENGTH {
            let data = ChangeReportingAction {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                action: buffer[4],
                ..ChangeReportingAction::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CHANGE_RPRT))
        }
    }

    fn len(&self) -> usize {
        CHANGE_RPRT_LENGTH + MIN_IE_SIZE
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
fn change_reporting_action_ie_marshal_test() {
    let encoded: [u8; 5] = [0x83, 0x00, 0x01, 0x00, 0x01];
    let decoded = ChangeReportingAction {
        t: CHANGE_RPRT,
        length: CHANGE_RPRT_LENGTH as u16,
        ins: 0,
        action: 1,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn change_reporting_action_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x83, 0x00, 0x01, 0x00, 0x01];
    let decoded = ChangeReportingAction {
        t: CHANGE_RPRT,
        length: CHANGE_RPRT_LENGTH as u16,
        ins: 0,
        action: 1,
    };
    assert_eq!(ChangeReportingAction::unmarshal(&encoded).unwrap(), decoded);
}
