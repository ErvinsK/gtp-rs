// Remote User ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Remote User ID IE TV

pub const REMOTE_USR_ID: u8 = 192;

// Remote User ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteUserId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub imsi: String,
    pub msisdn: Option<String>,
    pub imei: Option<String>,
}

impl Default for RemoteUserId {
    fn default() -> Self {
        RemoteUserId {
            t: REMOTE_USR_ID,
            length: 0,
            ins: 0,
            imsi: "".to_string(),
            msisdn: None,
            imei: None,
        }
    }
}

impl From<RemoteUserId> for InformationElement {
    fn from(i: RemoteUserId) -> Self {
        InformationElement::RemoteUserId(i)
    }
}

impl IEs for RemoteUserId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(REMOTE_USR_ID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.imei.is_some(), self.msisdn.is_some()) {
            (false, false) => {
                buffer_ie.push(0x00);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
            }
            (false, true) => {
                buffer_ie.push(0x01);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
                if let Some(i) = &self.msisdn {
                    let m = tbcd_encode(i);
                    buffer_ie.push(m.len() as u8);
                    buffer_ie.extend(m);
                }
            }
            (true, false) => {
                buffer_ie.push(0x02);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
                if let Some(i) = &self.imei {
                    let m = tbcd_encode(i);
                    buffer_ie.push(m.len() as u8);
                    buffer_ie.extend(m);
                }
            }
            (true, true) => {
                buffer_ie.push(0x03);
                let i = tbcd_encode(&self.imsi);
                buffer_ie.push(i.len() as u8);
                buffer_ie.extend(i);
                if let Some(i) = &self.msisdn {
                    let m = tbcd_encode(i);
                    buffer_ie.push(m.len() as u8);
                    buffer_ie.extend(m);
                }
                if let Some(i) = &self.imei {
                    let m = tbcd_encode(i);
                    buffer_ie.push(m.len() as u8);
                    buffer_ie.extend(m);
                }
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 2 {
            let mut data = RemoteUserId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..RemoteUserId::default()
            };
            let mut cursor = 6;
            let mut l;
            match buffer[4] & 0x03 {
                0 => {
                    l = buffer[5] as usize;
                    match buffer[cursor..(cursor + l)].try_into() {
                        Ok(i) => data.imsi = tbcd_decode(i),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)),
                    }
                    data.msisdn = None;
                    data.imei = None;
                }
                1 => {
                    l = buffer[5] as usize;
                    match buffer[cursor..(cursor + l)].try_into() {
                        Ok(i) => data.imsi = tbcd_decode(i),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)),
                    }
                    cursor += l + 1;
                    if cursor > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    l = buffer[cursor - 1] as usize;
                    if (cursor + l) > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    data.msisdn = Some(tbcd_decode(&buffer[cursor..(cursor + l)]));
                    data.imei = None;
                }
                2 => {
                    l = buffer[5] as usize;
                    match buffer[cursor..(cursor + l)].try_into() {
                        Ok(i) => data.imsi = tbcd_decode(i),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)),
                    }
                    cursor += l + 1;
                    if cursor > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    l = buffer[cursor - 1] as usize;
                    if (cursor + l) > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    data.imei = Some(tbcd_decode(&buffer[cursor..(cursor + l)]));
                    data.msisdn = None;
                }
                3 => {
                    l = buffer[5] as usize;
                    match buffer[cursor..(cursor + l)].try_into() {
                        Ok(i) => data.imsi = tbcd_decode(i),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)),
                    }
                    cursor += l + 1;
                    if cursor > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    l = buffer[cursor - 1] as usize;
                    if (cursor + l) > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    data.msisdn = Some(tbcd_decode(&buffer[cursor..(cursor + l)]));
                    cursor += l + 1;
                    if cursor > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    l = buffer[cursor - 1] as usize;
                    if (cursor + l) > buffer.len() {
                        return Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID));
                    }
                    data.imei = Some(tbcd_decode(&buffer[cursor..(cursor + l)]));
                }
                _ => return Err(GTPV2Error::IEIncorrect(REMOTE_USR_ID)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn remote_user_id_ie_unmarshal_test() {
    let encoded: [u8; 32] = [
        0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x08,
        0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10, 0x23,
        0x03, 0x30,
    ];
    let decoded = RemoteUserId {
        t: REMOTE_USR_ID,
        length: 28,
        ins: 0,
        imsi: "901405101961873".to_string(),
        msisdn: Some("882285100125112".to_string()),
        imei: Some("8676480401323003".to_string()),
    };
    let i = RemoteUserId::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn remote_user_id_ie_marshal_test() {
    let encoded: [u8; 32] = [
        0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x08,
        0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10, 0x23,
        0x03, 0x30,
    ];
    let decoded = RemoteUserId {
        t: REMOTE_USR_ID,
        length: 28,
        ins: 0,
        imsi: "901405101961873".to_string(),
        msisdn: Some("882285100125112".to_string()),
        imei: Some("8676480401323003".to_string()),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn remote_user_id_ie_invalid_length_unmarshal_test() {
    let encoded: [u8; 22] = [
        0xc0, 0x00, 0x1c, 0x00, 0x01, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x08,
        0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11,
    ];
    let i = RemoteUserId::unmarshal(&encoded);
    assert_eq!(i, Err(GTPV2Error::IEInvalidLength(REMOTE_USR_ID)));
}
