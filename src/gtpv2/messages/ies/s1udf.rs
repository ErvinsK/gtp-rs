// S1-U Data Forwarding IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{IpAddr, Ipv4Addr};

// S1-U Data Forwarding IE Type

pub const S1UDF: u8 = 91;

// S1-U Data Forwarding IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S1udf {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ebi: u8,
    pub sgw_ip: IpAddr,
    pub sgw_s1u_teid: u32,
}

impl Default for S1udf {
    fn default() -> S1udf {
        S1udf {
            t: S1UDF,
            length: 0,
            ins: 0,
            ebi: 0,
            sgw_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            sgw_s1u_teid: 0,
        }
    }
}

impl From<S1udf> for InformationElement {
    fn from(i: S1udf) -> Self {
        InformationElement::S1udf(i)
    }
}

impl IEs for S1udf {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(S1UDF);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.ebi & 0x0f);
        match self.sgw_ip {
            IpAddr::V4(i) => {
                buffer_ie.push(0x04);
                buffer_ie.extend_from_slice(&i.octets());
            }
            IpAddr::V6(i) => {
                buffer_ie.push(0x10);
                buffer_ie.extend_from_slice(&i.octets());
            }
        }
        buffer_ie.extend_from_slice(&self.sgw_s1u_teid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<S1udf, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = S1udf {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..S1udf::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.ebi = buffer[4] & 0x0f;
                match buffer[5] {
                    0x04 => {
                        data.sgw_ip = IpAddr::from([buffer[6], buffer[7], buffer[8], buffer[9]]);
                        data.sgw_s1u_teid =
                            u32::from_be_bytes([buffer[10], buffer[11], buffer[12], buffer[13]]);
                    }
                    0x10 => {
                        if buffer.len() >= 0x16 {
                            let mut dst = [0; 16];
                            dst.copy_from_slice(&buffer[6..22]);
                            data.sgw_ip = IpAddr::from(dst);
                            data.sgw_s1u_teid = u32::from_be_bytes([
                                buffer[22], buffer[23], buffer[24], buffer[25],
                            ]);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(S1UDF));
                        }
                    }
                    _ => return Err(GTPV2Error::IEIncorrect(S1UDF)),
                }

                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(S1UDF))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(S1UDF))
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
fn s1udf_ie_ipv4_unmarshal_test() {
    let encoded_ie: [u8; 14] = [
        0x5b, 0x00, 0x0a, 0x00, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = S1udf {
        t: S1UDF,
        length: 10,
        ins: 0,
        ebi: 1,
        sgw_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        sgw_s1u_teid: 0,
    };
    let i = S1udf::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn s1udf_ie_ipv6_unmarshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 26] = [
        0x5b, 0x00, 0x16, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = S1udf {
        t: S1UDF,
        length: 22,
        ins: 0,
        ebi: 1,
        sgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        sgw_s1u_teid: 0,
    };
    let i = S1udf::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn s1udf_ie_ipv4_marshal_test() {
    let encoded_ie: [u8; 14] = [
        0x5b, 0x00, 0x0a, 0x00, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = S1udf {
        t: S1UDF,
        length: 10,
        ins: 0,
        ebi: 1,
        sgw_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        sgw_s1u_teid: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn s1udf_ie_ipv6_marshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 26] = [
        0x5b, 0x00, 0x16, 0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = S1udf {
        t: S1UDF,
        length: 22,
        ins: 0,
        ebi: 1,
        sgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        sgw_s1u_teid: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn s1udf_ie_wrong_ip_address_type() {
    let encoded_ie: [u8; 14] = [
        0x5b, 0x00, 0x0a, 0x00, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let i = S1udf::unmarshal(&encoded_ie);
    assert_eq!(i, Err(GTPV2Error::IEIncorrect(S1UDF)));
}
