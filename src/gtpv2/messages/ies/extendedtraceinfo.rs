// Extended Trace Information IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// Extended Trace Information IE

pub const EXTTRACEINFO: u8 = 205;

// Extended Trace Information IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedTraceInformation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub trace_id: u32,
    pub trigger_events: Vec<u8>,
    pub list_ne_types: Vec<u8>,
    pub session_trace_depth: u8,
    pub list_interfaces: Vec<u8>,
    pub trace_collection_ip: IpAddr,
}

impl Default for ExtendedTraceInformation {
    fn default() -> Self {
        ExtendedTraceInformation {
            t: EXTTRACEINFO,
            length: 0,
            ins: 0,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            trace_id: 0,
            trigger_events: vec![],
            list_ne_types: vec![],
            session_trace_depth: 0,
            list_interfaces: vec![],
            trace_collection_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }
}

impl From<ExtendedTraceInformation> for InformationElement {
    fn from(i: ExtendedTraceInformation) -> Self {
        InformationElement::ExtendedTraceInformation(i)
    }
}

impl IEs for ExtendedTraceInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(EXTTRACEINFO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer_ie.extend_from_slice(&self.trace_id.to_be_bytes()[1..]);
        buffer_ie.push(self.trigger_events.len() as u8);
        buffer_ie.extend_from_slice(&self.trigger_events[..]);
        buffer_ie.push(self.list_ne_types.len() as u8);
        buffer_ie.extend_from_slice(&self.list_ne_types[..]);
        buffer_ie.push(self.session_trace_depth);
        buffer_ie.push(self.list_interfaces.len() as u8);
        buffer_ie.extend_from_slice(&self.list_interfaces[..]);
        match self.trace_collection_ip {
            IpAddr::V4(ip) => {
                buffer_ie.push(4);
                buffer_ie.extend_from_slice(&ip.octets());
            }
            IpAddr::V6(ip) => {
                buffer_ie.push(16);
                buffer_ie.extend_from_slice(&ip.octets());
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 7 {
            let mut data = ExtendedTraceInformation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                trace_id: u32::from_be_bytes([0x00, buffer[7], buffer[8], buffer[9]]),
                ..ExtendedTraceInformation::default()
            };
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..=6]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            let mut cursor: usize = 10;
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if buffer.len() < cursor + len {
                    return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO));
                }
                data.trigger_events
                    .extend_from_slice(&buffer[cursor..cursor + len]);
                cursor += len;
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if buffer.len() < cursor + len {
                    return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO));
                }
                data.list_ne_types
                    .extend_from_slice(&buffer[cursor..cursor + len]);
                cursor += len;
            }
            if buffer.len() < cursor + 1 {
                return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO));
            }
            data.session_trace_depth = buffer[cursor];
            cursor += 1;
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if buffer.len() < cursor + len {
                    return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO));
                }
                data.list_interfaces
                    .extend_from_slice(&buffer[cursor..cursor + len]);
                cursor += len;
            }
            {
                match buffer[cursor] {
                    4 => {
                        if buffer.len() < cursor + 5 {
                            return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO));
                        }
                        data.trace_collection_ip = IpAddr::V4(Ipv4Addr::new(
                            buffer[cursor + 1],
                            buffer[cursor + 2],
                            buffer[cursor + 3],
                            buffer[cursor + 4],
                        ));
                    }
                    16 => {
                        if buffer.len() < cursor + 17 {
                            return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO));
                        }
                        data.trace_collection_ip = IpAddr::V6(Ipv6Addr::new(
                            u16::from_be_bytes([buffer[cursor + 1], buffer[cursor + 2]]),
                            u16::from_be_bytes([buffer[cursor + 3], buffer[cursor + 4]]),
                            u16::from_be_bytes([buffer[cursor + 5], buffer[cursor + 6]]),
                            u16::from_be_bytes([buffer[cursor + 7], buffer[cursor + 8]]),
                            u16::from_be_bytes([buffer[cursor + 9], buffer[cursor + 10]]),
                            u16::from_be_bytes([buffer[cursor + 11], buffer[cursor + 12]]),
                            u16::from_be_bytes([buffer[cursor + 13], buffer[cursor + 14]]),
                            u16::from_be_bytes([buffer[cursor + 15], buffer[cursor + 16]]),
                        ));
                    }
                    _ => return Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO)),
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(EXTTRACEINFO))
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
fn ext_trace_info_ie_ipv4_marshal_test() {
    let encoded: [u8; 42] = [
        0xcd, 0x00, 0x26, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x09, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x09, 0x0a, 0xfa, 0xfa, 0xfa,
        0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0x04, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let decoded = ExtendedTraceInformation {
        length: 38,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xfffffa,
        trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        list_ne_types: vec![0xff, 0xff, 0xff, 0xff],
        session_trace_depth: 0x09,
        list_interfaces: vec![0xfa; 10],
        trace_collection_ip: IpAddr::V4(Ipv4Addr::new(139, 7, 133, 184)),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ext_trace_info_ie_ipv4_unmarshal_test() {
    let encoded: [u8; 42] = [
        0xcd, 0x00, 0x26, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x09, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x09, 0x0a, 0xfa, 0xfa, 0xfa,
        0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0x04, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let decoded = ExtendedTraceInformation {
        length: 38,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xfffffa,
        trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        list_ne_types: vec![0xff, 0xff, 0xff, 0xff],
        session_trace_depth: 0x09,
        list_interfaces: vec![0xfa; 10],
        trace_collection_ip: IpAddr::V4(Ipv4Addr::new(139, 7, 133, 184)),
        ..Default::default()
    };
    assert_eq!(
        ExtendedTraceInformation::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn ext_trace_info_ie_ipv6_marshal_test() {
    let encoded: [u8; 54] = [
        0xcd, 0x00, 0x32, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x09, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x09, 0x0a, 0xfa, 0xfa, 0xfa,
        0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0x10, 0x00, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    let decoded = ExtendedTraceInformation {
        length: 50,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xfffffa,
        trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        list_ne_types: vec![0xff, 0xff, 0xff, 0xff],
        session_trace_depth: 0x09,
        list_interfaces: vec![0xfa; 10],
        trace_collection_ip: IpAddr::V6(Ipv6Addr::new(
            0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        )),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ext_trace_info_ie_ipv6_unmarshal_test() {
    let encoded: [u8; 54] = [
        0xcd, 0x00, 0x32, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x09, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x09, 0x0a, 0xfa, 0xfa, 0xfa,
        0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0x10, 0x00, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    let decoded = ExtendedTraceInformation {
        length: 50,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xfffffa,
        trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        list_ne_types: vec![0xff, 0xff, 0xff, 0xff],
        session_trace_depth: 0x09,
        list_interfaces: vec![0xfa; 10],
        trace_collection_ip: IpAddr::V6(Ipv6Addr::new(
            0xfd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        )),
        ..Default::default()
    };
    assert_eq!(
        ExtendedTraceInformation::unmarshal(&encoded).unwrap(),
        decoded
    );
}
