// End User Address IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};
use std::net::{Ipv4Addr, Ipv6Addr};

// End User Address Type

pub const END_USER_ADDRESS: u8 = 128;

// PDP Type Organization

pub const ETSI: u8 = 0;
pub const IETF: u8 = 1;

// ETSI PDP Type Number

pub const PPP: u8 = 1;
pub const NONIP: u8 = 2;

// IETF PDP Type Number

pub const IPV4: u8 = 0x21;
pub const IPV6: u8 = 0x57;
pub const IPV46: u8 = 0x8D;

// End User Address IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndUserAddress {
    pub t: u8,
    pub length: u16,
    pub pdp_type_org: u8,
    pub pdp_type_nbr: u8,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl Default for EndUserAddress {
    fn default() -> Self {
        EndUserAddress {
            t: END_USER_ADDRESS,
            length: 2,
            pdp_type_org: IETF,
            pdp_type_nbr: IPV4,
            ipv4: None,
            ipv6: None,
        }
    }
}

impl IEs for EndUserAddress {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(0b11110000 | self.pdp_type_org);
        match (self.pdp_type_org, self.ipv4, self.ipv6) {
            (ETSI, _, _) => buffer_ie.push(self.pdp_type_nbr),
            (IETF, Some(i), None) => {
                buffer_ie.push(self.pdp_type_nbr);
                buffer_ie.extend_from_slice(&i.octets());
            }
            (IETF, None, Some(i)) => {
                buffer_ie.push(self.pdp_type_nbr);
                buffer_ie.extend_from_slice(&i.octets());
            }
            (IETF, Some(i), Some(j)) => {
                buffer_ie.push(self.pdp_type_nbr);
                buffer_ie.extend_from_slice(&i.octets());
                buffer_ie.extend_from_slice(&j.octets());
            }
            (IETF, _, _) => buffer_ie.push(self.pdp_type_nbr),
            (_, _, _) => (),
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 3 {
            let mut data = EndUserAddress {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            if check_tlv_ie_buffer(data.length, buffer) {
                data.pdp_type_org = buffer[3] & 0b00001111;
                data.pdp_type_nbr = buffer[4];
                match (data.length, data.pdp_type_org, data.pdp_type_nbr) {
                    (2, ETSI, PPP) => (),
                    (2, ETSI, NONIP) => (),
                    (2, IETF, IPV4) => (),
                    (2, IETF, IPV6) => (),
                    (2, IETF, IPV46) => (),
                    (6, IETF, IPV4) => {
                        let i: Result<[u8; 4], _> = buffer[5..=8].try_into();
                        match i {
                            Ok(j) => data.ipv4 = Some(Ipv4Addr::from(j)),
                            Err(_) => return Err(GTPV1Error::IEIncorrect),
                        }
                    }
                    (18, IETF, IPV6) => {
                        let i: Result<[u8; 16], _> = buffer[5..=20].try_into();
                        match i {
                            Ok(j) => data.ipv6 = Some(Ipv6Addr::from(j)),
                            Err(_) => return Err(GTPV1Error::IEIncorrect),
                        }
                    }
                    (22, IETF, IPV46) => {
                        let ip4: Result<[u8; 4], _> = buffer[5..=8].try_into();
                        match ip4 {
                            Ok(i) => data.ipv4 = Some(Ipv4Addr::from(i)),
                            Err(_) => return Err(GTPV1Error::IEIncorrect),
                        }
                        let ip6: Result<[u8; 16], _> = buffer[9..=24].try_into();
                        match ip6 {
                            Ok(i) => data.ipv6 = Some(Ipv6Addr::from(i)),
                            Err(_) => return Err(GTPV1Error::IEIncorrect),
                        }
                    }
                    (_, _, _) => return Err(GTPV1Error::IEIncorrect),
                }
                Ok(data)
            } else {
                Err(GTPV1Error::IEInvalidLength)
            }
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn end_user_address_ipv4_with_ip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 6,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV4,
        ipv4: Some(Ipv4Addr::new(100, 117, 130, 53)),
        ipv6: None,
    };
    let ie_marshalled: [u8; 9] = [0x80, 0x00, 0x06, 0xf1, 0x21, 0x64, 0x75, 0x82, 0x35];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_ipv4_with_ip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 6,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV4,
        ipv4: Some(Ipv4Addr::new(100, 117, 130, 53)),
        ipv6: None,
    };
    let ie_to_unmarshal: [u8; 9] = [0x80, 0x00, 0x06, 0xf1, 0x21, 0x64, 0x75, 0x82, 0x35];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_ipv4_without_ip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV4,
        ipv4: None,
        ipv6: None,
    };
    let ie_marshalled: [u8; 5] = [0x80, 0x00, 0x02, 0xf1, 0x21];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_ipv4_without_ip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV4,
        ipv4: None,
        ipv6: None,
    };
    let ie_to_unmarshal: [u8; 5] = [0x80, 0x00, 0x02, 0xf1, 0x21];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_ipv6_with_ip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 18,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV6,
        ipv4: None,
        ipv6: Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xffff, 0xffff)),
    };
    let ie_marshalled: [u8; 21] = [
        0x80, 0x00, 0x12, 0xf1, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_ipv6_with_ip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 18,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV6,
        ipv4: None,
        ipv6: Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xffff, 0xffff)),
    };
    let ie_to_unmarshal: [u8; 21] = [
        0x80, 0x00, 0x12, 0xf1, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_ipv6_without_ip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV6,
        ipv4: None,
        ipv6: None,
    };
    let ie_marshalled: [u8; 5] = [0x80, 0x00, 0x02, 0xf1, 0x57];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_ipv6_without_ip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV6,
        ipv4: None,
        ipv6: None,
    };
    let ie_to_unmarshal: [u8; 5] = [0x80, 0x00, 0x02, 0xf1, 0x57];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_ipv46_with_ip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 22,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV46,
        ipv4: Some(Ipv4Addr::new(100, 117, 130, 53)),
        ipv6: Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xffff, 0xffff)),
    };
    let ie_marshalled: [u8; 25] = [
        0x80, 0x00, 0x16, 0xf1, 0x8D, 0x64, 0x75, 0x82, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_ipv46_with_ip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 22,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV46,
        ipv4: Some(Ipv4Addr::new(100, 117, 130, 53)),
        ipv6: Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xffff, 0xffff)),
    };
    let ie_to_unmarshal: [u8; 25] = [
        0x80, 0x00, 0x16, 0xf1, 0x8D, 0x64, 0x75, 0x82, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_ipv46_without_ip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV46,
        ipv4: None,
        ipv6: None,
    };
    let ie_marshalled: [u8; 5] = [0x80, 0x00, 0x02, 0xf1, 0x8D];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_ipv46_without_ip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: IETF,
        pdp_type_nbr: IPV46,
        ipv4: None,
        ipv6: None,
    };
    let ie_to_unmarshal: [u8; 5] = [0x80, 0x00, 0x02, 0xf1, 0x8D];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_etsi_ppp_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: ETSI,
        pdp_type_nbr: PPP,
        ipv4: None,
        ipv6: None,
    };
    let ie_marshalled: [u8; 5] = [0x80, 0x00, 0x02, 0xf0, 0x01];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_etsi_ppp_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: ETSI,
        pdp_type_nbr: PPP,
        ipv4: None,
        ipv6: None,
    };
    let ie_to_unmarshal: [u8; 5] = [0x80, 0x00, 0x02, 0xf0, 0x01];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}

#[test]
fn end_user_address_etsi_nonip_ie_marshal_test() {
    let ie_to_marshal = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: ETSI,
        pdp_type_nbr: NONIP,
        ipv4: None,
        ipv6: None,
    };
    let ie_marshalled: [u8; 5] = [0x80, 0x00, 0x02, 0xf0, 0x02];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn end_user_address_etsi_nonip_ie_unmarshal_test() {
    let ie_unmarshalled = EndUserAddress {
        t: END_USER_ADDRESS,
        length: 2,
        pdp_type_org: ETSI,
        pdp_type_nbr: NONIP,
        ipv4: None,
        ipv6: None,
    };
    let ie_to_unmarshal: [u8; 5] = [0x80, 0x00, 0x02, 0xf0, 0x02];
    assert_eq!(
        EndUserAddress::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
