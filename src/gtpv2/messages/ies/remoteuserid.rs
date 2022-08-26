// Remote User ID IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// Remote User ID IE TV

pub const REMOTE_USR_ID:u8 = 192;

// Remote User ID IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteUserId {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub imsi:String,
    pub msisdn:Option<String>,
    pub imei:Option<String>, 
}

impl Default for RemoteUserId {
    fn default() -> Self {
        RemoteUserId { t: REMOTE_USR_ID, length:1, ins:0, imsi:"".to_string(), msisdn:None, imei:None }        
    }
}

impl IEs for RemoteUserId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.imei.is_some(), self.msisdn.is_some()) {
            (false, false) => {
                buffer_ie.push(0x00);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
            },
            (false, true) => {
                buffer_ie.push(0x01);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
                let m = tbcd_encode(&self.msisdn.unwrap());
                buffer_ie.push(m.len() as u8);
                buffer_ie.extend(m);
            },
            (true, false) => {
                buffer_ie.push(0x02);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
                let m = tbcd_encode(&self.imei.unwrap());
                buffer_ie.push(m.len() as u8);
                buffer_ie.extend(m);
            },
            (true, true) => {
                buffer_ie.push(0x03);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
                let m = tbcd_encode(&self.msisdn.unwrap());
                buffer_ie.push(m.len() as u8);
                buffer_ie.extend(m);
                let k = tbcd_encode(&self.imei.unwrap());
                buffer_ie.push(k.len() as u8);
                buffer_ie.extend(k);
            },
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= MIN_IE_SIZE+2 {
            let mut data = RemoteUserId::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            let mut cursor = 6+(buffer[5] as usize);
            match buffer[4] & 0x03 {
                0 => {
                    match buffer[5..=(6+cursor)].try_into() {
                        Ok(i) => data.imsi = tbcd_decode(i[1..]),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)), 
                    }
                    data.msisdn = None;
                    data.imei = None;
                },
                1 => {
                    match buffer[5..=(cursor+1)].try_into() {
                        Ok(i) => {
                            data.imsi = tbcd_decode(i[1..=cursor]);
                            cursor+=i[0] as usize;
                        },
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)), 
                    }
                    match buffer[(cursor+1)..=(6+cursor)].try_into() {
                        Ok(i) => {
                            data.imsi = tbcd_decode(i[1..]);
                            cursor+=i[0] as usize;
                        },
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)), 
                    }

                },
                2 => {
                    if check_tliv_ie_buffer(3, &buffer) {
                        data.max_packet_loss_ul = None;
                        data.max_packet_loss_dl = Some(u16::from_be_bytes([buffer[5],buffer[6]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS));
                    }
                },
                3 => {
                    if check_tliv_ie_buffer(5, &buffer) {
                        data.max_packet_loss_ul = Some(u16::from_be_bytes([buffer[5],buffer[6]]));
                        data.max_packet_loss_dl = Some(u16::from_be_bytes([buffer[7],buffer[8]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS));
                    }
                },
                _ => return Err(GTPV2Error::IEIncorrect(MAX_PACKET_LOSS)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn max_packet_loss_rate_ie_unmarshal_test () {
    let encoded:[u8;9]=[0xcb, 0x00, 0x05, 0x00, 0x03, 0x03, 0xe8, 0x03, 0xe7 ];
    let decoded = MaxPacketLossRate { t:MAX_PACKET_LOSS, length: 5, ins:0, max_packet_loss_ul:Some(0x3e8), max_packet_loss_dl:Some(0x3e7) };
    let i = MaxPacketLossRate::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn secondary_rat_udr_ie_marshal_test () {
    let encoded:[u8;9]=[0xcb, 0x00, 0x05, 0x00, 0x03, 0x03, 0xe8, 0x03, 0xe7 ];
    let decoded = MaxPacketLossRate { t:MAX_PACKET_LOSS, length: 5, ins:0, max_packet_loss_ul:Some(0x3e8), max_packet_loss_dl:Some(0x3e7) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
