// CSG Information Reporting Action IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// CSG Information Reproting Action IE Type

pub const CSG_INFO_REPORT: u8 = 195;

// CSG Information Reporting Action IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSGInformationReportingAction {
    pub t: u8,
    pub length: u16,
    pub action: u8, // Bit 1 – UCICSG: When set to '1', shall indicate to start reporting User CSG Info when the UE enters/leaves/access through the CSG Cell.
                    // Bit 2 – UCISHC: When set to '1', shall indicate to start reporting User CSG Info when the UE enters/leaves/access through Subscribed Hybrid Cell.
                    // Bit 3 – UCIUHC: When set to '1', shall indicate to start Reporting User CSG Info when the UE enters/leaves/access through Unsubscribed Hybrid Cell.
                    // All the bits 1 to 3 shall be set to 0 to stop reporting User CSG Info.
}

impl Default for CSGInformationReportingAction {
    fn default() -> CSGInformationReportingAction {
        CSGInformationReportingAction {
            t: CSG_INFO_REPORT,
            length: 1,
            action: 0,
        }
    }
}

impl IEs for CSGInformationReportingAction {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.action);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= 4 {
            let data = CSGInformationReportingAction {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                action: buffer[3] & 0b111,
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn csg_info_reporting_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xc3, 0x00, 0x01, 0x07];
    let test_struct = CSGInformationReportingAction {
        t: CSG_INFO_REPORT,
        length: 1,
        action: 7,
    };
    let i = CSGInformationReportingAction::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn csg_info_reporting_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xc3, 0x00, 0x01, 0x07];
    let test_struct = CSGInformationReportingAction {
        t: CSG_INFO_REPORT,
        length: 1,
        action: 7,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
