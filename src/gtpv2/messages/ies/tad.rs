// Traffic Aggregate Descriptor (TAD) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// TAD IE Type

pub const TAD: u8 = 85;

// TAD IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrafficAggregateDescription {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub tad: Vec<u8>,
}

impl Default for TrafficAggregateDescription {
    fn default() -> Self {
        TrafficAggregateDescription {
            t: TAD,
            length: 0,
            ins: 0,
            tad: vec![],
        }
    }
}

impl From<TrafficAggregateDescription> for InformationElement {
    fn from(i: TrafficAggregateDescription) -> Self {
        InformationElement::TrafficAggregateDescription(i)
    }
}

impl IEs for TrafficAggregateDescription {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TAD);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.tad.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = TrafficAggregateDescription {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..TrafficAggregateDescription::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.tad.extend_from_slice(
                    &buffer[MIN_IE_SIZE..(MIN_IE_SIZE + (data.length as usize))],
                );
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(TAD))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(TAD))
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
fn tad_ie_marshal_test() {
    let encoded: [u8; 8] = [0x55, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = TrafficAggregateDescription {
        t: TAD,
        length: 4,
        ins: 0,
        tad: vec![0, 0, 0, 0],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn tad_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x55, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = TrafficAggregateDescription {
        t: TAD,
        length: 4,
        ins: 0,
        tad: vec![0, 0, 0, 0],
    };
    assert_eq!(
        TrafficAggregateDescription::unmarshal(&encoded).unwrap(),
        decoded
    );
}
