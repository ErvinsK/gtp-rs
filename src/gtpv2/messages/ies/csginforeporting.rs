// CSG Information Reporting Action IE - according to 3GPP TS 29.247 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// CSG Information Reproting Action IE Type

pub const CSG_INFO_REPORT:u8 = 146;
pub const CSG_INFO_REPORT_LENGTH:usize = 1;

// CSG Information Reporting Action IE implementation 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSGInformationReportingAction {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub action:u8,      // Bit 1 – UCICSG: When set to '1', shall indicate to start reporting User CSG Info when the UE enters/leaves/access through the CSG Cell.
                        // Bit 2 – UCISHC: When set to '1', shall indicate to start reporting User CSG Info when the UE enters/leaves/access through Subscribed Hybrid Cell.
                        // Bit 3 – UCIUHC: When set to '1', shall indicate to start Reporting User CSG Info when the UE enters/leaves/access through Unsubscribed Hybrid Cell.
                        // All the bits 1 to 3 shall be set to 0 to stop reporting User CSG Info.
}

impl Default for CSGInformationReportingAction {
    fn default() -> CSGInformationReportingAction {
        CSGInformationReportingAction { t: CSG_INFO_REPORT, length:1, ins:0, action:0 }        
    }
}

impl From<CSGInformationReportingAction> for InformationElement {
    fn from(i: CSGInformationReportingAction) -> Self {
        InformationElement::CSGInformationReportingAction(i)
    }
}

impl IEs for CSGInformationReportingAction {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.action);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=CSG_INFO_REPORT_LENGTH+MIN_IE_SIZE {
            let mut data = CSGInformationReportingAction::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.action = buffer[4] & 0b111;
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CSG_INFO_REPORT))
        }
    }
    
    fn len (&self) -> usize {
       CSG_INFO_REPORT_LENGTH+MIN_IE_SIZE
    }
}

#[test]
fn csg_info_reporting_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x92, 0x00, 0x01, 0x00, 0x07];
    let decoded = CSGInformationReportingAction { t:CSG_INFO_REPORT, length: 1, ins: 0, action: 7 };
    let i = CSGInformationReportingAction::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn csg_info_reporting_ie_marshal_test () {
    let encoded:[u8;5]=[0x92, 0x00, 0x01, 0x00, 0x07];
    let decoded = CSGInformationReportingAction { t:CSG_INFO_REPORT, length: 1, ins: 0, action: 7 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}