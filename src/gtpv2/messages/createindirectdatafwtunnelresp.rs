use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_IND_DATA_FW_TUN_RESP: u8 = 167;

// Definition of GTPv2-C Create Indirect Data Forwarding Tunnel Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateIndirectDataForwardingTunnelResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub fteid_control: Option<Fteid>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateIndirectDataForwardingTunnelResponse {
    fn default() -> CreateIndirectDataForwardingTunnelResponse {
        CreateIndirectDataForwardingTunnelResponse {
            header: Gtpv2Header {
                msgtype: CREATE_IND_DATA_FW_TUN_RESP,
                teid: Some(0),
                ..Default::default()
            },
            cause: Cause::default(),
            fteid_control: None,
            bearer_ctxs: vec![],
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CreateIndirectDataForwardingTunnelResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateIndirectDataForwardingTunnelResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_IND_DATA_FW_TUN_RESP {
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
        let mut mandatory: [bool; 2] = [false, false];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if j.ins == 0 {
                        mandatory[0] = true;
                        self.cause = j.clone();
                    };
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone())
                    };
                }
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        mandatory[1] = true;
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
        match mandatory {
            [true, true] => Ok(true),
            [false, true] => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            [true, false] => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)),
            [false, false] => Err(GTPV2Error::MessageMandatoryIEMissing(0)),
        }
    }
}

#[test]
fn test_create_indirect_data_fw_tunnel_resp_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 107] = [
        0x48, 0xa7, 0x00, 0x67, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x09, 0x00, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89,
        0x55, 0x5d, 0x00, 0x39, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x00, 0x80,
        0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x57, 0x00, 0x09, 0x01, 0x84, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x56, 0x57, 0x00, 0x09, 0x02, 0x8f, 0x3b, 0x95, 0x98, 0x5a,
        0x3e, 0x99, 0x89, 0x57, 0x57, 0x00, 0x09, 0x06, 0xa6, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99,
        0x89, 0x59, 0x03, 0x00, 0x01, 0x00, 0x64, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62,
        0x9c, 0xc4,
    ];
    let decoded = CreateIndirectDataForwardingTunnelResponse {
        header: Gtpv2Header {
            msgtype: CREATE_IND_DATA_FW_TUN_RESP,
            piggyback: false,
            message_prio: None,
            length: 103,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Default::default()
        },
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
    let message = CreateIndirectDataForwardingTunnelResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_indirect_data_fw_tunnel_resp_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 107] = [
        0x48, 0xa7, 0x00, 0x67, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x09, 0x00, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89,
        0x55, 0x5d, 0x00, 0x39, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x00, 0x80,
        0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x57, 0x00, 0x09, 0x01, 0x84, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x56, 0x57, 0x00, 0x09, 0x02, 0x8f, 0x3b, 0x95, 0x98, 0x5a,
        0x3e, 0x99, 0x89, 0x57, 0x57, 0x00, 0x09, 0x06, 0xa6, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99,
        0x89, 0x59, 0x03, 0x00, 0x01, 0x00, 0x64, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62,
        0x9c, 0xc4,
    ];
    let decoded = CreateIndirectDataForwardingTunnelResponse {
        header: Gtpv2Header {
            msgtype: CREATE_IND_DATA_FW_TUN_RESP,
            piggyback: false,
            message_prio: None,
            length: 103,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Default::default()
        },
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
    assert_eq!(buffer, encoded);
}
