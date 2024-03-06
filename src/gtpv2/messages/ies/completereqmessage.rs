// Complete Request Message IE - according to 3GPP TS 29.274 V17.10.0 (2023-12) and 3GPP TS 24.008 V16.0.0 (2019-03)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Complete Request Message IE Type

pub const COMPLETE_REQ_MSG: u8 = 116;

// Complete Request Message Type Enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestMessage {
    Spare,
    AttachRequest(Vec<u8>),
    TauRequest(Vec<u8>),
}

// Complete Request Message IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteRequestMessage {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub message: RequestMessage,
}

impl Default for CompleteRequestMessage {
    fn default() -> Self {
        CompleteRequestMessage {
            t: COMPLETE_REQ_MSG,
            length: 0,
            ins: 0,
            message: RequestMessage::Spare,
        }
    }
}

impl From<CompleteRequestMessage> for InformationElement {
    fn from(i: CompleteRequestMessage) -> Self {
        InformationElement::CompleteRequestMessage(i)
    }
}

impl IEs for CompleteRequestMessage {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(COMPLETE_REQ_MSG);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.message.clone() {
            RequestMessage::Spare => buffer_ie.push(0x02),
            RequestMessage::AttachRequest(i) => {
                buffer_ie.push(0x00);
                buffer_ie.extend_from_slice(&i[..]);
            }
            RequestMessage::TauRequest(i) => {
                buffer_ie.push(0x01);
                buffer_ie.extend_from_slice(&i[..]);
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = CompleteRequestMessage {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..CompleteRequestMessage::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    0 => {
                        data.message = RequestMessage::AttachRequest(
                            buffer[5..MIN_IE_SIZE + data.length as usize].to_vec(),
                        )
                    }
                    1 => {
                        data.message = RequestMessage::TauRequest(
                            buffer[5..MIN_IE_SIZE + data.length as usize].to_vec(),
                        )
                    }
                    _ => data.message = RequestMessage::Spare,
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(COMPLETE_REQ_MSG))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(COMPLETE_REQ_MSG))
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
fn complete_request_msg_ie_marshal_test() {
    let encoded: [u8; 10] = [0x74, 0x00, 0x06, 0x00, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let decoded = CompleteRequestMessage {
        t: COMPLETE_REQ_MSG,
        length: 6,
        ins: 0,
        message: RequestMessage::AttachRequest(vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn complete_request_msg_ie_unmarshal_test() {
    let encoded: [u8; 10] = [0x74, 0x00, 0x06, 0x00, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee];
    let decoded = CompleteRequestMessage {
        t: COMPLETE_REQ_MSG,
        length: 6,
        ins: 0,
        message: RequestMessage::AttachRequest(vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
    };
    assert_eq!(
        CompleteRequestMessage::unmarshal(&encoded).unwrap(),
        decoded
    );
}
