// Monitoring Event Extension Information IE - according to 3GPP TS 29.274 V17.10.0 (2023-12), 3GPP TS 29.336 V15.8.0 ()

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
    pub ext_scef_ref_id: Option<u64>,
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
            ext_scef_ref_id: None,
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
        buffer_ie.push(MONITEVENTEXTINFO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flag = (self.ext_scef_ref_id.is_some() as u8) << 1 | (self.rmplrt.is_some() as u8);
        buffer_ie.push(flag);
        buffer_ie.extend_from_slice(&self.scef_ref_id.to_be_bytes());
        buffer_ie.push(self.scef_id.len() as u8);
        buffer_ie.extend_from_slice(self.scef_id.as_bytes());
        if let Some(i) = self.rmplrt {
            buffer_ie.extend_from_slice(&i.to_be_bytes()[1..]);
        }
        if let Some(i) = self.ext_scef_ref_id {
            buffer_ie.extend_from_slice(&i.to_be_bytes());
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<MonitoringEventExtensionInfo, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = MonitoringEventExtensionInfo {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Default::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.scef_ref_id = u32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]);
                let mut cursor: usize = 9;
                {
                    let len: usize = buffer[9] as usize;
                    if buffer.len() >= cursor + len {
                        cursor += 1;
                        data.scef_id =
                            match String::from_utf8(buffer[cursor..(cursor + len)].to_vec()) {
                                Ok(i) => i,
                                Err(_) => return Err(GTPV2Error::IEIncorrect(MONITEVENTEXTINFO)),
                            };
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO));
                    }
                }
                {
                    if buffer[4] & 0x01 == 0x01 {
                        if buffer.len() >= cursor + 3 {
                            data.rmplrt = Some(u32::from_be_bytes([
                                0x00,
                                buffer[cursor],
                                buffer[cursor + 1],
                                buffer[cursor + 2],
                            ]));
                            cursor += 3;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO));
                        }
                    }
                }
                {
                    if buffer[4] & 0x02 == 0x02 {
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
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO));
                        }
                    }
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
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn monitoringeventextinfo_ie_unmarshal_test() {
    let encoded_ie: [u8; 32] = [
        0xce, 0x00, 0x1c, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x65, 0x78, 0x61, 0x6d, 0x70,
        0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let test_struct = MonitoringEventExtensionInfo {
        t: MONITEVENTEXTINFO,
        length: 28,
        ins: 0,
        scef_ref_id: 0,
        scef_id: String::from("example.com"),
        rmplrt: Some(0xff),
        ext_scef_ref_id: Some(0xffffffffffffffff),
    };
    let i = MonitoringEventExtensionInfo::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn monitoringeventextinfo_ie_marshal_test() {
    let encoded_ie: [u8; 32] = [
        0xce, 0x00, 0x1c, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x65, 0x78, 0x61, 0x6d, 0x70,
        0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];
    let test_struct = MonitoringEventExtensionInfo {
        t: MONITEVENTEXTINFO,
        length: 28,
        ins: 0,
        scef_ref_id: 0,
        scef_id: String::from("example.com"),
        rmplrt: Some(0xff),
        ext_scef_ref_id: Some(0xffffffffffffffff),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn monitoringeventextinfo_ie_wrong_scefid_length() {
    let encoded_ie: [u8; 10] = [0xce, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04];
    let i = MonitoringEventExtensionInfo::unmarshal(&encoded_ie);
    assert_eq!(i, Err(GTPV2Error::IEInvalidLength(MONITEVENTEXTINFO)));
}
