// Fully Qualified Cause (F-Cause) IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) and 3GPP TS 24.008 V16.0.0 (2019-03)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// F-Cause IE Type

pub const FCAUSE:u8 = 119;

// F-Cause IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fcause {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub cause_type:u8,
    pub cause_field:Vec<u8>,
}

impl Default for Fcause {
    fn default() -> Self {
        Fcause { t: FCAUSE, length:0, ins:0, cause_type: 0, cause_field:vec!()}
    }
}

impl From<Fcause> for InformationElement {
    fn from(i: Fcause) -> Self {
        InformationElement::Fcause(i)
    }
}

impl IEs for Fcause {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.cause_type);
        buffer_ie.extend_from_slice(&self.cause_field[..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data=Fcause{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            data.cause_type = buffer[4] & 0x0f;
            if  check_tliv_ie_buffer(data.length, buffer) {
                data.cause_field.extend_from_slice(&buffer[5..]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(FCAUSE))
            } 
        } else {
            Err(GTPV2Error::IEInvalidLength(FCAUSE))
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
fn fcause_ie_marshal_test () {
    let encoded:[u8;7]=[0x77, 0x00, 0x03, 0x00, 0x00, 0xff, 0xaa];
    let decoded = Fcause { t:FCAUSE, length: 3, ins: 0, cause_type: 0, cause_field:vec!(0xff, 0xaa) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn fcause_ie_unmarshal_test () {
    let encoded:[u8;7]=[0x77, 0x00, 0x03, 0x00, 0x00, 0xff, 0xaa];
    let decoded = Fcause { t:FCAUSE, length: 3, ins: 0, cause_type: 0, cause_field:vec!(0xff, 0xaa) };
    assert_eq!(Fcause::unmarshal(&encoded).unwrap(), decoded);
}