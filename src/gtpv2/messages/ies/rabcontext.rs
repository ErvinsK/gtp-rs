// RAB Context IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// RAB Context IE Type

pub const RABCTX: u8 = 124;
pub const RABCTX_LENGTH: usize = 9;

// RAB Context IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RabContext {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub nsapi: u8,
    pub dl_gtpu_sqn: u16,
    pub ul_gtpu_sqn: u16,
    pub dl_pdcp_sqn: u16,
    pub ul_pdcp_sqn: u16,
}

impl Default for RabContext {
    fn default() -> Self {
        RabContext {
            t: RABCTX,
            length: RABCTX_LENGTH as u16,
            ins: 0,
            nsapi: 0,
            dl_gtpu_sqn: 0,
            ul_gtpu_sqn: 0,
            dl_pdcp_sqn: 0,
            ul_pdcp_sqn: 0,
        }
    }
}

impl From<RabContext> for InformationElement {
    fn from(i: RabContext) -> Self {
        InformationElement::RabContext(i)
    }
}

impl IEs for RabContext {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(RABCTX);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flag = ((self.ul_pdcp_sqn == 0) as u8) << 7
            | ((self.dl_pdcp_sqn == 0) as u8) << 6
            | ((self.ul_gtpu_sqn == 0) as u8) << 5
            | ((self.dl_gtpu_sqn == 0) as u8) << 4
            | self.nsapi;
        buffer_ie.push(flag);
        buffer_ie.extend_from_slice(&self.dl_gtpu_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ul_gtpu_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.dl_pdcp_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ul_pdcp_sqn.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + RABCTX_LENGTH {
            let data = RabContext {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                nsapi: buffer[4] & 0x0f,
                dl_gtpu_sqn: u16::from_be_bytes([buffer[5], buffer[6]]),
                ul_gtpu_sqn: u16::from_be_bytes([buffer[7], buffer[8]]),
                dl_pdcp_sqn: u16::from_be_bytes([buffer[9], buffer[10]]),
                ul_pdcp_sqn: u16::from_be_bytes([buffer[11], buffer[12]]),
                ..RabContext::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RABCTX))
        }
    }

    fn len(&self) -> usize {
        RABCTX_LENGTH + MIN_IE_SIZE
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
fn rab_context_ie_marshal_test() {
    let encoded: [u8; 13] = [
        0x7c, 0x00, 0x09, 0x00, 0x05, 0xff, 0x00, 0x00, 0xff, 0xaa, 0x00, 0x00, 0xaa,
    ];
    let decoded = RabContext {
        t: RABCTX,
        length: RABCTX_LENGTH as u16,
        ins: 0,
        nsapi: 5,
        dl_gtpu_sqn: 0xff00,
        ul_gtpu_sqn: 0x00ff,
        dl_pdcp_sqn: 0xaa00,
        ul_pdcp_sqn: 0x00aa,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn rab_context_ie_unmarshal_test() {
    let encoded: [u8; 13] = [
        0x7c, 0x00, 0x09, 0x00, 0x05, 0xff, 0x00, 0x00, 0xff, 0xaa, 0x00, 0x00, 0xaa,
    ];
    let decoded = RabContext {
        t: RABCTX,
        length: RABCTX_LENGTH as u16,
        ins: 0,
        nsapi: 5,
        dl_gtpu_sqn: 0xff00,
        ul_gtpu_sqn: 0x00ff,
        dl_pdcp_sqn: 0xaa00,
        ul_pdcp_sqn: 0x00aa,
    };
    assert_eq!(RabContext::unmarshal(&encoded).unwrap(), decoded);
}
