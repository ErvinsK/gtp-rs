// CN Operator Selection Entity (CNOSE) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{utils::*, errors::GTPV1Error, gtpc::messages::ies::commons::*};

// CNOSE IE TV

pub const CNOSE:u8 = 216;
pub const CNOSE_LENGTH:u16 = 1;

// CNOSE IE implementation 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CnOperatorSelectionEntity {
    pub t:u8,
    pub length:u16,
    pub selection_entity:SelectionMode,          // LAPI - Low Access Priority Indication
}

// Selection Mode enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionMode {
    ServingNetworkSelectedbyUE,
    ServingNetworkSelectedbyNetwork,
}

impl Default for CnOperatorSelectionEntity {
    fn default() -> CnOperatorSelectionEntity {
        CnOperatorSelectionEntity { t: CNOSE, length:CNOSE_LENGTH, selection_entity: SelectionMode::ServingNetworkSelectedbyUE }        
    }
}

impl IEs for CnOperatorSelectionEntity {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        match self.selection_entity {
            SelectionMode::ServingNetworkSelectedbyUE => buffer_ie.push(0x00),
            SelectionMode::ServingNetworkSelectedbyNetwork => buffer_ie.push(0x01),
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=(CNOSE_LENGTH+3) as usize {
            let mut data = CnOperatorSelectionEntity::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            match buffer[3] {
                0 => data.selection_entity = SelectionMode::ServingNetworkSelectedbyUE,
                1 => data.selection_entity = SelectionMode::ServingNetworkSelectedbyNetwork,
                i if i==2 || i==3 => data.selection_entity = SelectionMode::ServingNetworkSelectedbyNetwork,
                _ => return Err(GTPV1Error::IEIncorrect),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }
    
    fn len (&self) -> usize {
       CNOSE_LENGTH as usize + 3 
    }
}

#[test]
fn cnose_ie_unmarshal_test () {
    let encoded_ie:[u8;4]=[0xd8, 0x00, 0x01, 0x01];
    let test_struct = CnOperatorSelectionEntity { t:CNOSE, length: CNOSE_LENGTH, selection_entity: SelectionMode::ServingNetworkSelectedbyNetwork };
    let i = CnOperatorSelectionEntity::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn cnose_ie_marshal_test () {
    let encoded_ie:[u8;4]=[0xd8, 0x00, 0x01, 0x01];
    let test_struct = CnOperatorSelectionEntity { t:CNOSE, length: CNOSE_LENGTH, selection_entity: SelectionMode::ServingNetworkSelectedbyNetwork };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
