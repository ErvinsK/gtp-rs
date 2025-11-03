// Trace Reference IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Trace Reference IE TL

pub const TRACEREF: u8 = 83;
pub const TRACEREF_LENGTH: usize = 6;

// Trace Reference IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceReference {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub trace_id: u32,
}

impl Default for TraceReference {
    fn default() -> Self {
        TraceReference {
            t: TRACEREF,
            length: TRACEREF_LENGTH as u16,
            ins: 0,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            trace_id: 0,
        }
    }
}

impl From<TraceReference> for InformationElement {
    fn from(i: TraceReference) -> Self {
        InformationElement::TraceReference(i)
    }
}

impl IEs for TraceReference {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TRACEREF);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer_ie.extend_from_slice(&self.trace_id.to_be_bytes()[1..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + TRACEREF_LENGTH {
            let mut data = TraceReference {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                trace_id: u32::from_be_bytes([0x00, buffer[7], buffer[8], buffer[9]]),
                ..TraceReference::default()
            };
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..7]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TRACEREF))
        }
    }

    fn len(&self) -> usize {
        TRACEREF_LENGTH + MIN_IE_SIZE
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
fn trace_ref_ie_marshal_test() {
    let decoded = TraceReference {
        t: TRACEREF,
        length: TRACEREF_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xffffff,
    };
    let encoded: [u8; 10] = [0x53, 0x00, 0x06, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xff];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn trace_ref_ie_unmarshal_test() {
    let decoded = TraceReference {
        t: TRACEREF,
        length: TRACEREF_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        trace_id: 0xffffff,
    };
    let encoded: [u8; 10] = [0x53, 0x00, 0x06, 0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xff];
    assert_eq!(TraceReference::unmarshal(&encoded).unwrap(), decoded);
}
