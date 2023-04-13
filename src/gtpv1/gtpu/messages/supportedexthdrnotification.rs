use crate::gtpv1::errors::*;
use crate::gtpv1::gtpu::header::*;
use crate::gtpv1::gtpu::messages::commons::*;
use crate::gtpv1::gtpu::messages::ies::*;
use crate::gtpv1::utils::*;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

pub const SUPPORTED_EXTENSION_HEADERS_NOTIFICATION: u8 = 31;

// Definition of GTPv1-C Supported Extension Headers Notification

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedExtensionHeadersNotification {
    pub header: Gtpv1Header,
    pub list: ExtensionHeaderTypeList,
}

impl Default for SupportedExtensionHeadersNotification {
    fn default() -> SupportedExtensionHeadersNotification {
        let hdr = Gtpv1Header {
            msgtype: SUPPORTED_EXTENSION_HEADERS_NOTIFICATION,
            ..Default::default()
        };
        SupportedExtensionHeadersNotification {
            header: hdr,
            list: ExtensionHeaderTypeList::default(),
        }
    }
}

impl Messages for SupportedExtensionHeadersNotification {
    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        {
            let mut buffer_ie: Vec<u8> = vec![];
            self.list.marshal(&mut buffer_ie);
            buffer.append(&mut buffer_ie);
        }
        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut message = SupportedExtensionHeadersNotification::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != SUPPORTED_EXTENSION_HEADERS_NOTIFICATION {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if ((message.header.length + 8) as usize) <= buffer.len() {
            if message.header.get_header_size() == buffer.len() {
                return Err(GTPV1Error::MessageMandatoryIEMissing);
            }
            match buffer[message.header.get_header_size()] {
                EXTENSION_HEADER_TYPE_LIST => {
                    match ExtensionHeaderTypeList::unmarshal(
                        &buffer[message.header.get_header_size()..],
                    ) {
                        Ok(i) => message.list = i,
                        Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                    }
                    Ok(message)
                }
                _ => Err(GTPV1Error::MessageInvalidMessageFormat),
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn test_supported_ext_hdr_notification_unmarshal() {
    let encoded: [u8; 19] = [
        0x32, 0x1f, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x8d, 0x01, 0x00,
        0x01, 0x02, 0x03, 0x04,
    ];
    let decoded: SupportedExtensionHeadersNotification = SupportedExtensionHeadersNotification {
        header: Gtpv1Header {
            msgtype: SUPPORTED_EXTENSION_HEADERS_NOTIFICATION,
            length: 11,
            teid: 0,
            sequence_number: Some(63051),
            npdu_number: None,
            extension_headers: None,
        },
        list: ExtensionHeaderTypeList {
            t: EXTENSION_HEADER_TYPE_LIST,
            length: 1,
            list: vec![0x00, 0x01, 0x02, 0x03, 0x04],
        },
    };
    assert_eq!(
        SupportedExtensionHeadersNotification::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn test_supported_ext_hdr_notification_marshal() {
    let encoded: [u8; 19] = [
        0x32, 0x1f, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x8d, 0x05, 0x00,
        0x01, 0x02, 0x03, 0x04,
    ];
    let decoded: SupportedExtensionHeadersNotification = SupportedExtensionHeadersNotification {
        header: Gtpv1Header {
            msgtype: SUPPORTED_EXTENSION_HEADERS_NOTIFICATION,
            length: 11,
            teid: 0,
            sequence_number: Some(63051),
            npdu_number: None,
            extension_headers: None,
        },
        list: ExtensionHeaderTypeList {
            t: EXTENSION_HEADER_TYPE_LIST,
            length: 1,
            list: vec![0x00, 0x01, 0x02, 0x03, 0x04],
        },
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_supported_ext_hdr_notification_resp_without_mandatory_ie_unmarshal() {
    let encoded: [u8; 12] = [
        0x32, 0x1f, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00,
    ];
    assert_eq!(
        SupportedExtensionHeadersNotification::unmarshal(&encoded).unwrap_err(),
        GTPV1Error::MessageMandatoryIEMissing
    );
}

#[test]
fn test_supported_ext_hdr_notification_with_incorrect_mandatory_ie_unmarshal() {
    let encoded: [u8; 17] = [
        0x32, 0x1f, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x8d, 0x05, 0x00,
        0x01, 0x02,
    ];
    assert_eq!(
        SupportedExtensionHeadersNotification::unmarshal(&encoded).unwrap_err(),
        GTPV1Error::MessageMandatoryIEMissing
    );
}
