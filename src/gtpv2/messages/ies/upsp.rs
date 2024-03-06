// UP Security Policy IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// UP Security Policy IE TV

pub const UPSP: u8 = 218;
pub const UPSP_LENGTH: usize = 1;

// UP IP Policy Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpIpPolicy {
    #[default]
    NotNeeded = 0, // User Plane IP Integrity protection with ESP is not needed
    Preferred = 1, // User Plane IP Integrity protection with ESP is preferred
    Required = 2,  // User Plane IP Integrity protection with ESP is required
    Spare = 3,     // Spare
}

impl From<&UpIpPolicy> for u8 {
    fn from(i: &UpIpPolicy) -> u8 {
        match i {
            UpIpPolicy::NotNeeded => 0,
            UpIpPolicy::Preferred => 1,
            UpIpPolicy::Required => 2,
            UpIpPolicy::Spare => 3,
        }
    }
}

impl From<u8> for UpIpPolicy {
    fn from(i: u8) -> UpIpPolicy {
        match i {
            0 => UpIpPolicy::NotNeeded,
            1 => UpIpPolicy::Preferred,
            2 => UpIpPolicy::Required,
            3 => UpIpPolicy::Spare,
            _ => UpIpPolicy::Spare,
        }
    }
}

// UP Security Policy IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpSecurityPolicy {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub up_ip_policy: UpIpPolicy,
}

impl Default for UpSecurityPolicy {
    fn default() -> Self {
        UpSecurityPolicy {
            t: UPSP,
            length: UPSP_LENGTH as u16,
            ins: 0,
            up_ip_policy: UpIpPolicy::NotNeeded,
        }
    }
}

impl From<UpSecurityPolicy> for InformationElement {
    fn from(i: UpSecurityPolicy) -> Self {
        InformationElement::UpSecurityPolicy(i)
    }
}

impl IEs for UpSecurityPolicy {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(UPSP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.up_ip_policy));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= UPSP_LENGTH + MIN_IE_SIZE {
            let data = UpSecurityPolicy {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                up_ip_policy: UpIpPolicy::from(buffer[4] & 0x03),
                ..UpSecurityPolicy::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(UPSP))
        }
    }

    fn len(&self) -> usize {
        UPSP_LENGTH + MIN_IE_SIZE
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
fn upsp_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xda, 0x00, 0x01, 0x00, 0x01];
    let decoded = UpSecurityPolicy {
        t: UPSP,
        length: UPSP_LENGTH as u16,
        ins: 0,
        up_ip_policy: UpIpPolicy::Preferred,
    };
    let i = UpSecurityPolicy::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn upsp_ie_marshal_test() {
    let encoded: [u8; 5] = [0xda, 0x00, 0x01, 0x00, 0x01];
    let decoded = UpSecurityPolicy {
        t: UPSP,
        length: UPSP_LENGTH as u16,
        ins: 0,
        up_ip_policy: UpIpPolicy::Preferred,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
