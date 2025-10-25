// FQ-CSID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{Ipv4Addr, Ipv6Addr};

// FQ-CSID IE Type

pub const FQCSID: u8 = 132;

// Node-ID Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeId {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    NwAddr(u32), // Node-ID as 32 bit value, where most significant 20 bits are the binary encoded value of (MCC * 1000 + MNC). Least significant 12 bits is a 12 bit integer assigned by an operator to an MME, SGW, TWAN, ePDG or PGW. Other values of Node-ID Type are reserved.
}

impl From<Ipv4Addr> for NodeId {
    fn from(i: Ipv4Addr) -> Self {
        NodeId::V4(i)
    }
}

impl From<Ipv6Addr> for NodeId {
    fn from(i: Ipv6Addr) -> Self {
        NodeId::V6(i)
    }
}

impl From<u32> for NodeId {
    fn from(i: u32) -> Self {
        NodeId::NwAddr(i)
    }
}

// FQ-CSID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fqcsid {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub nodeid: NodeId,
    pub csid: Vec<u16>,
}

impl Default for Fqcsid {
    fn default() -> Self {
        Fqcsid {
            t: FQCSID,
            length: 6,
            ins: 0,
            nodeid: NodeId::V4(Ipv4Addr::new(0, 0, 0, 0)),
            csid: vec![0],
        }
    }
}

impl From<Fqcsid> for InformationElement {
    fn from(i: Fqcsid) -> Self {
        InformationElement::Fqcsid(i)
    }
}

impl IEs for Fqcsid {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(FQCSID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.nodeid {
            NodeId::V4(i) => {
                buffer_ie.push((self.csid.len() as u8) & 0x0f);
                buffer_ie.extend_from_slice(&i.octets());
            }
            NodeId::V6(i) => {
                buffer_ie.push(0x10 | ((self.csid.len() as u8) & 0x0f));
                buffer_ie.extend_from_slice(&i.octets());
            }
            NodeId::NwAddr(i) => {
                buffer_ie.push(0x20 | ((self.csid.len() as u8) & 0x0f));
                buffer_ie.extend_from_slice(&i.to_be_bytes());
            }
        }
        buffer_ie.append(
            &mut self
                .csid
                .clone()
                .iter()
                .flat_map(|x| x.to_be_bytes())
                .collect(),
        );
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = Fqcsid {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Default::default()
            };
            if !check_tliv_ie_buffer(data.length, buffer) {
                return Err(GTPV2Error::IEInvalidLength(FQCSID));
            }
            match buffer[4] >> 4 {
                0 => {
                    let cursor = (9 + 2 * (buffer[4] & 0x0f)) as usize;
                    if check_tliv_ie_buffer((cursor - 4) as u16, buffer) {
                        data.nodeid = NodeId::V4(Ipv4Addr::from([
                            buffer[5], buffer[6], buffer[7], buffer[8],
                        ]));
                        data.csid = buffer[9..cursor]
                            .to_vec()
                            .chunks(2)
                            .map(|x| u16::from_be_bytes([x[0], x[1]]))
                            .collect();
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(FQCSID));
                    }
                }
                1 => {
                    let cursor = (21 + 2 * (buffer[4] & 0x0f)) as usize;
                    if check_tliv_ie_buffer((cursor - 4) as u16, buffer) {
                        let mut dst = [0; 16];
                        dst.copy_from_slice(&buffer[5..21]);
                        data.nodeid = NodeId::V6(Ipv6Addr::from(dst));
                        data.csid = buffer[21..cursor]
                            .to_vec()
                            .chunks(2)
                            .map(|x| u16::from_be_bytes([x[0], x[1]]))
                            .collect();
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(FQCSID));
                    }
                }
                2 => {
                    let cursor = (9 + 2 * (buffer[4] & 0x0f)) as usize;
                    if check_tliv_ie_buffer((cursor - 4) as u16, buffer) {
                        data.nodeid = NodeId::NwAddr(u32::from_be_bytes([
                            buffer[5], buffer[6], buffer[7], buffer[8],
                        ]));
                        data.csid = buffer[9..cursor]
                            .to_vec()
                            .chunks(2)
                            .map(|x| u16::from_be_bytes([x[0], x[1]]))
                            .collect();
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(FQCSID));
                    }
                }
                _ => return Err(GTPV2Error::IEIncorrect(FQCSID)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(FQCSID))
        }
    }

    fn len(&self) -> usize {
        (self.length + 4) as usize
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
fn fqcsid_ie_ipv4_unmarshal_test() {
    let encoded: [u8; 11] = [
        0x84, 0x00, 0x07, 0x00, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff,
    ];
    let decoded = Fqcsid {
        t: FQCSID,
        length: 7,
        ins: 0,
        nodeid: NodeId::V4(Ipv4Addr::new(139, 7, 133, 184)),
        csid: vec![0xffff],
    };
    let i = Fqcsid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fqcsid_ie_ipv6_unmarshal_test() {
    let encoded: [u8; 25] = [
        0x84, 0x00, 0x15, 0x00, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xaa,
    ];
    let decoded = Fqcsid {
        t: FQCSID,
        length: 21,
        ins: 0,
        nodeid: Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0).into(),
        csid: vec![0xffff, 0xffaa],
    };
    let i = Fqcsid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fqcsid_ie_nwaddr_unmarshal_test() {
    let encoded: [u8; 11] = [
        0x84, 0x00, 0x07, 0x00, 0x21, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff,
    ];
    let decoded = Fqcsid {
        t: FQCSID,
        length: 7,
        ins: 0,
        nodeid: 0x8b0785b8.into(),
        csid: vec![0xffff],
    };
    let i = Fqcsid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fqcsid_ie_ipv4_marshal_test() {
    let encoded: [u8; 11] = [
        0x84, 0x00, 0x07, 0x00, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff,
    ];
    let decoded = Fqcsid {
        t: FQCSID,
        length: 7,
        ins: 0,
        nodeid: NodeId::V4(Ipv4Addr::new(139, 7, 133, 184)),
        csid: vec![0xffff],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn fqcsid_ie_ipv6_marshal_test() {
    let encoded: [u8; 25] = [
        0x84, 0x00, 0x15, 0x00, 0x12, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xaa,
    ];
    let decoded = Fqcsid {
        t: FQCSID,
        length: 21,
        ins: 0,
        nodeid: Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0).into(),
        csid: vec![0xffff, 0xffaa],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn fqcsid_ie_nwaddr_marshal_test() {
    let encoded: [u8; 11] = [
        0x84, 0x00, 0x07, 0x00, 0x21, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff,
    ];
    let decoded = Fqcsid {
        t: FQCSID,
        length: 7,
        ins: 0,
        nodeid: 0x8b0785b8.into(),
        csid: vec![0xffff],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
