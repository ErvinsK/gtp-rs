// Special Information Element with IE Type Extension field - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Special IE with IE Type Extension Field Type

pub const IETYPE_EXT: u8 = 254;

// Special IE with IE Type Extension Field implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialIEWithTypeExt {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ie_type_ext: u16,
    pub ie_data: Vec<u8>,
}

impl Default for SpecialIEWithTypeExt {
    fn default() -> Self {
        SpecialIEWithTypeExt {
            t: IETYPE_EXT,
            length: 0,
            ins: 0,
            ie_type_ext: 0,
            ie_data: vec![],
        }
    }
}

impl From<SpecialIEWithTypeExt> for InformationElement {
    fn from(i: SpecialIEWithTypeExt) -> Self {
        InformationElement::SpecialIEWithTypeExt(i)
    }
}

impl IEs for SpecialIEWithTypeExt {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(IETYPE_EXT);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.ie_type_ext.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ie_data[..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = SpecialIEWithTypeExt {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..SpecialIEWithTypeExt::default()
            };
            data.ie_type_ext = u16::from_be_bytes([buffer[4], buffer[5]]);
            if check_tliv_ie_buffer(data.length, buffer) {
                data.ie_data
                    .extend_from_slice(&buffer[6..MIN_IE_SIZE + (data.length as usize)]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(IETYPE_EXT))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(IETYPE_EXT))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        (self.ie_type_ext == 0) || (self.length == 0)
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn iewithietypeext_ie_marshal_test() {
    let encoded: [u8; 8] = [0xfe, 0x00, 0x04, 0x00, 0x01, 0x01, 0xff, 0xff];
    let decoded = SpecialIEWithTypeExt {
        t: IETYPE_EXT,
        length: 4,
        ins: 0,
        ie_type_ext: 257,
        ie_data: vec![0xff, 0xff],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn iewithietypeext_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0xfe, 0x00, 0x04, 0x00, 0x01, 0x01, 0xff, 0xff];
    let decoded = SpecialIEWithTypeExt {
        t: IETYPE_EXT,
        length: 4,
        ins: 0,
        ie_type_ext: 257,
        ie_data: vec![0xff, 0xff],
    };
    assert_eq!(SpecialIEWithTypeExt::unmarshal(&encoded).unwrap(), decoded);
}
