// Bearer Quality of Service (QoS) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Bearer QoS IE TL

pub const BEARERQOS: u8 = 80;
pub const BEARERQOS_LENGTH: usize = 22;

// Bearer QoS IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerQos {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pre_emption_vulnerability: u8,
    pub priority_level: u8,
    pub pre_emption_capability: u8,
    pub qci: u8,
    pub maxbr_ul: u64,
    pub maxbr_dl: u64,
    pub gbr_ul: u64,
    pub gbr_dl: u64,
}

impl Default for BearerQos {
    fn default() -> BearerQos {
        BearerQos {
            t: BEARERQOS,
            length: BEARERQOS_LENGTH as u16,
            ins: 0,
            pre_emption_vulnerability: 0,
            priority_level: 0,
            pre_emption_capability: 0,
            qci: 9,
            maxbr_ul: 0,
            maxbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
        }
    }
}

impl From<BearerQos> for InformationElement {
    fn from(i: BearerQos) -> Self {
        InformationElement::BearerQos(i)
    }
}

impl IEs for BearerQos {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(BEARERQOS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(
            (self.pre_emption_capability << 6)
                | (self.priority_level << 2)
                | self.pre_emption_vulnerability,
        );
        buffer_ie.push(self.qci);
        buffer_ie.extend_from_slice(&self.maxbr_ul.to_be_bytes()[3..]);
        buffer_ie.extend_from_slice(&self.maxbr_dl.to_be_bytes()[3..]);
        buffer_ie.extend_from_slice(&self.gbr_ul.to_be_bytes()[3..]);
        buffer_ie.extend_from_slice(&self.gbr_dl.to_be_bytes()[3..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= BEARERQOS_LENGTH + MIN_IE_SIZE {
            let data = BearerQos {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                pre_emption_capability: buffer[4] >> 6 & 0x01,
                priority_level: buffer[4] >> 2 & 0x0f,
                pre_emption_vulnerability: buffer[4] & 0x01,
                qci: buffer[5],
                maxbr_ul: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[6], buffer[7], buffer[8], buffer[9], buffer[10],
                ]),
                maxbr_dl: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[11], buffer[12], buffer[13], buffer[14], buffer[15],
                ]),
                gbr_ul: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[16], buffer[17], buffer[18], buffer[19], buffer[20],
                ]),
                gbr_dl: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[21], buffer[22], buffer[23], buffer[24], buffer[25],
                ]),
                ..BearerQos::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(BEARERQOS))
        }
    }

    fn len(&self) -> usize {
        BEARERQOS_LENGTH + MIN_IE_SIZE
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
fn bearer_qos_ie_unmarshal_test() {
    let encoded: [u8; 26] = [
        0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = BearerQos {
        t: BEARERQOS,
        length: BEARERQOS_LENGTH as u16,
        ins: 0,
        qci: 9,
        pre_emption_capability: 1,
        priority_level: 9,
        pre_emption_vulnerability: 0,
        maxbr_ul: 0,
        maxbr_dl: 0,
        gbr_ul: 0,
        gbr_dl: 0,
    };
    let i = BearerQos::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn bearer_qos_ie_marshal_test() {
    let encoded: [u8; 26] = [
        0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = BearerQos {
        t: BEARERQOS,
        length: BEARERQOS_LENGTH as u16,
        ins: 0,
        qci: 9,
        pre_emption_capability: 1,
        priority_level: 9,
        pre_emption_vulnerability: 0,
        maxbr_ul: 0,
        maxbr_dl: 0,
        gbr_ul: 0,
        gbr_dl: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
