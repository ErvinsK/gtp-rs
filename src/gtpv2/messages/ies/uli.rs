// User Location Information (ULI) IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// User Location Information (ULI) IE Type

pub const ULI:u8 = 86;

// Location Field definitions

trait Li {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized;
}

#[derive(Debug,Clone,PartialEq)]
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
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
        if buffer.len()>=7 {
            let mut data=Rai::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.lac = u16::from_be_bytes([buffer[3],buffer[4]]);
            data.rac = buffer[5];
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
        if buffer.len()>=7 {
            let mut data=MacroEnbId::default();
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[..=2]);
            data.macro_id = u32::from_be_bytes([0x00,buffer[3],buffer[4], buffer[5]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect)
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
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
            let i = self.ext_macro_id.to_be_bytes();
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
            match buffer[3] & 0x80 {
                0 => {
                    data.smenb = false;
                    data.ext_macro_id = u32::from_be_bytes([0x00,buffer[3],buffer[4],buffer[5]]);
                },
                1 => {
                    data.smenb = true;
                    data.ext_macro_id = u32::from_be_bytes([0x00,(buffer[3] & 0x03),buffer[4],buffer[5]]);
                },
            }
            Ok (data)
        } else {
            Err(GTPV2Error::IEIncorrect)
        }
    }
}


// CGI, SAI, RAI, TAI, ECGI, LAI, Macro eNB ID, Extended Macro eNB ID

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Uli {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub loc: Vec<Location>,
}

impl Default for Uli {
    fn default() -> Self {
        Uli { t: ULI, length: 0, ins:0, loc: vec!() }
    }
}

impl IEs for Uli {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let mut flags:u8 = 0;
        let mut buffer_li:Vec<u8> = vec!();
        for i in self.loc.iter() {
            match i {
                Location::Cgi(j) => {
                    if flags & 0x01 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x01;
                    }
                },
                Location::Sai(j) => {
                    if flags & 0x02 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x02;
                    }
                },
                Location::Rai(j) => {
                    if flags & 0x04 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x04;
                    }
                },
                Location::Tai(j) => {
                    if flags & 0x08 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x08;
                    }
                },
                Location::Ecgi(j) => {
                    if flags & 0x10 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x10;
                    }
                },
                Location::Lai(j) => {
                    if flags & 0x20 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x20;
                    }
                },
                Location::MacroEnbId(j) => {
                    if flags & 0x40 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x40;
                    }
                },
                Location::ExtMacroEnbId(j) => {
                    if flags & 0x80 == 0 {
                        j.marshal(&mut buffer_li);
                        flags = flags | 0x80;
                    }
                },
            }
        }
        buffer_ie.push(flags);
        buffer_ie.append(&mut buffer_li);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=(ULI_LENGTH+3) as usize {
            let mut data=Uli::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            match buffer[3] {
                0 => {
                    (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..=6]);
                    data.lac=u16::from_be_bytes([buffer[7],buffer[8]]);
                    data.loc=Location::Ci(u16::from_be_bytes([buffer[9], buffer[10]]));
                },
                1 => {
                    (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..=6]);
                    data.lac=u16::from_be_bytes([buffer[7],buffer[8]]);
                    data.loc=Location::Sac(u16::from_be_bytes([buffer[9], buffer[10]]));
                },
                2 => {
                    (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..=6]);
                    data.lac=u16::from_be_bytes([buffer[7],buffer[8]]);
                    data.loc=Location::Rac(buffer[9]);
                },
                _ => {
                    return Err(GTPV1Error::IEIncorrect);
                }
            }
            Ok (data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
        (ULI_LENGTH+3) as usize
    }

}

#[test]
fn uli_ie_marshal_test_cgi() {
    let ie_to_marshal = Uli { t:ULI, length: ULI_LENGTH, mcc:262, mnc:3, lac:48190, loc: Location::Ci(14076)};
    let ie_unmarshalled:[u8;11] = [0x98, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_unmarshalled);
}

#[test]
fn uli_ie_unmarshal_test_cgi() {
    let ie_to_marshal = Uli { t:ULI, length: ULI_LENGTH, mcc:262, mnc:3, lac:48190, loc: Location::Ci(14076)};
    let ie_unmarshalled:[u8;11] = [0x98, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc];
    assert_eq!(Uli::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}

#[test]
fn uli_ie_marshal_test_sai() {
    let ie_to_marshal = Uli { t:ULI, length: ULI_LENGTH, mcc:262, mnc:3, lac:48190, loc: Location::Sac(14076)};
    let ie_unmarshalled:[u8;11] = [0x98, 0x00, 0x08, 0x01, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_unmarshalled);
}

#[test]
fn uli_ie_unmarshal_test_sai() {
    let ie_to_marshal = Uli { t:ULI, length: ULI_LENGTH, mcc:262, mnc:3, lac:48190, loc: Location::Sac(14076)};
    let ie_unmarshalled:[u8;11] = [0x98, 0x00, 0x08, 0x01, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x36, 0xfc];
    assert_eq!(Uli::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}

#[test]
fn uli_ie_marshal_test_rai() {
    let ie_to_marshal = Uli { t:ULI, length: ULI_LENGTH, mcc:262, mnc:3, lac:48190, loc: Location::Rac(0x10)};
    let ie_unmarshalled:[u8;11] = [0x98, 0x00, 0x08, 0x02, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x10, 0xff];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_unmarshalled);
}

#[test]
fn uli_ie_unmarshal_test_rai() {
    let ie_to_marshal = Uli { t:ULI, length: ULI_LENGTH, mcc:262, mnc:3, lac:48190, loc: Location::Rac(0x10)};
    let ie_unmarshalled:[u8;11] = [0x98, 0x00, 0x08, 0x02, 0x62, 0xf2, 0x30, 0xbc, 0x3e, 0x10, 0xff];
    assert_eq!(Uli::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}

