// Charging ID IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Charging ID IE TL

pub const CHARGING_ID: u8 = 127;
pub const CHARGING_ID_LENGTH: usize = 4;

// Charging Characteristics IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct ChargingID {
    pub t: u8,
    pub value: u32, // 0 - Reserved Value
}

impl Default for ChargingID {
    fn default() -> Self {
        ChargingID {
            t: CHARGING_ID,
            value: 0,
        }
    }
}

impl IEs for ChargingID {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&u32::to_be_bytes(self.value));
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() > CHARGING_ID_LENGTH {
            let data = ChargingID {
                value: u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]),
                ..Default::default()
            };
            match data.value {
                0 => Err(GTPV1Error::IEIncorrect),
                _ => Ok(data),
            }
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        CHARGING_ID_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn charging_id_ie_marshal_test() {
    let ie_to_marshal = ChargingID {
        t: CHARGING_ID,
        value: 0xff,
    };
    let ie_marshalled: [u8; 5] = [0x7f, 0x00, 0x00, 0x00, 0xff];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn charging_id_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 5] = [0x7f, 0x00, 0x00, 0x00, 0xff];
    let ie_unmarshalled = ChargingID {
        t: CHARGING_ID,
        value: 0xff,
    };
    assert_eq!(
        ChargingID::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
