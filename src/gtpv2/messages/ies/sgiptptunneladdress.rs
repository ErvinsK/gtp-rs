// SGi PtP Tunnel Address IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{IpAddr, Ipv4Addr};

// SGi PtP Tunnel Address IE Type

pub const SGI_PTP_TUN_ADDRESS: u8 = 213;

// SGi PtP Address IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SgiPtpTunnelAddress {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ip: IpAddr,
    pub port: Option<u16>,
}

impl Default for SgiPtpTunnelAddress {
    fn default() -> Self {
        SgiPtpTunnelAddress {
            t: SGI_PTP_TUN_ADDRESS,
            length: 0,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: None,
        }
    }
}

impl From<SgiPtpTunnelAddress> for InformationElement {
    fn from(i: SgiPtpTunnelAddress) -> Self {
        InformationElement::SgiPtpTunnelAddress(i)
    }
}

impl IEs for SgiPtpTunnelAddress {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SGI_PTP_TUN_ADDRESS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flag = match (self.port.is_some(), self.ip) {
            (true, IpAddr::V4(_)) => 0x05,
            (true, IpAddr::V6(_)) => 0x06,
            (false, IpAddr::V4(_)) => 0x01,
            (false, IpAddr::V6(_)) => 0x02,
        };
        buffer_ie.push(flag);
        match self.ip {
            IpAddr::V4(i) => buffer_ie.extend_from_slice(&i.octets()),
            IpAddr::V6(i) => buffer_ie.extend_from_slice(&i.octets()),
        }
        if let Some(p) = self.port {
            buffer_ie.extend_from_slice(&p.to_be_bytes());
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = SgiPtpTunnelAddress {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..SgiPtpTunnelAddress::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                let mut cursor: usize = 5;
                match buffer[4] & 0x03 {
                    0x01 => {
                        if buffer.len() >= cursor + 4 {
                            data.ip = IpAddr::from([
                                buffer[cursor],
                                buffer[cursor + 1],
                                buffer[cursor + 2],
                                buffer[cursor + 3],
                            ]);
                            cursor += 4;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(SGI_PTP_TUN_ADDRESS));
                        }
                    }
                    0x02 => {
                        if buffer.len() >= cursor + 16 {
                            let mut dst = [0; 16];
                            dst.copy_from_slice(&buffer[cursor..cursor + 16]);
                            data.ip = IpAddr::from(dst);
                            cursor += 16;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(SGI_PTP_TUN_ADDRESS));
                        }
                    }
                    _ => return Err(GTPV2Error::IEIncorrect(SGI_PTP_TUN_ADDRESS)),
                }
                if buffer[4] & 0x04 == 0x04 {
                    if buffer.len() >= cursor + 2 {
                        data.port = Some(u16::from_be_bytes([buffer[cursor], buffer[cursor + 1]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(SGI_PTP_TUN_ADDRESS));
                    }
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(SGI_PTP_TUN_ADDRESS))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(SGI_PTP_TUN_ADDRESS))
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
fn sgi_ptp_tun_address_ie_ipv4_unmarshal_test() {
    let encoded_ie: [u8; 11] = [
        0xd5, 0x00, 0x07, 0x00, 0x05, 0x0a, 0x64, 0x32, 0x0a, 0x13, 0x88,
    ];
    let test_struct = SgiPtpTunnelAddress {
        length: 7,
        ip: IpAddr::V4(Ipv4Addr::new(10, 100, 50, 10)),
        port: Some(5000),
        ..SgiPtpTunnelAddress::default()
    };
    let i = SgiPtpTunnelAddress::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn sgi_ptp_tun_address_ie_ipv6_unmarshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 21] = [
        0xd5, 0x00, 0x11, 0x00, 0x02, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
        0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
    ];
    let test_struct = SgiPtpTunnelAddress {
        length: 17,
        ip: IpAddr::V6(Ipv6Addr::new(
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        )),
        port: None,
        ..SgiPtpTunnelAddress::default()
    };
    let i = SgiPtpTunnelAddress::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn sgi_ptp_tun_address_ie_ipv4_marshal_test() {
    let encoded_ie: [u8; 11] = [
        0xd5, 0x00, 0x07, 0x00, 0x05, 0x0a, 0x64, 0x32, 0x0a, 0x13, 0x88,
    ];
    let test_struct = SgiPtpTunnelAddress {
        length: 7,
        ip: IpAddr::V4(Ipv4Addr::new(10, 100, 50, 10)),
        port: Some(5000),
        ..SgiPtpTunnelAddress::default()
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn sgi_ptp_tun_address_ie_ipv6_marshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 21] = [
        0xd5, 0x00, 0x11, 0x00, 0x02, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
        0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
    ];
    let test_struct = SgiPtpTunnelAddress {
        length: 17,
        ip: IpAddr::V6(Ipv6Addr::new(
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        )),
        port: None,
        ..SgiPtpTunnelAddress::default()
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded_ie);
}
