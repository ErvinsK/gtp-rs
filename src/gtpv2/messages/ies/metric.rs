// Metric IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Metric IE Type

pub const METRIC: u8 = 182;
pub const METRIC_LENGTH: usize = 1;

// Metric IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metric {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub metric: u8, // Metric indicates a percentage and may take binary coded integer values from and including 0 up to and including 100. Other values shall be considered as 0.
}

impl Default for Metric {
    fn default() -> Self {
        Metric {
            t: METRIC,
            length: METRIC_LENGTH as u16,
            ins: 0,
            metric: 0,
        }
    }
}

impl From<Metric> for InformationElement {
    fn from(i: Metric) -> Self {
        InformationElement::Metric(i)
    }
}

impl IEs for Metric {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(METRIC);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.metric);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + METRIC_LENGTH {
            let data = Metric {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                metric: if buffer[4] > 100 { 0 } else { buffer[4] },
                ..Metric::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(METRIC))
        }
    }

    fn len(&self) -> usize {
        METRIC_LENGTH + MIN_IE_SIZE
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
fn metric_ie_marshal_test() {
    let encoded: [u8; 5] = [0xb6, 0x00, 0x01, 0x00, 0x60];
    let decoded = Metric {
        t: METRIC,
        length: METRIC_LENGTH as u16,
        ins: 0,
        metric: 0x60,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn metric_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xb6, 0x00, 0x01, 0x00, 0x60];
    let decoded = Metric {
        t: METRIC,
        length: METRIC_LENGTH as u16,
        ins: 0,
        metric: 0x60,
    };
    assert_eq!(Metric::unmarshal(&encoded).unwrap(), decoded);
}
