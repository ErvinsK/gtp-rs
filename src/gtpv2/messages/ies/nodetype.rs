// Node Type IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Node Type IE Type

pub const NODETYPE: u8 = 135;
pub const NODETYPE_LENGTH: usize = 1;

// Node Type Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Mme,
    Sgsn,
}

impl From<&Node> for u8 {
    fn from(i: &Node) -> u8 {
        match i {
            Node::Mme => 0,
            Node::Sgsn => 1,
        }
    }
}

impl From<u8> for Node {
    fn from(i: u8) -> Node {
        match i {
            0 => Node::Mme,
            1 => Node::Sgsn,
            _ => Node::Sgsn,
        }
    }
}

// Node Type IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeType {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub node: Node,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType {
            t: NODETYPE,
            length: NODETYPE_LENGTH as u16,
            ins: 0,
            node: Node::Mme,
        }
    }
}

impl From<NodeType> for InformationElement {
    fn from(i: NodeType) -> Self {
        InformationElement::NodeType(i)
    }
}

impl IEs for NodeType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(NODETYPE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.node));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= NODETYPE_LENGTH + MIN_IE_SIZE {
            let data = NodeType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                node: Node::from(buffer[4]),
                ..NodeType::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(NODETYPE))
        }
    }

    fn len(&self) -> usize {
        NODETYPE_LENGTH + MIN_IE_SIZE
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
fn node_type_ie_marshal_test() {
    let encoded: [u8; 5] = [0x87, 0x00, 0x01, 0x00, 0x00];
    let decoded = NodeType {
        t: NODETYPE,
        length: NODETYPE_LENGTH as u16,
        ins: 0,
        node: Node::Mme,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn apnrestriciton_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x87, 0x00, 0x01, 0x00, 0x00];
    let decoded = NodeType {
        t: NODETYPE,
        length: NODETYPE_LENGTH as u16,
        ins: 0,
        node: Node::Mme,
    };
    assert_eq!(NodeType::unmarshal(&encoded).unwrap(), decoded);
}
