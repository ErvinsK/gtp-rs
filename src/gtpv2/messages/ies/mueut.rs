// Mapped UE Usage Type (MUEUT) IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)
// Mapped UE Usage Type is defined in clause 5.8.1 of 3GPP TS 29.003

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// Mapped UE Usage Type (MUEUT) IE TL

pub const MUEUT:u8 = 200;
pub const MUEUT_LENGTH:usize = 2;

// Mapped UE Usage Type (MUEUT) IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct MappedUeUsageType {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub usage_type:u16,         
}

impl Default for MappedUeUsageType {
    fn default() -> MappedUeUsageType {
        MappedUeUsageType { t: MUEUT, length:MUEUT_LENGTH as u16, ins:0, usage_type:0 }        
    }
}

impl IEs for MappedUeUsageType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.usage_type.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= MUEUT_LENGTH+MIN_IE_SIZE {
            let mut data = MappedUeUsageType::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.usage_type = u16::from_be_bytes([buffer[4],buffer[5]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MUEUT))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn mueut_ie_unmarshal_test () {
    let encoded_ie:[u8;6]=[0xc8, 0x00, 0x02, 0x00, 0x00, 0x0f];
    let test_struct = MappedUeUsageType { t:MUEUT, length: MUEUT_LENGTH as u16, ins:0, usage_type: 15 };
    let i = MappedUeUsageType::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn mueut_ie_marshal_test () {
    let encoded_ie:[u8;6]=[0xc8, 0x00, 0x02, 0x00, 0x00, 0x0f];
    let test_struct = MappedUeUsageType { t:MUEUT, length: MUEUT_LENGTH as u16, ins:0, usage_type: 15 };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
