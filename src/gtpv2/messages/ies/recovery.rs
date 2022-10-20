// Recovery IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Recovery IE Type

pub const RECOVERY:u8 = 3;
pub const RECOVERY_LENGTH:usize = 1;

// Recovery IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recovery {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub recovery:u8,
}

impl Default for Recovery {
    fn default() -> Self {
        Recovery { t: RECOVERY, 
                length: RECOVERY_LENGTH as u16,
                ins: 0,
                recovery: 0,
            }
    }
}

impl From<Recovery> for InformationElement {
    fn from(i: Recovery) -> Self {
        InformationElement::Recovery(i)
    }
}

impl IEs for Recovery {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.recovery);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= (RECOVERY_LENGTH+MIN_IE_SIZE) {
            let mut data = Recovery::default();
            data.length = u16::from_be_bytes([buffer[1],buffer[2]]);
            data.ins = buffer[3] & 0x0f;
            data.recovery = buffer[4];
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RECOVERY))
        }    
    }

    fn len (&self) -> usize {
        (self.length+4) as usize
    }
}

#[test]
fn recovery_ie_marshal_test() {
    let decoded=Recovery{ t: RECOVERY, length: RECOVERY_LENGTH as u16, ins:0, recovery:4 };
    let encoded:[u8;5]=[0x03, 0x00, 0x01, 0x00, 0x04];
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn recovery_ie_unmarshal_test() {
    let decoded=Recovery{ t: RECOVERY, length: RECOVERY_LENGTH as u16, ins:0, recovery:4 };
    let encoded:[u8;5]=[0x03, 0x00, 0x01, 0x00, 0x04];
    assert_eq!(Recovery::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn recovery_ie_unmarshal_fail_test() {
    let encoded:[u8;4]=[0x03, 0x00, 0x01, 0x00];
    assert_eq!(Recovery::unmarshal(&encoded), Err(GTPV2Error::IEInvalidLength(RECOVERY)));
}

