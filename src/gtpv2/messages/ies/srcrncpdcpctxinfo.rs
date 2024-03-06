// Source RNC PDCP Context Info IE - according to 3GPP TS 29.274 V17.10.0 (2023-12) and 3GPP TS 24.008 V16.0.0 (2019-03)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Source RNC PDCP Context Info IE Type

pub const SRC_RNC_PDCP: u8 = 125;

// Source RNC PDCP Context Info IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRncPdcpContextInfo {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub rrc_container: Vec<u8>,
}

impl Default for SourceRncPdcpContextInfo {
    fn default() -> Self {
        SourceRncPdcpContextInfo {
            t: SRC_RNC_PDCP,
            length: 0,
            ins: 0,
            rrc_container: vec![],
        }
    }
}

impl From<SourceRncPdcpContextInfo> for InformationElement {
    fn from(i: SourceRncPdcpContextInfo) -> Self {
        InformationElement::SrcRncPdcpCtxInfo(i)
    }
}

impl IEs for SourceRncPdcpContextInfo {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SRC_RNC_PDCP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut self.rrc_container.clone());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = SourceRncPdcpContextInfo {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..SourceRncPdcpContextInfo::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.rrc_container
                    .extend_from_slice(&buffer[4..(data.length + 4) as usize]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(SRC_RNC_PDCP))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(SRC_RNC_PDCP))
        }
    }

    fn len(&self) -> usize {
        (self.length + 4) as usize
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
fn src_rnc_pdcp_ctx_ie_marshal_test() {
    let encoded: [u8; 24] = [
        0x7d, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = SourceRncPdcpContextInfo {
        t: SRC_RNC_PDCP,
        length: 20,
        ins: 0,
        rrc_container: vec![
            0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
            0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn src_rnc_pdcp_ie_unmarshal_test() {
    let encoded: [u8; 24] = [
        0x7d, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = SourceRncPdcpContextInfo {
        t: SRC_RNC_PDCP,
        length: 20,
        ins: 0,
        rrc_container: vec![
            0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
            0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    assert_eq!(
        SourceRncPdcpContextInfo::unmarshal(&encoded).unwrap(),
        decoded
    );
}
