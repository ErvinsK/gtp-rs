// RAT Type IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// RAT Type IE Type

pub const RATTYPE:u8 = 151;
pub const RATTYPE_LENGTH:u16 = 1;

// RAT Type Values 

pub const UTRAN:u8 = 1; 
pub const GERAN:u8 = 2;           
pub const WLAN:u8 = 3;           
pub const GAN:u8 = 4;         
pub const HSPA_EVO:u8 = 5;
pub const EUTRAN:u8 = 6;         

// RAT Type IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct RatType {
    pub t:u8,
    pub length:u16,
    pub rat_type:u8,
}

impl Default for RatType {
    fn default() -> Self {
        RatType { t: RATTYPE, length:RATTYPE_LENGTH, rat_type:GERAN}
    }
}

impl IEs for RatType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.push (self.rat_type);
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=4 {
            let mut data=RatType::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            match buffer[3] {
                i if i<=6 => data.rat_type=i,
                _ => return Err(GTPV1Error::IncorrectIE),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       (RATTYPE_LENGTH+3) as usize 
    }

}

#[test]
fn rattype_ie_marshal_test () {
    let ie_marshalled:[u8;4]=[0x97, 0x00, 0x01, 0x02];
    let ie_to_marshal = RatType { t:RATTYPE, length: RATTYPE_LENGTH, rat_type:2 };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn rattype_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;4]=[0x97, 0x00, 0x01, 0x02];
    let ie_unmarshalled = RatType { t:RATTYPE, length: RATTYPE_LENGTH, rat_type:2 };
    assert_eq!(RatType::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}