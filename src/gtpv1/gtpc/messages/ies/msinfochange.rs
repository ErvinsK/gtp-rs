// MS Info Change Reporting Action IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// MS Info Change Reproting Action IE TL

pub const MSINFO_CHANGE: u8 = 181;
pub const MSINFO_CHANGE_LENGTH: u16 = 1;

// MS Info Change Reporting Action IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MSInfoChangeReportingAction {
    pub t: u8,
    pub length: u16,
    pub action: u8, //  0 - "Stop Reporting", 1 - "Start Reporting CGI/SAI", 2 - "Start Reporting RAI", other values reserved
}

impl Default for MSInfoChangeReportingAction {
    fn default() -> MSInfoChangeReportingAction {
        MSInfoChangeReportingAction {
            t: MSINFO_CHANGE,
            length: MSINFO_CHANGE_LENGTH,
            action: 0,
        }
    }
}

impl IEs for MSInfoChangeReportingAction {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.action);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= (MSINFO_CHANGE_LENGTH + 3) as usize {
            let mut data = MSInfoChangeReportingAction {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match buffer[3] {
                i if i <= 2 => data.action = buffer[3],
                _ => return Err(GTPV1Error::IEIncorrect),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        MSINFO_CHANGE_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn ms_info_change_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xb5, 0x00, 0x01, 0x01];
    let test_struct = MSInfoChangeReportingAction {
        t: MSINFO_CHANGE,
        length: MSINFO_CHANGE_LENGTH,
        action: 1,
    };
    let i = MSInfoChangeReportingAction::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn ms_info_change_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xb5, 0x00, 0x01, 0x01];
    let test_struct = MSInfoChangeReportingAction {
        t: MSINFO_CHANGE,
        length: MSINFO_CHANGE_LENGTH,
        action: 1,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
