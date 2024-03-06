// IP Address IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{IpAddr, Ipv4Addr};

// IP Address IE Type

pub const IP_ADDRESS: u8 = 74;

// IP Address IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpAddress {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ip: IpAddr,
}

impl Default for IpAddress {
    fn default() -> IpAddress {
        IpAddress {
            t: IP_ADDRESS,
            length: 0,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

impl From<IpAddress> for InformationElement {
    fn from(i: IpAddress) -> Self {
        InformationElement::IpAddress(i)
    }
}

impl IEs for IpAddress {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(IP_ADDRESS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.ip {
            IpAddr::V4(i) => buffer_ie.extend_from_slice(&i.octets()),
            IpAddr::V6(i) => buffer_ie.extend_from_slice(&i.octets()),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<IpAddress, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = IpAddress {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..IpAddress::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match data.length {
                    0x04 => data.ip = IpAddr::from([buffer[4], buffer[5], buffer[6], buffer[7]]),
                    0x10 => {
                        if buffer.len() >= 0x14 {
                            let mut dst = [0; 16];
                            dst.copy_from_slice(&buffer[4..20]);
                            data.ip = IpAddr::from(dst);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(IP_ADDRESS));
                        }
                    }
                    _ => return Err(GTPV2Error::IEIncorrect(IP_ADDRESS)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(IP_ADDRESS))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(IP_ADDRESS))
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
fn ip_address_ie_ipv4_unmarshal_test() {
    let encoded_ie: [u8; 8] = [0x4a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let test_struct = IpAddress {
        t: IP_ADDRESS,
        length: 4,
        ins: 0,
        ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    };
    let i = IpAddress::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn ip_address_ie_ipv6_unmarshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 20] = [
        0x4a, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = IpAddress {
        t: IP_ADDRESS,
        length: 16,
        ins: 0,
        ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
    };
    let i = IpAddress::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn ip_address_ie_ipv4_marshal_test() {
    let encoded_ie: [u8; 8] = [0x4a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let test_struct = IpAddress {
        t: IP_ADDRESS,
        length: 4,
        ins: 0,
        ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn ip_address_ie_ipv6_marshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 20] = [
        0x4a, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = IpAddress {
        t: IP_ADDRESS,
        length: 16,
        ins: 0,
        ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
