use crate::gtpv1::errors::*;
use crate::gtpv1::gtpu::header::*;
use crate::gtpv1::gtpu::messages::commons::*;
use crate::gtpv1::utils::*;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

pub const GPDU: u8 = 255;

// Definition of GTPv1-U G-PDU Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gpdu {
    pub header: Gtpv1Header,
    pub tpdu: Vec<u8>,
}

impl Default for Gpdu {
    fn default() -> Gpdu {
        let hdr = Gtpv1Header {
            msgtype: GPDU,
            ..Default::default()
        };
        Gpdu {
            header: hdr,
            tpdu: vec![],
        }
    }
}

impl Messages for Gpdu {
    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        buffer.append(&mut self.tpdu.clone());
        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut message = Gpdu::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != GPDU {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length + 8) as usize <= buffer.len() {
            message.tpdu.extend_from_slice(
                &buffer[message.header.get_header_size()..(message.header.length + 8) as usize],
            );
            Ok(message)
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn test_gpdu_unmarshal() {
    let encoded: [u8; 13] = [
        0x32, 0xff, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0x01,
    ];
    let decoded = Gpdu {
        header: Gtpv1Header {
            msgtype: GPDU,
            length: 5,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
        tpdu: vec![1],
    };
    assert_eq!(Gpdu::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gpdu_marshal() {
    let encoded: [u8; 13] = [
        0x32, 0xff, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0x01,
    ];
    let decoded = Gpdu {
        header: Gtpv1Header {
            msgtype: GPDU,
            length: 5,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
        tpdu: vec![1],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gpdu_missing_tpdu_unmarshal() {
    let encoded: [u8; 12] = [
        0x32, 0xff, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00,
    ];
    assert_eq!(
        Gpdu::unmarshal(&encoded),
        Err(GTPV1Error::MessageLengthError)
    );
}
