// Source Identification IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*, targetid::CellIdentifier},
    utils::*,
};

// Source Identification IE Type

pub const SOURCEID: u8 = 129;

// Source RNC ID according to 3GPP TS 25.413 (9.2.1.24)
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SourceRncIdentifier {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub rnc_id: u16,
    pub ext_rnc_id: Option<u16>,
}

// Implementation of Source RNC ID IE

impl IEs for SourceRncIdentifier {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.rnc_id.to_be_bytes());
        if let Some(ext_rnc_id) = self.ext_rnc_id {
            buffer.extend_from_slice(&ext_rnc_id.to_be_bytes());
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        match buffer.len() {
            i if i < 6 => {
                let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
                let data = SourceRncIdentifier {
                    mcc,
                    mnc,
                    mnc_is_three_digits,
                    rnc_id: u16::from_be_bytes([buffer[3], buffer[4]]),
                    ext_rnc_id: None,
                };
                Ok(data)
            }
            j if j >= 7 => {
                let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[..=2]);
                let data = SourceRncIdentifier {
                    mcc,
                    mnc,
                    mnc_is_three_digits,
                    rnc_id: u16::from_be_bytes([buffer[3], buffer[4]]),
                    ext_rnc_id: Some(u16::from_be_bytes([buffer[5], buffer[6]])),
                };
                Ok(data)
            }
            _ => Err(GTPV2Error::IEInvalidLength(0)),
        }
    }

    fn len(&self) -> usize {
        match self.ext_rnc_id {
            Some(_) => 7,
            None => 5,
        }
    }

    fn is_empty(&self) -> bool {
        self.mcc == 0 && self.mnc == 0 && self.rnc_id == 0 && self.ext_rnc_id.is_none()
    }
    fn get_ins(&self) -> u8 {
        0
    }
    fn get_type(&self) -> u8 {
        0
    }
}

// Source Type Enum

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SourceType {
    SourceRncId(SourceRncIdentifier),
    SourceCellId(CellIdentifier),
    #[default]
    Spare,
}

impl From<&SourceType> for u8 {
    fn from(i: &SourceType) -> u8 {
        match i {
            SourceType::SourceCellId(_) => 0,
            SourceType::SourceRncId(_) => 1,
            _ => 2,
        }
    }
}

impl From<u8> for SourceType {
    fn from(i: u8) -> SourceType {
        match i {
            0 => SourceType::SourceCellId(Default::default()),
            1 => SourceType::SourceRncId(Default::default()),
            _ => SourceType::Spare,
        }
    }
}

impl IEs for SourceType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        match self {
            SourceType::SourceRncId(i) => i.marshal(buffer),
            SourceType::SourceCellId(i) => i.marshal(buffer),
            _ => (),
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data = match buffer[0] {
            0 => {
                if let Ok(i) = CellIdentifier::unmarshal(&buffer[1..]) {
                    SourceType::SourceCellId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            1 => {
                if let Ok(i) = SourceRncIdentifier::unmarshal(&buffer[1..]) {
                    SourceType::SourceRncId(i)
                } else {
                    return Err(GTPV2Error::IEIncorrect(0));
                }
            }
            _ => SourceType::Spare,
        };
        Ok(data)
    }

    fn len(&self) -> usize {
        match self {
            SourceType::SourceRncId(i) => i.len(),
            SourceType::SourceCellId(i) => i.len(),
            _ => 1,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            SourceType::SourceRncId(i) => i.is_empty(),
            SourceType::SourceCellId(i) => i.is_empty(),
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

// Source Identification IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceIdentification {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub target_cell: CellIdentifier,
    pub source_type: SourceType,
}

impl Default for SourceIdentification {
    fn default() -> Self {
        SourceIdentification {
            t: SOURCEID,
            length: 0,
            ins: 0,
            target_cell: CellIdentifier::default(),
            source_type: SourceType::default(),
        }
    }
}

impl From<SourceIdentification> for InformationElement {
    fn from(i: SourceIdentification) -> Self {
        InformationElement::SourceIdentification(i)
    }
}

impl IEs for SourceIdentification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SOURCEID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        self.target_cell.marshal(&mut buffer_ie);
        buffer_ie.push(u8::from(&self.source_type));
        self.source_type.marshal(&mut buffer_ie);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 13 {
            let mut data = SourceIdentification {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                target_cell: if let Ok(i) = CellIdentifier::unmarshal(&buffer[4..12]) {
                    i
                } else {
                    return Err(GTPV2Error::IEIncorrect(SOURCEID));
                },
                ..Default::default()
            };
            data.source_type = if let Ok(i) = SourceType::unmarshal(&buffer[12..]) {
                i
            } else {
                return Err(GTPV2Error::IEIncorrect(SOURCEID));
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SOURCEID))
        }
    }

