// Charging Characteristics IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Charging Characteristics IE TL

pub const CHARGING_CHARACTERISTICS: u8 = 26;
pub const CHARGING_CHARACTERISTICS_LENGTH: usize = 2;

// Charging Characteristics IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct ChargingCharacteristics {
    pub t: u8,
    pub value: u8, // Normal charging = 0b1000, Prepaid charging = 0b0100, Flat rate charging = 0b0010, Hot billing charging = 0b0001
}

impl Default for ChargingCharacteristics {
    fn default() -> Self {
        ChargingCharacteristics {
            t: CHARGING_CHARACTERISTICS,
            value: 0,
        }
    }
}

impl IEs for ChargingCharacteristics {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.value);
        buffer.push(0x00);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() > CHARGING_CHARACTERISTICS_LENGTH {
            let data = ChargingCharacteristics {
                value: buffer[1] & 0b1111,
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        CHARGING_CHARACTERISTICS_LENGTH + 1
    }

    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn charging_characteristics_ie_marshal_test() {
    let ie_to_marshal = ChargingCharacteristics {
        t: CHARGING_CHARACTERISTICS,
        value: 0b1000,
    };
    let ie_marshalled: [u8; 3] = [0x1a, 0x08, 0x00];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn charging_characteristics_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 3] = [0x1a, 0x08, 0x00];
    let ie_unmarshalled = ChargingCharacteristics {
        t: CHARGING_CHARACTERISTICS,
        value: 0b1000,
    };
    assert_eq!(
        ChargingCharacteristics::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
