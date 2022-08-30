// Service Indicator IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Service Indicator IE Type

pub const SRVCIND:u8 = 149;
pub const SRVCIND_LENGTH:usize = 1;

// Service Indicator IE implementation

//  Service indicator   Values (Decimal)
//      <spare>                 0
//  CS call indicator           1
//   SMS indicator              2
//      <spare>               3-255

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceIndicator {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub ind:u8,    
}

impl Default for ServiceIndicator {
    fn default() -> Self {
        ServiceIndicator { t: SRVCIND, length:SRVCIND_LENGTH as u16, ins:0, ind:0}
    }
}

impl From<ServiceIndicator> for InformationElement {
    fn from(i: ServiceIndicator) -> Self {
        InformationElement::ServiceIndicator(i)
    }
}

impl IEs for ServiceIndicator {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.ind);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+SRVCIND_LENGTH {
            let mut data=ServiceIndicator::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.ind = buffer[4];
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SRVCIND))
        }
    }

    fn len (&self) -> usize {
       SRVCIND_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn service_indicator_ie_marshal_test () {
    let encoded:[u8;5]=[0x95, 0x00, 0x01, 0x00, 0x02];
    let decoded = ServiceIndicator { t:SRVCIND, length: SRVCIND_LENGTH as u16, ins:0, ind: 0x02 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn service_indicator_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x95, 0x00, 0x01, 0x00, 0x02];
    let decoded = ServiceIndicator { t:SRVCIND, length: SRVCIND_LENGTH as u16, ins:0, ind: 0x02 };
    assert_eq!(ServiceIndicator::unmarshal(&encoded).unwrap(), decoded);
}
