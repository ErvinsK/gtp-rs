// Commons for GTP-C IEs

use crate::gtpv2::{errors::GTPV2Error, utils::*};

pub const MIN_IE_SIZE:usize = 4;

pub trait IEs {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized;
    fn len (&self) -> usize; // Total IE length = Type+Length+Instance+Value for TLIV messages
}

// Location Field definitions

pub trait Li {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized;
}

#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct Cgi {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub ci: u16, 
}

impl Default for Cgi {
    fn default() -> Self {
        Cgi { mcc: 0, mnc: 0, lac:0, ci:0}
    }
}

impl Li for Cgi {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.extend_from_slice(&self.ci.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=7 {
            let mut data=Cgi::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.lac = u16::from_be_bytes([buffer[3],buffer[4]]);
            data.ci = u16::from_be_bytes([buffer[5],buffer[6]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Sai {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub sac:u16,
} 

impl Default for Sai {
    fn default() -> Self {
        Sai { mcc: 0, mnc: 0, lac:0, sac:0}
    }
}

impl Li for Sai {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.extend_from_slice(&self.sac.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=7 {
            let mut data=Sai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.lac = u16::from_be_bytes([buffer[3],buffer[4]]);
            data.sac = u16::from_be_bytes([buffer[5],buffer[6]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Rai {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub rac: u8,
}

impl Default for Rai {
    fn default() -> Self {
        Rai { mcc: 0, mnc: 0, lac:0, rac:0}
    }
}

impl Li for Rai {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
        buffer.push(0xff);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=6 {
            let mut data=Rai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.lac = u16::from_be_bytes([buffer[3],buffer[4]]);
            data.rac = buffer[5];
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Tai {
    pub mcc: u16,
    pub mnc: u16,
    pub tac: u16,
}

impl Default for Tai {
    fn default() -> Self {
        Tai { mcc: 0, mnc: 0, tac:0}
    }
}

impl Li for Tai {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.tac.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=5 {
            let mut data=Tai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.tac = u16::from_be_bytes([buffer[3],buffer[4]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Ecgi {
    pub mcc: u16,
    pub mnc: u16,
    pub eci: u32,
}

impl Default for Ecgi {
    fn default() -> Self {
        Ecgi { mcc: 0, mnc: 0, eci:0}
    }
}

impl Li for Ecgi {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.eci.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=7 {
            let mut data=Ecgi::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.eci = u32::from_be_bytes([buffer[3],buffer[4], buffer[5],buffer[6]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct Lai {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
}

impl Default for Lai {
    fn default() -> Self {
        Lai { mcc: 0, mnc: 0, lac:0}
    }
}

impl Li for Lai {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=5 {
            let mut data=Lai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.lac = u16::from_be_bytes([buffer[3],buffer[4]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct MacroEnbId {
    pub mcc: u16,
    pub mnc: u16,
    pub macro_id: u32,
}

impl Default for MacroEnbId {
    fn default() -> Self {
        MacroEnbId { mcc: 0, mnc: 0, macro_id:0}
    }
}

impl Li for MacroEnbId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer.extend_from_slice(&self.macro_id.to_be_bytes()[1..]);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=6 {
            let mut data=MacroEnbId::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.macro_id = u32::from_be_bytes([0x00,buffer[3],buffer[4], buffer[5]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub struct ExtMacroEnbId {
    pub mcc: u16,
    pub mnc: u16,
    pub smenb: bool,
    pub ext_macro_id: u32,
}

impl Default for ExtMacroEnbId {
    fn default() -> Self {
        ExtMacroEnbId { mcc: 0, mnc: 0, smenb:false, ext_macro_id:0}
    }
}

impl Li for ExtMacroEnbId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        if self.smenb {
            let mut i = self.ext_macro_id.to_be_bytes();
            i[1]=(i[1] | 0x80) & 0x83;
            buffer.extend_from_slice(&i[1..]);
        } else {
            let i = self.ext_macro_id.to_be_bytes();
            buffer.extend_from_slice(&i[1..]);
        }   
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=6 {
            let mut data=ExtMacroEnbId::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            match buffer[3] >> 7 {
                0 => {
                    data.smenb = false;
                    data.ext_macro_id = u32::from_be_bytes([0x00,buffer[3],buffer[4],buffer[5]]);
                },
                1 => {
                    data.smenb = true;
                    data.ext_macro_id = u32::from_be_bytes([0x00,(buffer[3] & 0x03),buffer[4],buffer[5]]);
                },
                _ => (),
            }
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect(0))
        }
    }
}