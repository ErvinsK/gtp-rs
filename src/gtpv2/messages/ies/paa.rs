// PDN Address Allocation IE - according to 3GPP TS 29.247 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{Ipv4Addr, Ipv6Addr};

// PAA IE Type

pub const PAA: u8 = 79;

// PDN Type Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdnAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr, u8),
    DualStack(Ipv4Addr, Ipv6Addr, u8),
    NonIp,
    Ethernet,
}

// PAA IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdnAddressAllocation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ip: PdnAddress,
}

impl Default for PdnAddressAllocation {
    fn default() -> PdnAddressAllocation {
        PdnAddressAllocation {
            t: PAA,
            length: 0,
            ins: 0,
            ip: PdnAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

impl From<PdnAddressAllocation> for InformationElement {
    fn from(i: PdnAddressAllocation) -> Self {
        InformationElement::PdnAddressAllocation(i)
    }
}

impl IEs for PdnAddressAllocation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PAA);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.ip {
            PdnAddress::V4(i) => {
                buffer_ie.push(0x01);
                buffer_ie.extend_from_slice(&i.octets());
            }
            PdnAddress::V6(i, j) => {
                buffer_ie.push(0x02);
                buffer_ie.push(j);
                buffer_ie.extend_from_slice(&i.octets());
            }
            PdnAddress::DualStack(i, j, n) => {
                buffer_ie.push(0x03);
                buffer_ie.push(n);
                buffer_ie.extend_from_slice(&j.octets());
                buffer_ie.extend_from_slice(&i.octets());
            }
            PdnAddress::NonIp => buffer_ie.push(0x04),
            PdnAddress::Ethernet => buffer_ie.push(0x05),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<PdnAddressAllocation, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = PdnAddressAllocation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..PdnAddressAllocation::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    0x01 => {
                        if data.length >= 5 {
                            data.ip = PdnAddress::V4(Ipv4Addr::from([
                                buffer[5], buffer[6], buffer[7], buffer[8],
                            ]));
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(PAA));
                        }
                    }
                    0x02 => {
                        if data.length >= 18 {
                            let mut dst = [0; 16];
                            let prefix_length = buffer[5];
                            dst.copy_from_slice(&buffer[6..22]);
                            data.ip = PdnAddress::V6(Ipv6Addr::from(dst), prefix_length);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(PAA));
                        }
                    }
                    0x03 => {
                        if data.length >= 22 {
                            let mut dst = [0; 16];
                            let prefix_length = buffer[5];
                            dst.copy_from_slice(&buffer[6..22]);
                            let v4addr =
                                Ipv4Addr::from([buffer[22], buffer[23], buffer[24], buffer[25]]);
                            data.ip =
                                PdnAddress::DualStack(v4addr, Ipv6Addr::from(dst), prefix_length);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(PAA));
                        }
                    }
                    0x04 => data.ip = PdnAddress::NonIp,
                    0x05 => data.ip = PdnAddress::Ethernet,
                    _ => return Err(GTPV2Error::IEIncorrect(PAA)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(PAA))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(PAA))
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
fn paa_ie_ipv4_unmarshal_test() {
    let encoded: [u8; 9] = [0x4f, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 5,
        ins: 0,
        ip: PdnAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
    };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_ipv6_unmarshal_test() {
    let encoded: [u8; 22] = [
        0x4f, 0x00, 0x12, 0x00, 0x02, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 18,
        ins: 0,
        ip: PdnAddress::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 128),
    };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_ipv46_unmarshal_test() {
    let encoded: [u8; 26] = [
        0x4f, 0x00, 0x16, 0x00, 0x03, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 22,
        ins: 0,
        ip: PdnAddress::DualStack(
            Ipv4Addr::new(1, 0, 0, 0),
            Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0),
            128,
        ),
    };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_non_ip_unmarshal_test() {
    let encoded: [u8; 5] = [0x4f, 0x00, 0x01, 0x00, 0x04];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 1,
        ins: 0,
        ip: PdnAddress::NonIp,
    };
    let i = PdnAddressAllocation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paa_ie_ipv4_marshal_test() {
    let encoded: [u8; 9] = [0x4f, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 5,
        ins: 0,
        ip: PdnAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn paa_ie_ipv6_marshal_test() {
    let encoded: [u8; 22] = [
        0x4f, 0x00, 0x12, 0x00, 0x02, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 18,
        ins: 0,
        ip: PdnAddress::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 128),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn paa_ie_ipv46_marshal_test() {
    let encoded: [u8; 26] = [
        0x4f, 0x00, 0x16, 0x00, 0x03, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 22,
        ins: 0,
        ip: PdnAddress::DualStack(
            Ipv4Addr::new(1, 0, 0, 0),
            Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0),
            128,
        ),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn paa_ie_nonip_marshal_test() {
    let encoded: [u8; 5] = [0x4f, 0x00, 0x01, 0x00, 0x04];
    let decoded = PdnAddressAllocation {
        t: PAA,
        length: 1,
        ins: 0,
        ip: PdnAddress::NonIp,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
