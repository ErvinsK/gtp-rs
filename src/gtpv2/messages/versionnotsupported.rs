use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const VERSION_NOT_SUPPORTED: u8 = 3;

// Definition of GTPv2-C Version Not Supported

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionNotSupported {
    pub header: Gtpv2Header,
}

impl Default for VersionNotSupported {
    fn default() -> VersionNotSupported {
        let hdr = Gtpv2Header {
            msgtype: VERSION_NOT_SUPPORTED,
            ..Default::default()
        };
        VersionNotSupported { header: hdr }
    }
}

impl Messages for VersionNotSupported {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = VersionNotSupported::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }
        if message.header.msgtype != VERSION_NOT_SUPPORTED {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }
        Ok(message)
    }

    fn tovec(&self) -> Vec<InformationElement> {
        vec![]
    }

    fn fromvec(&mut self, _: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        Ok(true)
    }
}

#[test]
fn test_version_not_supported_unmarshal() {
    let encoded: [u8; 8] = [0x40, 0x03, 0x00, 0x04, 0x2d, 0xcc, 0x38, 0x00];
    let decoded = VersionNotSupported {
        header: Gtpv2Header {
            msgtype: VERSION_NOT_SUPPORTED,
            piggyback: false,
            message_prio: None,
            length: 4,
            teid: None,
            sqn: 0x2dcc38,
        },
    };
    assert_eq!(VersionNotSupported::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_echo_resp_marshal() {
    let encoded: [u8; 8] = [0x40, 0x03, 0x00, 0x04, 0x2d, 0xcc, 0x38, 0x00];
    let decoded = VersionNotSupported {
        header: Gtpv2Header {
            msgtype: VERSION_NOT_SUPPORTED,
            piggyback: false,
            message_prio: None,
            length: 4,
            teid: None,
            sqn: 0x2dcc38,
        },
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
