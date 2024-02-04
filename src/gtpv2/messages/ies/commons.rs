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
}

// Location Field definitions

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Cgi {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub ci: u16,
}

impl IEs for Cgi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.extend_from_slice(&self.ci.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 7 {
            let mut data = Cgi::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Sai {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub sac: u16,
}

impl IEs for Sai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.extend_from_slice(&self.sac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 7 {
            let mut data = Sai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Rai {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub rac: u8,
}

impl IEs for Rai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
        buffer.push(0xff);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 6 {
            let mut data = Rai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Tai {
    pub mcc: u16,
    pub mnc: u16,
    pub tac: u16,
}

impl IEs for Tai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.tac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 5 {
            let mut data = Tai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Ecgi {
    pub mcc: u16,
    pub mnc: u16,
    pub eci: u32,
}

impl IEs for Ecgi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.eci.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 7 {
            let mut data = Ecgi::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Lai {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
}

impl IEs for Lai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 5 {
            let mut data = Lai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct MacroEnbId {
    pub mcc: u16,
    pub mnc: u16,
    pub macro_id: u32,
}

impl IEs for MacroEnbId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.macro_id.to_be_bytes()[1..]);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 6 {
            let mut data = MacroEnbId::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct ExtMacroEnbId {
    pub mcc: u16,
    pub mnc: u16,
    pub smenb: bool,
    pub ext_macro_id: u32,
}

impl IEs for ExtMacroEnbId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
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
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
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

}

#[test]
fn test_auth_triplet_marshal() {
    let encoded_ie: [u8;28]= [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                              0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                              0x11, 0x12, 0x13, 0x14,
                              0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c];
    let test_struct = AuthTriplet {
        rand: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
               0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
        sres: [0x11, 0x12, 0x13, 0x14],
        kc: [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_auth_triplet_unmarshal() {
    let encoded_ie: [u8;28]= [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                              0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                              0x11, 0x12, 0x13, 0x14,
                              0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c];
    let test_struct = AuthTriplet {
        rand: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
               0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
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
            let mut cursor:usize = 16;
            let mut data = AuthQuintuplet::default();
            data.rand.copy_from_slice(&buffer[..cursor]);
            let xres_len = buffer[cursor] as usize;
            cursor += 1;
            if buffer.len() >= cursor+xres_len {
                data.xres.extend_from_slice(&buffer[cursor..cursor+xres_len]);
                cursor += xres_len;
                if buffer.len() >= cursor+32 {
                    data.ck.copy_from_slice(&buffer[cursor..cursor+16]);
                    cursor += 16;
                    data.ik.copy_from_slice(&buffer[cursor..cursor+16]);
                    cursor += 16;
                    let autn_len = buffer[cursor] as usize;
                    cursor += 1;
                    if buffer.len() >= cursor+autn_len {
                            data.autn.extend_from_slice(&buffer[cursor..cursor+autn_len]);
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
        self.rand == [0; 16] && self.xres.len() == 0 && self.ck == [0; 16] && self.ik == [0; 16] && self.autn.len() == 0
    }

    fn len(&self) -> usize {
        50+self.xres.len()+self.autn.len()
    }

}

#[test]
fn test_auth_quintuplet_marshal() {
    let encoded_ie: [u8;56]= [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                   0x03, 0x02, 0x07, 0x08,
                                   0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                   0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                   0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                   0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                   0x03, 0x03, 0x09, 0x0a];
                                  
    let test_struct = AuthQuintuplet {
        rand: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
        xres: vec![0x02, 0x07, 0x08],
        ck: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
             0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
        ik: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
        autn: vec![0x03, 0x09, 0x0a],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_auth_quintuplet_unmarshal () {
    let encoded_ie: [u8;56]= [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                0x03, 0x02, 0x07, 0x08,
                                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                0x03, 0x03, 0x09, 0x0a];
   
    let test_struct = AuthQuintuplet {
    rand: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
    xres: vec![0x02, 0x07, 0x08],
    ck: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
    ik: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
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
            let mut cursor:usize = 0;
            data.rand.copy_from_slice(&buffer[..cursor+16]);
            cursor += 16;
            let xres_len = buffer[cursor] as usize;
            cursor += 1;
            if buffer.len() >= cursor+xres_len {
                data.xres.extend_from_slice(&buffer[cursor..cursor+xres_len]);
                cursor += xres_len;
                let autn_len = buffer[cursor] as usize;
                cursor += 1;
                if buffer.len() >= cursor+autn_len {
                    data.autn.extend_from_slice(&buffer[cursor..cursor+autn_len]);
                    cursor += autn_len;
                    if buffer.len() >= cursor+32 {
                        data.kasme.copy_from_slice(&buffer[cursor..cursor+32]);
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
        self.rand == [0; 16] && self.xres.len() == 0 && self.autn.len() == 0 && self.kasme == [0; 32]
    }

    fn len(&self) -> usize {
        50+self.xres.len()+self.autn.len()
    }

}

#[test]
fn test_auth_quadruplet_marshal() {
    let encoded_ie: [u8;56]= [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                   0x03, 0x02, 0x07, 0x08,
                                   0x03, 0x03, 0x09, 0x0a,
                                   0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let test_struct = AuthQuadruplet {
        rand: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
        xres: vec![0x02, 0x07, 0x08],
        autn: vec![0x03, 0x09, 0x0a],
        kasme: [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_auth_quadruplet_unmarshal() {
    let encoded_ie: [u8;56]= [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                                   0x03, 0x02, 0x07, 0x08,
                                   0x03, 0x03, 0x09, 0x0a,
                                   0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let test_struct = AuthQuadruplet {
        rand: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10],
        xres: vec![0x02, 0x07, 0x08],
        autn: vec![0x03, 0x09, 0x0a],
        kasme: [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
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
    pub apn_rate_control_status_validity: [u8;8],
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
            let length = u16::from_be_bytes([buffer[0],buffer[1]]) as usize;
            let mut cursor:usize = 2;
            if buffer.len() >= length+cursor {
                let apn_length:usize = u16::from_be_bytes([buffer[cursor],buffer[cursor+1]]) as usize;
                cursor += 2;
                if buffer.len() >= apn_length+cursor {
                    data.apn = String::from_utf8_lossy(&buffer[cursor..cursor+apn_length]).to_string();
                    cursor += apn_length;
                    if buffer.len() >= cursor+20 {
                        data.uplink_rate_limit = u32::from_be_bytes([buffer[cursor], buffer[cursor+1], buffer[cursor+2], buffer[cursor+3]]);
                        cursor += 4;
                        data.nbr_of_exception_reports = u32::from_be_bytes([buffer[cursor], buffer[cursor+1], buffer[cursor+2], buffer[cursor+3]]);
                        cursor += 4;
                        data.downlink_rate_limit = u32::from_be_bytes([buffer[cursor], buffer[cursor+1], buffer[cursor+2], buffer[cursor+3]]);
                        cursor += 4;
                        data.apn_rate_control_status_validity.copy_from_slice(&buffer[cursor..cursor+8]);
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
        self.apn.is_empty() && self.uplink_rate_limit == 0 && self.nbr_of_exception_reports == 0 && self.downlink_rate_limit == 0 && self.apn_rate_control_status_validity == [0;8]
    }

    fn len(&self) -> usize {
        33+self.apn.len()
    }
}

#[test]

fn test_apn_rate_control_status_marshal() {
    let encoded_ie :[u8;27]= [ 0x00, 0x19, 0x00, 0x03, 0x61, 0x70, 0x6E, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let test_struct = ApnRateControlStatusMM {
        apn: "apn".to_string(),
        uplink_rate_limit: 0x12345678,
        nbr_of_exception_reports: 0x12345678,
        downlink_rate_limit: 0x12345678,
        apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06,0x07,0x08],
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

#[test]
fn test_apn_rate_control_status_unmarshal () {
    let encoded_ie :[u8;27]= [ 0x00, 0x19, 0x00, 0x03, 0x61, 0x70, 0x6E, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let test_struct = ApnRateControlStatusMM {
        apn: "apn".to_string(),
        uplink_rate_limit: 0x12345678,
        nbr_of_exception_reports: 0x12345678,
        downlink_rate_limit: 0x12345678,
        apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06,0x07,0x08],
    };
    assert_eq!(ApnRateControlStatusMM::unmarshal(&encoded_ie).unwrap(), test_struct);
}

pub enum SecurityMode {
    GsmKeyAndTriplets,
    UmtsKeyUsedCipherAndQuintuplets,
    GsmKeyUsedCipherAndQuintuplets,
    UmtsKeyAndQuintuplets,
    EpsSecurityContextAndQuadruplets,
    UmtsKeyQuadrupletsAndQuintuplets,
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
        }
    }
}

pub enum NasCipherValues {
    NoChiper,
    Eea1,
    Eea2,
    Eea3,
    Eea4,
    Eea5,
    Eea6,
    Eea7,
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
        }
    }
}

pub enum NasIntegrityProtectionValues {
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

pub enum CipherValues {
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

pub enum GprsIntegrityProtectionValues {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct AccessRestrictionMM {
    pub una: bool,      // UTRAN Not Allowed
    pub gena: bool,     // GERAN Not Allowed
    pub gana: bool,     // GAN Not Allowed
    pub ina: bool,      // I-HSPA-Evolution Not Allowed
    pub ena: bool,      // WB-E-UTRAN Not Allowed
    pub hnna: bool,     // HO-To-Non-3GPP Not Allowed
    pub nbna: bool,     // NB-IoT Not Allowed
    pub ecna: bool,     // Enhanced Coverage Not Allowed 
}

impl From<AccessRestrictionMM> for u8 {
    fn from(mode: AccessRestrictionMM) -> u8 {
        let mut value:u8 = 0;
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
        ecna: true,
    };
    assert_eq!(AccessRestrictionMM::from(0xff),test_struct);
    assert_eq!(u8::from(test_struct),0xff);
}