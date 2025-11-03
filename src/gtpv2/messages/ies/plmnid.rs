// PLMN ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// PLMN ID IE TL

pub const PLMNID: u8 = 120;
pub const PLMNID_LENGTH: usize = 3;

// PLMN ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlmnId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
}

impl Default for PlmnId {
    fn default() -> Self {
        PlmnId {
            t: PLMNID,
            length: PLMNID_LENGTH as u16,
            ins: 0,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
        }
    }
}

impl From<PlmnId> for InformationElement {
    fn from(i: PlmnId) -> Self {
        InformationElement::PlmnId(i)
    }
}

impl IEs for PlmnId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PLMNID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PLMNID_LENGTH {
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..7]);
            let data = PlmnId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                mcc,
                mnc,
                mnc_is_three_digits,
                ..PlmnId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PLMNID))
        }
    }

    fn len(&self) -> usize {
        PLMNID_LENGTH + MIN_IE_SIZE
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
fn plmnid_ie_marshal_test() {
    let decoded = PlmnId {
        t: PLMNID,
        length: PLMNID_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
    };
    let encoded: [u8; 7] = [0x78, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn plmnid_ie_unmarshal_test() {
    let decoded = PlmnId {
        t: PLMNID,
        length: PLMNID_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
    };
    let encoded: [u8; 7] = [0x78, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10];
    assert_eq!(PlmnId::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn plmnid_ie_three_digit_mnc_roundtrip_test() {
    let forced_three_digit = PlmnId {
        t: PLMNID,
        length: PLMNID_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 10,
        mnc_is_three_digits: true,
    };
    let forced_encoded: [u8; 7] = [0x78, 0x00, 0x03, 0x00, 0x99, 0x09, 0x10];
    let mut buffer: Vec<u8> = vec![];
    forced_three_digit.marshal(&mut buffer);
    assert_eq!(buffer, forced_encoded);
    assert_eq!(
        PlmnId::unmarshal(&forced_encoded).unwrap(),
        forced_three_digit
    );

    let numeric_three_digit = PlmnId {
        t: PLMNID,
        length: PLMNID_LENGTH as u16,
        ins: 0,
        mcc: 999,
        mnc: 742,
        mnc_is_three_digits: true,
    };
    let mut buffer_numeric: Vec<u8> = vec![];
    numeric_three_digit.marshal(&mut buffer_numeric);
    let expected_numeric: [u8; 7] = [0x78, 0x00, 0x03, 0x00, 0x99, 0x29, 0x47];
    assert_eq!(buffer_numeric, expected_numeric);
    let mut decoded_numeric = numeric_three_digit.clone();
    decoded_numeric.mnc_is_three_digits = true;
    assert_eq!(
        PlmnId::unmarshal(&expected_numeric).unwrap(),
        decoded_numeric
    );
}
