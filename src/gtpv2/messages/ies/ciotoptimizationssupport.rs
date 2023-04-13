// CIoT Optimization Support Indication IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// CIoT Optimization Support Indication IE TL

pub const CIOT_SUPPORT: u8 = 194;
pub const CIOT_SUPPORT_LENGTH: usize = 1;

// Node Features IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CIoTOptimizationSupportIndication {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub sgnipdn: bool, // SGNIPDN (SGi Non IP PDN Support Indication): Indicates the support of SGi Non IP PDN Connection
    pub scnipdn: bool, // SCNIPDN (SCEF Non IP PDN Support Indication): Indicates the support of SCEF Non IP PDN Connection
    pub awopdn: bool, // AWOPDN (Attach without PDN Support Indication): Indicates the support of Attach without PDN connection
    pub ihcsi: bool, // IHCSI (IP Header Compression Support Indication): Indicates the support of IP header compression based on ROHC framework
}

impl Default for CIoTOptimizationSupportIndication {
    fn default() -> Self {
        CIoTOptimizationSupportIndication {
            t: CIOT_SUPPORT,
            length: CIOT_SUPPORT_LENGTH as u16,
            ins: 0,
            sgnipdn: false,
            scnipdn: false,
            awopdn: false,
            ihcsi: false,
        }
    }
}

impl From<CIoTOptimizationSupportIndication> for InformationElement {
    fn from(i: CIoTOptimizationSupportIndication) -> Self {
        InformationElement::CIoTOptimizationSupportIndication(i)
    }
}

impl IEs for CIoTOptimizationSupportIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags = self
            .clone()
            .intoarray()
            .iter()
            .map(|x| if *x { 1 } else { 0 })
            .enumerate()
            .map(|(i, x)| x << i)
            .collect::<Vec<_>>()
            .iter()
            .sum::<u8>();
        buffer_ie.push(flags);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= CIOT_SUPPORT_LENGTH + MIN_IE_SIZE {
            let mut data = CIoTOptimizationSupportIndication {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            let flags = [buffer[4]; 5]
                .iter()
                .enumerate()
                .map(|(i, x)| ((*x >> i) & 0x01) == 1)
                .collect::<Vec<bool>>();
            data.fromarray(&flags[..]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CIOT_SUPPORT))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl CIoTOptimizationSupportIndication {
    fn intoarray(self) -> [bool; 4] {
        [self.sgnipdn, self.scnipdn, self.awopdn, self.ihcsi]
    }
    fn fromarray(&mut self, i: &[bool]) {
        self.sgnipdn = i[0];
        self.scnipdn = i[1];
        self.awopdn = i[2];
        self.ihcsi = i[3];
    }
}

#[test]
fn ciot_support_ie_marshal_test() {
    let encoded: [u8; 5] = [0xc2, 0x00, 0x01, 0x00, 0x09];
    let decoded = CIoTOptimizationSupportIndication {
        t: CIOT_SUPPORT,
        length: CIOT_SUPPORT_LENGTH as u16,
        ins: 0,
        sgnipdn: true,
        scnipdn: false,
        awopdn: false,
        ihcsi: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ciot_support_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xc2, 0x00, 0x01, 0x00, 0x09];
    let decoded = CIoTOptimizationSupportIndication {
        t: CIOT_SUPPORT,
        length: CIOT_SUPPORT_LENGTH as u16,
        ins: 0,
        sgnipdn: true,
        scnipdn: false,
        awopdn: false,
        ihcsi: true,
    };
    assert_eq!(
        CIoTOptimizationSupportIndication::unmarshal(&encoded).unwrap(),
        decoded
    );
}
