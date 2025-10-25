use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const PGW_RESTART_NOTIF: u8 = 180;

// Definition of GTPv2-C PGW Restart Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgwRestartNotification {
    pub header: Gtpv2Header,
    pub pgw_addr_control: IpAddress,
    pub sgw_addr_control: IpAddress,
    pub cause: Option<Cause>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for PgwRestartNotification {
    fn default() -> PgwRestartNotification {
        PgwRestartNotification {
            header: Gtpv2Header {
                msgtype: PGW_RESTART_NOTIF,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            pgw_addr_control: IpAddress::default(),
            sgw_addr_control: IpAddress::default(),
            cause: None,
            private_ext: vec![],
        }
    }
}

impl Messages for PgwRestartNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = PgwRestartNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != PGW_RESTART_NOTIF {
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

        elements.push(self.pgw_addr_control.clone().into());
        elements.push(self.sgw_addr_control.clone().into());

        if let Some(i) = self.cause.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 2] = [false, false];
        for e in elements.iter() {
            match e {
                InformationElement::IpAddress(j) => match (j.ins, mandatory[0], mandatory[1]) {
                    (0, false, _) => {
                        mandatory[0] = true;
                        self.pgw_addr_control = j.clone();
                    }
                    (1, _, false) => {
                        mandatory[1] = true;
                        self.sgw_addr_control = j.clone();
                    }
                    _ => (),
                },
                InformationElement::Cause(j) => {
                    if j.ins == 0 {
                        self.cause = Some(j.clone());
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory[0] && mandatory[1] {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(IP_ADDRESS))
        }
    }
}

#[test]
fn test_pgw_restart_notif_unmarshal() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 56] = [
        0x48, 0xb4, 0x00, 0x34, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x4a, 0x00, 0x04,
        0x00, 0x64, 0x14, 0x14, 0x0a, 0x4a, 0x00, 0x10, 0x01, 0x00, 0xfd, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02, 0x00, 0x10,
        0x00, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = PgwRestartNotification {
        header: Gtpv2Header {
            msgtype: PGW_RESTART_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 52,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        pgw_addr_control: IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(100, 20, 20, 10)),
            ..IpAddress::default()
        },
        sgw_addr_control: IpAddress {
            length: 16,
            ins: 1,
            ip: IpAddr::V6(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0)),
            ..IpAddress::default()
        },
        cause: Some(Cause {
            value: 0x10,
            ..Default::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = PgwRestartNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_pgw_restart_notif_marshal() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 56] = [
        0x48, 0xb4, 0x00, 0x34, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x4a, 0x00, 0x04,
        0x00, 0x64, 0x14, 0x14, 0x0a, 0x4a, 0x00, 0x10, 0x01, 0x00, 0xfd, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02, 0x00, 0x10,
        0x00, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = PgwRestartNotification {
        header: Gtpv2Header {
            msgtype: PGW_RESTART_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 52,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        pgw_addr_control: IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(100, 20, 20, 10)),
            ..IpAddress::default()
        },
        sgw_addr_control: IpAddress {
            length: 16,
            ins: 1,
            ip: IpAddr::V6(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0)),
            ..IpAddress::default()
        },
        cause: Some(Cause {
            value: 0x10,
            ..Default::default()
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
