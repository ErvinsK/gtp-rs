// CAMEL Charging Information Container (CIC) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// CAMEL Charging Information Container (CIC) IE Type

pub const CAMELCIC:u8 = 155;

// CAMEL Charging Information Container (CIC) IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct CamelChargingInfoContainer {
    pub t:u8,
    pub length:u16,
    pub camel_cic:Vec<u8>,
}

impl Default for CamelChargingInfoContainer {
    fn default() -> Self {
        CamelChargingInfoContainer { t: CAMELCIC, length:0, camel_cic:vec!()}
    }
}

impl IEs for CamelChargingInfoContainer {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.append(&mut self.camel_cic.clone());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=3 {
            let mut data=CamelChargingInfoContainer::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            if buffer.len()>=(data.length+3) as usize {
                data.camel_cic.extend_from_slice(&buffer[3..(3+data.length as usize)]);
                Ok(data)
            } else {
                Err(GTPV1Error::InvalidIELength)
            }
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       (self.length + 3) as usize 
    }

}

#[test]
fn camelcic_ie_marshal_test () {
    let ie_marshalled:[u8;7]=[0x9B, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00];
    let ie_to_marshal = CamelChargingInfoContainer { t:CAMELCIC, length: 4, camel_cic:vec!(0,0,0,0) };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn camelcic_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;7]=[0x9B, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00];
    let ie_unmarshalled = CamelChargingInfoContainer { t:CAMELCIC, length: 4, camel_cic:vec!(0,0,0,0) };
    assert_eq!(CamelChargingInfoContainer::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}