use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const PDU_NOTIFICATION_REQUEST: u8 = 27;

// Definition of GTPv1-C PDU Notification Request

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PDUNotificationRequest {
    pub header: Gtpv1Header,
    pub imsi: Imsi,
    pub teid_control: Teid,
    pub end_user_address: EndUserAddress,
    pub apn: Apn,
    pub pco: Option<Pco>,
    pub ggsn_ip_control: GsnAddress,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for PDUNotificationRequest {
    fn default() -> PDUNotificationRequest {
        let hdr = Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REQUEST,
            ..Default::default()
        };
        PDUNotificationRequest {
            header: hdr,
            imsi: Imsi::default(),
            teid_control: Teid::default(),
            end_user_address: EndUserAddress::default(),
            apn: Apn::default(),
            pco: None,
            ggsn_ip_control: GsnAddress::default(),
            private_extension: None,
        }
    }
}

impl Messages for PDUNotificationRequest {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal IMSI IE

        self.imsi.marshal(buffer);

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

        // Marshal GGSN Address for Control plane IE

        self.ggsn_ip_control.marshal(buffer);

        // Marshal Private Extension IE

        if let Some(i) = self.private_extension {
            i.marshal(buffer)
        };

        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = PDUNotificationRequest::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != PDU_NOTIFICATION_REQUEST {
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
                        IMSI => match Imsi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.imsi = i;
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
                        GSN_ADDRESS => match GsnAddress::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.ggsn_ip_control = i;
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
            match (
                msg_hash.get(&IMSI),
                msg_hash.get(&TEID_CONTROL),
                msg_hash.get(&END_USER_ADDRESS),
                msg_hash.get(&APN),
                msg_hash.get(&GSN_ADDRESS),
            ) {
                (Some(_), Some(_), Some(_), Some(_), Some(_)) => Ok(message),
                _ => Err(GTPV1Error::MessageMandatoryIEMissing),
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn pdu_notification_req_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 93] = [
        0x32, 0x1b, 0x0, 0x55, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x11, 0xa6, 0x97, 0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa,
        0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3, 0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3,
        0x6e, 0x65, 0x74, 0x84, 0x0, 0x20, 0x80, 0x80, 0x21, 0x10, 0x1, 0x0, 0x0, 0x10, 0x81, 0x6,
        0x0, 0x0, 0x0, 0x0, 0x83, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0xa, 0x0, 0x0, 0x5,
        0x0, 0x0, 0x11, 0x0, 0x85, 0x0, 0x4, 0x3e, 0x99, 0x89, 0x60,
    ];
    let decoded = PDUNotificationRequest {
        header: Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REQUEST,
            length: 85,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Imsi {
            t: 2,
            imsi: "901405101744546".to_string(),
        },
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
        pco: Some(Pco {
            t: 132,
            length: 32,
            pco: vec![
                128, 128, 33, 16, 1, 0, 0, 16, 129, 6, 0, 0, 0, 0, 131, 6, 0, 0, 0, 0, 0, 13, 0, 0,
                10, 0, 0, 5, 0, 0, 17, 0,
            ],
        }),
        ggsn_ip_control: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        },
        private_extension: None,
    };
    assert_eq!(
        PDUNotificationRequest::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn pdu_notification_req_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 93] = [
        0x32, 0x1b, 0x0, 0x55, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x11, 0xa6, 0x97, 0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa,
        0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3, 0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3,
        0x6e, 0x65, 0x74, 0x84, 0x0, 0x20, 0x80, 0x80, 0x21, 0x10, 0x1, 0x0, 0x0, 0x10, 0x81, 0x6,
        0x0, 0x0, 0x0, 0x0, 0x83, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0xa, 0x0, 0x0, 0x5,
        0x0, 0x0, 0x11, 0x0, 0x85, 0x0, 0x4, 0x3e, 0x99, 0x89, 0x60,
    ];
    let decoded = PDUNotificationRequest {
        header: Gtpv1Header {
            msgtype: PDU_NOTIFICATION_REQUEST,
            length: 85,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Imsi {
            t: 2,
            imsi: "901405101744546".to_string(),
        },
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
        pco: Some(Pco {
            t: 132,
            length: 32,
            pco: vec![
                128, 128, 33, 16, 1, 0, 0, 16, 129, 6, 0, 0, 0, 0, 131, 6, 0, 0, 0, 0, 0, 13, 0, 0,
                10, 0, 0, 5, 0, 0, 17, 0,
            ],
        }),
        ggsn_ip_control: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        },
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pdu_notification_req_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 93] = [
        0x32, 0x1b, 0x0, 0x55, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x11, 0xa6, 0x97, 0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa,
        0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3, 0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3,
        0x6e, 0x65, 0x74, 0x85, 0x0, 0x4, 0x3e, 0x99, 0x89, 0x60, 0x84, 0x0, 0x20, 0x80, 0x80,
        0x21, 0x10, 0x1, 0x0, 0x0, 0x10, 0x81, 0x6, 0x0, 0x0, 0x0, 0x0, 0x83, 0x6, 0x0, 0x0, 0x0,
        0x0, 0x0, 0xd, 0x0, 0x0, 0xa, 0x0, 0x0, 0x5, 0x0, 0x0, 0x11, 0x0,
    ];
    assert_eq!(
        PDUNotificationRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn pdu_notification_req_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 86] = [
        0x32, 0x1b, 0x0, 0x4e, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x11, 0xa6, 0x97, 0x49, 0xf4, 0x80, 0x0, 0x6, 0xf1, 0x21, 0xa,
        0xdb, 0x3b, 0x30, 0x83, 0x0, 0xd, 0x3, 0x69, 0x6f, 0x74, 0x4, 0x31, 0x6e, 0x63, 0x65, 0x3,
        0x6e, 0x65, 0x74, 0x84, 0x0, 0x20, 0x80, 0x80, 0x21, 0x10, 0x1, 0x0, 0x0, 0x10, 0x81, 0x6,
        0x0, 0x0, 0x0, 0x0, 0x83, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0xa, 0x0, 0x0, 0x5,
        0x0, 0x0, 0x11, 0x0,
    ];
    assert_eq!(
        PDUNotificationRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
