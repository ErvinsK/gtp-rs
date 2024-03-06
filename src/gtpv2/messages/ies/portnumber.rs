// Port Number IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Port Number IE Type

pub const PORT_NBR: u8 = 126;
pub const PORT_NBR_LENGTH: usize = 2;

// Port Number IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortNumber {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub port: u16,
}

impl Default for PortNumber {
    fn default() -> Self {
        PortNumber {
            t: PORT_NBR,
            length: PORT_NBR_LENGTH as u16,
            ins: 0,
            port: 0,
        }
    }
}

impl From<PortNumber> for InformationElement {
    fn from(i: PortNumber) -> Self {
        InformationElement::PortNumber(i)
    }
}

impl IEs for PortNumber {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PORT_NBR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.port.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + PORT_NBR_LENGTH {
            let data = PortNumber {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                port: u16::from_be_bytes([buffer[4], buffer[5]]),
                ..PortNumber::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PORT_NBR))
        }
    }

    fn len(&self) -> usize {
        PORT_NBR_LENGTH + MIN_IE_SIZE
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
fn port_number_ie_marshal_test() {
    let encoded: [u8; 6] = [0x7e, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = PortNumber {
        t: PORT_NBR,
        length: PORT_NBR_LENGTH as u16,
        ins: 0,
        port: 0xffff,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn port_number_ie_unmarshal_test() {
    let encoded: [u8; 6] = [0x7e, 0x00, 0x02, 0x00, 0xff, 0xff];
    let decoded = PortNumber {
        t: PORT_NBR,
        length: PORT_NBR_LENGTH as u16,
        ins: 0,
        port: 0xffff,
    };
    assert_eq!(PortNumber::unmarshal(&encoded).unwrap(), decoded);
}
