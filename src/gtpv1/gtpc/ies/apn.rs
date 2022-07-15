// APN IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::gtpc::ies::commons::{*};

// APN IE Type

pub const APN:u8 = 131;

// NSAPI IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct Apn {
    pub t:u8,
    pub length:u16,
    pub name:String,
}

impl Default for Apn {
    fn default() -> Self {
        Apn { t: APN, length:0, name: "".to_string() }
    }
}

impl IEs for Apn {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        let n:Vec<_> = self.name.split(".").collect();
        let mut z:Vec<u8>=vec!();
        for i in n.iter() {
            z.push(i.len() as u8);
            z.extend_from_slice(i.as_bytes());
        }
        buffer.append(&mut z);
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=3 {
            let mut data=Apn::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            let mut donor:Vec<u8>=buffer[3..(3+data.length as usize)].to_vec();
            let mut k:Vec<Vec<char>>=vec!();
            loop {
                if !donor.is_empty() {
                    let index:Vec<_> = donor.drain(..1).collect();
                    let mut part:Vec<_> = donor.drain(..index[0] as usize).map(|x| x as char).collect();
                    part.push('.');
                    k.push(part);
                } else {
                    break;
                }
            }
            let mut p:Vec<char> = k.into_iter().flatten().collect();
            let _ = p.pop();
            data.name = p.into_iter().collect();
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        self.length as usize
    }
}

#[test]
fn apn_ie_marshal_test () {
    let ie_marshalled:[u8;16]=[0x83, 0x00, 0x0d, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d];
    let ie_to_marshal = Apn { t:APN, length: 13, name: "test.net.com".to_string() };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn apn_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;16]=[0x83, 0x00, 0x0d, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d];
    let ie_unmarshalled = Apn { t:APN, length: 13, name: "test.net.com".to_string() };
    assert_eq!(Apn::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}
