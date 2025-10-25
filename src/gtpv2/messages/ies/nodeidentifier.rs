// Node Identifier IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Node Identifier IE Type

pub const NODE_ID: u8 = 176;

// Node Identfier IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeIdentifier {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub node_name: String,
    pub node_realm: String,
}

impl Default for NodeIdentifier {
    fn default() -> Self {
        NodeIdentifier {
            t: NODE_ID,
            length: 0,
            ins: 0,
            node_name: "".to_string(),
            node_realm: "".to_string(),
        }
    }
}

impl From<NodeIdentifier> for InformationElement {
    fn from(i: NodeIdentifier) -> Self {
        InformationElement::NodeIdentifier(i)
    }
}

impl IEs for NodeIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(NODE_ID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let node = self.node_name.as_bytes();
        buffer_ie.push(node.len() as u8);
        buffer_ie.extend_from_slice(node);
        let realm = self.node_realm.as_bytes();
        buffer_ie.push(realm.len() as u8);
        buffer_ie.extend_from_slice(realm);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = NodeIdentifier {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..NodeIdentifier::default()
            };
            let mut cursor = MIN_IE_SIZE + 1;
            if check_tliv_ie_buffer(data.length, buffer) {
                if (cursor + buffer[4] as usize + 1) <= buffer.len() {
                    let donor: Vec<u8> = buffer[cursor..(cursor + buffer[4] as usize)].to_vec();
                    data.node_name = donor.iter().map(|x| *x as char).collect();
                    cursor += buffer[4] as usize;
                } else {
                    return Err(GTPV2Error::IEInvalidLength(NODE_ID));
                }
                if ((cursor + 1) + buffer[cursor] as usize) <= buffer.len() {
                    let donor: Vec<u8> =
                        buffer[(cursor + 1)..((cursor + 1) + buffer[cursor] as usize)].to_vec();
                    data.node_realm = donor.iter().map(|x| *x as char).collect();
                } else {
                    return Err(GTPV2Error::IEInvalidLength(NODE_ID));
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(NODE_ID))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(NODE_ID))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn node_id_ie_marshal_test() {
    let encoded: [u8; 58] = [
        0xb0, 0x00, 0x36, 0x00, 0x13, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x2e, 0x6e, 0x6f, 0x64, 0x65,
        0x73, 0x2e, 0x70, 0x67, 0x77, 0x2e, 0x73, 0x65, 0x2e, 0x21, 0x65, 0x70, 0x63, 0x2e, 0x6d,
        0x6e, 0x63, 0x30, 0x35, 0x2e, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x2e, 0x33, 0x67, 0x70,
        0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    ];

    let decoded = NodeIdentifier {
        t: NODE_ID,
        length: 54,
        ins: 0,
        node_name: "topon.nodes.pgw.se.".to_string(),
        node_realm: "epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn node_id_ie_unmarshal_test() {
    let encoded: [u8; 58] = [
        0xb0, 0x00, 0x36, 0x00, 0x13, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x2e, 0x6e, 0x6f, 0x64, 0x65,
        0x73, 0x2e, 0x70, 0x67, 0x77, 0x2e, 0x73, 0x65, 0x2e, 0x21, 0x65, 0x70, 0x63, 0x2e, 0x6d,
        0x6e, 0x63, 0x30, 0x35, 0x2e, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x2e, 0x33, 0x67, 0x70,
        0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    ];

    let decoded = NodeIdentifier {
        t: NODE_ID,
        length: 54,
        ins: 0,
        node_name: "topon.nodes.pgw.se.".to_string(),
        node_realm: "epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
    };
    assert_eq!(NodeIdentifier::unmarshal(&encoded).unwrap(), decoded);
}
