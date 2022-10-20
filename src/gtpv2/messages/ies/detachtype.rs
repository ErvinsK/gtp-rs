// Detach Type IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// Detach Type IE Type

pub const DETACHTYPE:u8 = 150;
pub const DETACHTYPE_LENGTH:usize = 1;

// Detach Type IE implementation

//     Detach Type           Values (Decimal)
//     <reserved>                 0
//      PS Detach                 1
//  Combined PS/CS Detach         2
//      <spare>                 3-255

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetachType {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub detach_type:u8,    
}

impl Default for DetachType {
    fn default() -> Self {
        DetachType { t: DETACHTYPE, length:DETACHTYPE_LENGTH as u16, ins:0, detach_type:0}
    }
}

impl From<DetachType> for InformationElement {
    fn from(i: DetachType) -> Self {
        InformationElement::DetachType(i)
    }
}

impl IEs for DetachType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.detach_type);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+DETACHTYPE_LENGTH {
            let mut data=DetachType::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.detach_type = buffer[4];
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(DETACHTYPE))
        }
    }

    fn len (&self) -> usize {
       DETACHTYPE_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn detach_type_ie_marshal_test () {
    let encoded:[u8;5]=[0x96, 0x00, 0x01, 0x00, 0x02];
    let decoded = DetachType { t:DETACHTYPE, length: DETACHTYPE_LENGTH as u16, ins:0, detach_type: 0x02 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn detach_type_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x96, 0x00, 0x01, 0x00, 0x02];
    let decoded = DetachType { t:DETACHTYPE, length: DETACHTYPE_LENGTH as u16, ins:0, detach_type: 0x02 };
    assert_eq!(DetachType::unmarshal(&encoded).unwrap(), decoded);
}
