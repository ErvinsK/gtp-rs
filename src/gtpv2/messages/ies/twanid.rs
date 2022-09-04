// TWAN Identifier IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// TWAN Identifier IE Type

pub const TWAN_ID:u8 = 169;

// TWAN Identifier IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct TwanId {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub ssid: Vec<u8>,
    pub bssid: Option<Vec<u8>>,
    pub civic_address: Option<Vec<u8>>,
    pub twan_plmnid: Option<Vec<u8>>,
    pub twan_op_name: Option<Vec<u8>>,
    pub relay_id: Option<(u8,Vec<u8>)>,
    pub circuit_id: Option<Vec<u8>>,       
}

impl Default for TwanId {
    fn default() -> TwanId {
        TwanId { t: TWAN_ID, length:1, ins:0, ssid:vec!(), bssid:None, civic_address:None, twan_plmnid:None, twan_op_name:None, relay_id:None, circuit_id:None }        
    }
}

impl From<TwanId> for InformationElement {
    fn from(i: TwanId) -> Self {
        InformationElement::TwanId(i)
    }
}

impl IEs for TwanId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags:u8 = vec!(self.relay_id.is_some(), self.twan_op_name.is_some(), self.twan_plmnid.is_some(), self.civic_address.is_some(), self.bssid.is_some()).iter().enumerate().map(|(x, i)| if *i { 0b10000>>x } else { 0 }).sum();
        buffer_ie.push(flags);
        buffer_ie.push(self.ssid.len() as u8);
        buffer_ie.extend_from_slice(&self.ssid[..]);
        match &self.bssid {
            Some(i) => buffer_ie.extend_from_slice(&i[..]),
            None => (),
        }
        match &self.civic_address {
            Some(i) => {
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend_from_slice(&i[..]);
            },
            None => (),
        }
        match &self.twan_plmnid {
            Some(i) => buffer_ie.extend_from_slice(&i[..]),
            None => (),
        }
        match &self.twan_op_name {
            Some(i) => {
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend_from_slice(&i[..]);
            },
            None => (),
        }
        match &self.relay_id {
            Some((i,j)) => {
                buffer_ie.push(*i);
                buffer_ie.push(j.len() as u8);
                buffer_ie.extend_from_slice(&j[..]);
            },
            None => (),
        }
        match &self.circuit_id {
            Some(i) => {
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend_from_slice(&i[..]);
            },
            None => (),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+2 {
            let mut data = TwanId::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            let flags = buffer[4];
            let mut cursor = buffer[5] as usize;
            match buffer[6..6+cursor].try_into() {
                Ok(i) => data.ssid = i,
                Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
            }
            cursor +=6;
            match flags & 0x01 {
                    1 => {
                        match buffer[cursor..cursor+6].try_into() {
                            Ok(i) => data.bssid = Some(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                        }
                        cursor+=6; 
                    },
                    _ => data.bssid = None,
                }
            match (flags >> 1) & 0x01 {
                    1 => {
                        if cursor <= buffer.len() {
                            let field = buffer[cursor] as usize;
                            cursor+=1;
                            match buffer[cursor..cursor+field].try_into() {
                                Ok(i) => data.civic_address = Some(i),
                                Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                            }
                            cursor+=field;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                        }
                    },
                    _ => data.civic_address = None,
            }
            match (flags >> 2) & 0x01 {
                1 => {
                    match buffer[cursor..cursor+3].try_into() {
                            Ok(i) => data.twan_plmnid = Some(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                        }
                    cursor+=3;
                },
                _ => data.twan_plmnid = None,
            }
            match (flags >> 3) & 0x01 {
                1 => {
                    if cursor <= buffer.len() {
                        let field = buffer[cursor] as usize;
                        cursor+=1;
                        match buffer[cursor..cursor+field].try_into() {
                            Ok(i) => data.twan_op_name = Some(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                        }
                        cursor+=field;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                },
                _ => data.twan_op_name = None,
            }
            match (flags >> 4) & 0x01 {
                1 => {
                    if cursor+2 <= buffer.len() {
                        let relay_id = buffer[cursor+1];
                        let field = buffer[cursor+2] as usize;
                        cursor+=2;
                        match buffer[cursor..cursor+field].try_into() {
                            Ok(i) => data.relay_id = Some((relay_id,i)),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                        }
                        cursor+=field;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                    if cursor <= buffer.len() {
                        let field = buffer[cursor] as usize;
                        cursor+=1;
                        match buffer[cursor..cursor+field].try_into() {
                            Ok(i) => data.circuit_id = Some(i),
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                        }
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                },
                _ => { 
                    data.relay_id = None;
                    data.circuit_id = None;
                },
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TWAN_ID))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn twan_id_ie_unmarshal_test () {
    let encoded:[u8;19]=[0xa9, 0x00, 0x0f, 0x00, 0x03, 0x03, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x00, 0x00, 0x00];
    let decoded = TwanId { t:TWAN_ID, length: 15, ins:0,
         ssid:vec!(0x00, 0x00, 0x00), 
         bssid:Some(vec!(0xff, 0xff, 0xff, 0xff, 0xff, 0xff)), 
         civic_address:Some(vec!(0x00, 0x00, 0x00)), twan_plmnid:None, twan_op_name:None, relay_id:None, circuit_id:None };
    let i = TwanId::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn twan_id_ie_marshal_test () {
    let encoded:[u8;9]=[0xa9, 0x00, 0x05, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00];
    let decoded = TwanId { t:TWAN_ID, length: 5, ins:0, ssid:vec!(0x00, 0x00, 0x00), bssid:None, civic_address:None, twan_plmnid:None, twan_op_name:None, relay_id:None, circuit_id:None };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
