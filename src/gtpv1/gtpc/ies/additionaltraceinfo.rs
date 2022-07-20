// Additional Trace Info IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// Additional Trace Info IE Type

pub const ADDITIONALTRACEINFO:u8 = 162;
pub const ADDITIONALTRACEINFO_LENGTH:u16 = 9;

// Additional Trace Info IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct AdditionalTraceInfo {
    pub t:u8,
    pub length:u16,
    pub trace_ref:u32,
    pub trace_rec_session_ref:u16,
    pub triggering_events:u8,
    pub trace_depth:u8,
    pub interface_list:u8,
    pub trace_activity_control:u8,
}

impl Default for AdditionalTraceInfo {
    fn default() -> Self {
        AdditionalTraceInfo { t: ADDITIONALTRACEINFO, length:ADDITIONALTRACEINFO_LENGTH, trace_ref:0, trace_rec_session_ref:0, triggering_events:0, trace_depth:0, interface_list:0, trace_activity_control:0}
    }
}

impl IEs for AdditionalTraceInfo {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        {
            let slice = self.trace_ref.to_be_bytes();
            buffer.extend_from_slice(&slice[1..]); // Trace Reference is u24 not u32
        }
        buffer.extend_from_slice(&self.trace_rec_session_ref.to_be_bytes());
        buffer.push(self.triggering_events);
        buffer.push(self.trace_depth);
        buffer.push(self.interface_list);
        buffer.push(self.trace_activity_control);
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=(ADDITIONALTRACEINFO_LENGTH as usize +3) {
            let mut data=AdditionalTraceInfo::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.trace_ref = u32::from_be_bytes([0x00, buffer[3], buffer[4], buffer[5]]);
            data.trace_rec_session_ref = u16::from_be_bytes([buffer[6], buffer[7]]);
            data.triggering_events = buffer[8];
            data.trace_depth = buffer[9];
            data.interface_list = buffer[10];
            data.trace_activity_control = buffer[11];
            Ok(data)
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       (ADDITIONALTRACEINFO_LENGTH + 3) as usize 
    }

}

#[test]
fn additionaltraceinfo_ie_marshal_test () {
    let ie_marshalled:[u8;12]=[0xA2, 0x00, 0x09, 0x00, 0xff, 0xff, 0x00, 0xff, 0x01, 0x01, 0x01, 0x01];
    let ie_to_marshal = AdditionalTraceInfo { t:ADDITIONALTRACEINFO, length: ADDITIONALTRACEINFO_LENGTH, trace_ref:0xffff, trace_rec_session_ref:0xff, triggering_events: 1, trace_depth: 1, interface_list: 1, trace_activity_control:1 };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn additionaltraceinfo_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;12]=[0xA2, 0x00, 0x09, 0x00, 0xff, 0xff, 0x00, 0xff, 0x01, 0x01, 0x01, 0x01];
    let ie_unmarshalled = AdditionalTraceInfo { t:ADDITIONALTRACEINFO, length: ADDITIONALTRACEINFO_LENGTH, trace_ref:0xffff, trace_rec_session_ref:0xff, triggering_events: 1, trace_depth: 1, interface_list: 1, trace_activity_control:1 };
    assert_eq!(AdditionalTraceInfo::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}