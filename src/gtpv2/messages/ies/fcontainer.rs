// Fully Qualified Container (F-Container) - according to 3GPP TS 29.274 V15.9.0 (2019-09) and 3GPP TS 24.008 V16.0.0 (2019-03)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// F-Container IE Type

pub const FCONTAINER:u8 = 118;

// F-Container Type Enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Container {
    Reserved,
    Utran(Vec<u8>),
    Bss(Vec<u8>),
    Eutran(Vec<u8>),
    Nbifom(Vec<u8>),
    EnDc(Vec<u8>),
    Unknown(Vec<u8>),   // Container Type is put into the first element of the containing vector
}

// F-Container IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fcontainer {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub container:Container,
}

impl Default for Fcontainer {
    fn default() -> Self {
        Fcontainer { t: FCONTAINER, length:0, ins:0, container:Container::Reserved}
    }
}

impl From<Fcontainer> for InformationElement {
    fn from(i: Fcontainer) -> Self {
        InformationElement::Fcontainer(i)
    }
}

impl IEs for Fcontainer {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.container.clone() {
            Container::Reserved => buffer_ie.push(0x00),
            Container::Utran(i) => {
                buffer_ie.push(0x01);
                buffer_ie.extend_from_slice(&i[..]);
            },
            Container::Bss(i) => {
                buffer_ie.push(0x02);
                buffer_ie.extend_from_slice(&i[..]);
            },
            Container::Eutran(i) => {
                buffer_ie.push(0x03);
                buffer_ie.extend_from_slice(&i[..]);
            },
            Container::Nbifom(i) => {
                buffer_ie.push(0x04);
                buffer_ie.extend_from_slice(&i[..]);
            },
            Container::EnDc(i) => {
                buffer_ie.push(0x05);
                buffer_ie.extend_from_slice(&i[..]);
            },
            Container::Unknown(i) => {
                buffer_ie.push(i[0]);
                buffer_ie.extend_from_slice(&i[1..]);
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data=Fcontainer{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            if  check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    0 => data.container = Container::Reserved,
                    1 => {
                        match buffer[5..].try_into() {
                            Ok(i) =>  data.container = Container::Utran(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(FCONTAINER)),
                        }
                    },
                    2 => {
                        match buffer[5..].try_into() {
                            Ok(i) =>  data.container = Container::Bss(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(FCONTAINER)),
                        }
                    },
                    3 => {
                        match buffer[5..].try_into() {
                            Ok(i) =>  data.container = Container::Eutran(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(FCONTAINER)),
                        }
                    },
                    4 => {
                        match buffer[5..].try_into() {
                            Ok(i) =>  data.container = Container::Nbifom(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(FCONTAINER)),
                        }
                    },
                    5 => {
                        match buffer[5..].try_into() {
                            Ok(i) =>  data.container = Container::EnDc(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(FCONTAINER)),
                        }
                    },
                    _ => {
                        match buffer[5..].try_into() {
                            Ok(mut i) =>  {
                                let mut container = vec!(buffer[4]);
                                container.append(&mut i);
                                data.container = Container::Unknown(container);
                            },
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(FCONTAINER)),
                        }
                    },
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(FCONTAINER))
            } 
        } else {
            Err(GTPV2Error::IEInvalidLength(FCONTAINER))
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
fn fcontainer_ie_marshal_test () {
    let encoded:[u8;10]=[0x76, 0x00, 0x06, 0x00, 0x03, 0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let decoded = Fcontainer { t:FCONTAINER, length: 6, ins: 0, container: Container::Eutran(vec!(0xaa, 0xbb, 0xcc, 0xdd, 0xee)) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn fcontainer_ie_unmarshal_test () {
    let encoded:[u8;10]=[0x76, 0x00, 0x06, 0x00, 0x03, 0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let decoded = Fcontainer { t:FCONTAINER, length: 6, ins: 0, container: Container::Eutran(vec!(0xaa, 0xbb, 0xcc, 0xdd, 0xee)) };
    assert_eq!(Fcontainer::unmarshal(&encoded).unwrap(), decoded);
}