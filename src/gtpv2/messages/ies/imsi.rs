// IMSI IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// IMSI IE Type

pub const IMSI:u8 = 1;

// IMSI IE implementation 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imsi {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub imsi:String,
}

impl Default for Imsi {
    fn default() -> Imsi {
        Imsi {  t: IMSI,
                length: 0,
                ins: 0, 
                imsi: "".to_string(),
            }        
    }
}

impl From<Imsi> for InformationElement {
    fn from(i: Imsi) -> Self {
        InformationElement::Imsi(i)
    }
}

impl IEs for Imsi {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend(tbcd_encode(&self.imsi));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Imsi, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = Imsi{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3] & 0x0f;
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4..=(data.length as usize)+3].try_into() {
                Ok(i) => data.imsi = tbcd_decode(i),
                Err(_) => return Err(GTPV2Error::IEIncorrect(IMSI)), 
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(IMSI))
            }
        } else {
            Err(GTPV2Error::IEIncorrect(IMSI))
        }    
    }
    
    fn len (&self) -> usize {
       (self.length+4) as usize 
    }

    fn is_empty (&self) -> bool {
        self.length == 0
    }
}

#[test]
fn imsi_ie_unmarshal_test () {
    let encoded_ie:[u8;12]=[0x01, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3];
    let test_struct = Imsi { t:0x01, length:0x08, ins:0x00, imsi:"901405101961873".to_string(), };
    let i = Imsi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn imsi_ie_marshal_test () {
    let encoded_ie:[u8;12]=[0x01, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3];
    let test_struct = Imsi { t:0x01, length:0x08, ins:0x00, imsi:"901405101961873".to_string(), };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn imsi_ie_unmarshal_buffer_test () {
    let encoded_ie:[u8;12]=[0x01, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3];
    let test_struct = Imsi { t:0x01, length:0x08, ins:0x00, imsi:"901405101961873".to_string(), };
    let i = Imsi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}
