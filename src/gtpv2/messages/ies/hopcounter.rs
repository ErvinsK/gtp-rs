// Hop Counter IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// Hop Counter IE Type

pub const HOP_CNTR:u8 = 113;
pub const HOP_CNTR_LENGTH:usize = 1;

// Hop Counter IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct HopCounter {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub hop_counter:u8,
}

impl Default for HopCounter {
    fn default() -> Self {
        HopCounter { t: HOP_CNTR, length:HOP_CNTR_LENGTH as u16, ins:0, hop_counter:0}
    }
}

impl IEs for HopCounter {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.hop_counter);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+HOP_CNTR_LENGTH {
            let mut data=HopCounter::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.hop_counter = buffer[4];
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(HOP_CNTR))
        }
    }

    fn len (&self) -> usize {
       (self.length as usize)+MIN_IE_SIZE 
    }

}

#[test]
fn hop_counter_ie_marshal_test () {
    let encoded:[u8;5]=[0x71, 0x00, 0x01, 0x00, 0x01];
    let decoded = HopCounter { t:HOP_CNTR, length: HOP_CNTR_LENGTH as u16, ins:0, hop_counter:1 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn hop_counter_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x71, 0x00, 0x01, 0x00, 0x01];
    let decoded = HopCounter { t:HOP_CNTR, length: HOP_CNTR_LENGTH as u16, ins:0, hop_counter:1 };
    assert_eq!(HopCounter::unmarshal(&encoded).unwrap(), decoded);
}
