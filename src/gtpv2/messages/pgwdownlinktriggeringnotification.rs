use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const PGW_DL_TRIGGER_NOTIF: u8 = 103;

// Definition of GTPv2-C PGW Downlink Triggering Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgwDownlinkTriggeringNotification {
    pub header: Gtpv2Header,
    pub imsi: Imsi,
    pub mme_sgsn_id: Option<IpAddress>,
    pub pgw_fteid: Option<Fteid>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for PgwDownlinkTriggeringNotification {
    fn default() -> PgwDownlinkTriggeringNotification {
        PgwDownlinkTriggeringNotification {
            header: Gtpv2Header {
                msgtype: PGW_DL_TRIGGER_NOTIF,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            imsi: Imsi::default(),
            mme_sgsn_id: None,
            pgw_fteid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for PgwDownlinkTriggeringNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = PgwDownlinkTriggeringNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != PGW_DL_TRIGGER_NOTIF {
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

        if let Some(i) = self.mme_sgsn_id.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.pgw_fteid.clone() {
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
                InformationElement::Imsi(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        self.imsi = j.clone();
                        mandatory = true;
                    };
                }
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.mme_sgsn_id.is_none()) {
                        self.mme_sgsn_id = Some(j.clone());
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.pgw_fteid.is_none()) {
                        self.pgw_fteid = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
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
fn test_pgw_downlink_triggering_notif_unmarshal() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 71] = [
        0x48, 0x67, 0x00, 0x43, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x4a, 0x00, 0x04, 0x00, 0x64, 0x14,
        0x14, 0x0a, 0x57, 0x00, 0x19, 0x00, 0xc7, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2,
        0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x27, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = PgwDownlinkTriggeringNotification {
        header: Gtpv2Header {
            msgtype: PGW_DL_TRIGGER_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 67,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        },
        mme_sgsn_id: Some(IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(100, 20, 20, 10)),
            ..IpAddress::default()
        }),
        pgw_fteid: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 7,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = PgwDownlinkTriggeringNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_pgw_downlink_triggering_notif_marshal() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 71] = [
        0x48, 0x67, 0x00, 0x43, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x4a, 0x00, 0x04, 0x00, 0x64, 0x14,
        0x14, 0x0a, 0x57, 0x00, 0x19, 0x00, 0xc7, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2,
        0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x27, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = PgwDownlinkTriggeringNotification {
        header: Gtpv2Header {
            msgtype: PGW_DL_TRIGGER_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 67,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        },
        mme_sgsn_id: Some(IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(100, 20, 20, 10)),
            ..IpAddress::default()
        }),
        pgw_fteid: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 7,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer, encoded);
}
