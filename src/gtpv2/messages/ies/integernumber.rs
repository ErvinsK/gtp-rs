// Integer Number IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Interger Number IE Type

pub const INT_NMBR: u8 = 187;

// Integer Number IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerNumber {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub number: Vec<u8>, // The Integer Number value is encoded with the number of octets defined in the Length field
}

impl Default for IntegerNumber {
    fn default() -> Self {
        IntegerNumber {
            t: INT_NMBR,
            length: 0,
            ins: 0,
            number: vec![],
        }
    }
}

impl From<IntegerNumber> for InformationElement {
    fn from(i: IntegerNumber) -> Self {
        InformationElement::IntegerNumber(i)
    }
}

impl IEs for IntegerNumber {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(INT_NMBR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.number.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = IntegerNumber {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..IntegerNumber::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.number
                    .extend_from_slice(&buffer[MIN_IE_SIZE..MIN_IE_SIZE + (data.length as usize)]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(INT_NMBR))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(INT_NMBR))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
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
fn integer_number_ie_marshal_test() {
    let encoded: [u8; 6] = [0xbb, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = IntegerNumber {
        t: INT_NMBR,
        length: 2,
        ins: 0,
        number: vec![0xff, 0xff],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn integer_number_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0xbb, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = IntegerNumber {
        t: INT_NMBR,
        length: 2,
        ins: 0,
        number: vec![0xff, 0xff],
    };
    assert_eq!(IntegerNumber::unmarshal(&encoded).unwrap(), decoded);
}
