// Maximum Packet Loss Rate IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Maximum Packet Loss Rate IE TV

pub const MAX_PACKET_LOSS: u8 = 203;

// Maximum Packet Loss Rate IE implementation
// The Maximum Packet Loss Rate for UL and DL shall be coded as an unsigned integer in the range of 0 to 1000.
// It shall be interpreted as Ratio of lost packets per number of packets sent, expressed in tenth of percent.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxPacketLossRate {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub max_packet_loss_ul: Option<u16>,
    pub max_packet_loss_dl: Option<u16>,
}

impl Default for MaxPacketLossRate {
    fn default() -> Self {
        MaxPacketLossRate {
            t: MAX_PACKET_LOSS,
            length: 0,
            ins: 0,
            max_packet_loss_ul: None,
            max_packet_loss_dl: None,
        }
    }
}

impl From<MaxPacketLossRate> for InformationElement {
    fn from(i: MaxPacketLossRate) -> Self {
        InformationElement::MaxPacketLossRate(i)
    }
}

impl IEs for MaxPacketLossRate {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MAX_PACKET_LOSS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (
            self.max_packet_loss_dl.is_some(),
            self.max_packet_loss_ul.is_some(),
        ) {
            (false, false) => buffer_ie.push(0x00),
            (false, true) => {
                buffer_ie.push(0x01);
                buffer_ie.extend_from_slice(&self.max_packet_loss_ul.unwrap().to_be_bytes());
            }
            (true, false) => {
                buffer_ie.push(0x02);
                buffer_ie.extend_from_slice(&self.max_packet_loss_dl.unwrap().to_be_bytes());
            }
            (true, true) => {
                buffer_ie.push(0x03);
                buffer_ie.extend_from_slice(&self.max_packet_loss_ul.unwrap().to_be_bytes());
                buffer_ie.extend_from_slice(&self.max_packet_loss_dl.unwrap().to_be_bytes());
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() > MIN_IE_SIZE {
            let mut data = MaxPacketLossRate {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..MaxPacketLossRate::default()
            };
            match buffer[4] & 0x03 {
                0 => {
                    data.max_packet_loss_dl = None;
                    data.max_packet_loss_ul = None;
                }
                1 => {
                    if check_tliv_ie_buffer(3, buffer) {
                        data.max_packet_loss_ul = Some(u16::from_be_bytes([buffer[5], buffer[6]]));
                        data.max_packet_loss_dl = None;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS));
                    }
                }
                2 => {
                    if check_tliv_ie_buffer(3, buffer) {
                        data.max_packet_loss_ul = None;
                        data.max_packet_loss_dl = Some(u16::from_be_bytes([buffer[5], buffer[6]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS));
                    }
                }
                3 => {
                    if check_tliv_ie_buffer(5, buffer) {
                        data.max_packet_loss_ul = Some(u16::from_be_bytes([buffer[5], buffer[6]]));
                        data.max_packet_loss_dl = Some(u16::from_be_bytes([buffer[7], buffer[8]]));
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS));
                    }
                }
                _ => return Err(GTPV2Error::IEIncorrect(MAX_PACKET_LOSS)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MAX_PACKET_LOSS))
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
fn max_packet_loss_rate_ie_unmarshal_test() {
    let encoded: [u8; 9] = [0xcb, 0x00, 0x05, 0x00, 0x03, 0x03, 0xe8, 0x03, 0xe7];
    let decoded = MaxPacketLossRate {
        t: MAX_PACKET_LOSS,
        length: 5,
        ins: 0,
        max_packet_loss_ul: Some(0x3e8),
        max_packet_loss_dl: Some(0x3e7),
    };
    let i = MaxPacketLossRate::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn secondary_rat_udr_ie_marshal_test() {
    let encoded: [u8; 9] = [0xcb, 0x00, 0x05, 0x00, 0x03, 0x03, 0xe8, 0x03, 0xe7];
    let decoded = MaxPacketLossRate {
        t: MAX_PACKET_LOSS,
        length: 5,
        ins: 0,
        max_packet_loss_ul: Some(0x3e8),
        max_packet_loss_dl: Some(0x3e7),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
