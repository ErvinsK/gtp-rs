// CSG Membership Indication (CMI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// CSG Membership Indication (CMI) IE Type

pub const CMI: u8 = 148;
pub const CMI_LENGTH: usize = 1;

// CSG Membership Indication Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsgMembership {
    CsgMembership,
    NonCsgMembership,
}

impl From<&CsgMembership> for u8 {
    fn from(i: &CsgMembership) -> u8 {
        match i {
            CsgMembership::CsgMembership => 0,
            CsgMembership::NonCsgMembership => 1,
        }
    }
}

impl From<u8> for CsgMembership {
    fn from(i: u8) -> CsgMembership {
        match i {
            0 => CsgMembership::CsgMembership,
            1 => CsgMembership::NonCsgMembership,
            _ => CsgMembership::NonCsgMembership,
        }
    }
}

// CSG Membership Indication (CMI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsgMembershipIndication {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub cmi: CsgMembership,
}

impl Default for CsgMembershipIndication {
    fn default() -> Self {
        CsgMembershipIndication {
            t: CMI,
            length: CMI_LENGTH as u16,
            ins: 0,
            cmi: CsgMembership::NonCsgMembership,
        }
    }
}

impl From<CsgMembershipIndication> for InformationElement {
    fn from(i: CsgMembershipIndication) -> Self {
        InformationElement::CsgMembershipIndication(i)
    }
}

impl IEs for CsgMembershipIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(CMI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.cmi));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= CMI_LENGTH + MIN_IE_SIZE {
            let data = CsgMembershipIndication {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                cmi: CsgMembership::from(buffer[4]),
                ..CsgMembershipIndication::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CMI))
        }
    }

    fn len(&self) -> usize {
        CMI_LENGTH + MIN_IE_SIZE
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
fn cmi_ie_marshal_test() {
    let encoded: [u8; 5] = [0x94, 0x00, 0x01, 0x00, 0x01];
    let decoded = CsgMembershipIndication {
        cmi: CsgMembership::NonCsgMembership,
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn cmi_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x94, 0x00, 0x01, 0x00, 0x01];
    let decoded = CsgMembershipIndication {
        cmi: CsgMembership::NonCsgMembership,
        ..Default::default()
    };
    assert_eq!(
        CsgMembershipIndication::unmarshal(&encoded).unwrap(),
        decoded
    );
}
