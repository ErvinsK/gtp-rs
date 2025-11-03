// User Location Information (ULI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// User Location Information (ULI) IE Type

pub const ULI: u8 = 86;

// CGI, SAI, RAI, TAI, ECGI, LAI, Macro eNB ID, Extended Macro eNB ID

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Location {
    Cgi(Cgi),
    Sai(Sai),
    Rai(Rai),
    Tai(Tai),
    Ecgi(Ecgi),
    Lai(Lai),
    MacroEnbId(MacroEnbId),
    ExtMacroEnbId(ExtMacroEnbId),
}
// User Location Information (ULI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uli {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub loc: Vec<Location>,
}

impl Default for Uli {
    fn default() -> Self {
        Uli {
            t: ULI,
            length: 0,
            ins: 0,
            loc: vec![],
        }
    }
}

impl From<Uli> for InformationElement {
    fn from(i: Uli) -> Self {
        InformationElement::Uli(i)
    }
}

impl IEs for Uli {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let mut flags: u8 = 0;
        let mut buffer_li: Vec<u8> = vec![];
        let mut sorted_loc = self.loc.clone();
        sorted_loc.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for i in sorted_loc.iter() {
            match i {
                Location::Cgi(j) => {
                    if flags & 0x01 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x01;
                    }
                }
                Location::Sai(j) => {
                    if flags & 0x02 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x02;
                    }
                }
                Location::Rai(j) => {
                    if flags & 0x04 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x04;
                    }
                }
                Location::Tai(j) => {
                    if flags & 0x08 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x08;
                    }
                }
                Location::Ecgi(j) => {
                    if flags & 0x10 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x10;
                    }
                }
                Location::Lai(j) => {
                    if flags & 0x20 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x20;
                    }
                }
                Location::MacroEnbId(j) => {
                    if flags & 0x40 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x40;
                    }
                }
                Location::ExtMacroEnbId(j) => {
                    if flags & 0x80 == 0 {
                        j.marshal(&mut buffer_li);
                        flags |= 0x80;
                    }
                }
            }
        }
        buffer_ie.push(flags);
        buffer_ie.append(&mut buffer_li);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Uli {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Uli::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                let order = to_flags(&buffer[4]);
                let mut cursor: usize = 5;
                for i in order.iter() {
                    match i {
                        1 => {
                            match Cgi::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::Cgi(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 7;
                        }
                        2 => {
                            match Sai::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::Sai(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 7;
                        }
                        3 => {
                            match Rai::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::Rai(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 7;
                        }
                        4 => {
                            match Tai::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::Tai(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 5;
                        }
                        5 => {
                            match Ecgi::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::Ecgi(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 7;
                        }
                        6 => {
                            match Lai::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::Lai(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 5;
                        }
                        7 => {
                            match MacroEnbId::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::MacroEnbId(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 6;
                        }
                        8 => {
                            match ExtMacroEnbId::unmarshal(&buffer[cursor..]) {
                                Ok(k) => data.loc.push(Location::ExtMacroEnbId(k)),
                                Err(n) => return Err(n),
                            }
                            cursor += 6;
                        }
                        _ => (),
                    }
                }
            } else {
                return Err(GTPV2Error::IEInvalidLength(ULI));
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ULI))
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

// Helper function to convert flag octets into Vector of u8
fn to_flags(i: &u8) -> Vec<u8> {
    let mut results = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut flags = vec![];
    for j in 0..=7 {
        if i & (0x01 << j) == (0x01 << j) {
            flags.push(true);
        } else {
            flags.push(false);
        }
    }
    let mut iter = flags.iter();
    results.retain(|_| *iter.next().unwrap());
    results
}

#[test]
fn uli_ie_marshal_test_tai_ecgi() {
    let decoded = Uli {
        t: ULI,
        length: 13,
        ins: 0,
        loc: vec![
            Location::Ecgi(Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983298,
            }),
            Location::Tai(Tai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                tac: 0x0bd9,
            }),
        ],
    };
    let encoded: [u8; 17] = [
        0x56, 0x00, 0x0d, 0x00, 0x18, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x01, 0xba,
        0x40, 0x02,
    ];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn uli_ie_unmarshal_test_tai_ecgi() {
    let decoded = Uli {
        t: ULI,
        length: 13,
        ins: 0,
        loc: vec![
            Location::Tai(Tai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                tac: 0x0bd9,
            }),
            Location::Ecgi(Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983298,
            }),
        ],
    };
    let encoded: [u8; 17] = [
        0x56, 0x00, 0x0d, 0x00, 0x18, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x01, 0xba,
        0x40, 0x02,
    ];
    assert_eq!(Uli::unmarshal(&encoded), Ok(decoded));
}

#[test]
fn uli_ie_unmarshal_test_tai_ecgi_invalid_length() {
    let encoded: [u8; 16] = [
        0x56, 0x00, 0x0d, 0x00, 0x18, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x01, 0xba, 0x40,
        0x02,
    ];
    assert_eq!(
        Uli::unmarshal(&encoded),
        Err(GTPV2Error::IEInvalidLength(ULI))
    );
}
