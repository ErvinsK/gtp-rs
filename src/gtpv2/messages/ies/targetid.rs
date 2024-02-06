// Target Identification IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Target Identification IE Type

pub const TARGETID: u8 = 121;

// Targets

// RNC ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RncId {
    pub rai:Rai,
    pub rnc_id: u16,
    pub ext_rnc_id: Option<u16>,
}

// Macro eNB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroEnbId {
    pub macro_enb_id: MacroEnbId,
    pub tac: u16, 
}

// Extended Macro eNB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedMacroEnbId {
    pub ext_macro_enb_id: ExtendedMacroEnbId,
    pub tac: u16,
}

// Cell Identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellId {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub rac: u8,
    pub ci: u16,
}

// gNodeB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GNbId {
    pub mcc: u16,
    pub mnc: u16,
    pub gnb_id: Vec<u8>,
    pub etac: [u8; 3],          // 5GS Tracking Area Code
}   

// Macro NG-eNB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacronGeNbId {
    pub macro_ng_enb_id: MacroEnbId,
    pub etac: [u8; 3],          // 5GS Tracking Area Code
}   

// Extended NG-eNB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacronGeNbId {
    pub macro_ng_enb_id: ExtendedMacroEnbId,
    pub etac: [u8; 3],          // 5GS Tracking Area Code
}   

// NG-eNB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnGNbId {
    pub mcc: u16,
    pub mnc: u16,
    pub en_gnb_id: Vec<u8>,
    pub tac: Option<u16>,
    pub etac: Option<[u8; 3]>,    // 5GS Tracking Area Code
}   

// Target Type Enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetType {
    RncId,
    MacroeNbId,
    CellId,
    HomeeNbId,
    ExtendedMacroeNbId,
    GNbId,
    MacrongeNbId,
    ExtendedngeNbId,
    EngNbId,
    Spare,
}

impl From<&TargetType> for u8 {
    fn from(i: &TargetType) -> u8 {
        match i {
            TargetType::RncId => 0,
            TargetType::MacroEnbId => 1,
            TargetType::CellId => 2,
            TargetType::HomeEnbId => 3,
            TargetType::ExtendedMacroEnbId => 4,
            TargetType::GNbId => 5,
            TargetType::MacronGeNbId => 6,
            TargetType::ExtendedNgeNbId => 7,
            TargetType::EnGNbId => 8,
            _ => 9,
        }
    }
}

impl From<u8> for TargetType {
    fn from(i: u8) -> TargetType {
        match i {
            0 => TargetType::RncId,
            1 => TargetType::MacroEnbId,
            2 => TargetType::CellId,
            3 => TargetType::HomeEnbId,
            4 => TargetType::ExtendedMacroEnbId,
            5 => TargetType::GNbId,
            6 => TargetType::MacronGeNbId,
            7 => TargetType::ExtendedNgeNbId,
            8 => TargetType::EnGNbId,
            _ => TargetType::Spare,
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
        buffer_ie.push(self.t);
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
                ins : buffer[3],
                node : Node::from(buffer[4]),
                ..Default::default()
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
