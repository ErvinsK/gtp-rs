// Commons for GTP-C IEs

use crate::gtpv2::{errors::GTPV2Error, utils::*};

pub const MIN_IE_SIZE: usize = 4;

pub trait IEs {
    fn marshal(&self, buffer: &mut Vec<u8>);
    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error>
    where
        Self: Sized;
    fn len(&self) -> usize; // Total IE length = Type+Length+Instance+Value for TLIV messages
    fn is_empty(&self) -> bool; // is_empty() method
    fn get_ins(&self) -> u8; // get_ins() method
    fn get_type(&self) -> u8; // get_type() method
}

// Location Field definitions

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Cgi {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
    pub ci: u16,
}

impl IEs for Cgi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.extend_from_slice(&self.ci.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 7 {
            let mut data = Cgi::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.lac = u16::from_be_bytes([buffer[3], buffer[4]]);
            data.ci = u16::from_be_bytes([buffer[5], buffer[6]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.lac == 0 && self.ci == 0
    }
    fn len(&self) -> usize {
        7
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Sai {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
    pub sac: u16,
}

impl IEs for Sai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.extend_from_slice(&self.sac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 7 {
            let mut data = Sai::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.lac = u16::from_be_bytes([buffer[3], buffer[4]]);
            data.sac = u16::from_be_bytes([buffer[5], buffer[6]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.lac == 0 && self.sac == 0
    }
    fn len(&self) -> usize {
        7
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Rai {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
    pub rac: u8,
}

impl IEs for Rai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
        buffer.push(0xff);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 6 {
            let mut data = Rai::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.lac = u16::from_be_bytes([buffer[3], buffer[4]]);
            data.rac = buffer[5];
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.lac == 0 && self.rac == 0
    }
    fn len(&self) -> usize {
        6
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Tai {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub tac: u16,
}

impl IEs for Tai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.tac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 5 {
            let mut data = Tai::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.tac = u16::from_be_bytes([buffer[3], buffer[4]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.tac == 0
    }
    fn len(&self) -> usize {
        5
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Ecgi {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub eci: u32,
}

impl IEs for Ecgi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.eci.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 7 {
            let mut data = Ecgi::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.eci = u32::from_be_bytes([buffer[3], buffer[4], buffer[5], buffer[6]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.eci == 0
    }
    fn len(&self) -> usize {
        7
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Lai {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
}

impl IEs for Lai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 5 {
            let mut data = Lai::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.lac = u16::from_be_bytes([buffer[3], buffer[4]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.lac == 0
    }

    fn len(&self) -> usize {
        5
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct MacroEnbId {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub macro_id: u32,
}

impl IEs for MacroEnbId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.macro_id.to_be_bytes()[1..]);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 6 {
            let mut data = MacroEnbId::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.macro_id = u32::from_be_bytes([0x00, buffer[3], buffer[4], buffer[5]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.macro_id == 0
    }

    fn len(&self) -> usize {
        6
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct ExtMacroEnbId {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub smenb: bool,
    pub ext_macro_id: u32,
}

impl IEs for ExtMacroEnbId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        if self.smenb {
            let mut i = self.ext_macro_id.to_be_bytes();
            i[1] = (i[1] | 0x80) & 0x83;
            buffer.extend_from_slice(&i[1..]);
        } else {
            let i = self.ext_macro_id.to_be_bytes();
            buffer.extend_from_slice(&i[1..]);
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 6 {
            let mut data = ExtMacroEnbId::default();
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            match buffer[3] >> 7 {
                0 => {
                    data.smenb = false;
                    data.ext_macro_id = u32::from_be_bytes([0x00, buffer[3], buffer[4], buffer[5]]);
                }
                1 => {
                    data.smenb = true;
                    data.ext_macro_id =
                        u32::from_be_bytes([0x00, (buffer[3] & 0x03), buffer[4], buffer[5]]);
                }
                _ => (),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.ext_macro_id == 0
    }

    fn len(&self) -> usize {
        6
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// MM Context Authentication Field definitions

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct AuthTriplet {
    pub rand: [u8; 16],
    pub sres: [u8; 4],
    pub kc: [u8; 8],
}

impl IEs for AuthTriplet {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.rand[..]);
        buffer.extend_from_slice(&self.sres[..]);
        buffer.extend_from_slice(&self.kc[..]);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 28 {
            let mut data = AuthTriplet::default();
            data.rand.copy_from_slice(&buffer[..16]);
            data.sres.copy_from_slice(&buffer[16..20]);
            data.kc.copy_from_slice(&buffer[20..28]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.rand == [0; 16] && self.sres == [0; 4] && self.kc == [0; 8]
    }

    fn len(&self) -> usize {
        28
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[test]
fn test_auth_triplet_marshal() {
    let encoded_ie: [u8; 28] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
    ];
    let test_struct = AuthTriplet {
        rand: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        sres: [0x11, 0x12, 0x13, 0x14],
        kc: [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_auth_triplet_unmarshal() {
    let encoded_ie: [u8; 28] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
    ];
    let test_struct = AuthTriplet {
        rand: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        sres: [0x11, 0x12, 0x13, 0x14],
        kc: [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c],
    };
    assert_eq!(AuthTriplet::unmarshal(&encoded_ie).unwrap(), test_struct);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct AuthQuintuplet {
    pub rand: [u8; 16],
    pub xres: Vec<u8>,
    pub ck: [u8; 16],
    pub ik: [u8; 16],
    pub autn: Vec<u8>,
}

impl IEs for AuthQuintuplet {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.rand[..]);
        buffer.push(self.xres.len() as u8);
        buffer.extend_from_slice(&self.xres[..]);
        buffer.extend_from_slice(&self.ck[..]);
        buffer.extend_from_slice(&self.ik[..]);
        buffer.push(self.autn.len() as u8);
        buffer.extend_from_slice(&self.autn[..]);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 16 {
            let mut cursor: usize = 16;
            let mut data = AuthQuintuplet::default();
            data.rand.copy_from_slice(&buffer[..cursor]);
            let xres_len = buffer[cursor] as usize;
            cursor += 1;
            if buffer.len() >= cursor + xres_len {
                data.xres
                    .extend_from_slice(&buffer[cursor..cursor + xres_len]);
                cursor += xres_len;
                if buffer.len() >= cursor + 32 {
                    data.ck.copy_from_slice(&buffer[cursor..cursor + 16]);
                    cursor += 16;
                    data.ik.copy_from_slice(&buffer[cursor..cursor + 16]);
                    cursor += 16;
                    let autn_len = buffer[cursor] as usize;
                    cursor += 1;
                    if buffer.len() >= cursor + autn_len {
                        data.autn
                            .extend_from_slice(&buffer[cursor..cursor + autn_len]);
                        Ok(data)
                    } else {
                        Err(GTPV2Error::IEIncorrect(0))
                    }
                } else {
                    Err(GTPV2Error::IEIncorrect(0))
                }
            } else {
                Err(GTPV2Error::IEIncorrect(0))
            }
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.rand == [0; 16]
            && self.xres.len() == 0
            && self.ck == [0; 16]
            && self.ik == [0; 16]
            && self.autn.len() == 0
    }

    fn len(&self) -> usize {
        50 + self.xres.len() + self.autn.len()
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[test]
fn test_auth_quintuplet_marshal() {
    let encoded_ie: [u8; 56] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a,
    ];

    let test_struct = AuthQuintuplet {
        rand: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        xres: vec![0x02, 0x07, 0x08],
        ck: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        ik: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        autn: vec![0x03, 0x09, 0x0a],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_auth_quintuplet_unmarshal() {
    let encoded_ie: [u8; 56] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a,
    ];

    let test_struct = AuthQuintuplet {
        rand: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        xres: vec![0x02, 0x07, 0x08],
        ck: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        ik: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        autn: vec![0x03, 0x09, 0x0a],
    };
    assert_eq!(AuthQuintuplet::unmarshal(&encoded_ie).unwrap(), test_struct);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct AuthQuadruplet {
    pub rand: [u8; 16],
    pub xres: Vec<u8>,
    pub autn: Vec<u8>,
    pub kasme: [u8; 32],
}

impl IEs for AuthQuadruplet {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.rand[..]);
        buffer.push(self.xres.len() as u8);
        buffer.extend_from_slice(&self.xres[..]);
        buffer.push(self.autn.len() as u8);
        buffer.extend_from_slice(&self.autn[..]);
        buffer.extend_from_slice(&self.kasme[..]);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 17 {
            let mut data = AuthQuadruplet::default();
            let mut cursor: usize = 0;
            data.rand.copy_from_slice(&buffer[..cursor + 16]);
            cursor += 16;
            let xres_len = buffer[cursor] as usize;
            cursor += 1;
            if buffer.len() >= cursor + xres_len {
                data.xres
                    .extend_from_slice(&buffer[cursor..cursor + xres_len]);
                cursor += xres_len;
                let autn_len = buffer[cursor] as usize;
                cursor += 1;
                if buffer.len() >= cursor + autn_len {
                    data.autn
                        .extend_from_slice(&buffer[cursor..cursor + autn_len]);
                    cursor += autn_len;
                    if buffer.len() >= cursor + 32 {
                        data.kasme.copy_from_slice(&buffer[cursor..cursor + 32]);
                    } else {
                        return Err(GTPV2Error::IEIncorrect(0));
                    }
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            } else {
                return Err(GTPV2Error::IEIncorrect(0));
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.rand == [0; 16]
            && self.xres.len() == 0
            && self.autn.len() == 0
            && self.kasme == [0; 32]
    }

    fn len(&self) -> usize {
        50 + self.xres.len() + self.autn.len()
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[test]
fn test_auth_quadruplet_marshal() {
    let encoded_ie: [u8; 56] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x03, 0x03, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = AuthQuadruplet {
        rand: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        xres: vec![0x02, 0x07, 0x08],
        autn: vec![0x03, 0x09, 0x0a],
        kasme: [
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_auth_quadruplet_unmarshal() {
    let encoded_ie: [u8; 56] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x03, 0x03, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let test_struct = AuthQuadruplet {
        rand: [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ],
        xres: vec![0x02, 0x07, 0x08],
        autn: vec![0x03, 0x09, 0x0a],
        kasme: [
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
    };
    assert_eq!(AuthQuadruplet::unmarshal(&encoded_ie).unwrap(), test_struct);
}

// MM Context APN Rate Control Status Field definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct ApnRateControlStatusMM {
    pub apn: String,
    pub uplink_rate_limit: u32,
    pub nbr_of_exception_reports: u32,
    pub downlink_rate_limit: u32,
    pub apn_rate_control_status_validity: [u8; 8],
}

impl IEs for ApnRateControlStatusMM {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut body: Vec<u8> = vec![];
        body.extend_from_slice(&(self.apn.len() as u16).to_be_bytes());
        body.extend_from_slice(self.apn.as_bytes());
        body.extend_from_slice(&self.uplink_rate_limit.to_be_bytes());
        body.extend_from_slice(&self.nbr_of_exception_reports.to_be_bytes());
        body.extend_from_slice(&self.downlink_rate_limit.to_be_bytes());
        body.extend_from_slice(&self.apn_rate_control_status_validity[..]);
        buffer.extend_from_slice(&(body.len() as u16).to_be_bytes());
        buffer.append(&mut body);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 2 {
            let mut data = ApnRateControlStatusMM::default();
            let length = u16::from_be_bytes([buffer[0], buffer[1]]) as usize;
            let mut cursor: usize = 2;
            if buffer.len() >= length + cursor {
                let apn_length: usize =
                    u16::from_be_bytes([buffer[cursor], buffer[cursor + 1]]) as usize;
                cursor += 2;
                if buffer.len() >= apn_length + cursor {
                    data.apn =
                        String::from_utf8_lossy(&buffer[cursor..cursor + apn_length]).to_string();
                    cursor += apn_length;
                    if buffer.len() >= cursor + 20 {
                        data.uplink_rate_limit = u32::from_be_bytes([
                            buffer[cursor],
                            buffer[cursor + 1],
                            buffer[cursor + 2],
                            buffer[cursor + 3],
                        ]);
                        cursor += 4;
                        data.nbr_of_exception_reports = u32::from_be_bytes([
                            buffer[cursor],
                            buffer[cursor + 1],
                            buffer[cursor + 2],
                            buffer[cursor + 3],
                        ]);
                        cursor += 4;
                        data.downlink_rate_limit = u32::from_be_bytes([
                            buffer[cursor],
                            buffer[cursor + 1],
                            buffer[cursor + 2],
                            buffer[cursor + 3],
                        ]);
                        cursor += 4;
                        data.apn_rate_control_status_validity
                            .copy_from_slice(&buffer[cursor..cursor + 8]);
                    } else {
                        return Err(GTPV2Error::IEIncorrect(0));
                    }
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            } else {
                return Err(GTPV2Error::IEIncorrect(0));
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.apn.is_empty()
            && self.uplink_rate_limit == 0
            && self.nbr_of_exception_reports == 0
            && self.downlink_rate_limit == 0
            && self.apn_rate_control_status_validity == [0; 8]
    }

    fn len(&self) -> usize {
        24 + self.apn.len()
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[test]

fn test_apn_rate_control_status_marshal() {
    let encoded_ie: [u8; 27] = [
        0x00, 0x19, 0x00, 0x03, 0x61, 0x70, 0x6E, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78,
        0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];
    let test_struct = ApnRateControlStatusMM {
        apn: "apn".to_string(),
        uplink_rate_limit: 0x12345678,
        nbr_of_exception_reports: 0x12345678,
        downlink_rate_limit: 0x12345678,
        apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_apn_rate_control_status_unmarshal() {
    let encoded_ie: [u8; 27] = [
        0x00, 0x19, 0x00, 0x03, 0x61, 0x70, 0x6E, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78,
        0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];
    let test_struct = ApnRateControlStatusMM {
        apn: "apn".to_string(),
        uplink_rate_limit: 0x12345678,
        nbr_of_exception_reports: 0x12345678,
        downlink_rate_limit: 0x12345678,
        apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    };
    assert_eq!(
        ApnRateControlStatusMM::unmarshal(&encoded_ie).unwrap(),
        test_struct
    );
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum SecurityMode {
    GsmKeyAndTriplets,
    UmtsKeyUsedCipherAndQuintuplets,
    GsmKeyUsedCipherAndQuintuplets,
    UmtsKeyAndQuintuplets,
    EpsSecurityContextAndQuadruplets,
    UmtsKeyQuadrupletsAndQuintuplets,
    Spare,
}

impl From<SecurityMode> for u8 {
    fn from(mode: SecurityMode) -> u8 {
        match mode {
            SecurityMode::GsmKeyAndTriplets => 0,
            SecurityMode::UmtsKeyUsedCipherAndQuintuplets => 1,
            SecurityMode::GsmKeyUsedCipherAndQuintuplets => 2,
            SecurityMode::UmtsKeyAndQuintuplets => 3,
            SecurityMode::EpsSecurityContextAndQuadruplets => 4,
            SecurityMode::UmtsKeyQuadrupletsAndQuintuplets => 5,
            SecurityMode::Spare => 6,
        }
    }
}

impl From<u8> for SecurityMode {
    fn from(value: u8) -> SecurityMode {
        match value {
            0 => SecurityMode::GsmKeyAndTriplets,
            1 => SecurityMode::UmtsKeyUsedCipherAndQuintuplets,
            2 => SecurityMode::GsmKeyUsedCipherAndQuintuplets,
            3 => SecurityMode::UmtsKeyAndQuintuplets,
            4 => SecurityMode::EpsSecurityContextAndQuadruplets,
            5 => SecurityMode::UmtsKeyQuadrupletsAndQuintuplets,
            _ => SecurityMode::Spare,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub enum NasCipherValues {
    #[default]
    NoChiper,
    Eea1,
    Eea2,
    Eea3,
    Eea4,
    Eea5,
    Eea6,
    Eea7,
    Spare,
}

impl From<NasCipherValues> for u8 {
    fn from(mode: NasCipherValues) -> u8 {
        match mode {
            NasCipherValues::NoChiper => 0,
            NasCipherValues::Eea1 => 1,
            NasCipherValues::Eea2 => 2,
            NasCipherValues::Eea3 => 3,
            NasCipherValues::Eea4 => 4,
            NasCipherValues::Eea5 => 5,
            NasCipherValues::Eea6 => 6,
            NasCipherValues::Eea7 => 7,
            NasCipherValues::Spare => 8,
        }
    }
}

impl From<u8> for NasCipherValues {
    fn from(value: u8) -> NasCipherValues {
        match value {
            0 => NasCipherValues::NoChiper,
            1 => NasCipherValues::Eea1,
            2 => NasCipherValues::Eea2,
            3 => NasCipherValues::Eea3,
            4 => NasCipherValues::Eea4,
            5 => NasCipherValues::Eea5,
            6 => NasCipherValues::Eea6,
            7 => NasCipherValues::Eea7,
            _ => NasCipherValues::Spare,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub enum NasIntegrityProtectionValues {
    #[default]
    NoIntegrity,
    Eia1,
    Eia2,
    Eia3,
    Eia4,
    Eia5,
    Eia6,
    Eia7,
}

impl From<NasIntegrityProtectionValues> for u8 {
    fn from(mode: NasIntegrityProtectionValues) -> u8 {
        match mode {
            NasIntegrityProtectionValues::NoIntegrity => 0,
            NasIntegrityProtectionValues::Eia1 => 1,
            NasIntegrityProtectionValues::Eia2 => 2,
            NasIntegrityProtectionValues::Eia3 => 3,
            NasIntegrityProtectionValues::Eia4 => 4,
            NasIntegrityProtectionValues::Eia5 => 5,
            NasIntegrityProtectionValues::Eia6 => 6,
            NasIntegrityProtectionValues::Eia7 => 7,
        }
    }
}

impl From<u8> for NasIntegrityProtectionValues {
    fn from(value: u8) -> NasIntegrityProtectionValues {
        match value {
            0 => NasIntegrityProtectionValues::NoIntegrity,
            1 => NasIntegrityProtectionValues::Eia1,
            2 => NasIntegrityProtectionValues::Eia2,
            3 => NasIntegrityProtectionValues::Eia3,
            4 => NasIntegrityProtectionValues::Eia4,
            5 => NasIntegrityProtectionValues::Eia5,
            6 => NasIntegrityProtectionValues::Eia6,
            7 => NasIntegrityProtectionValues::Eia7,
            _ => NasIntegrityProtectionValues::NoIntegrity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub enum CipherValues {
    #[default]
    NoCipher,
    Gea1,
    Gea2,
    Gea3,
    Gea4,
    Gea5,
    Gea6,
    Gea7,
}

impl From<CipherValues> for u8 {
    fn from(mode: CipherValues) -> u8 {
        match mode {
            CipherValues::NoCipher => 0,
            CipherValues::Gea1 => 1,
            CipherValues::Gea2 => 2,
            CipherValues::Gea3 => 3,
            CipherValues::Gea4 => 4,
            CipherValues::Gea5 => 5,
            CipherValues::Gea6 => 6,
            CipherValues::Gea7 => 7,
        }
    }
}

impl From<u8> for CipherValues {
    fn from(value: u8) -> CipherValues {
        match value {
            1 => CipherValues::Gea1,
            2 => CipherValues::Gea2,
            3 => CipherValues::Gea3,
            4 => CipherValues::Gea4,
            5 => CipherValues::Gea5,
            6 => CipherValues::Gea6,
            7 => CipherValues::Gea7,
            _ => CipherValues::NoCipher,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub enum GprsIntegrityProtectionValues {
    #[default]
    NoIntegrity,
    Gia4,
    Gia5,
}

impl From<GprsIntegrityProtectionValues> for u8 {
    fn from(mode: GprsIntegrityProtectionValues) -> u8 {
        match mode {
            GprsIntegrityProtectionValues::NoIntegrity => 0,
            GprsIntegrityProtectionValues::Gia4 => 4,
            GprsIntegrityProtectionValues::Gia5 => 5,
        }
    }
}

impl From<u8> for GprsIntegrityProtectionValues {
    fn from(value: u8) -> GprsIntegrityProtectionValues {
        match value {
            4 => GprsIntegrityProtectionValues::Gia4,
            5 => GprsIntegrityProtectionValues::Gia5,
            _ => GprsIntegrityProtectionValues::NoIntegrity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct AccessRestrictionMM {
    pub una: bool,  // UTRAN Not Allowed
    pub gena: bool, // GERAN Not Allowed
    pub gana: bool, // GAN Not Allowed
    pub ina: bool,  // I-HSPA-Evolution Not Allowed
    pub ena: bool,  // WB-E-UTRAN Not Allowed
    pub hnna: bool, // HO-To-Non-3GPP Not Allowed
    pub nbna: bool, // NB-IoT Not Allowed
    pub ecna: bool, // Enhanced Coverage Not Allowed
}

impl From<AccessRestrictionMM> for u8 {
    fn from(mode: AccessRestrictionMM) -> u8 {
        let mut value: u8 = 0;
        value += if mode.una { 0x01 } else { 0 };
        value += if mode.gena { 0x02 } else { 0 };
        value += if mode.gana { 0x04 } else { 0 };
        value += if mode.ina { 0x08 } else { 0 };
        value += if mode.ena { 0x10 } else { 0 };
        value += if mode.hnna { 0x20 } else { 0 };
        value += if mode.nbna { 0x40 } else { 0 };
        value += if mode.ecna { 0x80 } else { 0 };
        value
    }
}

impl From<u8> for AccessRestrictionMM {
    fn from(value: u8) -> AccessRestrictionMM {
        AccessRestrictionMM {
            una: (value & 0x01) != 0,
            gena: (value & 0x02) != 0,
            gana: (value & 0x04) != 0,
            ina: (value & 0x08) != 0,
            ena: (value & 0x10) != 0,
            hnna: (value & 0x20) != 0,
            nbna: (value & 0x40) != 0,
            ecna: (value & 0x80) != 0,
        }
    }
}

#[test]
fn test_access_restriction_mm() {
    let test_struct = AccessRestrictionMM {
        una: true,
        gena: true,
        gana: true,
        ina: true,
        ena: true,
        hnna: true,
        nbna: true,
        ecna: false,
    };
    assert_eq!(AccessRestrictionMM::from(0x7f), test_struct);
    assert_eq!(u8::from(test_struct), 0x7f);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct ExtendedAccessRestrictionMM {
    pub nruna: bool,   // NR-U in 5GS Not Allowed
    pub nrusrna: bool, // New Radio Unlicensed as Secondary RAT Not Allowed
    pub nrna: bool,    // NR in 5GS Not Allowed
    pub ussrna: bool, // Unlicensed Spectrum in the form of LAA or LWA/LWIP as Secondary RAT Not Allowed
    pub nrsrna: bool, // NR as Secondary RAT Not Allowed
}

impl From<ExtendedAccessRestrictionMM> for u8 {
    fn from(mode: ExtendedAccessRestrictionMM) -> u8 {
        let mut value: u8 = 0;
        value += if mode.nrsrna { 0x01 } else { 0 };
        value += if mode.ussrna { 0x02 } else { 0 };
        value += if mode.nrna { 0x04 } else { 0 };
        value += if mode.nrusrna { 0x08 } else { 0 };
        value += if mode.nruna { 0x10 } else { 0 };
        value
    }
}

impl From<u8> for ExtendedAccessRestrictionMM {
    fn from(value: u8) -> ExtendedAccessRestrictionMM {
        ExtendedAccessRestrictionMM {
            nrsrna: (value & 0x01) != 0,
            ussrna: (value & 0x02) != 0,
            nrna: (value & 0x04) != 0,
            nrusrna: (value & 0x08) != 0,
            nruna: (value & 0x10) != 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct AmbrMM {
    pub uplink: u32,
    pub downlink: u32,
}

impl IEs for AmbrMM {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.uplink.to_be_bytes());
        buffer.extend_from_slice(&self.downlink.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 8 {
            let data = AmbrMM {
                uplink: u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
                downlink: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.uplink == 0 && self.downlink == 0
    }

    fn len(&self) -> usize {
        8
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct OldEpsSecurityContext {
    pub old_ksi: u8,
    pub old_ncc: Option<u8>,
    pub old_kasme: [u8; 32],
    pub old_next_hop: Option<[u8; 32]>,
}

impl IEs for OldEpsSecurityContext {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        {
            let mut byte: u8 = if self.old_next_hop.is_some() {
                0x80
            } else {
                0x00
            };
            byte |= (self.old_ksi & 0x07) << 3;
            if let Some(ncc) = self.old_ncc {
                byte |= ncc & 0x07;
            }
            buffer.push(byte);
        }
        buffer.extend_from_slice(&self.old_kasme[..]);
        if let Some(next_hop) = self.old_next_hop {
            buffer.extend_from_slice(&next_hop[..]);
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        match buffer[0] >> 7 {
            0x00 => {
                if buffer.len() >= 33 {
                    let data = OldEpsSecurityContext {
                        old_ksi: (buffer[0] & 0x38) >> 3,
                        old_ncc: None,
                        old_kasme: buffer[1..33].try_into().unwrap(),
                        old_next_hop: None,
                    };
                    Ok(data)
                } else {
                    Err(GTPV2Error::IEIncorrect(0))
                }
            }
            0x01 => {
                if buffer.len() >= 65 {
                    let data = OldEpsSecurityContext {
                        old_ksi: (buffer[0] & 0x38) >> 3,
                        old_ncc: Some(buffer[0] & 0x07),
                        old_kasme: buffer[1..33].try_into().unwrap(),
                        old_next_hop: Some(buffer[33..65].try_into().unwrap()),
                    };
                    Ok(data)
                } else {
                    Err(GTPV2Error::IEIncorrect(0))
                }
            }
            _ => Err(GTPV2Error::IEIncorrect(0)),
        }
    }

    fn is_empty(&self) -> bool {
        self.old_ksi == 0 && self.old_kasme == [0; 32] && self.old_next_hop.is_none()
    }

    fn len(&self) -> usize {
        if self.old_next_hop.is_some() {
            65
        } else {
            33
        }
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

#[test]
fn test_old_eps_security_context_short_marshal() {
    let encoded_ie: [u8; 33] = [
        0x28, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
    ];
    let test_struct = OldEpsSecurityContext {
        old_ksi: 5,
        old_ncc: None,
        old_kasme: [0xff; 32],
        old_next_hop: None,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_old_eps_security_context_short_unmarshal() {
    let encoded_ie: [u8; 33] = [
        0x28, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
    ];
    let test_struct = OldEpsSecurityContext {
        old_ksi: 5,
        old_ncc: None,
        old_kasme: [0xff; 32],
        old_next_hop: None,
    };
    assert_eq!(
        OldEpsSecurityContext::unmarshal(&encoded_ie).unwrap(),
        test_struct
    );
}

#[test]
fn test_old_eps_security_context_long_marshal() {
    let encoded_ie: [u8; 65] = [
        0xad, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff,
    ];
    let test_struct = OldEpsSecurityContext {
        old_ksi: 5,
        old_ncc: Some(5),
        old_kasme: [0xff; 32],
        old_next_hop: Some([0xff; 32]),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_old_eps_security_context_long_unmarshal() {
    let encoded_ie: [u8; 65] = [
        0xad, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff,
    ];
    let test_struct = OldEpsSecurityContext {
        old_ksi: 5,
        old_ncc: Some(5),
        old_kasme: [0xff; 32],
        old_next_hop: Some([0xff; 32]),
    };
    assert_eq!(
        OldEpsSecurityContext::unmarshal(&encoded_ie).unwrap(),
        test_struct
    );
}
