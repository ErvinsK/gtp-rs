// Monitoring Event Information IE - according to 3GPP TS 29.274 V17.10.0 (2023-12), 3GPP TS 29.336 V15.8.0 ()

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
    pub nsui: bool, // Notify SCEF when UE becomes Idle Flag
    pub nsur: bool, // Notify SCEF when UE becomes Reachable Flag
    pub nscf: bool, // Notify SCEF about Communication Failure Flag
    pub scef_ref_id: u32,
    pub scef_id: String,
    pub rem_nbr_reports: u16, // Remaining Minimum Periodic Location Report Time
    pub ext_scef_ref_id: Option<u64>, // Extended SCEF Reference ID
}

impl Default for MonitoringEventInformation {
    fn default() -> MonitoringEventInformation {
        MonitoringEventInformation {
            t: MONITOREVENTINFO,
            length: 0,
            ins: 0,
            nsui: false,
            nsur: false,
            nscf: false,
            scef_ref_id: 0,
            scef_id: String::new(),
            rem_nbr_reports: 0,
            ext_scef_ref_id: None,
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
        buffer_ie.push(MONITOREVENTINFO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        let flag = ((self.ext_scef_ref_id.is_some() as u8) << 7)
            | ((self.nscf as u8) << 6)
            | ((self.nsui as u8) << 5)
            | ((self.nsur as u8) << 4)
            | (self.ins & 0x0f);
        buffer_ie.push(flag);
        buffer_ie.extend_from_slice(&self.scef_ref_id.to_be_bytes());
        buffer_ie.push(self.scef_id.len() as u8);
        buffer_ie.extend_from_slice(self.scef_id.as_bytes());
        buffer_ie.extend_from_slice(&self.rem_nbr_reports.to_be_bytes());
        if let Some(i) = self.ext_scef_ref_id {
            buffer_ie.extend_from_slice(&i.to_be_bytes());
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 5 {
            let mut data = MonitoringEventInformation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                nscf: buffer[3] & 0x40 == 0x40,
                nsui: buffer[3] & 0x20 == 0x20,
                nsur: buffer[3] & 0x10 == 0x10,
                ins: buffer[3] & 0x0f,
                scef_ref_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..MonitoringEventInformation::default()
            };
            let mut cursor: usize = 8;
            let len = buffer[cursor] as usize;
            cursor += 1;
            if buffer.len() >= cursor + len {
                if let Ok(scef_id) = String::from_utf8(buffer[cursor..cursor + len].to_vec()) {
                    data.scef_id = scef_id;
                    cursor += len;
                } else {
                    return Err(GTPV2Error::IEIncorrect(MONITOREVENTINFO));
                }
                if buffer.len() >= cursor + 2 {
                    data.rem_nbr_reports = u16::from_be_bytes([buffer[cursor], buffer[cursor + 1]]);
                    cursor += 2;
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MONITOREVENTINFO));
                }
                if buffer[3] & 0x80 == 0x80 {
                    if buffer.len() >= cursor + 8 {
                        data.ext_scef_ref_id = Some(u64::from_be_bytes([
                            buffer[cursor],
                            buffer[cursor + 1],
                            buffer[cursor + 2],
                            buffer[cursor + 3],
                            buffer[cursor + 4],
                            buffer[cursor + 5],
                            buffer[cursor + 6],
                            buffer[cursor + 7],
                        ]));
                        Ok(data)
                    } else {
                        Err(GTPV2Error::IEInvalidLength(MONITOREVENTINFO))
                    }
                } else {
                    Ok(data)
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
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn monitoringeventinfo_ie_marshal_test() {
    let encoded_ie: [u8; 35] = [
        0xbd, 0x00, 0x1f, 0xa0, 0x00, 0x00, 0xff, 0xff, 0x10, 0x73, 0x63, 0x65, 0x66, 0x2e, 0x65,
        0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0xff, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xee, 0xaa,
    ];
    let test_struct = MonitoringEventInformation {
        t: MONITOREVENTINFO,
        length: 31,
        ins: 0,
        nscf: false,
        nsui: true,
        nsur: false,
        scef_ref_id: 0xffff,
        scef_id: String::from("scef.example.com"),
        rem_nbr_reports: 0xff,
        ext_scef_ref_id: Some(0xffffeeaa),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn monitoringeventinfo_ie_unmarshal_test() {
    let encoded_ie: [u8; 35] = [
        0xbd, 0x00, 0x1f, 0xa0, 0x00, 0x00, 0xff, 0xff, 0x10, 0x73, 0x63, 0x65, 0x66, 0x2e, 0x65,
        0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0xff, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xee, 0xaa,
    ];
    let test_struct = MonitoringEventInformation {
        t: MONITOREVENTINFO,
        length: 31,
        ins: 0,
        nscf: false,
        nsui: true,
        nsur: false,
        scef_ref_id: 0xffff,
        scef_id: String::from("scef.example.com"),
        rem_nbr_reports: 0xff,
        ext_scef_ref_id: Some(0xffffeeaa),
    };
    let i = MonitoringEventInformation::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}
