// OMC ID IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// OMC ID IE Type

pub const OMCID:u8 = 143;

// OMC ID IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct OmcId {
    pub t:u8,
    pub length:u16,
    pub omcid:Vec<u8>,
}

impl Default for OmcId {
    fn default() -> Self {
        OmcId { t: OMCID, length:0, omcid:vec!()}
    }
}

impl IEs for OmcId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.append(&mut self.omcid.clone());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=3 {
            let mut data=OmcId::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            if  check_tlv_ie_buffer(data.length, buffer) {
                data.omcid.extend_from_slice(&buffer[3..(data.length+3) as usize]);
                Ok(data)
            } else {
                Err(GTPV1Error::InvalidIELength)
            } 
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       (self.length+3) as usize 
    }

}

#[test]
fn omcid_ie_marshal_test () {
    let ie_marshalled:[u8;5]=[0x8f, 0x00, 0x02, 0x80, 0x80];
    let ie_to_marshal = OmcId { t:OMCID, length: 2, omcid: vec!(0x80, 0x80) };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn omcid_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;5]=[0x8f, 0x00, 0x02, 0x80, 0x80];
    let ie_unmarshalled = OmcId { t:OMCID, length: 2, omcid: vec!(0x80, 0x80) };
    assert_eq!(OmcId::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}