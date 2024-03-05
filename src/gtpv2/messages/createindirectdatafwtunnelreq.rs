use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_IND_DATA_FW_TUN_REQ: u8 = 166;

// Definition of GTPv2-C Create Indirect Data Forwarding Tunnel Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateIndirectDataForwardingTunnelRequest {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub mei: Option<Mei>,
    pub indication: Option<Indication>,
    pub fteid_control: Option<Fteid>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateIndirectDataForwardingTunnelRequest {
    fn default() -> CreateIndirectDataForwardingTunnelRequest {
        CreateIndirectDataForwardingTunnelRequest {
            header: Gtpv2Header {
                msgtype: CREATE_IND_DATA_FW_TUN_REQ,
                teid: Some(0),
                ..Default::default()
            },
            imsi: None,
            mei: None,
            indication: None,
            fteid_control: None,
            bearer_ctxs: vec![],
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CreateIndirectDataForwardingTunnelRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateIndirectDataForwardingTunnelRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_IND_DATA_FW_TUN_REQ {
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

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mei.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

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
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone())
                    };
                }
                InformationElement::Mei(j) => {
                    if let (0, true) = (j.ins, self.mei.is_none()) {
                        self.mei = Some(j.clone())
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone())
                    };
                }
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        mandatory = true;
                        self.bearer_ctxs.push(j.clone());
                    }
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX))
        }
    }
}

#[test]
fn test_create_indirect_data_fw_tunnel_req_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 139] = [
        0x48, 0xa6, 0x00, 0x87, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0x4d, 0x00, 0x0a, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x09, 0x00, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e,
        0x99, 0x89, 0x55, 0x5d, 0x00, 0x39, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09,
        0x00, 0x80, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x57, 0x00, 0x09, 0x01, 0x84,
        0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, 0x56, 0x57, 0x00, 0x09, 0x02, 0x8f, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x57, 0x57, 0x00, 0x09, 0x06, 0xa6, 0x3b, 0x95, 0x98, 0x5a,
        0x3e, 0x99, 0x89, 0x59, 0x03, 0x00, 0x01, 0x00, 0x64, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateIndirectDataForwardingTunnelRequest {
        header: Gtpv2Header {
            msgtype: CREATE_IND_DATA_FW_TUN_REQ,
            piggyback: false,
            message_prio: None,
            length: 135,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Some(Imsi {
            length: 8,
            imsi: "901405101961873".to_string(),
            ..Default::default()
        }),
        mei: Some(Mei {
            mei: "8676480401323003".to_string(),
            ..Default::default()
        }),
        indication: Some(Indication {
            dtf: true,
            ..Default::default()
        }),
        fteid_control: Some(Fteid {
            length: 9,
            ins: 0,
            interface: 5,
            teid: 0x3b95985a,
            ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
            ..Default::default()
        }),

        bearer_ctxs: vec![BearerContext {
            length: 57,
            ebi: Ebi {
                value: 5,
                ..Default::default()
            },
            fteids: vec![
                Fteid {
                    length: 9,
                    ins: 0,
                    interface: 0,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
                    ipv6: None,
                    ..Default::default()
                },
                Fteid {
                    length: 9,
                    ins: 1,
                    interface: 4,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 86)),
                    ipv6: None,
                    ..Default::default()
                },
                Fteid {
                    length: 9,
                    ins: 2,
                    interface: 15,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 87)),
                    ipv6: None,
                    ..Default::default()
                },
                Fteid {
                    length: 9,
                    ins: 6,
                    interface: 38,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 89)),
                    ipv6: None,
                    ..Default::default()
                },
            ],
            ..BearerContext::default()
        }],
        recovery: Some(Recovery {
            recovery: 100,
            ..Default::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = CreateIndirectDataForwardingTunnelRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_indirect_data_fw_tunnel_req_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 139] = [
        0x48, 0xa6, 0x00, 0x87, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0x4d, 0x00, 0x0a, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x09, 0x00, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e,
        0x99, 0x89, 0x55, 0x5d, 0x00, 0x39, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09,
        0x00, 0x80, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x57, 0x00, 0x09, 0x01, 0x84,
        0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, 0x56, 0x57, 0x00, 0x09, 0x02, 0x8f, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x57, 0x57, 0x00, 0x09, 0x06, 0xa6, 0x3b, 0x95, 0x98, 0x5a,
        0x3e, 0x99, 0x89, 0x59, 0x03, 0x00, 0x01, 0x00, 0x64, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateIndirectDataForwardingTunnelRequest {
        header: Gtpv2Header {
            msgtype: CREATE_IND_DATA_FW_TUN_REQ,
            piggyback: false,
            message_prio: None,
            length: 135,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Some(Imsi {
            length: 8,
            imsi: "901405101961873".to_string(),
            ..Default::default()
        }),
        mei: Some(Mei {
            mei: "8676480401323003".to_string(),
            ..Default::default()
        }),
        indication: Some(Indication {
            dtf: true,
            ..Default::default()
        }),
        fteid_control: Some(Fteid {
            length: 9,
            ins: 0,
            interface: 5,
            teid: 0x3b95985a,
            ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
            ..Default::default()
        }),

        bearer_ctxs: vec![BearerContext {
            length: 57,
            ebi: Ebi {
                value: 5,
                ..Default::default()
            },
            fteids: vec![
                Fteid {
                    length: 9,
                    ins: 0,
                    interface: 0,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
                    ipv6: None,
                    ..Default::default()
                },
                Fteid {
                    length: 9,
                    ins: 1,
                    interface: 4,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 86)),
                    ipv6: None,
                    ..Default::default()
                },
                Fteid {
                    length: 9,
                    ins: 2,
                    interface: 15,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 87)),
                    ipv6: None,
                    ..Default::default()
                },
                Fteid {
                    length: 9,
                    ins: 6,
                    interface: 38,
                    teid: 0x3b95985a,
                    ipv4: Some(Ipv4Addr::new(62, 153, 137, 89)),
                    ipv6: None,
                    ..Default::default()
                },
            ],
            ..BearerContext::default()
        }],
        recovery: Some(Recovery {
            recovery: 100,
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
