// User CSG Information (UCI) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{utils::*, errors::GTPV1Error, gtpc::messages::ies::commons::*};

// User CSG Information (UCI) IE TVL

pub const UCI:u8 = 194;
pub const UCI_LENGTH:u16 = 8;

// Access mode enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    ClosedMode,
    HybridMode,
    Reserved,
}

// CSG Membership Indication (CMI) enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmi {
    CsgMembership,
    NonCsgMembership,
}

// User CSG Information (UCI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uci {
    pub t: u8,
    pub length: u16,
    pub mcc: u16,
    pub mnc: u16,
    pub csgid: u32,
    pub access_mode: AccessMode,
    pub cmi: Cmi,

}

impl Default for Uci {
    fn default() -> Self {
        Uci { t: UCI, length: UCI_LENGTH, mcc: 0, mnc: 0, csgid: 0, access_mode: AccessMode::ClosedMode, cmi: Cmi::CsgMembership }
    }
}

impl IEs for Uci {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer_ie.extend_from_slice(&(self.csgid & 0x07ffffff).to_be_bytes());
        match (&self.access_mode, &self.cmi) {
            (AccessMode::ClosedMode, Cmi::CsgMembership) => buffer_ie.push(0x00),
            (AccessMode::ClosedMode, Cmi::NonCsgMembership) => buffer_ie.push(0x01),
            (AccessMode::HybridMode, Cmi::CsgMembership) => buffer_ie.push(0x40),
            (AccessMode::HybridMode, Cmi::NonCsgMembership) => buffer_ie.push(0x41),
            (AccessMode::Reserved, Cmi::CsgMembership) => buffer_ie.push(0x80),
            (AccessMode::Reserved, Cmi::NonCsgMembership) => buffer_ie.push(0x81), 
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=(UCI_LENGTH+3) as usize {
            let mut data=Uci::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[3..=5]);
            data.csgid=u32::from_be_bytes([buffer[6],buffer[7],buffer[8],buffer[9]]);
            match (buffer[10] & 0xc0) >> 6 {
                0 => data.access_mode = AccessMode::ClosedMode,
                1 => data.access_mode = AccessMode::HybridMode,
                _ => data.access_mode = AccessMode::Reserved,
            }
            match buffer[10] & 0x01 {
                0 => data.cmi = Cmi::CsgMembership,
                1 => data.cmi = Cmi::NonCsgMembership,
                _ => (),
            }                   
            Ok (data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
        (UCI_LENGTH+3) as usize
    }

}

#[test]
fn uci_ie_marshal_test() {
    let ie_to_marshal = Uci { t:UCI, length: UCI_LENGTH, mcc:262, mnc:3, csgid:48190, access_mode:AccessMode::ClosedMode, cmi:Cmi::NonCsgMembership};
    let ie_unmarshalled:[u8;11] = [0xC2, 0x00, 0x08, 0x62, 0xf2, 0x30, 0x00, 0x00, 0xbc, 0x3e, 0x01];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_unmarshalled);
}

#[test]
fn uci_ie_unmarshal_test() {
    let ie_to_marshal = Uci { t:UCI, length: UCI_LENGTH, mcc:262, mnc:3, csgid:48190, access_mode:AccessMode::HybridMode, cmi:Cmi::NonCsgMembership};
    let ie_unmarshalled:[u8;11] = [0xC2, 0x00, 0x08, 0x62, 0xf2, 0x30, 0x00, 0x00, 0xbc, 0x3e, 0x41];
    assert_eq!(Uci::unmarshal(&ie_unmarshalled).unwrap(), ie_to_marshal);
}