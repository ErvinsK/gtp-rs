use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const FWD_RELOC_RESP: u8 = 134;

// Definition of GTPv2-C Forward Relocation Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardRelocationResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub fteid_control: Option<Fteid>,
    pub indication: Option<Indication>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub f_causes: Vec<Fcause>,
    pub f_containers: Vec<Fcontainer>,
    pub mme_sgsn_ldn: Option<Ldn>,
    pub sgsn_fqdn: Option<Fqdn>,
    pub mme_fqdn: Option<Fqdn>,
    pub node_nbrs: Vec<NodeNumber>,
    pub node_ids: Vec<NodeIdentifier>,
    pub srvcc_reject_cause: Option<SrvccCause>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ForwardRelocationResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: FWD_RELOC_RESP,
            teid: Some(0),
            ..Default::default()
        };
        ForwardRelocationResponse {
            header: hdr,
            cause: Cause::default(),
            fteid_control: None,
            indication: None,
            bearer_ctxs: vec![],
            f_causes: vec![],
            f_containers: vec![],
            mme_sgsn_ldn: None,
            sgsn_fqdn: None,
            mme_fqdn: None,
            node_nbrs: vec![],
            node_ids: vec![],
            srvcc_reject_cause: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ForwardRelocationResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ForwardRelocationResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != FWD_RELOC_RESP {
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
            elements.push(i.into());
        }

        if let Some(i) = self.indication.clone() {
            elements.push(InformationElement::Indication(i));
        }

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        self.f_causes
            .iter()
            .for_each(|x| elements.push(InformationElement::Fcause(x.clone())));

        self.f_containers
            .iter()
            .for_each(|x| elements.push(InformationElement::Fcontainer(x.clone())));

        if let Some(i) = self.mme_sgsn_ldn.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.sgsn_fqdn.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mme_fqdn.clone() {
            elements.push(i.into());
        }

        self.node_nbrs
            .iter()
            .for_each(|x| elements.push(InformationElement::NodeNumber(x.clone())));

        self.node_ids
            .iter()
            .for_each(|x| elements.push(InformationElement::NodeIdentifier(x.clone())));

        if let Some(i) = self.srvcc_reject_cause.clone() {
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
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        self.cause = j;
                        mandatory = true;
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, false) = (j.ins, self.fteid_control.is_some()) {
                        self.fteid_control = Some(j);
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j);
                    }
                }
                InformationElement::BearerContext(j) => {
                    if j.ins < 4 {
                        self.bearer_ctxs.push(j);
                    }
                }
                InformationElement::Fcause(j) => {
                    if j.ins < 3 {
                        self.f_causes.push(j);
                    }
                }
                InformationElement::Fcontainer(j) => {
                    if j.ins < 3 {
                        self.f_containers.push(j);
                    }
                }
                InformationElement::Ldn(j) => {
                    if let (0, true) = (j.ins, self.mme_sgsn_ldn.is_none()) {
                        self.mme_sgsn_ldn = Some(j);
                    }
                }
                InformationElement::Fqdn(j) => {
                    match (j.ins, self.sgsn_fqdn.is_none(), self.mme_fqdn.is_none()) {
                        (0, true, _) => self.sgsn_fqdn = Some(j),
                        (1, _, true) => self.mme_fqdn = Some(j),
                        _ => (),
                    }
                }
                InformationElement::NodeNumber(j) => {
                    if j.ins < 3 {
                        self.node_nbrs.push(j);
                    }
                }
                InformationElement::NodeIdentifier(j) => {
                    if j.ins < 4 {
                        self.node_ids.push(j);
                    }
                }
                InformationElement::SrvccCause(j) => {
                    if let (0, true) = (j.ins, self.srvcc_reject_cause.is_none()) {
                        self.srvcc_reject_cause = Some(j);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
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
fn test_fwd_reloc_resp_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 129] = [
        0x48, 0x86, 0x00, 0x7d, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x09, 0x00, 0x86, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85,
        0xb8, 0x5d, 0x00, 0x1f, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00, 0x16, 0x00, 0x64,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x00, 0x1f, 0x03, 0x49, 0x00, 0x01, 0x00, 0x06,
        0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0x00, 0x0a, 0x00,
        0x09, 0x91, 0x99, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0xaf, 0x00, 0x0a, 0x02, 0x09,
        0x91, 0x99, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let decoded = ForwardRelocationResponse {
        header: Gtpv2Header {
            msgtype: FWD_RELOC_RESP,
            piggyback: false,
            message_prio: None,
            length: 125,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x27892f70,
            ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
            ipv6: None,
        }),
        bearer_ctxs: vec![
            BearerContext {
                ins: 0,
                ebi: Ebi {
                    t: EBI,
                    length: EBI_LENGTH as u16,
                    ins: 0,
                    value: 5,
                },
                length: 31,
                bearer_qos: Some(BearerQos {
                    t: BEARERQOS,
                    length: BEARERQOS_LENGTH as u16,
                    ins: 0,
                    pre_emption_vulnerability: 0,
                    priority_level: 9,
                    pre_emption_capability: 1,
                    qci: 9,
                    maxbr_ul: 0,
                    maxbr_dl: 0,
                    gbr_ul: 0,
                    gbr_dl: 0,
                }),
                ..BearerContext::default()
            },
            BearerContext {
                ins: 3,
                ebi: Ebi {
                    t: EBI,
                    length: EBI_LENGTH as u16,
                    ins: 0,
                    value: 6,
                },
                length: 31,
                bearer_qos: Some(BearerQos {
                    t: BEARERQOS,
                    length: BEARERQOS_LENGTH as u16,
                    ins: 0,
                    pre_emption_vulnerability: 0,
                    priority_level: 9,
                    pre_emption_capability: 1,
                    qci: 9,
                    maxbr_ul: 0,
                    maxbr_dl: 0,
                    gbr_ul: 0,
                    gbr_dl: 0,
                }),
                ..BearerContext::default()
            },
        ],
        node_nbrs: vec![
            NodeNumber {
                length: 0x0a,
                node_number: "991405101961873".to_string(),
                ..NodeNumber::default()
            },
            NodeNumber {
                length: 0x0a,
                ins: 2,
                node_number: "991405101961873".to_string(),
                ..NodeNumber::default()
            },
        ],
        ..ForwardRelocationResponse::default()
    };
    let message = ForwardRelocationResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_fwd_reloc_resp_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 129] = [
        0x48, 0x86, 0x00, 0x7d, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x09, 0x00, 0x86, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85,
        0xb8, 0x5d, 0x00, 0x1f, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00, 0x16, 0x00, 0x64,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x00, 0x1f, 0x03, 0x49, 0x00, 0x01, 0x00, 0x06,
        0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0x00, 0x0a, 0x00,
        0x09, 0x91, 0x99, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0xaf, 0x00, 0x0a, 0x02, 0x09,
        0x91, 0x99, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3,
    ];
    let decoded = ForwardRelocationResponse {
        header: Gtpv2Header {
            msgtype: FWD_RELOC_RESP,
            piggyback: false,
            message_prio: None,
            length: 125,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x27892f70,
            ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
            ipv6: None,
        }),
        bearer_ctxs: vec![
            BearerContext {
                ins: 0,
                ebi: Ebi {
                    t: EBI,
                    length: EBI_LENGTH as u16,
                    ins: 0,
                    value: 5,
                },
                length: 31,
                bearer_qos: Some(BearerQos {
                    t: BEARERQOS,
                    length: BEARERQOS_LENGTH as u16,
                    ins: 0,
                    pre_emption_vulnerability: 0,
                    priority_level: 9,
                    pre_emption_capability: 1,
                    qci: 9,
                    maxbr_ul: 0,
                    maxbr_dl: 0,
                    gbr_ul: 0,
                    gbr_dl: 0,
                }),
                ..BearerContext::default()
            },
            BearerContext {
                ins: 3,
                ebi: Ebi {
                    t: EBI,
                    length: EBI_LENGTH as u16,
                    ins: 0,
                    value: 6,
                },
                length: 31,
                bearer_qos: Some(BearerQos {
                    t: BEARERQOS,
                    length: BEARERQOS_LENGTH as u16,
                    ins: 0,
                    pre_emption_vulnerability: 0,
                    priority_level: 9,
                    pre_emption_capability: 1,
                    qci: 9,
                    maxbr_ul: 0,
                    maxbr_dl: 0,
                    gbr_ul: 0,
                    gbr_dl: 0,
                }),
                ..BearerContext::default()
            },
        ],
        node_nbrs: vec![
            NodeNumber {
                length: 0x0a,
                node_number: "991405101961873".to_string(),
                ..NodeNumber::default()
            },
            NodeNumber {
                length: 0x0a,
                ins: 2,
                node_number: "991405101961873".to_string(),
                ..NodeNumber::default()
            },
        ],
        ..ForwardRelocationResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
