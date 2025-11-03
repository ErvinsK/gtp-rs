// Target Identification IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Target Identification IE Type

pub const TARGETID: u8 = 121;

// Targets

// RNC ID
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RncIdentifier {
    pub rai: Rai,
    pub rnc_id: u16,
    pub ext_rnc_id: Option<u16>,
}

// Implementation of RNC ID IE

impl IEs for RncIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut rai_buffer: Vec<u8> = vec![];
        self.rai.marshal(&mut rai_buffer);
        buffer.extend_from_slice(&rai_buffer[..rai_buffer.len() - 1]);
        buffer.extend_from_slice(&self.rnc_id.to_be_bytes());
        if let Some(ext_rnc_id) = self.ext_rnc_id {
            buffer.extend_from_slice(&ext_rnc_id.to_be_bytes());
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        match buffer.len() {
            i if i == 8 || (8..10).contains(&i) => {
                let rai = if let Ok(i) = Rai::unmarshal(&buffer[0..6]) {
                    i
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                };
                let data = RncIdentifier {
                    rai,
                    rnc_id: u16::from_be_bytes([buffer[6], buffer[7]]),
                    ext_rnc_id: None,
                };
                Ok(data)
            }
            j if j >= 10 => {
                let rai = if let Ok(i) = Rai::unmarshal(&buffer[0..6]) {
                    i
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                };
                let data = RncIdentifier {
                    rai,
                    rnc_id: u16::from_be_bytes([buffer[6], buffer[7]]),
                    ext_rnc_id: Some(u16::from_be_bytes([buffer[8], buffer[9]])),
                };
                Ok(data)
            }
            _ => Err(GTPV2Error::IEInvalidLength(0)),
        }
    }

    fn len(&self) -> usize {
        match self.ext_rnc_id {
            Some(_) => 10,
            None => 8,
        }
    }

    fn is_empty(&self) -> bool {
        self.rai.is_empty() && self.rnc_id == 0 && self.ext_rnc_id.is_none()
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Macro eNB ID
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MacroEnbIdentifier {
    pub macro_enb_id: MacroEnbId,
    pub tac: u16,
}

impl IEs for MacroEnbIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.macro_enb_id.marshal(buffer);
        buffer.extend_from_slice(&self.tac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 8 {
            let macro_enb_id = if let Ok(i) = MacroEnbId::unmarshal(&buffer[0..6]) {
                i
            } else {
                return Err(GTPV2Error::IEIncorrect(0));
            };
            let data = MacroEnbIdentifier {
                macro_enb_id,
                tac: u16::from_be_bytes([buffer[6], buffer[7]]),
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        8
    }

    fn is_empty(&self) -> bool {
        self.macro_enb_id.is_empty() && self.tac == 0
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Extended Macro eNB ID
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExtendedMacroEnbIdentifier {
    pub ext_macro_enb_id: ExtMacroEnbId,
    pub tac: u16,
}

impl IEs for ExtendedMacroEnbIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.ext_macro_enb_id.marshal(buffer);
        buffer.extend_from_slice(&self.tac.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 8 {
            let data = ExtendedMacroEnbIdentifier {
                ext_macro_enb_id: if let Ok(i) = ExtMacroEnbId::unmarshal(&buffer[0..6]) {
                    i
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                },
                tac: u16::from_be_bytes([buffer[6], buffer[7]]),
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        8
    }

    fn is_empty(&self) -> bool {
        self.ext_macro_enb_id.is_empty() && self.tac == 0
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Cell Identifier
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CellIdentifier {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
    pub rac: u8,
    pub ci: u16,
}

impl IEs for CellIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
        buffer.extend_from_slice(&self.ci.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 8 {
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            let data = CellIdentifier {
                mcc,
                mnc,
                mnc_is_three_digits,
                lac: u16::from_be_bytes([buffer[3], buffer[4]]),
                rac: buffer[5],
                ci: u16::from_be_bytes([buffer[6], buffer[7]]),
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        8
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.lac == 0 && self.rac == 0 && self.ci == 0
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// gNodeB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GNbIdentifier {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub gnb_id_length: u8, // gNodeB ID length from 22 to 32 bits
    pub gnb_id: u32,       // gNodeB ID length from 22 to 32 bits
    pub etac: [u8; 3],     // 5GS Tracking Area Code (24 bits)
}

impl Default for GNbIdentifier {
    fn default() -> Self {
        GNbIdentifier {
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            gnb_id_length: 22,
            gnb_id: 0,
            etac: [0; 3],
        }
    }
}

impl IEs for GNbIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.push(self.gnb_id_length);
        buffer.extend_from_slice(&self.gnb_id.to_be_bytes());
        buffer.extend_from_slice(&self.etac);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 11 {
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            let data = GNbIdentifier {
                mcc,
                mnc,
                mnc_is_three_digits,
                gnb_id_length: buffer[3],
                gnb_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                etac: [buffer[8], buffer[9], buffer[10]],
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        11
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.gnb_id == 0 && self.etac == [0; 3]
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Macro NG-eNB ID
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MacronGeNbIdentifier {
    pub macro_ng_enb_id: MacroEnbId,
    pub etac: [u8; 3], // 5GS Tracking Area Code
}

impl IEs for MacronGeNbIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.macro_ng_enb_id.marshal(buffer);
        buffer.extend_from_slice(&self.etac);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 9 {
            let data = MacronGeNbIdentifier {
                macro_ng_enb_id: if let Ok(i) = MacroEnbId::unmarshal(&buffer[0..6]) {
                    i
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                },
                etac: [buffer[6], buffer[7], buffer[8]],
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        9
    }

    fn is_empty(&self) -> bool {
        self.macro_ng_enb_id.is_empty() && self.etac == [0; 3]
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Extended NG-eNB ID
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExtendedGeNbIdentifier {
    pub macro_ng_enb_id: ExtMacroEnbId,
    pub etac: [u8; 3], // 5GS Tracking Area Code
}

impl IEs for ExtendedGeNbIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.macro_ng_enb_id.marshal(buffer);
        buffer.extend_from_slice(&self.etac);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 9 {
            let data = ExtendedGeNbIdentifier {
                macro_ng_enb_id: if let Ok(i) = ExtMacroEnbId::unmarshal(&buffer[0..6]) {
                    i
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                },
                etac: [buffer[6], buffer[7], buffer[8]],
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        9
    }

    fn is_empty(&self) -> bool {
        self.macro_ng_enb_id.is_empty() && self.etac == [0; 3]
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// en-gNB ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnGNbIdentifier {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub en_gnb_id_length: u8, // gNodeB ID length from 22 to 32 bits
    pub en_gnb_id: u32,       // gNodeB ID length from 22 to 32 bits
    pub tac: Option<u16>,
    pub etac: Option<[u8; 3]>, // 5GS Tracking Area Code (24 bits)
}

impl Default for EnGNbIdentifier {
    fn default() -> Self {
        EnGNbIdentifier {
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            en_gnb_id_length: 22,
            en_gnb_id: 0,
            tac: None,
            etac: None,
        }
    }
}

impl IEs for EnGNbIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        match (self.tac.is_some(), self.etac.is_some()) {
            (true, true) => buffer.push(0xC0 | self.en_gnb_id_length),
            (true, false) => buffer.push(0x80 | self.en_gnb_id_length),
            (false, true) => buffer.push(0x40 | self.en_gnb_id_length),
            (false, false) => buffer.push(self.en_gnb_id_length),
        }
        buffer.extend_from_slice(&self.en_gnb_id.to_be_bytes());
        if let Some(tac) = self.tac {
            buffer.extend_from_slice(&tac.to_be_bytes());
        }
        if let Some(etac) = self.etac {
            buffer.extend_from_slice(&etac);
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 8 {
            let mut data = EnGNbIdentifier {
                en_gnb_id_length: buffer[3] & 0x3F,
                en_gnb_id: u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
                ..Default::default()
            };
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
            data.mcc = mcc;
            data.mnc = mnc;
            data.mnc_is_three_digits = mnc_is_three_digits;
            if buffer[3] & 0x40 != 0 && buffer.len() >= 10 {
                data.tac = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
            } else {
                return Err(GTPV2Error::IEInvalidLength(0));
            }
            if buffer[3] & 0x80 != 0 && buffer.len() >= 13 {
                data.etac = Some([buffer[10], buffer[11], buffer[12]]);
            } else {
                return Err(GTPV2Error::IEInvalidLength(0));
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(0))
        }
    }

    fn len(&self) -> usize {
        match (self.tac, self.etac) {
            (Some(_), Some(_)) => 14,
            (Some(_), None) => 11,
            _ => 8,
        }
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0
            && self.mnc == 0
            && self.en_gnb_id == 0
            && self.tac.is_none()
            && self.etac.is_none()
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}
// Target Type Enum

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TargetType {
    RncId(RncIdentifier),
    MacroeNbId(MacroEnbIdentifier),
    CellId(CellIdentifier),
    MacroEnbIdentifier(MacroEnbId),
    HomeeNbId(MacroEnbIdentifier),
    ExtendedMacroeNbId(ExtendedMacroEnbIdentifier),
    GNbId(GNbIdentifier),
    MacrongeNbId(MacronGeNbIdentifier),
    ExtendedngeNbId(ExtendedGeNbIdentifier),
    EngNbId(EnGNbIdentifier),
    #[default]
    Spare,
}

impl From<&TargetType> for u8 {
    fn from(i: &TargetType) -> u8 {
        match i {
            TargetType::RncId(_) => 0,
            TargetType::MacroeNbId(_) => 1,
            TargetType::CellId(_) => 2,
            TargetType::HomeeNbId(_) => 3,
            TargetType::ExtendedMacroeNbId(_) => 4,
            TargetType::GNbId(_) => 5,
            TargetType::MacrongeNbId(_) => 6,
            TargetType::ExtendedngeNbId(_) => 7,
            TargetType::EngNbId(_) => 8,
            _ => 9,
        }
    }
}

impl From<u8> for TargetType {
    fn from(i: u8) -> TargetType {
        match i {
            0 => TargetType::RncId(Default::default()),
            1 => TargetType::MacroeNbId(Default::default()),
            2 => TargetType::CellId(Default::default()),
            3 => TargetType::HomeeNbId(Default::default()),
            4 => TargetType::ExtendedMacroeNbId(Default::default()),
            5 => TargetType::GNbId(Default::default()),
            6 => TargetType::MacrongeNbId(Default::default()),
            7 => TargetType::ExtendedngeNbId(Default::default()),
            8 => TargetType::EngNbId(Default::default()),
            _ => TargetType::Spare,
        }
    }
}

impl IEs for TargetType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        match self {
            TargetType::RncId(i) => i.marshal(buffer),
            TargetType::MacroeNbId(i) => i.marshal(buffer),
            TargetType::CellId(i) => i.marshal(buffer),
            TargetType::HomeeNbId(i) => i.marshal(buffer),
            TargetType::ExtendedMacroeNbId(i) => i.marshal(buffer),
            TargetType::GNbId(i) => i.marshal(buffer),
            TargetType::MacrongeNbId(i) => i.marshal(buffer),
            TargetType::ExtendedngeNbId(i) => i.marshal(buffer),
            TargetType::EngNbId(i) => i.marshal(buffer),
            _ => (),
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data = match buffer[0] {
            0 => {
                if let Ok(i) = RncIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::RncId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            1 => {
                if let Ok(i) = MacroEnbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::MacroeNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            2 => {
                if let Ok(i) = CellIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::CellId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            3 => {
                if let Ok(i) = MacroEnbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::HomeeNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            4 => {
                if let Ok(i) = ExtendedMacroEnbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::ExtendedMacroeNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            5 => {
                if let Ok(i) = GNbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::GNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            6 => {
                if let Ok(i) = MacronGeNbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::MacrongeNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            7 => {
                if let Ok(i) = ExtendedGeNbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::ExtendedngeNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            8 => {
                if let Ok(i) = EnGNbIdentifier::unmarshal(&buffer[1..]) {
                    TargetType::EngNbId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            _ => TargetType::Spare,
        };
        Ok(data)
    }

    fn len(&self) -> usize {
        match self {
            TargetType::RncId(i) => i.len(),
            TargetType::MacroeNbId(i) => i.len(),
            TargetType::CellId(i) => i.len(),
            TargetType::HomeeNbId(i) => i.len(),
            TargetType::ExtendedMacroeNbId(i) => i.len(),
            TargetType::GNbId(i) => i.len(),
            TargetType::MacrongeNbId(i) => i.len(),
            TargetType::ExtendedngeNbId(i) => i.len(),
            TargetType::EngNbId(i) => i.len(),
            _ => 1,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            TargetType::RncId(i) => i.is_empty(),
            TargetType::MacroeNbId(i) => i.is_empty(),
            TargetType::CellId(i) => i.is_empty(),
            TargetType::HomeeNbId(i) => i.is_empty(),
            TargetType::ExtendedMacroeNbId(i) => i.is_empty(),
            TargetType::GNbId(i) => i.is_empty(),
            TargetType::MacrongeNbId(i) => i.is_empty(),
            TargetType::ExtendedngeNbId(i) => i.is_empty(),
            TargetType::EngNbId(i) => i.is_empty(),
            _ => true,
        }
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Target Identification IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetIdentification {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub target_type: TargetType,
}

impl Default for TargetIdentification {
    fn default() -> Self {
        TargetIdentification {
            t: TARGETID,
            length: 0,
            ins: 0,
            target_type: Default::default(),
        }
    }
}

impl From<TargetIdentification> for InformationElement {
    fn from(i: TargetIdentification) -> Self {
        InformationElement::TargetIdentification(i)
    }
}

impl IEs for TargetIdentification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(TARGETID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.target_type));
        self.target_type.marshal(&mut buffer_ie);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 5 {
            let mut data = TargetIdentification {
                t: TARGETID,
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                ..Default::default()
            };
            data.target_type = if let Ok(i) = TargetType::unmarshal(&buffer[4..]) {
                i
            } else {
                return Err(GTPV2Error::IEIncorrect(TARGETID));
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(TARGETID))
        }
    }

    fn len(&self) -> usize {
        5 + self.target_type.len()
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

//Unit Tests - Target Identification IE (RNC ID)

#[test]
fn target_id_ie_rnc_id_marshal_test() {
    let encoded: [u8; 15] = [
        0x79, 0x00, 0x0b, 0x00, 0x00, 0x62, 0xf3, 0x10, 0xff, 0xff, 0xaa, 0xff, 0xaa, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 11,
        ins: 0,
        target_type: TargetType::RncId(RncIdentifier {
            rai: Rai {
                mcc: 263,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                rac: 0xaa,
            },
            rnc_id: 0xffaa,
            ext_rnc_id: Some(4098),
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_unmarshal_test() {
    let encoded: [u8; 15] = [
        0x79, 0x00, 0x0b, 0x00, 0x00, 0x62, 0xf3, 0x10, 0xff, 0xff, 0xaa, 0xff, 0xaa, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 11,
        ins: 0,
        target_type: TargetType::RncId(RncIdentifier {
            rai: Rai {
                mcc: 263,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                rac: 0xaa,
            },
            rnc_id: 0xffaa,
            ext_rnc_id: Some(4098),
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (Macro eNB ID)

#[test]
fn target_id_ie_macro_enb_id_marshal_test() {
    let encoded: [u8; 13] = [
        0x79, 0x00, 0x09, 0x00, 0x01, 0x62, 0xf3, 0x10, 0x00, 0xff, 0xff, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 9,
        ins: 0,
        target_type: TargetType::MacroeNbId(MacroEnbIdentifier {
            macro_enb_id: MacroEnbId {
                mcc: 263,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0xffff,
            },
            tac: 4098,
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_macro_enb_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x79, 0x00, 0x09, 0x00, 0x01, 0x62, 0xf3, 0x10, 0x00, 0xff, 0xff, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 9,
        ins: 0,
        target_type: TargetType::MacroeNbId(MacroEnbIdentifier {
            macro_enb_id: MacroEnbId {
                mcc: 263,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0xffff,
            },
            tac: 4098,
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (Extended Macro eNB ID)

#[test]
fn target_id_ie_ext_macro_enb_id_marshal_test() {
    let encoded: [u8; 13] = [
        0x79, 0x00, 0x09, 0x00, 0x04, 0x62, 0xf3, 0x40, 0x02, 0x00, 0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 9,
        ins: 0,
        target_type: TargetType::ExtendedMacroeNbId(ExtendedMacroEnbIdentifier {
            ext_macro_enb_id: ExtMacroEnbId {
                mcc: 263,
                mnc: 4,
                mnc_is_three_digits: false,
                smenb: false,
                ext_macro_id: 131072,
            },
            tac: 4098,
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_ext_macro_enb_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x79, 0x00, 0x09, 0x00, 0x04, 0x62, 0xf3, 0x40, 0x02, 0x00, 0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 9,
        ins: 0,
        target_type: TargetType::ExtendedMacroeNbId(ExtendedMacroEnbIdentifier {
            ext_macro_enb_id: ExtMacroEnbId {
                mcc: 263,
                mnc: 4,
                mnc_is_three_digits: false,
                smenb: false,
                ext_macro_id: 131072,
            },
            tac: 4098,
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (Cell ID)

#[test]
fn target_id_ie_cell_id_marshal_test() {
    let encoded: [u8; 13] = [
        0x79, 0x00, 0x09, 0x00, 0x02, 0x62, 0xf3, 0x40, 0x10, 0x02, 0x02, 0x00, 0x10,
    ];
    let decoded = TargetIdentification {
        length: 9,
        ins: 0,
        target_type: TargetType::CellId(CellIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            lac: 4098,
            rac: 2,
            ci: 16,
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_cell_id_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x79, 0x00, 0x09, 0x00, 0x02, 0x62, 0xf3, 0x40, 0x10, 0x02, 0x02, 0x00, 0x10,
    ];
    let decoded = TargetIdentification {
        length: 9,
        ins: 0,
        target_type: TargetType::CellId(CellIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            lac: 4098,
            rac: 2,
            ci: 16,
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (gNodeB ID)

#[test]
fn target_id_ie_gnodeb_id_marshal_test() {
    let encoded: [u8; 16] = [
        0x79, 0x00, 0x0c, 0x00, 0x05, 0x62, 0xf3, 0x40, 0x16, 0x00, 0x00, 0x10, 0x02, 0x00, 0x10,
        0x02,
    ];
    let decoded = TargetIdentification {
        length: 12,
        ins: 0,
        target_type: TargetType::GNbId(GNbIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            gnb_id_length: 22,
            gnb_id: 4098,
            etac: [0, 16, 2],
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_gnodeb_id_unmarshal_test() {
    let encoded: [u8; 16] = [
        0x79, 0x00, 0x0c, 0x00, 0x05, 0x62, 0xf3, 0x40, 0x16, 0x00, 0x00, 0x10, 0x02, 0x00, 0x10,
        0x02,
    ];
    let decoded = TargetIdentification {
        length: 12,
        ins: 0,
        target_type: TargetType::GNbId(GNbIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            gnb_id_length: 22,
            gnb_id: 4098,
            etac: [0, 16, 2],
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (Macro NG-eNB ID)

#[test]
fn target_id_ie_macro_gnodeb_id_marshal_test() {
    let encoded: [u8; 14] = [
        0x79, 0x00, 0x0a, 0x00, 0x06, 0x62, 0xf3, 0x40, 0x00, 0x10, 0x02, 0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 10,
        ins: 0,
        target_type: TargetType::MacrongeNbId(MacronGeNbIdentifier {
            macro_ng_enb_id: MacroEnbId {
                mcc: 263,
                mnc: 4,
                mnc_is_three_digits: false,
                macro_id: 4098,
            },
            etac: [0, 16, 2],
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_macro_gnodeb_id_unmarshal_test() {
    let encoded: [u8; 14] = [
        0x79, 0x00, 0x0a, 0x00, 0x06, 0x62, 0xf3, 0x40, 0x00, 0x10, 0x02, 0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 10,
        ins: 0,
        target_type: TargetType::MacrongeNbId(MacronGeNbIdentifier {
            macro_ng_enb_id: MacroEnbId {
                mcc: 263,
                mnc: 4,
                mnc_is_three_digits: false,
                macro_id: 4098,
            },
            etac: [0, 16, 2],
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (Extended NG-eNB ID)

#[test]
fn target_id_ie_extended_macro_gnodeb_id_marshal_test() {
    let encoded: [u8; 14] = [
        0x79, 0x00, 0x0a, 0x00, 0x07, 0x62, 0xf3, 0x40, 0x00, 0x10, 0x02, 0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 10,
        ins: 0,
        target_type: TargetType::ExtendedngeNbId(ExtendedGeNbIdentifier {
            macro_ng_enb_id: ExtMacroEnbId {
                mcc: 263,
                mnc: 4,
                mnc_is_three_digits: false,
                smenb: false,
                ext_macro_id: 4098,
            },
            etac: [0, 16, 2],
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_extended_macro_gnodeb_id_unmarshal_test() {
    let encoded: [u8; 14] = [
        0x79, 0x00, 0x0a, 0x00, 0x07, 0x62, 0xf3, 0x40, 0x00, 0x10, 0x02, 0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 10,
        ins: 0,
        target_type: TargetType::ExtendedngeNbId(ExtendedGeNbIdentifier {
            macro_ng_enb_id: ExtMacroEnbId {
                mcc: 263,
                mnc: 4,
                mnc_is_three_digits: false,
                smenb: false,
                ext_macro_id: 4098,
            },
            etac: [0, 16, 2],
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Target Identification IE (en-gNB ID)

#[test]
fn target_id_ie_engnodeb_id_marshal_test() {
    let encoded: [u8; 18] = [
        0x79, 0x00, 0x0e, 0x00, 0x08, 0x62, 0xf3, 0x40, 0xd6, 0x00, 0x00, 0x10, 0x02, 0x10, 0x02,
        0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 14,
        ins: 0,
        target_type: TargetType::EngNbId(EnGNbIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            en_gnb_id_length: 22,
            en_gnb_id: 4098,
            etac: Some([0, 16, 2]),
            tac: Some(4098),
        }),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //println!("{:#04x?}", buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn target_id_ie_engnodeb_id_unmarshal_test() {
    let encoded: [u8; 18] = [
        0x79, 0x00, 0x0e, 0x00, 0x08, 0x62, 0xf3, 0x40, 0xd6, 0x00, 0x00, 0x10, 0x02, 0x10, 0x02,
        0x00, 0x10, 0x02,
    ];
    let decoded = TargetIdentification {
        length: 14,
        ins: 0,
        target_type: TargetType::EngNbId(EnGNbIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            en_gnb_id_length: 22,
            en_gnb_id: 4098,
            etac: Some([0, 16, 2]),
            tac: Some(4098),
        }),
        ..Default::default()
    };
    assert_eq!(TargetIdentification::unmarshal(&encoded).unwrap(), decoded);
}
