// Additional RRM Policy Index (ARPI) ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Additional RRM Policy Index (ARPI) ID IE Type

pub const ARPI: u8 = 207;
pub const ARPI_LENGTH: usize = 4;

// Additional RRM Policy Index (ARPI) ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalRrmPolicyIndex {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub arpi: u32,
}

impl Default for AdditionalRrmPolicyIndex {
    fn default() -> Self {
        AdditionalRrmPolicyIndex {
            t: ARPI,
            length: ARPI_LENGTH as u16,
            ins: 0,
            arpi: 0,
        }
    }
}

impl From<AdditionalRrmPolicyIndex> for InformationElement {
    fn from(i: AdditionalRrmPolicyIndex) -> Self {
        InformationElement::AdditionalRrmPolicyIndex(i)
    }
}

impl IEs for AdditionalRrmPolicyIndex {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ARPI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.arpi.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + ARPI_LENGTH {
            let data = AdditionalRrmPolicyIndex {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                arpi: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..AdditionalRrmPolicyIndex::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ARPI))
        }
    }

    fn len(&self) -> usize {
        ARPI_LENGTH + MIN_IE_SIZE
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
fn arpi_ie_marshal_test() {
    let encoded: [u8; 8] = [0xcf, 0x00, 0x04, 0x00, 0xff, 0xff, 0x00, 0xff];
    let decoded = AdditionalRrmPolicyIndex {
        t: ARPI,
        length: 4,
        ins: 0,
        arpi: 0xffff00ff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn arpi_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0xcf, 0x00, 0x04, 0x00, 0xff, 0xff, 0x00, 0xff];
    let decoded = AdditionalRrmPolicyIndex {
        t: ARPI,
        length: 4,
        ins: 0,
        arpi: 0xffff00ff,
    };
    assert_eq!(
        AdditionalRrmPolicyIndex::unmarshal(&encoded).unwrap(),
        decoded
    );
}
