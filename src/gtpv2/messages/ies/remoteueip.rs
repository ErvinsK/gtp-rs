// Remote UE IP IE - according to 3GPP TS 29.274 V17.10.0 (2023-12) and 3GPP TS 24.301 9.9.4.20

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{Ipv4Addr, Ipv6Addr};

// Remote UE IP IE Type

pub const REMOTE_UE_IP: u8 = 193;

// Remote IP Address Type Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteIpAddress {
    V4(Ipv4Addr), // 0x01
    V6(Ipv6Addr), // 0x02
    NonIp,        // 0x00
}

// Remote UE IP IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteUeIpInformation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ip: RemoteIpAddress,
}

impl Default for RemoteUeIpInformation {
    fn default() -> Self {
        RemoteUeIpInformation {
            t: REMOTE_UE_IP,
            length: 0,
            ins: 0,
            ip: RemoteIpAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

impl From<RemoteUeIpInformation> for InformationElement {
    fn from(i: RemoteUeIpInformation) -> Self {
        InformationElement::RemoteUeIpInformation(i)
    }
}

impl IEs for RemoteUeIpInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(REMOTE_UE_IP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.ip {
            RemoteIpAddress::V4(i) => {
                buffer_ie.push(0x01);
                buffer_ie.extend_from_slice(&i.octets());
            }
            RemoteIpAddress::V6(i) => {
                buffer_ie.push(0x02);
                buffer_ie.extend_from_slice(&i.octets()[..8]);
            }
            RemoteIpAddress::NonIp => buffer_ie.push(0x00),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = RemoteUeIpInformation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..RemoteUeIpInformation::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    0x01 => {
                        if data.length >= 5 {
                            data.ip = RemoteIpAddress::V4(Ipv4Addr::from([
                                buffer[5], buffer[6], buffer[7], buffer[8],
                            ]));
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP));
                        }
                    }
                    0x02 => {
                        if data.length >= 9 {
                            let mut v = vec![];
                            v.extend_from_slice(&buffer[5..13]);
                            v.append(&mut vec![0; 8]);
                            let v_slice = v.as_slice();
                            let array: [u8; 16] = match v_slice.try_into() {
                                Ok(i) => i,
                                Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP)),
                            };
                            data.ip = RemoteIpAddress::V6(Ipv6Addr::from(array));
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP));
                        }
                    }
                    0x00 => data.ip = RemoteIpAddress::NonIp,
                    _ => return Err(GTPV2Error::IEIncorrect(REMOTE_UE_IP)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP))
        }
    }

    fn len(&self) -> usize {
        (self.length + 4) as usize
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
fn remote_ue_ip_ie_nonip_unmarshal_test() {
    let encoded: [u8; 5] = [0xc1, 0x00, 0x01, 0x00, 0x00];
    let decoded = RemoteUeIpInformation {
        t: REMOTE_UE_IP,
        length: 1,
        ins: 0,
        ip: RemoteIpAddress::NonIp,
    };
    let i = RemoteUeIpInformation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn remote_ue_ip_ie_ipv4_unmarshal_test() {
    let encoded: [u8; 9] = [0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34];
    let decoded = RemoteUeIpInformation {
        t: REMOTE_UE_IP,
        length: 5,
        ins: 0,
        ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
    };
    let i = RemoteUeIpInformation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn remote_ue_ip_ie_ipv6_unmarshal_test() {
    let encoded: [u8; 13] = [
        0xc1, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = RemoteUeIpInformation {
        t: REMOTE_UE_IP,
        length: 9,
        ins: 0,
        ip: RemoteIpAddress::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
    };
    let i = RemoteUeIpInformation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn remote_ue_ip_ie_ipv4_marshal_test() {
    let encoded: [u8; 9] = [0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34];
    let decoded = RemoteUeIpInformation {
        t: REMOTE_UE_IP,
        length: 5,
        ins: 0,
        ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn remote_ue_ip_ie_ipv6_marshal_test() {
    let encoded: [u8; 13] = [
        0xc1, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = RemoteUeIpInformation {
        t: REMOTE_UE_IP,
        length: 9,
        ins: 0,
        ip: RemoteIpAddress::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn remote_ue_ip_ie_nonip_marshal_test() {
    let encoded: [u8; 5] = [0xc1, 0x00, 0x01, 0x00, 0x00];
    let decoded = RemoteUeIpInformation {
        t: REMOTE_UE_IP,
        length: 1,
        ins: 0,
        ip: RemoteIpAddress::NonIp,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
