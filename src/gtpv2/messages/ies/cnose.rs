// CN Operator Selection Entity (CNOSE) IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// CNOSE IE TV

pub const CNOSE:u8 = 173;
pub const CNOSE_LENGTH:usize = 1;

// CNOSE IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct CnOperatorSelectionEntity {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub selection_entity:SelectionMode,         
}

// Selection Mode enum
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionMode {
    ServingNetworkSelectedbyUE,
    ServingNetworkSelectedbyNetwork,
}

impl Default for CnOperatorSelectionEntity {
    fn default() -> CnOperatorSelectionEntity {
        CnOperatorSelectionEntity { t: CNOSE, length:CNOSE_LENGTH as u16, ins:0, selection_entity: SelectionMode::ServingNetworkSelectedbyUE }        
    }
}

impl From<CnOperatorSelectionEntity> for InformationElement {
    fn from(i: CnOperatorSelectionEntity) -> Self {
        InformationElement::CnOperatorSelectionEntity(i)
    }
}

impl IEs for CnOperatorSelectionEntity {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.selection_entity {
            SelectionMode::ServingNetworkSelectedbyUE => buffer_ie.push(0x00),
            SelectionMode::ServingNetworkSelectedbyNetwork => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=CNOSE_LENGTH+MIN_IE_SIZE {
            let mut data = CnOperatorSelectionEntity::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            match buffer[4] {
                0 => data.selection_entity = SelectionMode::ServingNetworkSelectedbyUE,
                1 => data.selection_entity = SelectionMode::ServingNetworkSelectedbyNetwork,
               2 | 3 => data.selection_entity = SelectionMode::ServingNetworkSelectedbyNetwork,
                _ => return Err(GTPV2Error::IEIncorrect(CNOSE)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CNOSE))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn cnose_ie_unmarshal_test () {
    let encoded_ie:[u8;5]=[0xad, 0x00, 0x01, 0x00, 0x01];
    let test_struct = CnOperatorSelectionEntity { t:CNOSE, length: CNOSE_LENGTH as u16, ins:0, selection_entity: SelectionMode::ServingNetworkSelectedbyNetwork };
    let i = CnOperatorSelectionEntity::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn cnose_ie_marshal_test () {
    let encoded_ie:[u8;5]=[0xad, 0x00, 0x01, 0x00, 0x01];
    let test_struct = CnOperatorSelectionEntity { t:CNOSE, length: CNOSE_LENGTH as u16, ins:0, selection_entity: SelectionMode::ServingNetworkSelectedbyNetwork };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