    fn len(&self) -> usize {
        13 + self.source_type.len()
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

//Unit Tests - Source Identification IE (RNC ID)

#[test]
fn source_id_ie_rnc_id_marshal_test() {
    let encoded: [u8; 20] = [
        0x81, 0x00, 0x10, 0x00, 0x62, 0xf3, 0x10, 0xff, 0xff, 0xaa, 0xff, 0xaa, 0x01, 0x62, 0xf3,
        0x10, 0xff, 0xaa, 0x10, 0x02,
    ];
    let decoded = SourceIdentification {
        length: 16,
        ins: 0,
        target_cell: CellIdentifier {
            mcc: 263,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            rac: 0xaa,
            ci: 0xffaa,
        },
        source_type: SourceType::SourceRncId(SourceRncIdentifier {
            mcc: 263,
            mnc: 1,
            mnc_is_three_digits: false,
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
fn source_id_ie_source_rnc_id_unmarshal_test() {
    let encoded: [u8; 20] = [
        0x81, 0x00, 0x10, 0x00, 0x62, 0xf3, 0x10, 0xff, 0xff, 0xaa, 0xff, 0xaa, 0x01, 0x62, 0xf3,
        0x10, 0xff, 0xaa, 0x10, 0x02,
    ];
    let decoded = SourceIdentification {
        length: 16,
        ins: 0,
        target_cell: CellIdentifier {
            mcc: 263,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            rac: 0xaa,
            ci: 0xffaa,
        },
        source_type: SourceType::SourceRncId(SourceRncIdentifier {
            mcc: 263,
            mnc: 1,
            mnc_is_three_digits: false,
            rnc_id: 0xffaa,
            ext_rnc_id: Some(4098),
        }),
        ..Default::default()
    };
    assert_eq!(SourceIdentification::unmarshal(&encoded).unwrap(), decoded);
}

// Unit Tests - Source Identification IE (Cell ID)

#[test]
fn source_id_ie_cell_id_marshal_test() {
    let encoded: [u8; 21] = [
        0x81, 0x00, 0x11, 0x00, 0x62, 0xf3, 0x40, 0x10, 0x02, 0x02, 0x00, 0x10, 0x00, 0x62, 0xf3,
        0x40, 0x10, 0x02, 0x02, 0x00, 0x10,
    ];
    let decoded = SourceIdentification {
        length: 17,
        ins: 0,
        target_cell: CellIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            lac: 4098,
            rac: 2,
            ci: 16,
        },
        source_type: SourceType::SourceCellId(CellIdentifier {
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
fn source_id_ie_cell_id_unmarshal_test() {
    let encoded: [u8; 21] = [
        0x81, 0x00, 0x11, 0x00, 0x62, 0xf3, 0x40, 0x10, 0x02, 0x02, 0x00, 0x10, 0x00, 0x62, 0xf3,
        0x40, 0x10, 0x02, 0x02, 0x00, 0x10,
    ];
    let decoded = SourceIdentification {
        length: 17,
        ins: 0,
        target_cell: CellIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            lac: 4098,
            rac: 2,
            ci: 16,
        },
        source_type: SourceType::SourceCellId(CellIdentifier {
            mcc: 263,
            mnc: 4,
            mnc_is_three_digits: false,
            lac: 4098,
            rac: 2,
            ci: 16,
        }),
        ..Default::default()
    };
    assert_eq!(SourceIdentification::unmarshal(&encoded).unwrap(), decoded);
}
