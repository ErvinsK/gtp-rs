// PSCell ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// PSCell ID IE TL

pub const PSCELL_ID: u8 = 217;
pub const PSCELL_ID_LENGTH: usize = 8;

// PSCell ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PSCellId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub nr_cgi: [u8; 5],
}

impl Default for PSCellId {
    fn default() -> Self {
        PSCellId {
            t: PSCELL_ID,
            length: PSCELL_ID_LENGTH as u16,
            ins: 0,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            nr_cgi: [0u8; 5],
        }
    }
}

impl From<PSCellId> for InformationElement {
    fn from(i: PSCellId) -> Self {
        InformationElement::PSCellId(i)
    }
}

impl IEs for PSCellId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PSCELL_ID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer_ie.extend_from_slice(&self.nr_cgi);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PSCELL_ID_LENGTH {
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[4..7]);
            let data = PSCellId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                mcc,
                mnc,
                mnc_is_three_digits,
                nr_cgi: buffer[7..12].try_into().unwrap_or([0u8; 5]),
                ..PSCellId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PSCELL_ID))
        }
    }

    fn len(&self) -> usize {
        PSCELL_ID_LENGTH + MIN_IE_SIZE
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
fn pscellid_ie_marshal_test() {
    let decoded = PSCellId {
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        nr_cgi: [0xaa, 0xfc, 0xfd, 0xfe, 0xff],
        ..PSCellId::default()
    };
    let encoded: [u8; 12] = [
        0xd9, 0x00, 0x08, 0x00, 0x99, 0xf9, 0x10, 0xaa, 0xfc, 0xfd, 0xfe, 0xff,
    ];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pscellid_ie_unmarshal_test() {
    let decoded = PSCellId {
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        nr_cgi: [0xaa, 0xfc, 0xfd, 0xfe, 0xff],
        ..PSCellId::default()
    };
    let encoded: [u8; 12] = [
        0xd9, 0x00, 0x08, 0x00, 0x99, 0xf9, 0x10, 0xaa, 0xfc, 0xfd, 0xfe, 0xff,
    ];
    assert_eq!(PSCellId::unmarshal(&encoded).unwrap(), decoded);
}
