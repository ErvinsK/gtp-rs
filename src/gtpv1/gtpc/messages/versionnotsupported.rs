use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::utils::*;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const VERSION_NOT_SUPPORTED: u8 = 3;

// Definition of GTPv1-C Version Not Supported

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionNotSupported {
    pub header: Gtpv1Header,
}

impl Default for VersionNotSupported {
    fn default() -> VersionNotSupported {
        let hdr = Gtpv1Header {
            msgtype: VERSION_NOT_SUPPORTED,
            ..Default::default()
        };
        VersionNotSupported { header: hdr }
    }
}

impl Messages for VersionNotSupported {
    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut message = VersionNotSupported::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }
        if message.header.msgtype != VERSION_NOT_SUPPORTED {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }
        Ok(message)
    }
}

#[test]
fn test_version_not_supported_unmarshal() {
    let encoded: [u8; 12] = [
        0x32, 0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00,
    ];
    let decoded: VersionNotSupported = VersionNotSupported {
        header: Gtpv1Header {
            msgtype: VERSION_NOT_SUPPORTED,
            length: 4,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
    };
    assert_eq!(VersionNotSupported::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_version_not_supported_marshal() {
    let encoded: [u8; 12] = [
        0x32, 0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00,
    ];
    let decoded: VersionNotSupported = VersionNotSupported {
        header: Gtpv1Header {
            msgtype: VERSION_NOT_SUPPORTED,
            length: 4,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: None,
        },
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
