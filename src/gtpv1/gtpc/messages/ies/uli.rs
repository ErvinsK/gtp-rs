// User Location Information (ULI) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// User Location Information (ULI) IE TV

pub const ULI: u8 = 152;
pub const ULI_LENGTH: u16 = 8;

// CI, SAC and RAC enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    Ci(u16),
    Sac(u16),
    Rac(u8),
}
// User Location Information (ULI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uli {
    pub t: u8,
    pub length: u16,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
    pub loc: Location,
}

impl Default for Uli {
    fn default() -> Self {
        Uli {
            t: ULI,
            length: ULI_LENGTH,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            lac: 0,
            loc: Location::Ci(0),
        }
    }
}

impl IEs for Uli {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        match self.loc {
            Location::Ci(i) => {
                buffer_ie.push(0);
                buffer_ie.append(&mut mcc_mnc_encode(
                    self.mcc,
                    self.mnc,
                    self.mnc_is_three_digits,
                ));
                buffer_ie.extend_from_slice(&self.lac.to_be_bytes());
                buffer_ie.extend_from_slice(&i.to_be_bytes());
            }
            Location::Sac(j) => {
                buffer_ie.push(1);
                buffer_ie.append(&mut mcc_mnc_encode(
                    self.mcc,
                    self.mnc,
                    self.mnc_is_three_digits,
                ));
                buffer_ie.extend_from_slice(&self.lac.to_be_bytes());
                buffer_ie.extend_from_slice(&j.to_be_bytes());
            }
            Location::Rac(z) => {
                buffer_ie.push(2);
                buffer_ie.append(&mut mcc_mnc_encode(
                    self.mcc,
                    self.mnc,
                    self.mnc_is_three_digits,
                ));
                buffer_ie.extend_from_slice(&self.lac.to_be_bytes());
                buffer_ie.push(z);
                buffer_ie.push(0xff);
            }
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= (ULI_LENGTH + 3) as usize {
            let mut data = Uli {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            match buffer[3] {
                0 => {
                    let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..=6]);
                    data.mcc = mcc;
                    data.mnc = mnc;
                    data.mnc_is_three_digits = mnc_is_three_digits;
                    data.lac = u16::from_be_bytes([buffer[7], buffer[8]]);
                    data.loc = Location::Ci(u16::from_be_bytes([buffer[9], buffer[10]]));
                }
                1 => {
                    let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..=6]);
                    data.mcc = mcc;
                    data.mnc = mnc;
                    data.mnc_is_three_digits = mnc_is_three_digits;
                    data.lac = u16::from_be_bytes([buffer[7], buffer[8]]);
                    data.loc = Location::Sac(u16::from_be_bytes([buffer[9], buffer[10]]));
                }
                2 => {
                    let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..=6]);
                    data.mcc = mcc;
                    data.mnc = mnc;
                    data.mnc_is_three_digits = mnc_is_three_digits;
                    data.lac = u16::from_be_bytes([buffer[7], buffer[8]]);
                    data.loc = Location::Rac(buffer[9]);
                }
                _ => {
                    return Err(GTPV1Error::IEIncorrect);
                }
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (ULI_LENGTH + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn uli_ie_marshal_test_cgi() {
    let ie_to_marshal = Uli {
        t: ULI,
        length: ULI_LENGTH,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        lac: 48190,
        loc: Location::Ci(14076),
    };
    let ie_unmarshalled: [u8; 11] = [
        0x98, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc,
    ];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_unmarshalled);
}

#[test]
fn uli_ie_unmarshal_test_cgi() {
    let ie_to_marshal = Uli {
        t: ULI,
        length: ULI_LENGTH,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        lac: 48190,
        loc: Location::Ci(14076),
    };
    let ie_unmarshalled: [u8; 11] = [
        0x98, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc,
    ];
    assert_eq!(Uli::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}

#[test]
fn uli_ie_marshal_test_sai() {
    let ie_to_marshal = Uli {
        t: ULI,
        length: ULI_LENGTH,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        lac: 48190,
        loc: Location::Sac(14076),
    };
    let ie_unmarshalled: [u8; 11] = [
        0x98, 0x00, 0x08, 0x01, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc,
    ];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_unmarshalled);
}

#[test]
fn uli_ie_unmarshal_test_sai() {
    let ie_to_marshal = Uli {
        t: ULI,
        length: ULI_LENGTH,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        lac: 48190,
        loc: Location::Sac(14076),
    };
    let ie_unmarshalled: [u8; 11] = [
        0x98, 0x00, 0x08, 0x01, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc,
    ];
    assert_eq!(Uli::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}

#[test]
fn uli_ie_marshal_test_rai() {
    let ie_to_marshal = Uli {
        t: ULI,
        length: ULI_LENGTH,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        lac: 48190,
        loc: Location::Rac(0x10),
    };
    let ie_unmarshalled: [u8; 11] = [
        0x98, 0x00, 0x08, 0x02, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x10, 0xff,
    ];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_unmarshalled);
}

#[test]
fn uli_ie_unmarshal_test_rai() {
    let ie_to_marshal = Uli {
        t: ULI,
        length: ULI_LENGTH,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        lac: 48190,
        loc: Location::Rac(0x10),
    };
    let ie_unmarshalled: [u8; 11] = [
        0x98, 0x00, 0x08, 0x02, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x10, 0xff,
    ];
    assert_eq!(Uli::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}
