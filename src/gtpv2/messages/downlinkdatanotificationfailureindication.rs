use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DL_DATA_NOTIF_FAIL_IND: u8 = 70;

// Definition of GTPv2-C Downlink Data Notification Failure Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownlinkDataNotificationFailureIndication {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub orig_node: Option<NodeType>,
    pub imsi: Option<Imsi>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DownlinkDataNotificationFailureIndication {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: DL_DATA_NOTIF_FAIL_IND,
            teid: Some(0),
            ..Default::default()
        };
        DownlinkDataNotificationFailureIndication {
            header: hdr,
            cause: Cause::default(),
            orig_node: None,
            imsi: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DownlinkDataNotificationFailureIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DownlinkDataNotificationFailureIndication::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DL_DATA_NOTIF_FAIL_IND {
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

        elements.push(self.cause.clone().into());

        if let Some(i) = self.orig_node.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.cause, mandatory) = (j.clone(), true)
                    }
                }
                InformationElement::NodeType(j) => {
                    if let (0, true) = (j.ins, self.orig_node.is_none()) {
                        self.orig_node = Some(j.clone());
                    }
                }
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE))
        }
    }
}

#[test]
fn test_dl_data_notification_fail_ind_unmarshal() {
    let encoded: [u8; 45] = [
        0x48, 0x46, 0x00, 0x29, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x87, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50,
        0x01, 0x01, 0x37, 0x78, 0xf4, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = DownlinkDataNotificationFailureIndication {
        header: Gtpv2Header {
            msgtype: DL_DATA_NOTIF_FAIL_IND,
            piggyback: false,
            message_prio: None,
            length: 41,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        cause: Cause {
            value: 0x10,
            ..Default::default()
        },
        orig_node: Some(NodeType {
            node: Node::Mme,
            ..NodeType::default()
        }),
        imsi: Some(Imsi {
            length: 8,
            imsi: "901405101073874".to_string(),
            ..Imsi::default()
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let message = DownlinkDataNotificationFailureIndication::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_dl_data_notif_fail_ind_marshal() {
    let encoded: [u8; 45] = [
        0x48, 0x46, 0x00, 0x29, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x87, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x09, 0x41, 0x50,
        0x01, 0x01, 0x37, 0x78, 0xf4, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = DownlinkDataNotificationFailureIndication {
        header: Gtpv2Header {
            msgtype: DL_DATA_NOTIF_FAIL_IND,
            piggyback: false,
            message_prio: None,
            length: 41,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        cause: Cause {
            value: 0x10,
            ..Default::default()
        },
        orig_node: Some(NodeType {
            node: Node::Mme,
            ..NodeType::default()
        }),
        imsi: Some(Imsi {
            length: 8,
            imsi: "901405101073874".to_string(),
            ..Imsi::default()
        }),
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
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
