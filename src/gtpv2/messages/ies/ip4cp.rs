// IPv4 Configuation Parameters (IP4CP) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::Ipv4Addr;

// IP4CP Address IE Type

pub const IP4CP: u8 = 166;
pub const IP4CP_LENGTH: usize = 5;

// IP4CP IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ip4Cp {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub subnet_prefix: u8,
    pub ip: Ipv4Addr,
}

impl Default for Ip4Cp {
    fn default() -> Self {
        Ip4Cp {
            t: IP4CP,
            length: IP4CP_LENGTH as u16,
            ins: 0,
            subnet_prefix: 0,
            ip: Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}

impl From<Ip4Cp> for InformationElement {
    fn from(i: Ip4Cp) -> Self {
        InformationElement::Ip4Cp(i)
    }
}

impl IEs for Ip4Cp {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(IP4CP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.subnet_prefix);
        buffer_ie.extend_from_slice(&self.ip.octets());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + IP4CP_LENGTH {
            let data = Ip4Cp {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                subnet_prefix: buffer[4],
                ip: Ipv4Addr::from([buffer[5], buffer[6], buffer[7], buffer[8]]),
                ..Ip4Cp::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(IP4CP))
        }
    }

    fn len(&self) -> usize {
        IP4CP_LENGTH + MIN_IE_SIZE
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
fn ip4cp_address_ie_ipv4_unmarshal_test() {
    let encoded: [u8; 9] = [0xa6, 0x00, 0x05, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00];
    let decoded = Ip4Cp {
        t: IP4CP,
        length: 5,
        ins: 0,
        subnet_prefix: 32,
        ip: Ipv4Addr::new(0, 0, 0, 0),
    };
    let i = Ip4Cp::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn ip4cp_address_ie_ipv4_marshal_test() {
    let encoded: [u8; 9] = [0xa6, 0x00, 0x05, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00];
    let decoded = Ip4Cp {
        t: IP4CP,
        length: 5,
        ins: 0,
        subnet_prefix: 32,
        ip: Ipv4Addr::new(0, 0, 0, 0),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
