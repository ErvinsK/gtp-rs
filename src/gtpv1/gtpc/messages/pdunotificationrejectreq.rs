use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const PDU_NOTIFICATION_REJECT_REQUEST: u8 = 29;

// Definition of GTPv1-C PDU Notification Reject Request

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PDUNotificationRejectRequest {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub teid_control: Teid,
    pub end_user_address: EndUserAddress,
    pub apn: Apn,
    pub pco: Option<Pco>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for PDUNotificationRejectRequest {
    fn default() -> PDUNotificationRejectRequest {
        let hdr = Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REJECT_REQUEST,
            ..Default::default()
        };
        PDUNotificationRejectRequest {
            header: hdr,
            cause: Cause::default(),
            teid_control: Teid::default(),
            end_user_address: EndUserAddress::default(),
            apn: Apn::default(),
            pco: None,
            private_extension: None,
        }
    }
}

impl Messages for PDUNotificationRejectRequest {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Cause IE

        self.cause.marshal(buffer);

        // Marshal TEID Control Plane IE

        self.teid_control.marshal(buffer);

        // Marshal End User Address IE

        self.end_user_address.marshal(buffer);

        // Marshal APN IE

        self.apn.marshal(buffer);

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal Private Extension IE

        if let Some(i) = self.private_extension {
            i.marshal(buffer)
        };

        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = PDUNotificationRejectRequest::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != PDU_NOTIFICATION_REJECT_REQUEST {
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
                        TEID_CONTROL => match Teid::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.teid_control = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        END_USER_ADDRESS => match EndUserAddress::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.end_user_address = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        APN => match Apn::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.apn = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        PCO => match Pco::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.pco = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
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
            match (
                msg_hash.get(&CAUSE),
                msg_hash.get(&TEID_CONTROL),
                msg_hash.get(&END_USER_ADDRESS),
                msg_hash.get(&APN),
            ) {
                (Some(_), Some(_), Some(_), Some(_)) => Ok(message),
                _ => Err(GTPV1Error::MessageMandatoryIEMissing),
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn pdu_notification_reject_req_unmarshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 44] = [
        0x32, 0x1d, 0x0, 0x24, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x1, 0x5, 0x11, 0xa6,
        0x97, 0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa, 0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3,
        0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3, 0x6e, 0x65, 0x74,
    ];
    let decoded = PDUNotificationRejectRequest {
        header: Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REJECT_REQUEST,
            length: 36,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 5 },
        teid_control: Teid {
            t: TEID_CONTROL,
            teid: 2794932724,
        },
        end_user_address: EndUserAddress {
            t: 128,
            length: 6,
            pdp_type_org: 1,
            pdp_type_nbr: 33,
            ipv4: Some(Ipv4Addr::new(10, 219, 59, 48)),
            ipv6: None,
        },
        apn: Apn {
            t: 131,
            length: 13,
            name: "iot.1nce.net".to_string(),
        },
        pco: None,
        private_extension: None,
    };

    assert_eq!(
        PDUNotificationRejectRequest::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn pdu_notification_reject_req_marshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 44] = [
        0x32, 0x1d, 0x0, 0x24, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x1, 0x5, 0x11, 0xa6,
        0x97, 0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa, 0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3,
        0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3, 0x6e, 0x65, 0x74,
    ];
    let decoded = PDUNotificationRejectRequest {
        header: Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REJECT_REQUEST,
            length: 36,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 5 },
        teid_control: Teid {
            t: TEID_CONTROL,
            teid: 2794932724,
        },
        end_user_address: EndUserAddress {
            t: 128,
            length: 6,
            pdp_type_org: 1,
            pdp_type_nbr: 33,
            ipv4: Some(Ipv4Addr::new(10, 219, 59, 48)),
            ipv6: None,
        },
        apn: Apn {
            t: 131,
            length: 13,
            name: "iot.1nce.net".to_string(),
        },
        pco: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pdu_notification_reject_req_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 44] = [
        0x32, 0x1d, 0x0, 0x24, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x11, 0xa6, 0x97,
        0x49, 0xf4, 0x1, 0x5, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa, 0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd,
        0x3, 0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3, 0x6e, 0x65, 0x74,
    ];
    assert_eq!(
        PDUNotificationRejectRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn pdu_notification_reject_req_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 42] = [
        0x32, 0x1d, 0x0, 0x22, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x11, 0xa6, 0x97,
        0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa, 0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3, 0x69,
        0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3, 0x6e, 0x65, 0x74,
    ];
    assert_eq!(
        PDUNotificationRejectRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
