// Additional MM Context for SRVCC IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Additional MM Context for SRVCC IE  IE Type

pub const ADDMMCTXSRVCC: u8 = 159;

// Additional MM Context for SRVCC IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdditionalMmContextForSrvcc {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mobile_station_classmark2: Option<Vec<u8>>,
    pub mobile_station_classmark3: Option<Vec<u8>>,
    pub supported_codec_list: Option<Vec<u8>>,
}

impl Default for AdditionalMmContextForSrvcc {
    fn default() -> Self {
        AdditionalMmContextForSrvcc {
            t: ADDMMCTXSRVCC,
            length: 0,
            ins: 0,
            mobile_station_classmark2: None,
            mobile_station_classmark3: None,
            supported_codec_list: None,
        }
    }
}

impl From<AdditionalMmContextForSrvcc> for InformationElement {
    fn from(i: AdditionalMmContextForSrvcc) -> Self {
        InformationElement::AdditionalMmContextForSrvcc(i)
    }
}

impl IEs for AdditionalMmContextForSrvcc {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ADDMMCTXSRVCC);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        if let Some(i) = self.mobile_station_classmark2.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.mobile_station_classmark3.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.supported_codec_list.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = AdditionalMmContextForSrvcc {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..AdditionalMmContextForSrvcc::default()
            };
            let mut cursor: usize = 4;
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.mobile_station_classmark2 =
                            Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(ADDMMCTXSRVCC));
                    }
                } else {
                    cursor += 1;
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.mobile_station_classmark3 =
                            Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(ADDMMCTXSRVCC));
                    }
                } else {
                    cursor += 1;
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.supported_codec_list = Some(buffer[cursor..cursor + len].to_vec());
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(ADDMMCTXSRVCC));
                    }
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ADDMMCTXSRVCC))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn addmmctxsrvcc_ie_marshal_test() {
    let ie_marshalled: [u8; 19] = [
        0x9f, 0x00, 0x0f, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04,
        0x01, 0x02, 0x03, 0x04,
    ];
    let ie_to_marshal = AdditionalMmContextForSrvcc {
        length: 15,
        mobile_station_classmark2: Some(vec![0x01, 0x02, 0x03, 0x04]),
        mobile_station_classmark3: Some(vec![0x01, 0x02, 0x03, 0x04]),
        supported_codec_list: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn addmmctxsrvcc_ie_unmarshal_test() {
    let ie_marshalled: [u8; 19] = [
        0x9f, 0x00, 0x0f, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04,
        0x01, 0x02, 0x03, 0x04,
    ];
    let ie_to_marshal = AdditionalMmContextForSrvcc {
        length: 15,
        mobile_station_classmark2: Some(vec![0x01, 0x02, 0x03, 0x04]),
        mobile_station_classmark3: Some(vec![0x01, 0x02, 0x03, 0x04]),
        supported_codec_list: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ..Default::default()
    };
    assert_eq!(
        AdditionalMmContextForSrvcc::unmarshal(&ie_marshalled).unwrap(),
        ie_to_marshal
    );
}
