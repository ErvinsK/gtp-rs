// Services Authorized IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Services Authorized IE TV

pub const SERVICES_AUTH: u8 = 210;
pub const SERVICES_AUTH_LENGTH: usize = 2;

// Service Authorized IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicesAuthorized {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub vehicle_ue_auth: bool,    // Vehicle UE Authorized
    pub pedestrian_ue_auth: bool, // Pedestrian UE Authorized
}

impl Default for ServicesAuthorized {
    fn default() -> ServicesAuthorized {
        ServicesAuthorized {
            t: SERVICES_AUTH,
            length: SERVICES_AUTH_LENGTH as u16,
            ins: 0,
            vehicle_ue_auth: false,
            pedestrian_ue_auth: false,
        }
    }
}

impl From<ServicesAuthorized> for InformationElement {
    fn from(i: ServicesAuthorized) -> Self {
        InformationElement::ServicesAuthorized(i)
    }
}

impl IEs for ServicesAuthorized {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SERVICES_AUTH);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.vehicle_ue_auth {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        match self.pedestrian_ue_auth {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= SERVICES_AUTH_LENGTH + MIN_IE_SIZE {
            let data = ServicesAuthorized {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                vehicle_ue_auth: match buffer[4] {
                    0 => false,
                    1 => true,
                    _ => return Err(GTPV2Error::IEIncorrect(SERVICES_AUTH)),
                },
                pedestrian_ue_auth: match buffer[5] {
                    0 => false,
                    1 => true,
                    _ => return Err(GTPV2Error::IEIncorrect(SERVICES_AUTH)),
                },
                ..ServicesAuthorized::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SERVICES_AUTH))
        }
    }

    fn len(&self) -> usize {
        SERVICES_AUTH_LENGTH + MIN_IE_SIZE
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
fn services_auth_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0xd2, 0x00, 0x02, 0x00, 0x00, 0x01];
    let decoded = ServicesAuthorized {
        t: SERVICES_AUTH,
        length: SERVICES_AUTH_LENGTH as u16,
        ins: 0,
        vehicle_ue_auth: false,
        pedestrian_ue_auth: true,
    };
    let i = ServicesAuthorized::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn services_auth_ie_marshal_test() {
    let encoded: [u8; 6] = [0xd2, 0x00, 0x02, 0x00, 0x00, 0x01];
    let decoded = ServicesAuthorized {
        t: SERVICES_AUTH,
        length: SERVICES_AUTH_LENGTH as u16,
        ins: 0,
        vehicle_ue_auth: false,
        pedestrian_ue_auth: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
