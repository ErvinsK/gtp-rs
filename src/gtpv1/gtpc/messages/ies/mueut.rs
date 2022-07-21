// Mapped UE Usage Type (MUEUT) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{utils::*, errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Mapped UE Usage Type (MUEUT) IE TV

pub const MUEUT:u8 = 223;
pub const MUEUT_LENGTH:u16 = 2;

// Mapped UE Usage Type (MUEUT) IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct MappedUeUsageType {
    pub t:u8,
    pub length:u16,
    pub usage_type:u16,         
}

impl Default for MappedUeUsageType {
    fn default() -> MappedUeUsageType {
        MappedUeUsageType { t: MUEUT, length:MUEUT_LENGTH, usage_type:0 }        
    }
}

impl IEs for MappedUeUsageType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.extend_from_slice(&self.usage_type.to_be_bytes());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>= MUEUT_LENGTH as usize + 3 {
            let mut data = MappedUeUsageType::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.usage_type = u16::from_be_bytes([buffer[3],buffer[4]]);
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }
    
    fn len (&self) -> usize {
       MUEUT_LENGTH as usize + 3 
    }
}

#[test]
fn mueut_ie_unmarshal_test () {
    let encoded_ie:[u8;5]=[0xdf, 0x00, 0x02, 0x00, 0x0f];
    let test_struct = MappedUeUsageType { t:MUEUT, length: MUEUT_LENGTH, usage_type: 15 };
    let i = MappedUeUsageType::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn mueut_ie_marshal_test () {
    let encoded_ie:[u8;5]=[0xdf, 0x00, 0x02, 0x00, 0x0f];
    let test_struct = MappedUeUsageType { t:MUEUT, length: MUEUT_LENGTH, usage_type: 15 };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
