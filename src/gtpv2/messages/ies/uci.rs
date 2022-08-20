// User CSG Information (UCI) IE - according to 3GPP TS 29.247 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// User CSG Information (UCI) IE TL

pub const UCI:u8 = 145;
pub const UCI_LENGTH:usize = 8;

// Access mode enum

#[derive(Debug, Clone, PartialEq)]
pub enum AccessMode {
    ClosedMode,
    HybridMode,
    Reserved,
}

impl AccessMode {
    pub fn enum_to_value (i:&AccessMode) -> u8 {
        match i {
            AccessMode::ClosedMode => 0,
            AccessMode::HybridMode => 1,
            AccessMode::Reserved => 2,
        }
    }
    pub fn value_to_enum (i:u8) -> Result<AccessMode, GTPV2Error> {
        match i {
            0 => Ok(AccessMode::ClosedMode),
            1 => Ok(AccessMode::HybridMode),
            2 | 3 => Ok(AccessMode::Reserved),
            _ => Err(GTPV2Error::IEIncorrect),
        }
    }
} 

// CSG Membership Indication (CMI) enum

#[derive(Debug, Clone, PartialEq)]
pub enum Cmi {
    CsgMembership,
    NonCsgMembership,
}

impl Cmi {
    fn enum_to_value (i:&Cmi) -> u8 {
        match i {
            Cmi::NonCsgMembership => 0,
            Cmi::CsgMembership => 1,
        }
    }
    fn value_to_enum (i:u8) -> Result<Cmi, GTPV2Error> {
        match i {
            0 => Ok(Cmi::NonCsgMembership),
            1 => Ok(Cmi::CsgMembership),
            _ => Err(GTPV2Error::IEIncorrect),
        }
    }
} 

// User CSG Information (UCI) IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Uci {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub csgid: u32,
    pub access_mode: AccessMode,
    pub lcsg: bool,                 // Leave CSG flag
    pub cmi: Cmi,                   // CSG Membership Indication

}

impl Default for Uci {
    fn default() -> Self {
        Uci { t: UCI, length: UCI_LENGTH as u16, ins:0, mcc: 0, mnc: 0, csgid: 0, access_mode: AccessMode::ClosedMode, lcsg: false, cmi: Cmi::CsgMembership }
    }
}

impl IEs for Uci {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer_ie.extend_from_slice(&(self.csgid & 0x07ffffff).to_be_bytes());
        
        match self.lcsg {
            true => {
                let i = (AccessMode::enum_to_value(&self.access_mode) << 6) | 0x02 | Cmi::enum_to_value(&self.cmi);
                buffer_ie.push(i);
            },
            false => {
                let i = (AccessMode::enum_to_value(&self.access_mode) << 6) | Cmi::enum_to_value(&self.cmi);
                buffer_ie.push(i);
            },
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=UCI_LENGTH+MIN_IE_SIZE {
            let mut data=Uci::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..=6]);
            data.csgid=u32::from_be_bytes([buffer[7],buffer[8],buffer[9],buffer[10]]);
            match AccessMode::value_to_enum((buffer[11] & 0xc0) >> 6) {
                Ok(i) => data.access_mode = i,
                Err(j) => return Err(j),
            }
            match (buffer[11] & 0x02) >> 1 {
                0 => data.lcsg = false,
                _ => data.lcsg = true,
            }
            match Cmi::value_to_enum(buffer[11] & 0x01) {
                Ok(i) => data.cmi = i,
                Err(j) => return Err(j),
            }
            Ok (data)
        } else {
            Err(GTPV2Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
        UCI_LENGTH+MIN_IE_SIZE
    }

}

#[test]
fn uci_ie_marshal_test() {
    let decoded = Uci { t:UCI, length: UCI_LENGTH as u16, ins: 0, mcc:262, mnc:3, csgid:48190, access_mode:AccessMode::ClosedMode, lcsg: false, cmi:Cmi::CsgMembership};
    let encoded:[u8;12] = [0x91, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0x00, 0x00, 0xbc, 0x3e, 0x01];
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn uci_ie_unmarshal_test() {
    let decoded = Uci { t:UCI, length: UCI_LENGTH as u16, ins: 0, mcc:262, mnc:3, csgid:48190, access_mode:AccessMode::ClosedMode, lcsg: false, cmi:Cmi::CsgMembership};
    let encoded:[u8;12] = [0x91, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0x00, 0x00, 0xbc, 0x3e, 0x01];
    assert_eq!(Uci::unmarshal(&encoded).unwrap(), decoded);
}