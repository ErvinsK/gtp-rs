// EPS Bearer ID (EBI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// EBI IE TL

pub const EBI: u8 = 73;
pub const EBI_LENGTH: usize = 1;

// EBI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ebi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub value: u8,
}

impl Default for Ebi {
    fn default() -> Self {
        Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 0,
        }
    }
}

impl From<Ebi> for InformationElement {
    fn from(i: Ebi) -> Self {
        InformationElement::Ebi(i)
    }
}

impl IEs for Ebi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(EBI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.value);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= EBI_LENGTH + MIN_IE_SIZE {
            let data = Ebi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                value: buffer[4] & 0x0f,
                ..Ebi::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(EBI))
        }
    }

    fn len(&self) -> usize {
        EBI_LENGTH + MIN_IE_SIZE
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
fn ebi_ie_marshal_test() {
    let ie_to_marshal = Ebi {
        t: EBI,
        length: EBI_LENGTH as u16,
        ins: 0,
        value: 5,
    };
    let ie_marshalled: [u8; 5] = [0x49, 0x00, 0x01, 0x00, 0x05];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn ebi_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 5] = [0x49, 0x00, 0x01, 0x00, 0x05];
    let ie_unmarshalled = Ebi {
        t: EBI,
        length: EBI_LENGTH as u16,
        ins: 0,
        value: 5,
    };
    assert_eq!(Ebi::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}
