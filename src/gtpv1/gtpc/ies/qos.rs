// QoS IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) and 3GPP TS 24.008 V16.0.0 (2019-03)

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// QoS IE Type

pub const QOS:u8 = 135;

// PCO IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Qos {
    pub t:u8,
    pub length:u16,
    pub arp:u8,
    pub qos:Vec<u8>,
}

impl Default for Qos {
    fn default() -> Self {
        Qos { t: QOS, length:1, arp: 0, qos:vec!()}
    }
}

impl IEs for Qos {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.push(self.arp);
        buffer.append(&mut self.qos.clone());
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=3 {
            let mut data=Qos::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            if  check_tlv_ie_buffer(data.length, buffer) {
                data.arp = buffer[3];
                data.qos.extend_from_slice(&buffer[4..(data.length+3) as usize]);
                if data.qos.len() >=3 && data.qos.len()<=255 {
                    Ok(data)
                } else {
                    Err(GTPV1Error::IncorrectIE)
                }
            } else {
                Err(GTPV1Error::InvalidIELength)
            } 
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       (self.length+3) as usize 
    }

}

#[test]
fn pco_ie_marshal_test () {
    let ie_marshalled:[u8;15]=[0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, 0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40];
    let ie_to_marshal = Qos { t:QOS, length: 12, arp:3, qos: vec!(0x1b, 0x93, 0x1f, 0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40) };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn pco_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;15]=[0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, 0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40];
    let ie_unmarshalled = Qos { t:QOS, length: 12, arp:3, qos: vec!(0x1b, 0x93, 0x1f, 0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40) };
    assert_eq!(Qos::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}