// Flow Quality of Service (QoS) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Flow QoS IE TL

pub const FLOWQOS: u8 = 81;
pub const FLOWQOS_LENGTH: usize = 21;

// Flow QoS IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowQos {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub qci: u8,
    pub maxbr_ul: u64,
    pub maxbr_dl: u64,
    pub gbr_ul: u64,
    pub gbr_dl: u64,
}

impl Default for FlowQos {
    fn default() -> FlowQos {
        FlowQos {
            t: FLOWQOS,
            length: FLOWQOS_LENGTH as u16,
            ins: 0,
            qci: 9,
            maxbr_ul: 0,
            maxbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
        }
    }
}

impl From<FlowQos> for InformationElement {
    fn from(i: FlowQos) -> Self {
        InformationElement::FlowQos(i)
    }
}

impl IEs for FlowQos {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(FLOWQOS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.qci);
        buffer_ie.extend_from_slice(&self.maxbr_ul.to_be_bytes()[3..]);
        buffer_ie.extend_from_slice(&self.maxbr_dl.to_be_bytes()[3..]);
        buffer_ie.extend_from_slice(&self.gbr_ul.to_be_bytes()[3..]);
        buffer_ie.extend_from_slice(&self.gbr_dl.to_be_bytes()[3..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= FLOWQOS_LENGTH + MIN_IE_SIZE {
            let data = FlowQos {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                qci: buffer[4],
                maxbr_ul: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[5], buffer[6], buffer[7], buffer[8], buffer[9],
                ]),
                maxbr_dl: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[10], buffer[11], buffer[12], buffer[13], buffer[14],
                ]),
                gbr_ul: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[15], buffer[16], buffer[17], buffer[18], buffer[19],
                ]),
                gbr_dl: u64::from_be_bytes([
                    0x00, 0x00, 0x00, buffer[20], buffer[21], buffer[22], buffer[23], buffer[24],
                ]),
                ..FlowQos::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(FLOWQOS))
        }
    }

    fn len(&self) -> usize {
        FLOWQOS_LENGTH + MIN_IE_SIZE
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
fn flow_qos_ie_unmarshal_test() {
    let encoded: [u8; 25] = [
        0x51, 0x00, 0x15, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = FlowQos {
        t: FLOWQOS,
        length: FLOWQOS_LENGTH as u16,
        ins: 0,
        qci: 9,
        maxbr_ul: 0,
        maxbr_dl: 0,
        gbr_ul: 0,
        gbr_dl: 0,
    };
    let i = FlowQos::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn bearer_qos_ie_marshal_test() {
    let encoded: [u8; 25] = [
        0x51, 0x00, 0x15, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let decoded = FlowQos {
        t: FLOWQOS,
        length: FLOWQOS_LENGTH as u16,
        ins: 0,
        qci: 9,
        maxbr_ul: 0,
        maxbr_dl: 0,
        gbr_ul: 0,
        gbr_dl: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
