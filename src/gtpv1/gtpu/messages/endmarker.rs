use crate::gtpv1::errors::*;
use crate::gtpv1::gtpu::header::*;
use crate::gtpv1::gtpu::messages::commons::*;
use crate::gtpv1::gtpu::messages::ies::*;
use crate::gtpv1::utils::*;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

pub const END_MARKER: u8 = 254;

// Definition of GTPv1-U Echo Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndMarker {
    pub header: Gtpv1Header,
    pub private_ext: Option<PrivateExtension>,
}

impl Default for EndMarker {
    fn default() -> EndMarker {
        let hdr = Gtpv1Header {
            msgtype: END_MARKER,
            ..Default::default()
        };
        EndMarker {
            header: hdr,
            private_ext: None,
        }
    }
}

impl Messages for EndMarker {
    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        if let Some(i) = self.private_ext {
            let mut buffer_ie: Vec<u8> = vec![];
            i.marshal(&mut buffer_ie);
            buffer.append(&mut buffer_ie);
        }
        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut message = EndMarker::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != END_MARKER {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize) < buffer.len() {
            if let Ok(i) = PrivateExtension::unmarshal(&buffer[message.header.get_header_size()..])
            {
                message.private_ext = Some(i)
            };
        }
        Ok(message)
    }
}

#[test]
fn test_end_marker_unmarshal() {
    let encoded: [u8; 12] = [
        0x32, 0xfe, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x49, 0xca, 0x00, 0x00,
    ];
    let decoded = EndMarker {
        header: Gtpv1Header {
            msgtype: END_MARKER,
            length: 4,
            teid: 2,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
        private_ext: None,
    };
    assert_eq!(EndMarker::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_end_marker_marshal() {
    let encoded: [u8; 12] = [
        0x32, 0xfe, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x49, 0xca, 0x00, 0x00,
    ];
    let decoded = EndMarker {
        header: Gtpv1Header {
            msgtype: END_MARKER,
            length: 4,
            teid: 2,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
        private_ext: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_end_marker_with_private_ext_unmarshal() {
    let encoded: [u8; 20] = [
        0x32, 0xfe, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0xff, 0x00, 0x05,
        0x00, 0x08, 0x01, 0x02, 0x03,
    ];
    let decoded = EndMarker {
        header: Gtpv1Header {
            msgtype: END_MARKER,
            length: 12,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
        private_ext: Some(PrivateExtension {
            t: PRIVATE_EXTENSION,
            length: 5,
            extension_id: 8,
            extension_value: vec![1, 2, 3],
        }),
    };
    assert_eq!(EndMarker::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_end_marker_with_private_ext_marshal() {
    let encoded: [u8; 20] = [
        0x32, 0xfe, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0xff, 0x00, 0x05,
        0x00, 0x08, 0x01, 0x02, 0x03,
    ];
    let decoded = EndMarker {
        header: Gtpv1Header {
            msgtype: END_MARKER,
            length: 12,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
        private_ext: Some(PrivateExtension {
            t: PRIVATE_EXTENSION,
            length: 5,
            extension_id: 8,
            extension_value: vec![1, 2, 3],
        }),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
