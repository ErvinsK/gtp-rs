// S103 PDN Data Forwarding IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{IpAddr, Ipv4Addr};

// S103 PDN Data Forwarding IE Type

pub const S103_PDF: u8 = 90;

// S103 PDN Data Forwarding IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S103pdf {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub hsgw_ip: IpAddr,
    pub gre_key: u32,
    pub eps_bearer_ids: Vec<u8>,
}

impl Default for S103pdf {
    fn default() -> S103pdf {
        S103pdf {
            t: S103_PDF,
            length: 0,
            ins: 0,
            hsgw_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            gre_key: 0,
            eps_bearer_ids: Vec::new(),
        }
    }
}

impl From<S103pdf> for InformationElement {
    fn from(i: S103pdf) -> Self {
        InformationElement::S103pdf(i)
    }
}

impl IEs for S103pdf {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(S103_PDF);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.hsgw_ip {
            IpAddr::V4(i) => {
                buffer_ie.push(0x04);
                buffer_ie.extend_from_slice(&i.octets());
            }
            IpAddr::V6(i) => {
                buffer_ie.push(0x10);
                buffer_ie.extend_from_slice(&i.octets());
            }
        }
        buffer_ie.extend_from_slice(&self.gre_key.to_be_bytes());
        buffer_ie.push(self.eps_bearer_ids.len() as u8);
        buffer_ie.extend_from_slice(&self.eps_bearer_ids);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<S103pdf, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = S103pdf {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..S103pdf::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    0x04 => {
                        data.hsgw_ip = IpAddr::from([buffer[5], buffer[6], buffer[7], buffer[8]]);
                        data.gre_key =
                            u32::from_be_bytes([buffer[9], buffer[10], buffer[11], buffer[12]]);
                        if buffer.len() >= (0x0e + buffer[13] as usize) {
                            data.eps_bearer_ids
                                .extend_from_slice(&buffer[14..(14 + buffer[13] as usize)]);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(S103_PDF));
                        }
                    }
                    0x10 => {
                        if buffer.len() >= 25 {
                            let mut dst = [0; 16];
                            dst.copy_from_slice(&buffer[5..21]);
                            data.hsgw_ip = IpAddr::from(dst);
                            data.gre_key = u32::from_be_bytes([
                                buffer[21], buffer[22], buffer[23], buffer[24],
                            ]);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(S103_PDF));
                        }
                        if buffer.len() >= (25 + buffer[25] as usize) {
                            data.eps_bearer_ids
                                .extend_from_slice(&buffer[26..(26 + buffer[25] as usize)]);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(S103_PDF));
                        }
                    }
                    _ => return Err(GTPV2Error::IEIncorrect(S103_PDF)),
                }

                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(S103_PDF))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(S103_PDF))
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
fn s103pdf_ie_ipv4_unmarshal_test() {
    let encoded_ie: [u8; 16] = [
        0x5a, 0x00, 0x0c, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01,
        0x02,
    ];
    let test_struct = S103pdf {
        t: S103_PDF,
        length: 12,
        ins: 0,
        hsgw_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        gre_key: 0,
        eps_bearer_ids: vec![1, 2],
    };
    let i = S103pdf::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn s103pdf_ie_ipv6_unmarshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 28] = [
        0x5a, 0x00, 0x18, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x02,
    ];
    let test_struct = S103pdf {
        t: S103_PDF,
        length: 24,
        ins: 0,
        hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        gre_key: 0,
        eps_bearer_ids: vec![1, 2],
    };
    let i = S103pdf::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn s103pdf_ie_ipv6_sic_unmarshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 27] = [
        0x5a, 0x00, 0x17, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06,
    ];
    let test_struct = S103pdf {
        t: S103_PDF,
        length: 23,
        ins: 0,
        hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        gre_key: 0,
        eps_bearer_ids: vec![6],
    };
    let i = S103pdf::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn s103pdf_ie_ipv4_marshal_test() {
    let encoded_ie: [u8; 16] = [
        0x5a, 0x00, 0x0c, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01,
        0x02,
    ];
    let test_struct = S103pdf {
        t: S103_PDF,
        length: 12,
        ins: 0,
        hsgw_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        gre_key: 0,
        eps_bearer_ids: vec![1, 2],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn s103pdf_ie_ipv6_marshal_test() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded_ie: [u8; 28] = [
        0x5a, 0x00, 0x18, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x02,
    ];
    let test_struct = S103pdf {
        t: S103_PDF,
        length: 24,
        ins: 0,
        hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        gre_key: 0,
        eps_bearer_ids: vec![1, 2],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn s103pdf_ie_wrong_ip_address_type() {
    let encoded_ie: [u8; 15] = [
        0x5a, 0x00, 0x0b, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let i = S103pdf::unmarshal(&encoded_ie);
    assert_eq!(i, Err(GTPV2Error::IEIncorrect(S103_PDF)));
}
