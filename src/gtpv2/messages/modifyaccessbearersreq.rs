use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MODIFY_ACCESS_BRS_REQ: u8 = 211;

// Definition of GTPv2-C Modify Access Bearers Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyAccessBearersRequest {
    pub header: Gtpv2Header,
    pub indication: Option<Indication>,
    pub fteid_control: Option<Fteid>,
    pub delay_dl_pckt_notif_req: Option<DelayValue>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ModifyAccessBearersRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MODIFY_ACCESS_BRS_REQ,
            teid: Some(0),
            ..Default::default()
        };
        ModifyAccessBearersRequest {
            header: hdr,
            indication: None,
            fteid_control: None,
            delay_dl_pckt_notif_req: None,
            bearer_ctxs: vec![],
            recovery: None,
            secondary_rat_usage_report: vec![],
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ModifyAccessBearersRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyAccessBearersRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_ACCESS_BRS_REQ {
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

        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.delay_dl_pckt_notif_req.clone() {
            elements.push(i.into());
        }

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into());
        }

        self.secondary_rat_usage_report.iter().for_each(|x| {
            elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone()))
        });

        if let Some(i) = self.pscellid.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.iter() {
            match e {
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone());
                    }
                }
                InformationElement::DelayValue(j) => {
                    if let (0, true) = (j.ins, self.delay_dl_pckt_notif_req.is_none()) {
                        self.delay_dl_pckt_notif_req = Some(j.clone());
                    }
                }
                InformationElement::BearerContext(j) => match j.ins {
                    i if i < 2 => self.bearer_ctxs.push(j.clone()),
                    _ => (),
                },
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone());
                    }
                }
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j.clone());
                    }
                }
                InformationElement::PSCellId(j) => {
                    if let (0, true) = (j.ins, self.pscellid.is_none()) {
                        self.pscellid = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_modify_access_bearers_req_unmarshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 197] = [
        0x48, 0xd3, 0x00, 0xc1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x4d, 0x00, 0x0a,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x19, 0x00,
        0xca, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x5c, 0x00, 0x01, 0x00, 0x05,
        0x5d, 0x00, 0x1f, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00, 0x16, 0x00, 0x64, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x00, 0x1f, 0x01, 0x49, 0x00, 0x01, 0x00, 0x06, 0x50,
        0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0xaa,
        0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x06, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff,
    ];
    let decoded = ModifyAccessBearersRequest {
        header: Gtpv2Header {
            msgtype: MODIFY_ACCESS_BRS_REQ,
            piggyback: false,
            message_prio: None,
            length: 193,
            teid: Some(0),
            sqn: 0x68,
        },
        indication: Some(Indication {
            israi: true,
            ..Indication::default()
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        delay_dl_pckt_notif_req: Some(DelayValue {
            value: 5,
            ..DelayValue::default()
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
                ins: 1,
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
        recovery: Some(Recovery {
            recovery: 0xaa,
            ..Recovery::default()
        }),
        secondary_rat_usage_report: vec![
            SecondaryRatUsageDataReport {
                t: SCND_RAT_UDR,
                length: SCND_RAT_UDR_LENGTH as u16,
                ins: 0,
                irsgw: true,
                irpgw: true,
                rat_type: 0,
                ebi: 5,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
            SecondaryRatUsageDataReport {
                t: SCND_RAT_UDR,
                length: SCND_RAT_UDR_LENGTH as u16,
                ins: 0,
                irsgw: true,
                irpgw: true,
                rat_type: 0,
                ebi: 6,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
        ],
        ..ModifyAccessBearersRequest::default()
    };
    let message = ModifyAccessBearersRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_modify_access_bearers_req_marshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 197] = [
        0x48, 0xd3, 0x00, 0xc1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x4d, 0x00, 0x0a,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x19, 0x00,
        0xca, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x5c, 0x00, 0x01, 0x00, 0x05,
        0x5d, 0x00, 0x1f, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00, 0x16, 0x00, 0x64, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x00, 0x1f, 0x01, 0x49, 0x00, 0x01, 0x00, 0x06, 0x50,
        0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0xaa,
        0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x06, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff,
    ];
    let decoded = ModifyAccessBearersRequest {
        header: Gtpv2Header {
            msgtype: MODIFY_ACCESS_BRS_REQ,
            piggyback: false,
            message_prio: None,
            length: 193,
            teid: Some(0),
            sqn: 0x68,
        },
        indication: Some(Indication {
            israi: true,
            ..Indication::default()
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        delay_dl_pckt_notif_req: Some(DelayValue {
            value: 5,
            ..DelayValue::default()
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
                ins: 1,
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
        recovery: Some(Recovery {
            recovery: 0xaa,
            ..Recovery::default()
        }),
        secondary_rat_usage_report: vec![
            SecondaryRatUsageDataReport {
                t: SCND_RAT_UDR,
                length: SCND_RAT_UDR_LENGTH as u16,
                ins: 0,
                irsgw: true,
                irpgw: true,
                rat_type: 0,
                ebi: 5,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
            SecondaryRatUsageDataReport {
                t: SCND_RAT_UDR,
                length: SCND_RAT_UDR_LENGTH as u16,
                ins: 0,
                irsgw: true,
                irpgw: true,
                rat_type: 0,
                ebi: 6,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
        ],
        ..ModifyAccessBearersRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
