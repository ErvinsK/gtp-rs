// Presence Reporting Area Info IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Presence Reporting Area Info IE Type

pub const PRAI: u8 = 178;
pub const PRAI_LENGTH: usize = 8;

// Presence Reporting Area enum

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresenceReportingArea {
    // The PRA ID shall be encoded as an integer on 3 octets. The most significant bit of the PRA ID shall be set to 0 for UEdedicated PRA and shall be to 1 for Core Network predefined PRA.
    Ipra(u32),
    Opra(u32),
    Inapra(u32),
}

impl From<PresenceReportingArea> for Vec<u8> {
    fn from(i: PresenceReportingArea) -> Self {
        let mut result = vec![];
        match i {
            PresenceReportingArea::Inapra(j) => {
                result.extend_from_slice(&j.to_be_bytes()[1..]);
                result.push(0x08);
            }
            PresenceReportingArea::Ipra(j) => {
                result.extend_from_slice(&j.to_be_bytes()[1..]);
                result.push(0x01);
            }
            PresenceReportingArea::Opra(j) => {
                result.extend_from_slice(&j.to_be_bytes()[1..]);
                result.push(0x02);
            }
        }
        result
    }
}

impl From<&[u8]> for PresenceReportingArea {
    fn from(i: &[u8]) -> Self {
        match ((i[3] >> 3) & 0x01, (i[3] >> 1) & 0x01, i[3] & 0x01) {
            (0, 1, 0) => PresenceReportingArea::Opra(u32::from_be_bytes([0x00, i[0], i[1], i[2]])),
            (0, 0, 1) => PresenceReportingArea::Ipra(u32::from_be_bytes([0x00, i[0], i[1], i[2]])),
            _ => PresenceReportingArea::Inapra(u32::from_be_bytes([0x00, i[0], i[1], i[2]])),
        }
    }
}

// Presence Reporting Area Info IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceReportingAreaInformation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub prai: PresenceReportingArea,
    pub add_prai: Option<Vec<PresenceReportingArea>>,
}

impl Default for PresenceReportingAreaInformation {
    fn default() -> Self {
        PresenceReportingAreaInformation {
            t: PRAI,
            length: PRAI_LENGTH as u16,
            ins: 0,
            prai: PresenceReportingArea::Inapra(0),
            add_prai: None,
        }
    }
}

impl From<PresenceReportingAreaInformation> for InformationElement {
    fn from(i: PresenceReportingAreaInformation) -> Self {
        InformationElement::PresenceReportingAreaInformation(i)
    }
}

impl IEs for PresenceReportingAreaInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PRAI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.prai.clone().into());
        if let Some(i) = self.add_prai.clone() {
            let mut flag = buffer_ie[buffer_ie.len() - 1] | 0x04;
            let mut cursor = buffer_ie.len() - 1;
            buffer_ie[cursor] = flag;
            for k in i {
                buffer_ie.append(&mut k.into());
                let flag = buffer_ie[buffer_ie.len() - 1] | 0x04;
                cursor = buffer_ie.len() - 1;
                buffer_ie[cursor] = flag;
            }
            flag = buffer_ie[cursor] & 0x0b;
            buffer_ie[cursor] = flag;
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PRAI_LENGTH {
            let mut data = PresenceReportingAreaInformation {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                prai: PresenceReportingArea::from(&buffer[4..8]),
                ..PresenceReportingAreaInformation::default()
            };
            if (buffer[7] >> 2) & 0x01 == 1 {
                let mut cursor: usize = 8;
                let mut add_prai: Vec<PresenceReportingArea> = vec![];
                loop {
                    if cursor + 4 > buffer.len() {
                        break;
                    }
                    add_prai.push(PresenceReportingArea::from(&buffer[cursor..cursor + 4]));
                    if (buffer[cursor + 3] >> 2) & 0x01 == 0 {
                        break;
                    } else {
                        cursor += 4;
                    }
                }
                data.add_prai = Some(add_prai);
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PRAI))
        }
    }

    fn len(&self) -> usize {
        PRAI_LENGTH + MIN_IE_SIZE
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
fn prai_ie_marshal_test() {
    let encoded: [u8; 12] = [
        0xb2, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0xff, 0x02,
    ];
    let decoded = PresenceReportingAreaInformation {
        t: PRAI,
        length: 8,
        ins: 0,
        prai: PresenceReportingArea::Ipra(0x00),
        add_prai: Some(vec![PresenceReportingArea::Opra(0xff)]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn prai_ie_unmarshal_test() {
    let encoded: [u8; 12] = [
        0xb2, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0xff, 0x02,
    ];
    let decoded = PresenceReportingAreaInformation {
        t: PRAI,
        length: 8,
        ins: 0,
        prai: PresenceReportingArea::Ipra(0x00),
        add_prai: Some(vec![PresenceReportingArea::Opra(0xff)]),
    };
    assert_eq!(
        PresenceReportingAreaInformation::unmarshal(&encoded).unwrap(),
        decoded
    );
}
