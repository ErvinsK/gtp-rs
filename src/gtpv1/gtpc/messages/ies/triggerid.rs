// Trigger ID IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{utils::*, errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Trigger ID IE Type

pub const TRIGGERID:u8 = 142;

// Trigger ID IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerId {
    pub t:u8,
    pub length:u16,
    pub triggerid:Vec<u8>,
}

impl Default for TriggerId {
    fn default() -> Self {
        TriggerId { t: TRIGGERID, length:0, triggerid:vec!()}
    }
}

impl IEs for TriggerId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.append(&mut self.triggerid.clone());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=3 {
            let mut data=TriggerId::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            if  check_tlv_ie_buffer(data.length, buffer) {
                data.triggerid.extend_from_slice(&buffer[3..(data.length+3) as usize]);
                Ok(data)
            } else {
                Err(GTPV1Error::IEInvalidLength)
            } 
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
       (self.length+3) as usize 
    }

}

#[test]
fn triggerid_ie_marshal_test () {
    let ie_marshalled:[u8;5]=[0x8E, 0x00, 0x02, 0x80, 0x80];
    let ie_to_marshal = TriggerId { t:TRIGGERID, length: 2, triggerid: vec!(0x80, 0x80) };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn triggerid_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;5]=[0x8E, 0x00, 0x02, 0x80, 0x80];
    let ie_unmarshalled = TriggerId { t:TRIGGERID, length: 2, triggerid: vec!(0x80, 0x80) };
    assert_eq!(TriggerId::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}