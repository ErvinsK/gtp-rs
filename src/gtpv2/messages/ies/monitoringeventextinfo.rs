// Monitoring Event Extension Information IE - according to 3GPP TS 29.274 V15.9.0 (2019-09), 3GPP TS 29.336 V15.8.0 ()

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Monitoring Event Extension Information IE Type

pub const MONITEVENTEXTINFO: u8 = 206;

// Monitoring Event Extension Information IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoringEventExtensionInfo {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub scef_ref_id: u32,
    pub scef_id: String,
    pub rmplrt: Option<u32>, // Remaining Minimum Periodic Location Report Time
}

impl Default for MonitoringEventExtensionInfo {
    fn default() -> MonitoringEventExtensionInfo {
        MonitoringEventExtensionInfo {
            t: MONITEVENTEXTINFO,
            length: 6,
            ins: 0,
            scef_ref_id: 0,
            scef_id: String::new(),
            rmplrt: None,
        }
    }
}

impl From<MonitoringEventExtensionInfo> for InformationElement {
    fn from(i: MonitoringEventExtensionInfo) -> Self {
        InformationElement::MonitoringEventExtensionInfo(i)
    }
}

impl IEs for MonitoringEventExtensionInfo {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.rmplrt {
            Some(_) => buffer_ie.push(0x01),
            None => buffer_ie.push(0x00),
        }
        buffer_ie.extend_from_slice(&self.scef_ref_id.to_be_bytes());
        buffer_ie.push(self.scef_id.len() as u8);
        buffer_ie.extend_from_slice(self.scef_id.as_bytes());
        if let Some(i) = self.rmplrt {
            buffer_ie.extend_from_slice(&i.to_be_bytes()[1..]);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<MonitoringEventExtensionInfo, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = MonitoringEventExtensionInfo {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3] & 0x0f;
            if check_tliv_ie_buffer(data.length, buffer) {
                data.scef_ref_id = u32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]);
                if buffer.len() >= 9 + (buffer[9] as usize) {
                    data.scef_id =
                        String::from_utf8(buffer[10..(10 + (buffer[9] as usize))].to_vec())
                            .unwrap();
                    if buffer[4] == 0x01 {
                        if buffer.len() >= 13 + (buffer[9] as usize) {
                            data.rmplrt = Some(u32::from_be_bytes([
                                0x00,
                                buffer[10 + (buffer[9] as usize)],
                                buffer[11 + (buffer[9] as usize)],
                                buffer[12 + (buffer[9] as usize)],
                            ]));
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO));
                        }
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO));
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO))
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
fn monitoringeventextinfo_ie_unmarshal_test() {
    let encoded_ie: [u8; 24] = [
        0xce, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x65, 0x78, 0x61, 0x6d, 0x70,
        0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0xff,
    ];
    let test_struct = MonitoringEventExtensionInfo {
        t: MONITEVENTEXTINFO,
        length: 20,
        ins: 0,
        scef_ref_id: 0,
        scef_id: String::from("example.com"),
        rmplrt: Some(0xff),
    };
    let i = MonitoringEventExtensionInfo::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn monitoringeventextinfo_ie_marshal_test() {
    let encoded_ie: [u8; 24] = [
        0xce, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x65, 0x78, 0x61, 0x6d, 0x70,
        0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0xff,
    ];
    let test_struct = MonitoringEventExtensionInfo {
        t: MONITEVENTEXTINFO,
        length: 20,
        ins: 0,
        scef_ref_id: 0,
        scef_id: String::from("example.com"),
        rmplrt: Some(0xff),
    };
    //println!("{:#04x?}", String::from("example.com").as_bytes());
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn monitoringeventextinfo_ie_wrong_scefid_length() {
    let encoded_ie: [u8; 10] = [0xce, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04];
    let i = MonitoringEventExtensionInfo::unmarshal(&encoded_ie);
    assert_eq!(i, Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO)));
}
