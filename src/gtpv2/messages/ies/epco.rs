// Extended PCO IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 
// Extended Protocol Configuration Options information element is specified as per clause 9.9.4.26 of 3GPP TS 24.301

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// Extended PCO IE Type

pub const EPCO:u8 = 197;

// Extended PCO IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Epco {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub epco:Vec<u8>,
}

impl Default for Epco {
    fn default() -> Self {
        Epco { t: EPCO, length:0, ins:0, epco:vec!()}
    }
}

impl IEs for Epco {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.epco.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE {
            let mut data=Epco::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            if  check_tliv_ie_buffer(data.length, buffer) {
                data.epco.extend_from_slice(&buffer[4..(data.length+4) as usize]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(EPCO))
            } 
        } else {
            Err(GTPV2Error::IEInvalidLength(EPCO))
        }
    }

    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }

}

#[test]
fn epco_ie_marshal_test () {
    let encoded:[u8;24]=[0xc5, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00];
    let decoded = Epco { t:EPCO, length: 20, ins: 0, epco: vec!(0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn apco_ie_unmarshal_test () {
    let encoded:[u8;24]=[0xc5, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00];
    let decoded = Epco { t:EPCO, length: 20, ins: 0, epco: vec!(0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00) };
    assert_eq!(Epco::unmarshal(&encoded).unwrap(), decoded);
}