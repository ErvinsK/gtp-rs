// Sequence Number IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Sequence Number IE Type

pub const SQN:u8 = 183;
pub const SQN_LENGTH:usize = 4;

// Sequence Number IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Sqn {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub sqn:u32,    
}

impl Default for Sqn {
    fn default() -> Self {
        Sqn { t: SQN, length:SQN_LENGTH as u16, ins:0, sqn:0}
    }
}

impl From<Sqn> for InformationElement {
    fn from(i: Sqn) -> Self {
        InformationElement::Sqn(i)
    }
}

impl IEs for Sqn {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.sqn.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+SQN_LENGTH {
            let mut data=Sqn::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.sqn = u32::from_be_bytes([buffer[4],buffer[5],buffer[6],buffer[7]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SQN))
        }
    }

    fn len (&self) -> usize {
       SQN_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn sqn_ie_marshal_test () {
    let encoded:[u8;8]=[0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11];
    let decoded = Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn sqn_ie_unmarshal_test () {
    let encoded:[u8;8]=[0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11];
    let decoded = Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 };
    assert_eq!(Sqn::unmarshal(&encoded).unwrap(), decoded);
}
