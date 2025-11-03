// TWAN Identifier IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use std::str::from_utf8;

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// TWAN Identifier IE Type

pub const TWAN_ID: u8 = 169;

// TWAN Identifier IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwanId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ssid: Vec<u8>,
    pub bssid: Option<Vec<u8>>,
    pub civic_address: Option<Vec<u8>>,
    pub twan_plmnid: Option<(u16, u16, bool)>, // MCC, MNC and 3-digit flag
    pub twan_op_name: Option<String>,
    pub relay_id: Option<(u8, Vec<u8>)>,
    pub circuit_id: Option<Vec<u8>>,
}

impl Default for TwanId {
    fn default() -> TwanId {
        TwanId {
            t: TWAN_ID,
            length: 1,
            ins: 0,
            ssid: vec![],
            bssid: None,
            civic_address: None,
            twan_plmnid: None,
            twan_op_name: None,
            relay_id: None,
            circuit_id: None,
        }
    }
}

impl From<TwanId> for InformationElement {
    fn from(i: TwanId) -> Self {
        InformationElement::TwanId(i)
    }
}

impl IEs for TwanId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TWAN_ID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags: u8 = [
            self.relay_id.is_some(),
            self.twan_op_name.is_some(),
            self.twan_plmnid.is_some(),
            self.civic_address.is_some(),
            self.bssid.is_some(),
        ]
        .iter()
        .enumerate()
        .map(|(x, i)| if *i { 0b10000 >> x } else { 0 })
        .sum();
        buffer_ie.push(flags);
        buffer_ie.push(self.ssid.len() as u8);
        buffer_ie.extend_from_slice(&self.ssid[..]);
        if let Some(i) = &self.bssid {
            buffer_ie.extend_from_slice(&i[..]);
        }
        if let Some(i) = &self.civic_address {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i[..]);
        }
        if let Some((i, j, mnc_is_three_digits)) = &self.twan_plmnid {
            buffer_ie.extend_from_slice(&mcc_mnc_encode(*i, *j, *mnc_is_three_digits));
        }
        if let Some(i) = &self.twan_op_name {
            let b = i.as_bytes();
            buffer_ie.push(b.len() as u8);
            buffer_ie.extend_from_slice(b);
        }
        if let Some(i) = &self.relay_id {
            buffer_ie.push(i.0);
            buffer_ie.push(i.1.len() as u8);
            buffer_ie.extend_from_slice(&i.1[..]);
        }
        if let Some(i) = &self.circuit_id {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i[..]);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 2 {
            let mut data = TwanId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..TwanId::default()
            };
            let flags = buffer[4];
            let mut cursor = buffer[5] as usize;
            data.ssid = buffer[6..6 + cursor].to_vec();
            cursor += 6;
            match flags & 0x01 {
                1 => {
                    data.bssid = Some(buffer[cursor..cursor + 6].into());
                    cursor += 6;
                }
                _ => data.bssid = None,
            }
            match (flags >> 1) & 0x01 {
                1 => {
                    if cursor <= buffer.len() {
                        let field = buffer[cursor] as usize;
                        cursor += 1;
                        data.civic_address = Some(buffer[cursor..cursor + field].to_vec());
                        cursor += field;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                }
                _ => data.civic_address = None,
            }
            match (flags >> 2) & 0x01 {
                1 => {
                    match buffer[cursor..cursor + 3].try_into() {
                        Ok(i) => data.twan_plmnid = Some(mcc_mnc_decode(i)),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                    }
                    cursor += 3;
                }
                _ => data.twan_plmnid = None,
            }
            match (flags >> 3) & 0x01 {
                1 => {
                    if cursor <= buffer.len() {
                        let field = buffer[cursor] as usize;
                        cursor += 1;
                        match buffer[cursor..cursor + field].try_into() {
                            Ok(i) => match from_utf8(i) {
                                Ok(j) => data.twan_op_name = Some(j.to_string()),
                                Err(_) => return Err(GTPV2Error::IEIncorrect(TWAN_ID)),
                            },
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(TWAN_ID)),
                        }
                        cursor += field;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                }
                _ => data.twan_op_name = None,
            }
            match (flags >> 4) & 0x01 {
                1 => {
                    if cursor < buffer.len() {
                        let relay_id = buffer[cursor];
                        let field = buffer[cursor + 1] as usize;
                        cursor += 2;
                        if cursor + field <= buffer.len() {
                            data.relay_id =
                                Some((relay_id, buffer[cursor..cursor + field].to_vec()));
                            cursor += field;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                        }
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                    if cursor <= buffer.len() {
                        let field = buffer[cursor] as usize;
                        cursor += 1;
                        if cursor + field <= buffer.len() {
                            data.circuit_id = Some(buffer[cursor..cursor + field].to_vec());
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                        }
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(TWAN_ID));
                    }
                }
                _ => {
                    data.relay_id = None;
                    data.circuit_id = None;
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TWAN_ID))
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
fn twan_id_ie_unmarshal_test() {
    let encoded: [u8; 37] = [
        0xa9, 0x00, 0x21, 0x00, 0x1f, 0x03, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x03, 0x00, 0x00, 0x00, 0x99, 0xf9, 0x10, 0x04, 0x74, 0x65, 0x73, 0x74, 0x00, 0x04, 0xff,
        0xff, 0xff, 0xff, 0x03, 0xaa, 0xaa, 0xaa,
    ];
    let decoded = TwanId {
        t: TWAN_ID,
        length: 33,
        ins: 0,
        ssid: vec![0x00, 0x00, 0x00],
        bssid: Some(vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
        civic_address: Some(vec![0x00, 0x00, 0x00]),
        twan_plmnid: Some((999, 1, false)),
        twan_op_name: Some("test".to_string()),
        relay_id: Some((0, vec![0xff, 0xff, 0xff, 0xff])),
        circuit_id: Some(vec![0xaa, 0xaa, 0xaa]),
    };
    let i = TwanId::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn twan_id_ie_marshal_test() {
    let encoded: [u8; 37] = [
        0xa9, 0x00, 0x21, 0x00, 0x1f, 0x03, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x03, 0x00, 0x00, 0x00, 0x99, 0xf9, 0x10, 0x04, 0x74, 0x65, 0x73, 0x74, 0x00, 0x04, 0xff,
        0xff, 0xff, 0xff, 0x03, 0xaa, 0xaa, 0xaa,
    ];
    let decoded = TwanId {
        t: TWAN_ID,
        length: 33,
        ins: 0,
        ssid: vec![0x00, 0x00, 0x00],
        bssid: Some(vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
        civic_address: Some(vec![0x00, 0x00, 0x00]),
        twan_plmnid: Some((999, 1, false)),
        twan_op_name: Some("test".to_string()),
        relay_id: Some((0, vec![0xff, 0xff, 0xff, 0xff])),
        circuit_id: Some(vec![0xaa, 0xaa, 0xaa]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
