// Monitoring Event Information IE - according to 3GPP TS 29.274 V15.9.0 (2019-09), 3GPP TS 29.336 V15.8.0 ()

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Monitoring Event Information IE Type

pub const MONITOREVENTINFO: u8 = 189;

// Monitoring Event Information IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoringEventInformation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub nsui: bool,                                         // Notify SCEF when UE becomes Idle Flag
    pub nsur: bool,                                         // Notify SCEF when UE becomes Reachable Flag
    pub scef_ref_id: u32,
    pub scef_id: String,
    pub rem_nbr_reports: u16,                                // Remaining Minimum Periodic Location Report Time
}

impl Default for MonitoringEventInformation {
    fn default() -> MonitoringEventInformation {
        MonitoringEventInformation {
            t: MONITOREVENTINFO,
            length: 0,
            ins: 0,
            nsui: false,
            nsur: false,
            scef_ref_id: 0,
            scef_id: String::new(),
            rem_nbr_reports: 0,
        }
    }
}

impl From<MonitoringEventInformation> for InformationElement {
    fn from(i: MonitoringEventInformation) -> Self {
        InformationElement::MonitoringEventInformation(i)
    }
}

impl IEs for MonitoringEventInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        match (self.nsui, self.nsur) {
            (true, true) => buffer_ie.push(0x30 | self.ins),
            (true, false) => buffer_ie.push(0x20 | self.ins),
            (false, true) => buffer_ie.push(0x10 | self.ins),
            _ => buffer_ie.push(self.ins),
        }
        buffer_ie.extend_from_slice(&self.scef_ref_id.to_be_bytes());
        buffer_ie.push(self.scef_id.len() as u8);
        buffer_ie.extend_from_slice(self.scef_id.as_bytes());
        buffer_ie.extend_from_slice(&self.rem_nbr_reports.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE+5 {
            let mut data = MonitoringEventInformation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                nsui: buffer[3] & 0x20 == 0x20,
                nsur: buffer[3] & 0x10 == 0x10,
                ins: buffer[3] & 0x0f,
                scef_ref_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..Default::default()
            }; 
            let mut cursor:usize = 8; 
            let len = buffer[cursor] as usize;
            cursor += 1;            
            if buffer.len() >= cursor + len {
                if let Ok(scef_id) = String::from_utf8(buffer[cursor..cursor+len].to_vec()) {
                    data.scef_id = scef_id;
                    cursor += len;
                } else {
                    return Err(GTPV2Error::IEIncorrect(MONITOREVENTINFO));
                }
                if buffer.len() >= cursor + 2 {
                    data.rem_nbr_reports = u16::from_be_bytes([buffer[cursor], buffer[cursor+1]]);
                    Ok(data)
                } else {
                    Err(GTPV2Error::IEInvalidLength(MONITOREVENTINFO))
                }
            } else {
                Err(GTPV2Error::IEInvalidLength(MONITOREVENTINFO))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(MONITOREVENTINFO))
        }   
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn monitoringeventinfo_ie_marshal_test() {
    let encoded_ie: [u8; 27] = [
        0xbd,        0x00,        0x17,        0x20,        0x00,        0x00,        0xff,        0xff,
        0x10,        0x73,        0x63,        0x65,        0x66,        0x2e,        0x65,        0x78,
        0x61,        0x6d,        0x70,        0x6c,        0x65,        0x2e,        0x63,        0x6f,
        0x6d,        0x00,        0xff,
        ];
    let test_struct = MonitoringEventInformation {
        t: MONITOREVENTINFO,
        length: 23,
        ins: 0,
        nsui: true,
        nsur: false,
        scef_ref_id: 0xffff,
        scef_id: String::from("scef.example.com"),
        rem_nbr_reports: 0xff,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn monitoringeventinfo_ie_unmarshal_test() {
    let encoded_ie: [u8; 27] = [
        0xbd,        0x00,        0x17,        0x20,        0x00,        0x00,        0xff,        0xff,
        0x10,        0x73,        0x63,        0x65,        0x66,        0x2e,        0x65,        0x78,
        0x61,        0x6d,        0x70,        0x6c,        0x65,        0x2e,        0x63,        0x6f,
        0x6d,        0x00,        0xff,
        ];
    let test_struct = MonitoringEventInformation {
        t: MONITOREVENTINFO,
        length: 23,
        ins: 0,
        nsui: true,
        nsur: false,
        scef_ref_id: 0xffff,
        scef_id: String::from("scef.example.com"),
        rem_nbr_reports: 0xff,
    };
    let i = MonitoringEventInformation::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}
