// Charging Characteristics IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Charging Characteristics IE Type

pub const CHARGINGCHAR: u8 = 95;
pub const CHARGINGCHAR_LENGTH: usize = 2;

// Charging Characteristics IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChargingCharacteristics {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub charging_char: u16,
}

impl Default for ChargingCharacteristics {
    fn default() -> Self {
        ChargingCharacteristics {
            t: CHARGINGCHAR,
            length: 2,
            ins: 0,
            charging_char: 0,
        }
    }
}

impl From<ChargingCharacteristics> for InformationElement {
    fn from(i: ChargingCharacteristics) -> Self {
        InformationElement::ChargingCharacteristics(i)
    }
}

impl IEs for ChargingCharacteristics {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CHARGINGCHAR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.charging_char.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + CHARGINGCHAR_LENGTH {
            let data = ChargingCharacteristics {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                charging_char: u16::from_be_bytes([buffer[4], buffer[5]]),
                ..ChargingCharacteristics::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CHARGINGCHAR))
        }
    }

    fn len(&self) -> usize {
        CHARGINGCHAR_LENGTH + MIN_IE_SIZE
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
fn chargingchar_ie_marshal_test() {
    let encoded: [u8; 6] = [0x5f, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = ChargingCharacteristics {
        t: CHARGINGCHAR,
        length: 2,
        ins: 0,
        charging_char: 0xffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn chargingchar_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x5f, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = ChargingCharacteristics {
        t: CHARGINGCHAR,
        length: 2,
        ins: 0,
        charging_char: 0xffff,
    };
    assert_eq!(
        ChargingCharacteristics::unmarshal(&encoded).unwrap(),
        decoded
    );
}
