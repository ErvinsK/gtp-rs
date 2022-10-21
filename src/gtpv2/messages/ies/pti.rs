// Procedure Transaction ID (PTI) IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// PTI IE Type

pub const PTI:u8 = 100;
pub const PTI_LENGTH:usize = 1;

// PTI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pti {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub pti:u8,
}

impl Default for Pti {
    fn default() -> Self {
        Pti { t: PTI, length:PTI_LENGTH as u16, ins:0, pti:0}
    }
}

impl From<Pti> for InformationElement {
    fn from(i: Pti) -> Self {
        InformationElement::Pti(i)
    }
}

impl IEs for Pti {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.pti);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+PTI_LENGTH {
            let mut data=Pti{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            data.pti = buffer[4];
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PTI))
        }
    }

    fn len (&self) -> usize {
       PTI_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn pti_ie_marshal_test () {
    let encoded:[u8;5]=[0x64, 0x00, 0x01, 0x00, 0x01];
    let decoded = Pti { t:PTI, length: PTI_LENGTH as u16, ins:0, pti:1 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn pti_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x64, 0x00, 0x01, 0x00, 0x01];
    let decoded = Pti { t:PTI, length: PTI_LENGTH as u16, ins:0, pti:1 };
    assert_eq!(Pti::unmarshal(&encoded).unwrap(), decoded);
}
