// PDU Numbers IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// PDU Numbers IE Type

pub const PDUNMBRS: u8 = 110;
pub const PDUNMBRS_LENGTH: usize = 9;

// PDU Numbers IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PduNumbers {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub nsapi: u8,
    pub dl_gtpu_sqn: u16,
    pub ul_gtpu_sqn: u16,
    pub send_npdu: u16,
    pub receive_npdu: u16,
}

impl Default for PduNumbers {
    fn default() -> Self {
        PduNumbers {
            t: PDUNMBRS,
            length: PDUNMBRS_LENGTH as u16,
            ins: 0,
            nsapi: 0,
            dl_gtpu_sqn: 0,
            ul_gtpu_sqn: 0,
            send_npdu: 0,
            receive_npdu: 0,
        }
    }
}

impl From<PduNumbers> for InformationElement {
    fn from(i: PduNumbers) -> Self {
        InformationElement::PduNumbers(i)
    }
}

impl IEs for PduNumbers {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PDUNMBRS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.nsapi);
        buffer_ie.extend_from_slice(&self.dl_gtpu_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ul_gtpu_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.send_npdu.to_be_bytes());
        buffer_ie.extend_from_slice(&self.receive_npdu.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PDUNMBRS_LENGTH {
            let data = PduNumbers {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                nsapi: buffer[4] & 0x0f,
                dl_gtpu_sqn: u16::from_be_bytes([buffer[5], buffer[6]]),
                ul_gtpu_sqn: u16::from_be_bytes([buffer[7], buffer[8]]),
                send_npdu: u16::from_be_bytes([buffer[9], buffer[10]]),
                receive_npdu: u16::from_be_bytes([buffer[11], buffer[12]]),
                ..PduNumbers::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PDUNMBRS))
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
fn pdu_numbers_ie_marshal_test() {
    let encoded: [u8; 13] = [
        0x6e, 0x00, 0x09, 0x00, 0x05, 0xff, 0x00, 0x00, 0xff, 0xaa, 0x00, 0x00, 0xaa,
    ];
    let decoded = PduNumbers {
        t: PDUNMBRS,
        length: PDUNMBRS_LENGTH as u16,
        ins: 0,
        nsapi: 5,
        dl_gtpu_sqn: 0xff00,
        ul_gtpu_sqn: 0x00ff,
        send_npdu: 0xaa00,
        receive_npdu: 0x00aa,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pdu_numbers_ie_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x6e, 0x00, 0x09, 0x00, 0x05, 0xff, 0x00, 0x00, 0xff, 0xaa, 0x00, 0x00, 0xaa,
    ];
    let decoded = PduNumbers {
        t: PDUNMBRS,
        length: PDUNMBRS_LENGTH as u16,
        ins: 0,
        nsapi: 5,
        dl_gtpu_sqn: 0xff00,
        ul_gtpu_sqn: 0x00ff,
        send_npdu: 0xaa00,
        receive_npdu: 0x00aa,
    };
    assert_eq!(PduNumbers::unmarshal(&encoded).unwrap(), decoded);
}
