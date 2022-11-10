// CSG ID IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// CSG ID Type

pub const CSGID:u8 = 147;
pub const CSGID_LENGTH:usize = 4;

// CSG ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsgId {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub csgid:u32,
}

impl Default for CsgId {
    fn default() -> Self {
        CsgId { t: CSGID, length:CSGID_LENGTH as u16, ins:0, csgid:0}
    }
}

impl From<CsgId> for InformationElement {
    fn from(i: CsgId) -> Self {
        InformationElement::CsgId(i)
    }
}

impl IEs for CsgId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.csgid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+CSGID_LENGTH {
            let mut data=CsgId {
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            data.csgid = u32::from_be_bytes([(buffer[4] & 0x07),buffer[5],buffer[6],buffer[7]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(CSGID))
        }
    }

    fn len (&self) -> usize {
       (self.length as usize)+MIN_IE_SIZE 
    }

    fn is_empty (&self) -> bool {
        self.length == 0
    }
}

#[test]
fn csgid_ie_marshal_test () {
    let encoded:[u8;8]=[0x93, 0x00, 0x04, 0x00, 0x07, 0xff, 0xff, 0xff];
    let decoded = CsgId { t:CSGID, length: CSGID_LENGTH as u16, ins:0, csgid:0x7ffffff };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn csgid_ie_unmarshal_test () {
    let encoded:[u8;8]=[0x93, 0x00, 0x04, 0x00, 0x07, 0xff, 0xff, 0xff];
    let decoded = CsgId { t:CSGID, length: CSGID_LENGTH as u16, ins:0, csgid:0x7ffffff };
    assert_eq!(CsgId::unmarshal(&encoded).unwrap(), decoded);
}
