// Paging and Service Information IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Paging and Service Information IE TL

pub const PAGING_SRVC_INFO: u8 = 186;
pub const PAGING_SRVC_INFO_LENGTH: usize = 2;

// Paging and Service Information IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PagingServiceInfo {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ebi: u8,
    pub paging_policy: Option<u8>,
}

impl Default for PagingServiceInfo {
    fn default() -> Self {
        PagingServiceInfo {
            t: PAGING_SRVC_INFO,
            length: PAGING_SRVC_INFO_LENGTH as u16,
            ins: 0,
            ebi: 0,
            paging_policy: None,
        }
    }
}

impl From<PagingServiceInfo> for InformationElement {
    fn from(i: PagingServiceInfo) -> Self {
        InformationElement::PagingServiceInfo(i)
    }
}

impl IEs for PagingServiceInfo {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PAGING_SRVC_INFO);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.ebi);
        match self.paging_policy {
            Some(i) => {
                buffer_ie.push(0x01);
                buffer_ie.push(i);
            }
            None => buffer_ie.push(0x00),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= PAGING_SRVC_INFO_LENGTH + MIN_IE_SIZE {
            let data = PagingServiceInfo {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ebi: buffer[4],
                paging_policy: match buffer[5] {
                    0 => None,
                    1 => Some(buffer[6]),
                    _ => return Err(GTPV2Error::IEIncorrect(PAGING_SRVC_INFO)),
                },
                ..PagingServiceInfo::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PAGING_SRVC_INFO))
        }
    }

    fn len(&self) -> usize {
        if self.paging_policy.is_some() {
            PAGING_SRVC_INFO_LENGTH + MIN_IE_SIZE + 1
        } else {
            PAGING_SRVC_INFO_LENGTH + MIN_IE_SIZE
        }
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
fn paging_service_info_ie_unmarshal_test() {
    let encoded: [u8; 7] = [0xba, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03];
    let decoded = PagingServiceInfo {
        t: PAGING_SRVC_INFO,
        length: 3,
        ins: 0,
        ebi: 2,
        paging_policy: Some(0x03),
    };
    let i = PagingServiceInfo::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paging_service_info_ie_marshal_test() {
    let encoded: [u8; 7] = [0xba, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03];
    let decoded = PagingServiceInfo {
        t: PAGING_SRVC_INFO,
        length: 3,
        ins: 0,
        ebi: 2,
        paging_policy: Some(0x03),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
