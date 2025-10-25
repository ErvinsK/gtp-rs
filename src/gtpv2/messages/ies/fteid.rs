// F-TEID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{Ipv4Addr, Ipv6Addr};

// F-TEID IE Type

pub const FTEID: u8 = 87;

// F-TEID IE implementation

// F-TEID Interfaces:
// 0:	S1-U eNodeB GTP-U interface
// 1:	S1-U SGW GTP-U interface
// 2:	S12 RNC GTP-U interface
// 3:	S12 SGW GTP-U interface
// 4:	S5/S8 SGW GTP-U interface
// 5:	S5/S8 PGW GTP-U interface
// 6:	S5/S8 SGW GTP-C interface
// 7:	S5/S8 PGW GTP-C interface
// 8:	S5/S8 SGW PMIPv6 interface (the 32 bit GRE key is encoded in 32 bit TEID field)
// 9:	S5/S8 PGW PMIPv6 interface (the 32 bit GRE key is encoded in the 32 bit TEID field, see clause 6.3 in 3GPP TS 29.275 [26])
// 10:	S11 MME GTP-C interface
// 11:	S11/S4 SGW GTP-C interface
// 12:	S10/N26 MME GTP-C interface
// 13:	S3 MME GTP-C interface
// 14:	S3 SGSN GTP-C interface
// 15:	S4 SGSN GTP-U interface
// 16:	S4 SGW GTP-U interface
// 17:	S4 SGSN GTP-C interface
// 18:	S16 SGSN GTP-C interface
// 19:	eNodeB GTP-U interface for DL data forwarding
// 20:	eNodeB GTP-U interface for UL data forwarding
// 21:	RNC GTP-U interface for data forwarding
// 22:	SGSN GTP-U interface for data forwarding
// 23:	SGW/UPF GTP-U interface for DL data forwarding
// 24:	Sm MBMS GW GTP-C interface
// 25:	Sn MBMS GW GTP-C interface
// 26:	Sm MME GTP-C interface
// 27:	Sn SGSN GTP-C interface
// 28: SGW GTP-U interface for UL data forwarding
// 29: Sn SGSN GTP-U interface
// 30: S2b ePDG GTP-C interface
// 31: S2b-U ePDG GTP-U interface
// 32: S2b PGW GTP-C interface
// 33: S2b-U PGW GTP-U interface
// 34:	S2a TWAN GTP-U interface
// 35:	S2a TWAN GTP-C interface
// 36: S2a PGW GTP-C interface
// 37: S2a PGW GTP-U interface
// 38: S11 MME GTP-U interface
// 39: S11 SGW GTP-U interface
// 40:	N26 AMF GTP-C interface
// 41: N19mb UPF GTP-U interface

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fteid {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub interface: u8,
    pub teid: u32,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl Default for Fteid {
    fn default() -> Fteid {
        Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 0,
            teid: 0,
            ipv4: Some(Ipv4Addr::new(0, 0, 0, 0)),
            ipv6: None,
        }
    }
}

impl From<Fteid> for InformationElement {
    fn from(i: Fteid) -> Self {
        InformationElement::Fteid(i)
    }
}

impl IEs for Fteid {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(FTEID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.ipv4.is_some(), self.ipv6.is_some()) {
            (true, true) => buffer_ie.push(0xC0 | self.interface),
            (true, false) => buffer_ie.push(0x80 | self.interface),
            (false, true) => buffer_ie.push(0x40 | self.interface),
            (false, false) => buffer_ie.push(0xC0 | self.interface),
        }
        buffer_ie.extend_from_slice(&self.teid.to_be_bytes());
        if let Some(i) = self.ipv4 {
            buffer_ie.extend_from_slice(&i.octets())
        };
        if let Some(i) = self.ipv6 {
            buffer_ie.extend_from_slice(&i.octets())
        };
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Fteid {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Fteid::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.interface = buffer[4] & 0x3f;
                data.teid = u32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]);
                match (buffer[4] >> 7, (buffer[4] >> 6) & 0x01) {
                    (1, 1) => {
                        data.ipv4 = Some(Ipv4Addr::from([
                            buffer[9], buffer[10], buffer[11], buffer[12],
                        ]));
                        let mut dst = [0; 16];
                        dst.copy_from_slice(&buffer[13..29]);
                        data.ipv6 = Some(Ipv6Addr::from(dst));
                    }
                    (1, 0) => {
                        data.ipv4 = Some(Ipv4Addr::from([
                            buffer[9], buffer[10], buffer[11], buffer[12],
                        ]));
                        data.ipv6 = None;
                    }
                    (0, 1) => {
                        data.ipv4 = None;
                        let mut dst = [0; 16];
                        dst.copy_from_slice(&buffer[9..25]);
                        data.ipv6 = Some(Ipv6Addr::from(dst));
                    }
                    _ => return Err(GTPV2Error::IEIncorrect(FTEID)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(FTEID))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(FTEID))
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
fn fteid_ie_ipv4_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x57, 0x00, 0x09, 0x00, 0x86, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let decoded = Fteid {
        t: FTEID,
        length: 9,
        ins: 0,
        interface: 6,
        teid: 0x27892f70,
        ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
        ipv6: None,
    };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fteid_ie_ipv6_unmarshal_test() {
    let encoded: [u8; 25] = [
        0x57, 0x00, 0x15, 0x00, 0x46, 0x27, 0x89, 0x2f, 0x70, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Fteid {
        t: FTEID,
        length: 0x15,
        ins: 0,
        interface: 6,
        teid: 0x27892f70,
        ipv4: None,
        ipv6: Some(Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0)),
    };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fteid_ie_ipv46_unmarshal_test() {
    let encoded: [u8; 29] = [
        0x57, 0x00, 0x19, 0x00, 0xc6, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Fteid {
        t: FTEID,
        length: 0x19,
        ins: 0,
        interface: 6,
        teid: 0x27892f70,
        ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
        ipv6: Some(Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0)),
    };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn fteid_ie_wrong_flags_unmarshal_test() {
    let encoded: [u8; 29] = [
        0x57, 0x00, 0x19, 0x00, 0x06, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    //let decoded = Fteid { t:FTEID, length:0x19, ins:0, interface:6, teid: 0x27892f70, ipv4: Some(Ipv4Addr::new(139,7,133,184)), ipv6:Some(Ipv6Addr::new(1,0,0,0,0,0,0,0)) };
    let i = Fteid::unmarshal(&encoded);
    assert_eq!(i, Err(GTPV2Error::IEIncorrect(FTEID)));
}

#[test]
fn fteid_ie_ipv4_marshal_test() {
    let encoded: [u8; 13] = [
        0x57, 0x00, 0x09, 0x00, 0x86, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let decoded = Fteid {
        t: FTEID,
        length: 9,
        ins: 0,
        interface: 6,
        teid: 0x27892f70,
        ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
        ipv6: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn fteid_ie_ipv6_marshal_test() {
    let encoded: [u8; 25] = [
        0x57, 0x00, 0x15, 0x00, 0x46, 0x27, 0x89, 0x2f, 0x70, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Fteid {
        t: FTEID,
        length: 0x15,
        ins: 0,
        interface: 6,
        teid: 0x27892f70,
        ipv4: None,
        ipv6: Some(Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0)),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn fteid_ie_ipv46_marshal_test() {
    let encoded: [u8; 29] = [
        0x57, 0x00, 0x19, 0x00, 0xc6, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = Fteid {
        t: FTEID,
        length: 0x19,
        ins: 0,
        interface: 6,
        teid: 0x27892f70,
        ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
        ipv6: Some(Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 0)),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
