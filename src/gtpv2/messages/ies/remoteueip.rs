// Remote UE IP IE - according to 3GPP TS 29.247 V15.9.0 (2019-09) and 3GPP TS 24.301 9.9.4.20

use std::{net::{Ipv4Addr, Ipv6Addr}};
use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// Remote UE IP IE Type

pub const REMOTE_UE_IP:u8 = 193;

// Remote IP Address Type Enum

#[derive(Debug, Clone, PartialEq)]
pub enum RemoteIpAddress {
    V4(Ipv4Addr),           // 0x01
    V6(Ipv6Addr),           // 0x02
    NonIp,                  // 0x00
}

// Remote UE IP IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteUeIpInformation {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub ip:RemoteIpAddress,
}

impl Default for RemoteUeIpInformation {
    fn default() -> Self {
        RemoteUeIpInformation {
            t:REMOTE_UE_IP,
            length:9,
            ins:0,
            ip:RemoteIpAddress::V4(Ipv4Addr::new(0,0,0,0)),
        }
    }
}

impl IEs for RemoteUeIpInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.ip {
            RemoteIpAddress::V4(i) => {
                buffer_ie.push(0x01);
                buffer_ie.extend_from_slice(&i.octets());
            },
            RemoteIpAddress::V6(i)=> {
                buffer_ie.push(0x02);
                buffer_ie.extend_from_slice(&i.octets()[..8]);
            },
            RemoteIpAddress::NonIp => buffer_ie.push(0x00), 
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+1 {
            let mut data = RemoteUeIpInformation::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    0x01 => {
                        if data.length>=5 {
                            data.ip = RemoteIpAddress::V4(Ipv4Addr::from([buffer[5], buffer[6], buffer[7], buffer[8]]));
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP));
                        }
                    },
                    0x02 => {
                        if data.length>=9 {
                            let mut dst = [0;16];
                            let mut m = [0;8];
                            m.copy_from_slice(&buffer[5..13]);
                            dst.iter().skip(8).zip(m.iter()).map(|(i,j)| j+i);
                            data.ip = RemoteIpAddress::V6(Ipv6Addr::from(dst));
                        } else { 
                            return Err(GTPV2Error::IEInvalidLength(REMOTE_UE_IP));
                        }   
                        },
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
        (self.length+4) as usize
    }
}

#[test]
fn paa_ie_ipv4_unmarshal_test () {
    let encoded:[u8;9]=[0x4f,0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation { t:PAA, length:5, ins:0, ip: PdnAddress::V4(Ipv4Addr::new(0,0,0,0)) };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_ipv6_unmarshal_test () {
    let encoded:[u8;22]=[0x4f, 0x00, 0x12, 0x00, 0x02, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation { t:PAA, length:18, ins:0, ip: PdnAddress::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0), 128) };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_ipv46_unmarshal_test () {
    let encoded:[u8;26]=[0x4f, 0x00, 0x16, 0x00, 0x03, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation { t:PAA, length:22, ins:0, ip: PdnAddress::DualStack(Ipv4Addr::new(1,0,0,0), Ipv6Addr::new(1,0,0,0,0,0,0,0), 128) };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_non_ip_unmarshal_test () {
    let encoded:[u8;5]=[0x4f, 0x00, 0x01, 0x00, 0x04];
    let decoded = PdnAddressAllocation { t:PAA, length:1, ins:0, ip: PdnAddress::NonIp };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_ipv4_marshal_test () {
    let encoded:[u8;9]=[0x4f,0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation { t:PAA, length:5, ins:0, ip: PdnAddress::V4(Ipv4Addr::new(0,0,0,0)) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn paa_ie_ipv6_marshal_test () {
    let encoded:[u8;22]=[0x4f, 0x00, 0x12, 0x00, 0x02, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation { t:PAA, length:18, ins:0, ip: PdnAddress::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0), 128) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn paa_ie_ipv46_marshal_test () {
    let encoded:[u8;26]=[0x4f, 0x00, 0x16, 0x00, 0x03, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation { t:PAA, length:22, ins:0, ip: PdnAddress::DualStack(Ipv4Addr::new(1,0,0,0), Ipv6Addr::new(1,0,0,0,0,0,0,0), 128) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn paa_ie_nonip_marshal_test () {
    let encoded:[u8;5]=[0x4f, 0x00, 0x01, 0x00, 0x04];
    let decoded = PdnAddressAllocation { t:PAA, length:1, ins:0, ip: PdnAddress::NonIp };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}


