use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const REMOTE_UE_REPORT_NOTIF: u8 = 40;

// Definition of GTPv2-C Remote UE Report Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteUeReportNotification {
    pub header: Gtpv2Header,
    pub remote_ue_ctx_connected: Vec<RemoteUeContext>,
    pub remote_ue_ctx_disconnected: Vec<RemoteUeContext>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for RemoteUeReportNotification {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: REMOTE_UE_REPORT_NOTIF,
            teid: Some(0),
            ..Default::default()
        };
        RemoteUeReportNotification {
            header: hdr,
            remote_ue_ctx_connected: vec![],
            remote_ue_ctx_disconnected: vec![],
            private_ext: vec![],
        }
    }
}

impl Messages for RemoteUeReportNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = RemoteUeReportNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != REMOTE_UE_REPORT_NOTIF {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }
        let offset = message.header.length as usize + MANDATORY_HDR_LENGTH;

        if buffer.len() >= offset {
            match InformationElement::decoder(&buffer[MAX_HEADER_LENGTH..offset]) {
                Ok(i) => match message.fromvec(i) {
                    Ok(_) => Ok(message),
                    Err(j) => Err(j),
                },
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }

    fn tovec(&self) -> Vec<InformationElement> {
        let mut elements: Vec<InformationElement> = vec![];

        self.remote_ue_ctx_connected
            .iter()
            .for_each(|x| elements.push(InformationElement::RemoteUeContext(x.clone())));

        self.remote_ue_ctx_disconnected
            .iter()
            .for_each(|x| elements.push(InformationElement::RemoteUeContext(x.clone())));

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.iter() {
            match e {
                InformationElement::RemoteUeContext(j) => match j.ins {
                    0 => self.remote_ue_ctx_connected.push(j.clone()),
                    1 => self.remote_ue_ctx_disconnected.push(j.clone()),
                    _ => (),
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_remote_ue_report_notification_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 157] = [
        0x48, 0x28, 0x00, 0x99, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0xbf, 0x00, 0x29,
        0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
        0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10,
        0x23, 0x03, 0x30, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34, 0xbf, 0x00, 0x29,
        0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf4,
        0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf3, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10,
        0x23, 0x03, 0x40, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x35, 0xbf, 0x00, 0x29,
        0x01, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf5,
        0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf4, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10,
        0x23, 0x03, 0x50, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x36, 0xff, 0x00, 0x06,
        0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = RemoteUeReportNotification {
        header: Gtpv2Header {
            msgtype: REMOTE_UE_REPORT_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 153,
            teid: Some(0),
            sqn: 0x68,
        },
        remote_ue_ctx_connected: vec![
            RemoteUeContext {
                t: REMOTE_UE_CTX,
                length: 41,
                ins: 0,
                user_id: RemoteUserId {
                    t: REMOTE_USR_ID,
                    length: 28,
                    ins: 0,
                    imsi: "901405101961873".to_string(),
                    msisdn: Some("882285100125112".to_string()),
                    imei: Some("8676480401323003".to_string()),
                },
                ue_ip: Some(RemoteUeIpInformation {
                    t: REMOTE_UE_IP,
                    length: 5,
                    ins: 0,
                    ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
                }),
            },
            RemoteUeContext {
                t: REMOTE_UE_CTX,
                length: 41,
                ins: 0,
                user_id: RemoteUserId {
                    t: REMOTE_USR_ID,
                    length: 28,
                    ins: 0,
                    imsi: "901405101961874".to_string(),
                    msisdn: Some("882285100125113".to_string()),
                    imei: Some("8676480401323004".to_string()),
                },
                ue_ip: Some(RemoteUeIpInformation {
                    t: REMOTE_UE_IP,
                    length: 5,
                    ins: 0,
                    ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 53)),
                }),
            },
        ],
        remote_ue_ctx_disconnected: vec![RemoteUeContext {
            t: REMOTE_UE_CTX,
            length: 41,
            ins: 1,
            user_id: RemoteUserId {
                t: REMOTE_USR_ID,
                length: 28,
                ins: 0,
                imsi: "901405101961875".to_string(),
                msisdn: Some("882285100125114".to_string()),
                imei: Some("8676480401323005".to_string()),
            },
            ue_ip: Some(RemoteUeIpInformation {
                t: REMOTE_UE_IP,
                length: 5,
                ins: 0,
                ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 54)),
            }),
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let message = RemoteUeReportNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_remote_ue_report_notification_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 157] = [
        0x48, 0x28, 0x00, 0x99, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0xbf, 0x00, 0x29,
        0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
        0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10,
        0x23, 0x03, 0x30, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34, 0xbf, 0x00, 0x29,
        0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf4,
        0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf3, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10,
        0x23, 0x03, 0x40, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x35, 0xbf, 0x00, 0x29,
        0x01, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf5,
        0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf4, 0x08, 0x68, 0x67, 0x84, 0x40, 0x10,
        0x23, 0x03, 0x50, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x36, 0xff, 0x00, 0x06,
        0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = RemoteUeReportNotification {
        header: Gtpv2Header {
            msgtype: REMOTE_UE_REPORT_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 153,
            teid: Some(0),
            sqn: 0x68,
        },
        remote_ue_ctx_connected: vec![
            RemoteUeContext {
                t: REMOTE_UE_CTX,
                length: 41,
                ins: 0,
                user_id: RemoteUserId {
                    t: REMOTE_USR_ID,
                    length: 28,
                    ins: 0,
                    imsi: "901405101961873".to_string(),
                    msisdn: Some("882285100125112".to_string()),
                    imei: Some("8676480401323003".to_string()),
                },
                ue_ip: Some(RemoteUeIpInformation {
                    t: REMOTE_UE_IP,
                    length: 5,
                    ins: 0,
                    ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
                }),
            },
            RemoteUeContext {
                t: REMOTE_UE_CTX,
                length: 41,
                ins: 0,
                user_id: RemoteUserId {
                    t: REMOTE_USR_ID,
                    length: 28,
                    ins: 0,
                    imsi: "901405101961874".to_string(),
                    msisdn: Some("882285100125113".to_string()),
                    imei: Some("8676480401323004".to_string()),
                },
                ue_ip: Some(RemoteUeIpInformation {
                    t: REMOTE_UE_IP,
                    length: 5,
                    ins: 0,
                    ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 53)),
                }),
            },
        ],
        remote_ue_ctx_disconnected: vec![RemoteUeContext {
            t: REMOTE_UE_CTX,
            length: 41,
            ins: 1,
            user_id: RemoteUserId {
                t: REMOTE_USR_ID,
                length: 28,
                ins: 0,
                imsi: "901405101961875".to_string(),
                msisdn: Some("882285100125114".to_string()),
                imei: Some("8676480401323005".to_string()),
            },
            ue_ip: Some(RemoteUeIpInformation {
                t: REMOTE_UE_IP,
                length: 5,
                ins: 0,
                ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 54)),
            }),
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer, encoded);
}
