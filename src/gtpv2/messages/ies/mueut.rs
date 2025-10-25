// Mapped UE Usage Type (MUEUT) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
// Mapped UE Usage Type is defined in clause 5.8.1 of 3GPP TS 29.003

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Mapped UE Usage Type (MUEUT) IE TL

pub const MUEUT: u8 = 200;
pub const MUEUT_LENGTH: usize = 2;

// Mapped UE Usage Type (MUEUT) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedUeUsageType {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub usage_type: u16,
}

impl Default for MappedUeUsageType {
    fn default() -> MappedUeUsageType {
        MappedUeUsageType {
            t: MUEUT,
            length: MUEUT_LENGTH as u16,
            ins: 0,
            usage_type: 0,
        }
    }
}

impl From<MappedUeUsageType> for InformationElement {
    fn from(i: MappedUeUsageType) -> Self {
        InformationElement::MappedUeUsageType(i)
    }
}

impl IEs for MappedUeUsageType {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MUEUT);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.usage_type.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MUEUT_LENGTH + MIN_IE_SIZE {
            let data = MappedUeUsageType {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                usage_type: u16::from_be_bytes([buffer[4], buffer[5]]),
                ..MappedUeUsageType::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MUEUT))
        }
    }

    fn len(&self) -> usize {
        MUEUT_LENGTH + MIN_IE_SIZE
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
fn mueut_ie_unmarshal_test() {
    let encoded_ie: [u8; 6] = [0xc8, 0x00, 0x02, 0x00, 0x00, 0x0f];
    let test_struct = MappedUeUsageType {
        t: MUEUT,
        length: MUEUT_LENGTH as u16,
        ins: 0,
        usage_type: 15,
    };
    let i = MappedUeUsageType::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn mueut_ie_marshal_test() {
    let encoded_ie: [u8; 6] = [0xc8, 0x00, 0x02, 0x00, 0x00, 0x0f];
    let test_struct = MappedUeUsageType {
        t: MUEUT,
        length: MUEUT_LENGTH as u16,
        ins: 0,
        usage_type: 15,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
