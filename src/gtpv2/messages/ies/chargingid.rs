// Charging ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Charging ID IE Type

pub const CHARGINGID: u8 = 94;
pub const CHARGINGID_LENGTH: usize = 4;

// Charging ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChargingId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub charging_id: u32,
}

impl Default for ChargingId {
    fn default() -> Self {
        ChargingId {
            t: CHARGINGID,
            length: CHARGINGID_LENGTH as u16,
            ins: 0,
            charging_id: 0,
        }
    }
}

impl From<ChargingId> for InformationElement {
    fn from(i: ChargingId) -> Self {
        InformationElement::ChargingId(i)
    }
}

impl IEs for ChargingId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CHARGINGID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.charging_id.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + CHARGINGID_LENGTH {
            let data = ChargingId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                charging_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..ChargingId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CHARGINGID))
        }
    }

    fn len(&self) -> usize {
        CHARGINGID_LENGTH + MIN_IE_SIZE
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
fn chargingid_ie_marshal_test() {
    let encoded: [u8; 8] = [0x5e, 0x00, 0x04, 0x00, 0xff, 0xff, 0x00, 0xff];
    let decoded = ChargingId {
        t: CHARGINGID,
        length: 4,
        ins: 0,
        charging_id: 0xffff00ff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn chargingid_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0x5e, 0x00, 0x04, 0x00, 0xff, 0xff, 0x00, 0xff];
    let decoded = ChargingId {
        t: CHARGINGID,
        length: 4,
        ins: 0,
        charging_id: 0xffff00ff,
    };
    assert_eq!(ChargingId::unmarshal(&encoded).unwrap(), decoded);
}
