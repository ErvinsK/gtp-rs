// User CSG Information (UCI) IE - according to 3GPP TS 29.247 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// User CSG Information (UCI) IE TL

pub const UCI: u8 = 145;
pub const UCI_LENGTH: usize = 8;

// Access mode enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    ClosedMode,
    HybridMode,
    Reserved,
}

impl From<&AccessMode> for u8 {
    fn from(i: &AccessMode) -> u8 {
        match i {
            AccessMode::ClosedMode => 0,
            AccessMode::HybridMode => 1,
            AccessMode::Reserved => 2,
        }
    }
}

impl From<u8> for AccessMode {
    fn from(i: u8) -> AccessMode {
        match i {
            0 => AccessMode::ClosedMode,
            1 => AccessMode::HybridMode,
            2 | 3 => AccessMode::Reserved,
            _ => AccessMode::Reserved,
        }
    }
}

// CSG Membership Indication (CMI) enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmi {
    CsgMembership,
    NonCsgMembership,
}

impl From<&Cmi> for u8 {
    fn from(i: &Cmi) -> u8 {
        match i {
            Cmi::NonCsgMembership => 0,
            Cmi::CsgMembership => 1,
        }
    }
}

impl From<u8> for Cmi {
    fn from(i: u8) -> Cmi {
        match i {
            0 => Cmi::NonCsgMembership,
            1 => Cmi::CsgMembership,
            _ => Cmi::NonCsgMembership,
        }
    }
}

// User CSG Information (UCI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uci {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub csgid: u32,
    pub access_mode: AccessMode,
    pub lcsg: bool, // Leave CSG flag
    pub cmi: Cmi,   // CSG Membership Indication
}

impl Default for Uci {
    fn default() -> Self {
        Uci {
            t: UCI,
            length: UCI_LENGTH as u16,
            ins: 0,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            csgid: 0,
            access_mode: AccessMode::ClosedMode,
            lcsg: false,
            cmi: Cmi::CsgMembership,
        }
    }
}

impl From<Uci> for InformationElement {
    fn from(i: Uci) -> Self {
        InformationElement::Uci(i)
    }
}

impl IEs for Uci {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(UCI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer_ie.extend_from_slice(&(self.csgid & 0x07ffffff).to_be_bytes());

        match self.lcsg {
            true => {
                let i = (u8::from(&self.access_mode) << 6) | 0x02 | u8::from(&self.cmi);
                buffer_ie.push(i);
            }
            false => {
                let i = (u8::from(&self.access_mode) << 6) | u8::from(&self.cmi);
                buffer_ie.push(i);
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= UCI_LENGTH + MIN_IE_SIZE {
            let mut data = Uci {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                csgid: u32::from_be_bytes([buffer[7], buffer[8], buffer[9], buffer[10]]),
                ..Uci::default()
            };
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..=6]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            data.access_mode = ((buffer[11] & 0xc0) >> 6).into();
            match (buffer[11] & 0x02) >> 1 {
                0 => data.lcsg = false,
                _ => data.lcsg = true,
            }
            data.cmi = (buffer[11] & 0x01).into();
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(UCI))
        }
    }

    fn len(&self) -> usize {
        UCI_LENGTH + MIN_IE_SIZE
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
fn uci_ie_marshal_test() {
    let decoded = Uci {
        t: UCI,
        length: UCI_LENGTH as u16,
        ins: 0,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        csgid: 48190,
        access_mode: AccessMode::ClosedMode,
        lcsg: false,
        cmi: Cmi::CsgMembership,
    };
    let encoded: [u8; 12] = [
        0x91, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0x00, 0x00, 0xbc, 0x3e, 0x01,
    ];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn uci_ie_unmarshal_test() {
    let decoded = Uci {
        t: UCI,
        length: UCI_LENGTH as u16,
        ins: 0,
        mcc: 262,
        mnc: 3,
        mnc_is_three_digits: false,
        csgid: 48190,
        access_mode: AccessMode::ClosedMode,
        lcsg: false,
        cmi: Cmi::CsgMembership,
    };
    let encoded: [u8; 12] = [
        0x91, 0x00, 0x08, 0x00, 0x62, 0xf2, 0x30, 0x00, 0x00, 0xbc, 0x3e, 0x01,
    ];
    assert_eq!(Uci::unmarshal(&encoded).unwrap(), decoded);
}
