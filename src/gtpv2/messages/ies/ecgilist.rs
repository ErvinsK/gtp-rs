// ECGI List IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// ECGI List IE Type

pub const ECGILIST: u8 = 190;

// ECGI List IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcgiList {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ecgi_list: Option<Vec<Ecgi>>,
}

impl Default for EcgiList {
    fn default() -> Self {
        EcgiList {
            t: ECGILIST,
            length: 0,
            ins: 0,
            ecgi_list: None,
        }
    }
}

impl From<EcgiList> for InformationElement {
    fn from(i: EcgiList) -> Self {
        InformationElement::EcgiList(i)
    }
}

impl IEs for EcgiList {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(ECGILIST);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        if let Some(i) = self.ecgi_list.clone() {
            buffer_ie.extend_from_slice(&(i.len() as u16).to_be_bytes());
            for ecgi in i {
                ecgi.marshal(&mut buffer_ie);
            }
        } else {
            buffer_ie.push(0);
            buffer_ie.push(0);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + 2 {
            let mut data = EcgiList {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..EcgiList::default()
            };
            let mut cursor: usize = 4;
            let list_size = u16::from_be_bytes([buffer[cursor], buffer[cursor + 1]]) as usize;
            cursor += 2;
            match list_size {
                0 => (),
                _ => {
                    if buffer.len() >= cursor + 7 * list_size {
                        let mut ecgi_list = Vec::new();
                        for _ in 0..list_size {
                            if let Ok(ie) = Ecgi::unmarshal(&buffer[cursor..]) {
                                ecgi_list.push(ie);
                                cursor += 7;
                            } else {
                                return Err(GTPV2Error::IEIncorrect(ECGILIST));
                            }
                        }
                        data.ecgi_list = Some(ecgi_list);
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(ECGILIST));
                    }
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ECGILIST))
        }
    }

    fn len(&self) -> usize {
        match &self.ecgi_list {
            Some(i) => MIN_IE_SIZE + 2 + i.len() * 7,
            None => MIN_IE_SIZE + 2,
        }
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
fn ecgilist_ie_marshal_test() {
    let ie_marshalled: [u8; 27] = [
        0xbe, 0x00, 0x17, 0x00, 0x00, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x62, 0xf2,
        0x10, 0x01, 0xba, 0x40, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x01,
    ];
    let ie_to_marshal = EcgiList {
        length: 23,
        ecgi_list: Some(vec![
            Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983298,
            },
            Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983299,
            },
            Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983297,
            },
        ]),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn ecgilist_ie_unmarshal_test() {
    let ie_marshalled: [u8; 27] = [
        0xbe, 0x00, 0x17, 0x00, 0x00, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x62, 0xf2,
        0x10, 0x01, 0xba, 0x40, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x01,
    ];
    let ie_to_marshal = EcgiList {
        length: 23,
        ecgi_list: Some(vec![
            Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983298,
            },
            Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983299,
            },
            Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983297,
            },
        ]),
        ..Default::default()
    };
    assert_eq!(EcgiList::unmarshal(&ie_marshalled).unwrap(), ie_to_marshal);
}
