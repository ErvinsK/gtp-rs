use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const PGW_DL_TRIGGER_ACK: u8 = 104;

// Definition of GTPv2-C PGW Downlink Triggering Acknowledge Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgwDownlinkTriggeringAcknowledge {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub imsi: Option<Imsi>,
    pub mme_sgsn_id: Option<IpAddress>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for PgwDownlinkTriggeringAcknowledge {
    fn default() -> PgwDownlinkTriggeringAcknowledge {
        PgwDownlinkTriggeringAcknowledge {
            header: Gtpv2Header {
                msgtype: PGW_DL_TRIGGER_ACK,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            cause: Cause::default(),
            imsi: None,
            mme_sgsn_id: None,
            private_ext: vec![],
        }
    }
}

impl Messages for PgwDownlinkTriggeringAcknowledge {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = PgwDownlinkTriggeringAcknowledge::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != PGW_DL_TRIGGER_ACK {
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

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mme_sgsn_id.clone() {
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
                        self.cause = j.clone();
                        mandatory = true;
                    };
                }
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone());
                    }
                }
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.mme_sgsn_id.is_none()) {
                        self.mme_sgsn_id = Some(j.clone());
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
fn test_pgw_downlink_triggering_ack_unmarshal() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 48] = [
        0x48, 0x68, 0x00, 0x2c, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x01, 0x00, 0x08, 0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3,
        0x4a, 0x00, 0x04, 0x00, 0x64, 0x14, 0x14, 0x0a, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01,
        0x62, 0x9c, 0xc4,
    ];
    let decoded = PgwDownlinkTriggeringAcknowledge {
        header: Gtpv2Header {
            msgtype: PGW_DL_TRIGGER_ACK,
            piggyback: false,
            message_prio: None,
            length: 44,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        imsi: Some(Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        }),
        mme_sgsn_id: Some(IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(100, 20, 20, 10)),
            ..IpAddress::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = PgwDownlinkTriggeringAcknowledge::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_pgw_downlink_triggering_ack_marshal() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 48] = [
        0x48, 0x68, 0x00, 0x2c, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x01, 0x00, 0x08, 0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3,
        0x4a, 0x00, 0x04, 0x00, 0x64, 0x14, 0x14, 0x0a, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01,
        0x62, 0x9c, 0xc4,
    ];
    let decoded = PgwDownlinkTriggeringAcknowledge {
        header: Gtpv2Header {
            msgtype: PGW_DL_TRIGGER_ACK,
            piggyback: false,
            message_prio: None,
            length: 44,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        imsi: Some(Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        }),
        mme_sgsn_id: Some(IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(100, 20, 20, 10)),
            ..IpAddress::default()
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
