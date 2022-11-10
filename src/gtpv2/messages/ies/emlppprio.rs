// eMLPP Priority IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// eMLPP Priority IE TL

pub const EMLPP:u8 = 134;
pub const EMLPP_LENGTH:usize = 1;

// eMLPP Priority IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmlppPriority {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub priority:u8,
}

impl Default for EmlppPriority {
    fn default() -> Self {
        EmlppPriority { t: EMLPP, length:EMLPP_LENGTH as u16, ins:0, priority: 0 }
    }
}

impl From<EmlppPriority> for InformationElement {
    fn from(i: EmlppPriority) -> Self {
        InformationElement::EmlppPriority(i)
    }
}

impl IEs for EmlppPriority {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.priority);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=EMLPP_LENGTH+MIN_IE_SIZE {
            let mut data=EmlppPriority{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3] & 0x0f;
            data.priority = buffer[4] & 0x07;
            Ok(data) 
        } else {
            Err(GTPV2Error::IEInvalidLength(EMLPP))
        }
    }

    fn len (&self) -> usize {
        EMLPP_LENGTH+MIN_IE_SIZE
    }

    fn is_empty (&self) -> bool {
        self.length == 0
    }
}

#[test]
fn emlpp_ie_marshal_test() {
    let decoded=EmlppPriority { t: EMLPP, length:EMLPP_LENGTH as u16, ins:0, priority:0};
    let encoded:[u8;5]=[0x86, 0x00, 0x01, 0x00, 0x00];
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn emlpp_ie_unmarshal_test() {
    let decoded=EmlppPriority { t: EMLPP, length:EMLPP_LENGTH as u16, ins:0, priority:0};
    let encoded:[u8;5]=[0x86, 0x00, 0x01, 0x00, 0x00];
    assert_eq!(EmlppPriority::unmarshal(&encoded).unwrap(), decoded);
}