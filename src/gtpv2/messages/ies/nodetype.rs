// Node Type IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Node Type IE Type

pub const NODETYPE:u8 = 135;
pub const NODETYPE_LENGTH:usize = 1;

// Node Type Enum 

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Mme,
    Sgsn,
}

impl Node {
    fn enum_to_value (i:&Node) -> u8 {
        match i {
            Node::Mme => 0,
            Node::Sgsn => 1,
        }
    }
    fn value_to_enum (i:u8) -> Result<Node, GTPV2Error> {
        match i {
            0 => Ok(Node::Mme),
            1 => Ok(Node::Sgsn),
            _ => Err(GTPV2Error::IEIncorrect(NODETYPE)),
        }
    }
}

// Node Type IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeType {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub node:Node,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType { t: NODETYPE, length:NODETYPE_LENGTH as u16, ins:0, node:Node::Mme}
    }
}

impl From<NodeType> for InformationElement {
    fn from(i: NodeType) -> Self {
        InformationElement::NodeType(i)
    }
}

impl IEs for NodeType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push (Node::enum_to_value(&self.node));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=NODETYPE_LENGTH+MIN_IE_SIZE {
            let mut data=NodeType{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            match Node::value_to_enum(buffer[4]) {
                Ok(i) => data.node=i,
                Err(j) => return Err(j),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(NODETYPE))
        }
    }

    fn len (&self) -> usize {
       NODETYPE_LENGTH+MIN_IE_SIZE 
    }

    fn is_empty (&self) -> bool {
        self.length == 0
    }
}

#[test]
fn node_type_ie_marshal_test () {
    let encoded:[u8;5]=[0x87, 0x00, 0x01, 0x00, 0x00];
    let decoded = NodeType { t:NODETYPE, length: NODETYPE_LENGTH as u16, ins:0, node:Node::Mme };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn apnrestriciton_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x87, 0x00, 0x01, 0x00, 0x00];
    let decoded = NodeType { t:NODETYPE, length: NODETYPE_LENGTH as u16, ins:0, node:Node::Mme };
    assert_eq!(NodeType::unmarshal(&encoded).unwrap(), decoded);
}