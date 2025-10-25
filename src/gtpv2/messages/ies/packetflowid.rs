// Packet Flow ID IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Packet Flow ID TL

pub const PCKTFLOW: u8 = 123;
pub const PCKTFLOW_LENGTH: usize = 5;

// Packet Flow ID IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketFlowId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ebi: u8,
    pub flow_id: u32,
}

impl Default for PacketFlowId {
    fn default() -> Self {
        PacketFlowId {
            t: PCKTFLOW,
            length: PCKTFLOW_LENGTH as u16,
            ins: 0,
            ebi: 0,
            flow_id: 0,
        }
    }
}

impl From<PacketFlowId> for InformationElement {
    fn from(i: PacketFlowId) -> Self {
        InformationElement::PacketFlowId(i)
    }
}

impl IEs for PacketFlowId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PCKTFLOW);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.ebi);
        buffer_ie.extend_from_slice(&self.flow_id.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= PCKTFLOW_LENGTH + MIN_IE_SIZE {
            let data = PacketFlowId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ebi: buffer[4] & 0x0f,
                flow_id: u32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]),
                ..PacketFlowId::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PCKTFLOW))
        }
    }

    fn len(&self) -> usize {
        PCKTFLOW_LENGTH + MIN_IE_SIZE
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
fn packet_flow_id_ie_marshal_test() {
    let decoded = PacketFlowId {
        t: PCKTFLOW,
        length: PCKTFLOW_LENGTH as u16,
        ins: 0,
        ebi: 5,
        flow_id: 0xffffffff,
    };
    let encoded: [u8; 9] = [0x7b, 0x00, 0x05, 0x00, 0x05, 0xff, 0xff, 0xff, 0xff];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn packet_flow_id_ie_unmarshal_test() {
    let decoded = PacketFlowId {
        t: PCKTFLOW,
        length: PCKTFLOW_LENGTH as u16,
        ins: 0,
        ebi: 5,
        flow_id: 0xffffffff,
    };
    let encoded: [u8; 9] = [0x7b, 0x00, 0x05, 0x00, 0x05, 0xff, 0xff, 0xff, 0xff];
    assert_eq!(PacketFlowId::unmarshal(&encoded).unwrap(), decoded);
}
