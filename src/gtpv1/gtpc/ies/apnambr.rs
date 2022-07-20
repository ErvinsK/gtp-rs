// APN Aggregate Maximum Bit Rate (AMBR) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// APN-AMBR IE TL

pub const APNAMBR:u8 = 198;
pub const APNAMBR_LENGTH:u16 = 8;

// APN-AMBR IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct ApnAmbr {
    pub t:u8,
    pub length:u16,
    pub ambr_ul:u32,
    pub ambr_dl:u32,
}

impl Default for ApnAmbr {
    fn default() -> Self {
        ApnAmbr { t: APNAMBR, length:APNAMBR_LENGTH, ambr_ul:0, ambr_dl:0}
    }
}

impl IEs for ApnAmbr {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.extend_from_slice(&self.ambr_ul.to_be_bytes());
        buffer.extend_from_slice(&self.ambr_dl.to_be_bytes());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=APNAMBR_LENGTH as usize + 3 {
            let mut data=ApnAmbr::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ambr_ul = u32::from_be_bytes([buffer[3],buffer[4],buffer[5],buffer[6]]); 
            data.ambr_dl = u32::from_be_bytes([buffer[7],buffer[8],buffer[9],buffer[10]]);
            Ok(data)
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       APNAMBR_LENGTH as usize + 3 
    }

}

#[test]
fn apnambr_ie_marshal_test () {
    let ie_marshalled:[u8;11]=[0xc6, 0x00, 0x08, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40];
    let ie_to_marshal = ApnAmbr { t:APNAMBR, length: APNAMBR_LENGTH, ambr_ul:2000, ambr_dl:8000 };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn apnambr_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;11]=[0xc6, 0x00, 0x08, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40];
    let ie_unmarshalled = ApnAmbr { t:APNAMBR, length: APNAMBR_LENGTH, ambr_ul:2000, ambr_dl:8000 };
    assert_eq!(ApnAmbr::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}