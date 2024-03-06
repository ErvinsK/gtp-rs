// RAT Type IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// RAT Type IE Type

pub const RATTYPE: u8 = 82;
pub const RATTYPE_LENGTH: usize = 1;

/*
RAT Types	                Values (Decimal)
<reserved>              	0
UTRAN	                    1
GERAN	                    2
WLAN	                    3
GAN	                        4
HSPA Evolution	            5
EUTRAN (WB-E-UTRAN)	        6
Virtual	                    7
EUTRAN-NB-IoT	            8
LTE-M	                    9
NR	                        10
WB-E-UTRAN(LEO)	            11
WB-E-UTRAN(MEO)	            12
WB-E-UTRAN(GEO)	            13
WB-E-UTRAN(OTHERSAT)	    14
EUTRAN-NB-IoT(LEO)	        15
EUTRAN-NB-IoT(MEO)	        16
EUTRAN-NB-IoT(GEO)	        17
EUTRAN-NB-IoT(OTHERSAT)	    18
LTE-M(LEO)	                19
LTE-M(MEO)	                20
LTE-M(GEO)	                21
LTE-M(OTHERSAT)	            22
<spare>	                    23-255
*/

// RAT Type enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rat {
    Reserved,
    Utran,
    Geran,
    Wlan,
    Gan,
    HspaEvo,
    Eutran,
    Virtual,
    EutranNbiot,
    LteM,
    NR,
    WbEutranLeo,
    WbEutranMeo,
    WbEutranGeo,
    WbEutranOthersat,
    EutranNbiotLeo,
    EutranNbiotMeo,
    EutranNbiotGeo,
    EutranNbiotOthersat,
    LteMLeo,
    LteMMeo,
    LteMGeo,
    LteMOthersat,
    Spare,
}

impl From<Rat> for u8 {
    fn from(i: Rat) -> u8 {
        match i {
            Rat::Reserved => 0,
            Rat::Utran => 1,
            Rat::Geran => 2,
            Rat::Wlan => 3,
            Rat::Gan => 4,
            Rat::HspaEvo => 5,
            Rat::Eutran => 6,
            Rat::Virtual => 7,
            Rat::EutranNbiot => 8,
            Rat::LteM => 9,
            Rat::NR => 10,
            Rat::WbEutranLeo => 11,
            Rat::WbEutranMeo => 12,
            Rat::WbEutranGeo => 13,
            Rat::WbEutranOthersat => 14,
            Rat::EutranNbiotLeo => 15,
            Rat::EutranNbiotMeo => 16,
            Rat::EutranNbiotGeo => 17,
            Rat::EutranNbiotOthersat => 18,
            Rat::LteMLeo => 19,
            Rat::LteMMeo => 20,
            Rat::LteMGeo => 21,
            Rat::LteMOthersat => 22,
            Rat::Spare => 23,
        }
    }
}

impl From<u8> for Rat {
    fn from(i: u8) -> Rat {
        match i {
            0 => Rat::Reserved,
            1 => Rat::Utran,
            2 => Rat::Geran,
            3 => Rat::Wlan,
            4 => Rat::Gan,
            5 => Rat::HspaEvo,
            6 => Rat::Eutran,
            7 => Rat::Virtual,
            8 => Rat::EutranNbiot,
            9 => Rat::LteM,
            10 => Rat::NR,
            11 => Rat::WbEutranLeo,
            12 => Rat::WbEutranMeo,
            13 => Rat::WbEutranGeo,
            14 => Rat::WbEutranOthersat,
            15 => Rat::EutranNbiotLeo,
            16 => Rat::EutranNbiotMeo,
            17 => Rat::EutranNbiotGeo,
            18 => Rat::EutranNbiotOthersat,
            19 => Rat::LteMLeo,
            20 => Rat::LteMMeo,
            21 => Rat::LteMGeo,
            22 => Rat::LteMOthersat,
            _ => Rat::Spare,
        }
    }
}

// RAT Type IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatType {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub rat_type: Rat,
}

impl Default for RatType {
    fn default() -> Self {
        RatType {
            t: RATTYPE,
            length: RATTYPE_LENGTH as u16,
            ins: 0,
            rat_type: Rat::Eutran,
        }
    }
}

impl From<RatType> for InformationElement {
    fn from(i: RatType) -> Self {
        InformationElement::RatType(i)
    }
}

impl IEs for RatType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(RATTYPE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(self.rat_type.clone() as Rat));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + RATTYPE_LENGTH {
            let data = RatType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                rat_type: Rat::from(buffer[4]),
                ..RatType::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RATTYPE))
        }
    }

    fn len(&self) -> usize {
        RATTYPE_LENGTH + MIN_IE_SIZE
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
fn rattype_ie_marshal_test() {
    let encoded: [u8; 5] = [0x52, 0x00, 0x01, 0x00, 0x06];
    let decoded = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH as u16,
        ins: 0,
        rat_type: Rat::Eutran,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn rattype_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x52, 0x00, 0x01, 0x00, 0x06];
    let decoded = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH as u16,
        ins: 0,
        rat_type: Rat::Eutran,
    };
    assert_eq!(RatType::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn rattype_ie_unknown_rattype_unmarshal_test() {
    let encoded: [u8; 5] = [0x52, 0x00, 0x01, 0x00, 0xff];
    let decoded = RatType {
        t: RATTYPE,
        length: RATTYPE_LENGTH as u16,
        ins: 0,
        rat_type: Rat::Spare,
    };
    assert_eq!(RatType::unmarshal(&encoded).unwrap(), decoded);
}
