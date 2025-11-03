// Trace Information IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};
use std::net::Ipv4Addr;

// Trace Information IE TL

pub const TRACEINFO: u8 = 96;
pub const TRACEINFO_LENGTH: usize = 34;

// Trace Information IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceInformation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub trace_id: u32,
    pub trigger_events: Vec<u8>,
    pub list_ne_types: u16,
    pub trace_depth: u8,
    pub list_interfaces: Vec<u8>,
    pub trace_collection_ip: Ipv4Addr,
}

impl Default for TraceInformation {
    fn default() -> Self {
        TraceInformation {
            t: TRACEINFO,
            length: TRACEINFO_LENGTH as u16,
            ins: 0,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            trace_id: 0,
            trigger_events: vec![],
            list_ne_types: 0,
            trace_depth: 0,
            list_interfaces: vec![],
            trace_collection_ip: Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}

impl From<TraceInformation> for InformationElement {
    fn from(i: TraceInformation) -> Self {
        InformationElement::TraceInformation(i)
    }
}

impl IEs for TraceInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TRACEINFO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer_ie.extend_from_slice(&self.trace_id.to_be_bytes()[1..]);
        buffer_ie.append(&mut self.trigger_events.clone());
        buffer_ie.extend_from_slice(&self.list_ne_types.to_be_bytes());
        buffer_ie.push(self.trace_depth);
        buffer_ie.append(&mut self.list_interfaces.clone());
        buffer_ie.extend_from_slice(&self.trace_collection_ip.octets());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + TRACEINFO_LENGTH {
            let mut data = TraceInformation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..TraceInformation::default()
            };
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..=6]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.trace_id = u32::from_be_bytes([0x00, buffer[7], buffer[8], buffer[9]]);
            data.trigger_events.extend_from_slice(&buffer[10..=18]);
            data.list_ne_types = u16::from_be_bytes([buffer[19], buffer[20]]);
            data.trace_depth = buffer[21];
            data.list_interfaces.extend_from_slice(&buffer[22..=33]);
            data.trace_collection_ip =
                Ipv4Addr::from([buffer[34], buffer[35], buffer[36], buffer[37]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TRACEINFO))
        }
    }

    fn len(&self) -> usize {
        TRACEINFO_LENGTH + MIN_IE_SIZE
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
fn trace_info_ie_marshal_test() {
    let decoded = TraceInformation {
        t: TRACEINFO,
        length: TRACEINFO_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xfffffa,
        trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        list_ne_types: 0xaaaa,
        trace_depth: 0x09,
        list_interfaces: vec![0; 12],
        trace_collection_ip: Ipv4Addr::new(139, 7, 133, 184),
    };
    let encoded: [u8; 38] = [
        0x60, 0x00, 0x22, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn trace_info_ie_unmarshal_test() {
    let decoded = TraceInformation {
        t: TRACEINFO,
        length: TRACEINFO_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xfffffa,
        trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        list_ne_types: 0xaaaa,
        trace_depth: 0x09,
        list_interfaces: vec![0; 12],
        trace_collection_ip: Ipv4Addr::new(139, 7, 133, 184),
    };
    let encoded: [u8; 38] = [
        0x60, 0x00, 0x22, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x8b, 0x07, 0x85, 0xb8,
    ];
    assert_eq!(TraceInformation::unmarshal(&encoded).unwrap(), decoded);
}
