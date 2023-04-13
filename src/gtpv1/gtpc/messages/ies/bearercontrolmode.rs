// Bearer Control Mode IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Bearer Control Mode IE TL

pub const BEARER_CONTROL_MODE: u8 = 184;
pub const BEARER_CONTROL_MODE_LENGTH: u16 = 1;

// Bearer Control Mode IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerControlMode {
    pub t: u8,
    pub length: u16,
    pub bearer_ctrl_mode: u8, //  0 - "MS_only", 1 - "MS/NW", other values reserved
}

impl Default for BearerControlMode {
    fn default() -> BearerControlMode {
        BearerControlMode {
            t: BEARER_CONTROL_MODE,
            length: BEARER_CONTROL_MODE_LENGTH,
            bearer_ctrl_mode: 0,
        }
    }
}

impl IEs for BearerControlMode {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.bearer_ctrl_mode);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= (BEARER_CONTROL_MODE_LENGTH + 3) as usize {
            let mut data = BearerControlMode {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match buffer[3] {
                i if i <= 1 => data.bearer_ctrl_mode = buffer[3],
                _ => return Err(GTPV1Error::IEIncorrect),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        BEARER_CONTROL_MODE_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn bearer_ctrl_mode_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xb8, 0x00, 0x01, 0x00];
    let test_struct = BearerControlMode {
        t: BEARER_CONTROL_MODE,
        length: BEARER_CONTROL_MODE_LENGTH,
        bearer_ctrl_mode: 0,
    };
    let i = BearerControlMode::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn bearer_ctrl_mode_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xb8, 0x00, 0x01, 0x00];
    let test_struct = BearerControlMode {
        t: BEARER_CONTROL_MODE,
        length: BEARER_CONTROL_MODE_LENGTH,
        bearer_ctrl_mode: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
