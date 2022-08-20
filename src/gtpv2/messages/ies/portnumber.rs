// Port Number IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// Port Number IE Type

pub const PORT_NBR:u8 = 126;
pub const PORT_NBR_LENGTH:usize = 2;

// Port Number IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct PortNumber {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub port:u16,
}

impl Default for PortNumber {
    fn default() -> Self {
        PortNumber { t: PORT_NBR, length:PORT_NBR_LENGTH as u16, ins:0, port:0}
    }
}

impl IEs for PortNumber {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.port.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+PORT_NBR_LENGTH {
            let mut data=PortNumber::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.port = u16::from_be_bytes([buffer[4],buffer[5]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
       PORT_NBR_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn port_number_ie_marshal_test () {
    let encoded:[u8;6]=[0x7e, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = PortNumber { t:PORT_NBR, length: PORT_NBR_LENGTH as u16, ins:0, port: 0xffff };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn port_number_ie_unmarshal_test () {
    let encoded:[u8;6]=[0x7e, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = PortNumber { t:PORT_NBR, length: PORT_NBR_LENGTH as u16, ins:0, port: 0xffff };
    assert_eq!(PortNumber::unmarshal(&encoded).unwrap(), decoded);
}
