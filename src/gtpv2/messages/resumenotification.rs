use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const RESUME_NOTIF: u8 = 164;

// Definition of GTPv2-C Resume Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeNotification {
    pub header: Gtpv2Header,
    pub imsi: Imsi,
    pub linked_ebi: Option<Ebi>,
    pub orig_node: Option<NodeType>,
    pub fteid_control: Option<Fteid>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ResumeNotification {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: RESUME_NOTIF,
            teid: Some(0),
            ..Default::default()
        };
        ResumeNotification {
            header: hdr,
            imsi: Imsi::default(),
            linked_ebi: None,
            orig_node: None,
            fteid_control: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ResumeNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ResumeNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != RESUME_NOTIF {
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

        elements.push(self.imsi.clone().into());

        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.orig_node.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.imsi, mandatory) = (j, true);
                    }
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.linked_ebi.is_none()) {
                        self.linked_ebi = Some(j);
                    }
                }
                InformationElement::NodeType(j) => {
                    if let (0, true) = (j.ins, self.orig_node.is_none()) {
                        self.orig_node = Some(j);
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(IMSI))
        }
    }
}

#[test]
fn test_resume_notification_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 42] = [
        0x48, 0xa4, 0x00, 0x26, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57,
        0x00, 0x09, 0x00, 0x86, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, 0x2d,
    ];
    let decoded = ResumeNotification {
        header: Gtpv2Header {
            msgtype: RESUME_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 38,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Imsi {
            t: 0x01,
            length: 0x08,
            ins: 0x00,
            imsi: "901405101961873".to_string(),
        },
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        }),
        ..ResumeNotification::default()
    };
    let message = ResumeNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_resume_notification_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 42] = [
        0x48, 0xa4, 0x00, 0x26, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57,
        0x00, 0x09, 0x00, 0x86, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, 0x2d,
    ];
    let decoded = ResumeNotification {
        header: Gtpv2Header {
            msgtype: RESUME_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 38,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Imsi {
            t: 0x01,
            length: 0x08,
            ins: 0x00,
            imsi: "901405101961873".to_string(),
        },
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        }),
        ..ResumeNotification::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
