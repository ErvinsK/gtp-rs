// GSN Address IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use std::{net::{IpAddr, Ipv4Addr}};
use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// GSN Address Type

pub const GSN_ADDRESS:u8 = 133;

// GSN Address IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct GsnAddress {
    pub t:u8,
    pub length:u16,
    pub ip:IpAddr,
}

impl Default for GsnAddress {
    fn default() -> GsnAddress {
        GsnAddress {
            t:GSN_ADDRESS,
            length:4,
            ip:IpAddr::V4(Ipv4Addr::new(0,0,0,0)),
        }
    }
}

impl IEs for GsnAddress {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        match self.ip {
            IpAddr::V4(i) => buffer.extend_from_slice(&i.octets()),
            IpAddr::V6(i) => buffer.extend_from_slice(&i.octets()), 
        }
        set_tlv_ie_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<GsnAddress, GTPV1Error> {
        if buffer.len()>=3 {
            let mut data = GsnAddress::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            if check_tlv_ie_buffer(data.length, buffer) {
                match data.length {
                    0x04 => data.ip = IpAddr::from([buffer[3], buffer[4], buffer[5], buffer[6]]),
                    0x10 => {
                        if buffer.len()>=0x13 {
                            let mut dst = [0;16];
                            dst.copy_from_slice(&buffer[3..19]);
                            data.ip = IpAddr::from(dst);
                        } else { 
                            return Err(GTPV1Error::InvalidIELength);
                        }   
                        }
                    _ => return Err(GTPV1Error::IncorrectIE),
                }
                Ok(data)
            } else {
                Err(GTPV1Error::InvalidIELength)
            }
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }
    
    fn len(&self) -> usize {
        (self.length+3) as usize
    }
}

#[test]
fn gsn_address_ie_ipv4_unmarshal_test () {
    let encoded_ie:[u8;7]=[0x85, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00];
    let test_struct = GsnAddress { t:GSN_ADDRESS, length:4, ip:IpAddr::V4(Ipv4Addr::new(0,0,0,0)) };
    let i = GsnAddress::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn gsn_address_ie_ipv6_unmarshal_test () {
    use std::{net::{IpAddr, Ipv6Addr}};
    let encoded_ie:[u8;19]=[0x85, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let test_struct = GsnAddress { t:GSN_ADDRESS, length:16, ip:IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0)) };
    let i = GsnAddress::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn gsn_address_ie_ipv4_marshal_test () {
    let encoded_ie:[u8;7]=[0x85, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00];
    let test_struct = GsnAddress { t:GSN_ADDRESS, length:4, ip:IpAddr::V4(Ipv4Addr::new(0,0,0,0)) };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn gsn_address_ie_ipv6_marshal_test () {
    use std::{net::{IpAddr, Ipv6Addr}};
    let encoded_ie:[u8;19]=[0x85, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let test_struct = GsnAddress { t:GSN_ADDRESS, length:16, ip:IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0)) };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}