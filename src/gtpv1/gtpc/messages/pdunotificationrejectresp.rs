use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const PDU_NOTIFICATION_REJECT_RESPONSE: u8 = 30;

// Definition of GTPv1-C PDU Notification Reject Response

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PDUNotificationRejectResponse {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for PDUNotificationRejectResponse {
    fn default() -> PDUNotificationRejectResponse {
        let hdr = Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REJECT_RESPONSE,
            ..Default::default()
        };
        PDUNotificationRejectResponse {
            header: hdr,
            cause: Cause::default(),
            private_extension: None,
        }
    }
}

impl Messages for PDUNotificationRejectResponse {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Cause IE

        self.cause.marshal(buffer);

        // Marshal Private Extension IE

        if let Some(i) = self.private_extension {
            i.marshal(buffer)
        };

        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = PDUNotificationRejectResponse::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != PDU_NOTIFICATION_REJECT_RESPONSE {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length + 8) as usize <= buffer.len() {
            let mut cursor = message.header.len();
            let mut increment: u8 = 0;
            loop {
                if cursor >= buffer.len() {
                    break;
                }
                if buffer[cursor] >= increment {
                    match buffer[cursor] {
                        CAUSE => match Cause::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.cause = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        PRIVATE_EXTENSION => match PrivateExtension::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.private_extension = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        _ => return Err(GTPV1Error::MessageInvalidMessageFormat),
                    }
                } else {
                    return Err(GTPV1Error::MessageInvalidMessageFormat);
                }
            }
            if msg_hash.contains_key(&CAUSE) {
                Ok(message)
            } else {
                Err(GTPV1Error::MessageMandatoryIEMissing)
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn pdu_notification_reject_resp_unmarshal_test() {
    let encoded: [u8; 14] = [
        0x32, 0x1e, 0x0, 0x06, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x01, 0x80,
    ];
    let decoded = PDUNotificationRejectResponse {
        header: Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REJECT_RESPONSE,
            length: 6,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        private_extension: None,
    };
    assert_eq!(
        PDUNotificationRejectResponse::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn pdu_notification_reject_resp_marshal_test() {
    let encoded: [u8; 14] = [
        0x32, 0x1e, 0x0, 0x06, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x01, 0x80,
    ];
    let decoded = PDUNotificationRejectResponse {
        header: Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REJECT_RESPONSE,
            length: 6,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pdu_notification_reject_resp_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 20] = [
        0x32, 0x1e, 0x0, 0x0c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0xff, 0x00, 0x03,
        0x00, 0x01, 0x00, 0x01, 0x80,
    ];
    assert_eq!(
        PDUNotificationRejectResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn pdu_notification_reject_resp_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 18] = [
        0x32, 0x1e, 0x0, 0x0a, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0xff, 0x00, 0x03,
        0x00, 0x01, 0x00,
    ];
    assert_eq!(
        PDUNotificationRejectResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
