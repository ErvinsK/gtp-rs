// F-TEID IE - according to 3GPP TS 29.247 V15.9.0 (2019-09)

use std::{net::{Ipv4Addr, Ipv6Addr}};
use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// F-TEID IE Type

pub const FTEID:u8 = 87;

// F-TEID IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Fteid {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub interface:u8,
    pub teid:u32,
    pub ipv4:Option<Ipv4Addr>,
    pub ipv6:Option<Ipv6Addr>,
}

impl Default for Fteid {
    fn default() -> Fteid {
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface:0,
            teid:0,
            ipv4:Some(Ipv4Addr::new(0,0,0,0)),
            ipv6:None,
        }
    }
}

impl IEs for Fteid {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.ipv4.is_some(),self.ipv6.is_some()) {
            (true,true) => buffer_ie.push( 0xC0 | self.interface),
            (true,false) => buffer_ie.push( 0x80 | self.interface),
            (false,true) => buffer_ie.push( 0x40 | self.interface),
            (false,false) => buffer_ie.push( 0xC0 | self.interface),
        }
        buffer_ie.extend_from_slice(&self.teid.to_be_bytes());
        match self.ipv4 {
            Some(i) => buffer_ie.extend_from_slice(&i.octets()),
            None => (),
        }
        match self.ipv6 {
            Some(i) => buffer_ie.extend_from_slice(&i.octets()),
            None => (),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE {
            let mut data = Fteid::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            if check_tliv_ie_buffer(data.length, buffer) {
                data.interface = buffer[4] & 0x3f;
                data.teid = u32::from_be_bytes([buffer[5],buffer[6],buffer[7],buffer[8]]);
                match (buffer[4]>>7, (buffer[4]>>6) & 0x01) {
                    (1,1) => {
                        data.ipv4 = Some(Ipv4Addr::from([buffer[9], buffer[10], buffer[11], buffer[12]]));
                        let mut dst = [0;16];
                        dst.copy_from_slice(&buffer[13..29]);
                        data.ipv6 = Some(Ipv6Addr::from(dst));
                    },
                    (1,0) => {
                        data.ipv4 = Some(Ipv4Addr::from([buffer[9], buffer[10], buffer[11], buffer[12]]));
                        data.ipv6 = None;
                        },
                    (0,1) => {
                        data.ipv4 = None;
                        let mut dst = [0;16];
                        dst.copy_from_slice(&buffer[9..25]);
                        data.ipv6 = Some(Ipv6Addr::from(dst));
                    },
                    _ => return Err(GTPV2Error::IEIncorrect),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength)
            }
        } else {
            Err(GTPV2Error::IEInvalidLength)
        }
    }
    
    fn len(&self) -> usize {
        (self.length+4) as usize
    }
}

#[test]
fn fteid_ie_ipv4_unmarshal_test () {
    let encoded:[u8;13]=[0x57, 0x00, 0x09, 0x00, 0x86, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8];
    let decoded = Fteid { t:FTEID, length:9, ins:0, interface:6, teid: 0x27892f70, ipv4: Some(Ipv4Addr::new(139,7,133,184)), ipv6:None };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fteid_ie_ipv6_unmarshal_test () {
    let encoded:[u8;25]=[0x57, 0x00, 0x15, 0x00, 0x46, 0x27, 0x89, 0x2f, 0x70, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
    let decoded = Fteid { t:FTEID, length:0x15, ins:0, interface:6, teid: 0x27892f70, ipv4: None, ipv6:Some(Ipv6Addr::new(1,0,0,0,0,0,0,0)) };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fteid_ie_ipv46_unmarshal_test () {
    let encoded:[u8;29]=[0x57, 0x00, 0x19, 0x00, 0xc6, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
    let decoded = Fteid { t:FTEID, length:0x19, ins:0, interface:6, teid: 0x27892f70, ipv4: Some(Ipv4Addr::new(139,7,133,184)), ipv6:Some(Ipv6Addr::new(1,0,0,0,0,0,0,0)) };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fteid_ie_wrong_flags_unmarshal_test () {
    let encoded:[u8;29]=[0x57, 0x00, 0x19, 0x00, 0x06, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
    //let decoded = Fteid { t:FTEID, length:0x19, ins:0, interface:6, teid: 0x27892f70, ipv4: Some(Ipv4Addr::new(139,7,133,184)), ipv6:Some(Ipv6Addr::new(1,0,0,0,0,0,0,0)) };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i, Err(GTPV2Error::IEIncorrect));
}

#[test]
fn fteid_ie_ipv4_marshal_test () {
    let encoded:[u8;13]=[0x57, 0x00, 0x09, 0x00, 0x86, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8];
    let decoded = Fteid { t:FTEID, length:9, ins:0, interface:6, teid: 0x27892f70, ipv4: Some(Ipv4Addr::new(139,7,133,184)), ipv6:None };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn fteid_ie_ipv6_marshal_test () {
    let encoded:[u8;25]=[0x57, 0x00, 0x15, 0x00, 0x46, 0x27, 0x89, 0x2f, 0x70, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
    let decoded = Fteid { t:FTEID, length:0x15, ins:0, interface:6, teid: 0x27892f70, ipv4: None, ipv6:Some(Ipv6Addr::new(1,0,0,0,0,0,0,0)) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn fteid_ie_ipv46_marshal_test () {
    let encoded:[u8;29]=[0x57, 0x00, 0x19, 0x00, 0xc6, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];
    let decoded = Fteid { t:FTEID, length:0x19, ins:0, interface:6, teid: 0x27892f70, ipv4: Some(Ipv4Addr::new(139,7,133,184)), ipv6:Some(Ipv6Addr::new(1,0,0,0,0,0,0,0)) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}