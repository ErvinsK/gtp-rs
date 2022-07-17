// TFT IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) and 3GPP TS 24.008 V16.0.0 (2019-03)

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// TFT IE Type

pub const TFT:u8 = 137;

// TFT IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Tft {
    pub t:u8,
    pub length:u16,
    pub tft:Vec<u8>,
}

impl Default for Tft {
    fn default() -> Self {
        Tft { t: TFT, length:0, tft:vec!()}
    }
}

impl IEs for Tft {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.append(&mut self.tft.clone());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=3 {
            let mut data=Tft::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            if  check_tlv_ie_buffer(data.length, buffer) {
                data.tft.extend_from_slice(&buffer[3..(data.length+3) as usize]);
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
fn tft_ie_marshal_test () {
    let ie_marshalled:[u8;5]=[0x89, 0x00, 0x02, 0x80, 0x80];
    let ie_to_marshal = Tft { t:TFT, length: 2, tft: vec!(0x80, 0x80) };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn tft_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;5]=[0x89, 0x00, 0x02, 0x80, 0x80];
    let ie_unmarshalled = Tft { t:TFT, length: 2, tft: vec!(0x80, 0x80) };
    assert_eq!(Tft::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}