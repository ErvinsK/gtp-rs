// MBMS IP Multicast Distribution IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// MBMS IP Mutlicast Distribution IE Type

pub const MBMSIPMULTICASTDISTR: u8 = 142;

// MBMS HC Indicator Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MbmsHc {
    UncompressedHeader,
    CompressedHeader,
}

// MBMS IP Multicast Distribution IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsIpMulticastDistribution {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub common_teid: u32,
    pub ip_multicast_address_1: IpAddr,
    pub ip_multicast_address_2: IpAddr,
    pub mbms_hc: MbmsHc,
}

impl Default for MbmsIpMulticastDistribution {
    fn default() -> MbmsIpMulticastDistribution {
        MbmsIpMulticastDistribution {
            t: MBMSIPMULTICASTDISTR,
            length: 0,
            ins: 0,
            common_teid: 0,
            ip_multicast_address_1: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ip_multicast_address_2: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            mbms_hc: MbmsHc::UncompressedHeader,
        }
    }
}

impl From<MbmsIpMulticastDistribution> for InformationElement {
    fn from(i: MbmsIpMulticastDistribution) -> Self {
        InformationElement::MbmsIpMulticastDistribution(i)
    }
}

impl IEs for MbmsIpMulticastDistribution {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MBMSIPMULTICASTDISTR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.common_teid.to_be_bytes());
        match self.ip_multicast_address_1 {
            IpAddr::V4(i) => {
                buffer_ie.push(0x04);
                buffer_ie.extend_from_slice(&i.octets());
            }
            IpAddr::V6(i) => {
                buffer_ie.push(0x50);
                buffer_ie.extend_from_slice(&i.octets());
            }
        }
        match self.ip_multicast_address_2 {
            IpAddr::V4(i) => {
                buffer_ie.push(0x04);
                buffer_ie.extend_from_slice(&i.octets());
            }
            IpAddr::V6(i) => {
                buffer_ie.push(0x50);
                buffer_ie.extend_from_slice(&i.octets());
            }
        }
        match self.mbms_hc {
            MbmsHc::UncompressedHeader => buffer_ie.push(0x00),
            MbmsHc::CompressedHeader => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 4 {
            let mut data = MbmsIpMulticastDistribution {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                common_teid: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..MbmsIpMulticastDistribution::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                let mut cursor: usize = 8;
                match buffer[cursor] {
                    0x04 => {
                        cursor += 1;
                        if buffer.len() >= cursor + 4 {
                            data.ip_multicast_address_1 = IpAddr::V4(Ipv4Addr::from([
                                buffer[cursor],
                                buffer[cursor + 1],
                                buffer[cursor + 2],
                                buffer[cursor + 3],
                            ]));
                            cursor += 4;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR));
                        }
                    }
                    0x50 => {
                        cursor += 1;
                        if buffer.len() >= cursor + 16 {
                            let mut dst = [0; 16];
                            dst.copy_from_slice(&buffer[cursor..cursor + 16]);
                            data.ip_multicast_address_1 = IpAddr::V6(Ipv6Addr::from(dst));
                            cursor += 16;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR));
                        }
                    }
                    _ => {
                        return Err(GTPV2Error::IEIncorrect(MBMSIPMULTICASTDISTR));
                    }
                }
                match buffer[cursor] {
                    0x04 => {
                        cursor += 1;
                        if buffer.len() >= cursor + 4 {
                            data.ip_multicast_address_2 = IpAddr::V4(Ipv4Addr::from([
                                buffer[cursor],
                                buffer[cursor + 1],
                                buffer[cursor + 2],
                                buffer[cursor + 3],
                            ]));
                            cursor += 4;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR));
                        }
                    }
                    0x50 => {
                        cursor += 1;
                        if buffer.len() >= cursor + 16 {
                            let mut dst = [0; 16];
                            dst.copy_from_slice(&buffer[cursor..cursor + 16]);
                            data.ip_multicast_address_2 = IpAddr::V6(Ipv6Addr::from(dst));
                            cursor += 16;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR));
                        }
                    }
                    _ => {
                        return Err(GTPV2Error::IEIncorrect(MBMSIPMULTICASTDISTR));
                    }
                }
                if buffer.len() >= cursor {
                    match buffer[cursor] {
                        0x00 => data.mbms_hc = MbmsHc::UncompressedHeader,
                        0x01 => data.mbms_hc = MbmsHc::CompressedHeader,
                        _ => {
                            return Err(GTPV2Error::IEIncorrect(MBMSIPMULTICASTDISTR));
                        }
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR));
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(MBMSIPMULTICASTDISTR))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn mbmsipmulticastdistr_ie_marshal_test() {
    let encoded: [u8; 31] = [
        0x8E, 0x00, 0x1B, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x04, 0x0A, 0x0A, 0x0A, 0x01, 0x50, 0x00,
        0xFD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0x00,
    ];
    let decoded = MbmsIpMulticastDistribution {
        t: MBMSIPMULTICASTDISTR,
        length: 27,
        ins: 0,
        common_teid: 0xffff,
        ip_multicast_address_1: IpAddr::V4(Ipv4Addr::new(0x0a, 0x0a, 0x0a, 0x01)),
        ip_multicast_address_2: IpAddr::V6(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0xff)),
        mbms_hc: MbmsHc::UncompressedHeader,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mbmsipmulticastdistr_ie_unmarshal_test() {
    let encoded: [u8; 31] = [
        0x8E, 0x00, 0x1B, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x04, 0x0A, 0x0A, 0x0A, 0x01, 0x50, 0x00,
        0xFD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0x00,
    ];
    let decoded = MbmsIpMulticastDistribution {
        t: MBMSIPMULTICASTDISTR,
        length: 27,
        ins: 0,
        common_teid: 0xffff,
        ip_multicast_address_1: IpAddr::V4(Ipv4Addr::new(0x0a, 0x0a, 0x0a, 0x01)),
        ip_multicast_address_2: IpAddr::V6(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0xff)),
        mbms_hc: MbmsHc::UncompressedHeader,
    };
    let i = MbmsIpMulticastDistribution::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}
