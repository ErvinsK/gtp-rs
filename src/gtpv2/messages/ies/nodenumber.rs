// Node Number IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Node Number IE Type

pub const NODE_NMBR: u8 = 175;

// Node Number IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeNumber {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub node_number: String, // The Node number shall carry an ISDN number. SGSN/MME Number shall be in international format and the "nature of address indicator" shall indicate "international number".
}

impl Default for NodeNumber {
    fn default() -> NodeNumber {
        NodeNumber {
            t: NODE_NMBR,
            length: 0,
            ins: 0,
            node_number: "".to_string(),
        }
    }
}

impl From<NodeNumber> for InformationElement {
    fn from(i: NodeNumber) -> Self {
        InformationElement::NodeNumber(i)
    }
}

impl IEs for NodeNumber {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(NODE_NMBR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let mut number: Vec<u8> = vec![0x91];
        number.append(&mut tbcd_encode(&self.node_number));
        buffer_ie.push(number.len() as u8);
        buffer_ie.extend(number);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = NodeNumber {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Default::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                let cursor = buffer[4] as usize;
                match buffer[5..(cursor + 5)].try_into() {
                    Ok(i) => {
                        let n: &[u8] = i;
                        data.node_number = tbcd_decode(&n[1..])
                    }
                    Err(_) => return Err(GTPV2Error::IEIncorrect(NODE_NMBR)),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(NODE_NMBR))
            }
        } else {
            Err(GTPV2Error::IEIncorrect(NODE_NMBR))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn node_number_ie_unmarshal_test() {
    let encoded: [u8; 14] = [
        0xaf, 0x00, 0x0a, 0x00, 0x09, 0x91, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let decoded = NodeNumber {
        t: NODE_NMBR,
        length: 0x0a,
        ins: 0x00,
        node_number: "901405101961873".to_string(),
    };
    let i = NodeNumber::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn node_number_ie_marshal_test() {
    let encoded: [u8; 14] = [
        0xaf, 0x00, 0x0a, 0x00, 0x09, 0x91, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let decoded = NodeNumber {
        t: NODE_NMBR,
        length: 0x0a,
        ins: 0x00,
        node_number: "901405101961873".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn node_number_ie_unmarshal_buffer_test() {
    let encoded: [u8; 14] = [
        0xaf, 0x00, 0x0a, 0x00, 0x09, 0x91, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let decoded = NodeNumber {
        t: NODE_NMBR,
        length: 0x0a,
        ins: 0x00,
        node_number: "901405101961873".to_string(),
    };
    let i = NodeNumber::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}
