// Node Features IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Node Features IE TL

pub const NODEFEATURES: u8 = 152;
pub const NODEFEATURES_LENGTH: usize = 1;

// Node Features IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFeatures {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub prn: bool,   // PGW Restart Notification
    pub mabr: bool,  // Modify Access Bearers Request
    pub ntsr: bool,  // Network Triggered Service Restoration
    pub ciot: bool,  // Cellular IoT
    pub s1un: bool,  // S1-U path notification feature
    pub eth: bool,   // Ethernet PDN Type
    pub mtedt: bool, // Support of MT-EDT
    pub psset: bool, // Support of PGW-C/SMF Set
}

impl Default for NodeFeatures {
    fn default() -> Self {
        NodeFeatures {
            t: NODEFEATURES,
            length: NODEFEATURES_LENGTH as u16,
            ins: 0,
            prn: false,
            mabr: false,
            ntsr: false,
            ciot: false,
            s1un: false,
            eth: false,
            mtedt: false,
            psset: false,
        }
    }
}

impl From<NodeFeatures> for InformationElement {
    fn from(i: NodeFeatures) -> Self {
        InformationElement::NodeFeatures(i)
    }
}

impl IEs for NodeFeatures {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(NODEFEATURES);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags = (self.prn as u8)
            | ((self.mabr as u8) << 1)
            | ((self.ntsr as u8) << 2)
            | ((self.ciot as u8) << 3)
            | ((self.s1un as u8) << 4)
            | ((self.eth as u8) << 5)
            | ((self.mtedt as u8) << 6)
            | ((self.psset as u8) << 7);
        buffer_ie.push(flags);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= (NODEFEATURES_LENGTH + MIN_IE_SIZE) {
            let data = NodeFeatures {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                prn: (buffer[4] & 0x01) == 0x01,
                mabr: (buffer[4] & 0x02) == 0x02,
                ntsr: (buffer[4] & 0x04) == 0x04,
                ciot: (buffer[4] & 0x08) == 0x08,
                s1un: (buffer[4] & 0x10) == 0x10,
                eth: (buffer[4] & 0x20) == 0x20,
                mtedt: (buffer[4] & 0x40) == 0x40,
                psset: (buffer[4] & 0x80) == 0x80,
                ..NodeFeatures::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(NODEFEATURES))
        }
    }

    fn len(&self) -> usize {
        NODEFEATURES_LENGTH + MIN_IE_SIZE
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
fn node_features_ie_marshal_test() {
    let encoded: [u8; 5] = [0x98, 0x00, 0x01, 0x00, 0xff];
    let decoded = NodeFeatures {
        t: NODEFEATURES,
        length: NODEFEATURES_LENGTH as u16,
        ins: 0,
        prn: true,
        mabr: true,
        ntsr: true,
        ciot: true,
        s1un: true,
        eth: true,
        mtedt: true,
        psset: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn node_features_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x98, 0x00, 0x01, 0x00, 0x7e];
    let decoded = NodeFeatures {
        t: NODEFEATURES,
        length: NODEFEATURES_LENGTH as u16,
        ins: 0,
        prn: false,
        mabr: true,
        ntsr: true,
        ciot: true,
        s1un: true,
        eth: true,
        mtedt: true,
        psset: false,
    };
    assert_eq!(NodeFeatures::unmarshal(&encoded).unwrap(), decoded);
}
