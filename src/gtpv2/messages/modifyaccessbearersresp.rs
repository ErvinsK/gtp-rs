use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MODIFY_ACCESS_BRS_RESP: u8 = 212;

// Definition of GTPv2-C Modify Access Bearers Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyAccessBearersResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub indication: Option<Indication>,
    pub load_control_info: Option<LoadControl>,
    pub overload_control_info: Option<OverloadControlInfo>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ModifyAccessBearersResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MODIFY_ACCESS_BRS_RESP,
            teid: Some(0),
            ..Default::default()
        };
        ModifyAccessBearersResponse {
            header: hdr,
            cause: Cause::default(),
            bearer_ctxs: vec![],
            recovery: None,
            indication: None,
            load_control_info: None,
            overload_control_info: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ModifyAccessBearersResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyAccessBearersResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_ACCESS_BRS_RESP {
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

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.load_control_info.clone() {
            elements.push(InformationElement::LoadControlInfo(i.clone()));
        }

        if let Some(i) = self.overload_control_info.clone() {
            elements.push(InformationElement::OverloadControlInfo(i.clone()));
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
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
                    }
                }
                InformationElement::LoadControlInfo(j) => {
                    if let (0, true) = (j.ins, self.load_control_info.is_none()) {
                        self.load_control_info = Some(j.clone());
                    }
                }
                InformationElement::OverloadControlInfo(j) => {
                    if let (0, true) = (j.ins, self.overload_control_info.is_none()) {
                        self.overload_control_info = Some(j.clone());
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
fn test_modify_access_bearers_resp_unmarshal() {
    let encoded: [u8; 181] = [
        0x48, 0xd4, 0x00, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x5d, 0x00, 0x1f, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00, 0x16,
        0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x00, 0x1f, 0x01, 0x49, 0x00, 0x01,
        0x00, 0x06, 0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x01, 0x00, 0xaa, 0x4d, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6,
        0x00, 0x01, 0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03,
        0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x23, 0x00, 0xb7, 0x00, 0x04, 0x00,
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0x47,
        0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f,
        0x6d,
    ];
    let decoded = ModifyAccessBearersResponse {
        header: Gtpv2Header {
            msgtype: MODIFY_ACCESS_BRS_RESP,
            piggyback: false,
            message_prio: None,
            length: 177,
            teid: Some(0),
            sqn: 0x68,
        },
        indication: Some(Indication {
            israi: true,
            ..Indication::default()
        }),
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
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
        load_control_info: Some(LoadControl {
            t: LOAD_CNTRL,
            length: 31,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            load_metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            list: Some(vec![ApnRelativeCapacity {
                t: APN_REL_CAP,
                length: 14,
                ins: 0,
                relative_cap: 100,
                name: "test.net.com".to_string(),
            }]),
        }),
        overload_control_info: Some(OverloadControlInfo {
            t: OVERLOAD_CNTRL,
            length: 35,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            validity: EpcTimer {
                t: EPC_TIMER,
                length: EPC_TIMER_LENGTH as u16,
                ins: 0,
                timer_unit: 3,
                timer_value: 31,
            },
            list: Some(vec![Apn {
                t: APN,
                length: 13,
                ins: 0,
                name: "test.net.com".to_string(),
            }]),
        }),
        ..ModifyAccessBearersResponse::default()
    };
    let message = ModifyAccessBearersResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_modify_access_bearers_resp_marshal() {
    let encoded: [u8; 181] = [
        0x48, 0xd4, 0x00, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x5d, 0x00, 0x1f, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x50, 0x00, 0x16,
        0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x00, 0x1f, 0x01, 0x49, 0x00, 0x01,
        0x00, 0x06, 0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,
        0x01, 0x00, 0xaa, 0x4d, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6,
        0x00, 0x01, 0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03,
        0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x23, 0x00, 0xb7, 0x00, 0x04, 0x00,
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0x47,
        0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f,
        0x6d,
    ];
    let decoded = ModifyAccessBearersResponse {
        header: Gtpv2Header {
            msgtype: MODIFY_ACCESS_BRS_RESP,
            piggyback: false,
            message_prio: None,
            length: 177,
            teid: Some(0),
            sqn: 0x68,
        },
        indication: Some(Indication {
            israi: true,
            ..Indication::default()
        }),
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
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
        load_control_info: Some(LoadControl {
            t: LOAD_CNTRL,
            length: 31,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            load_metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            list: Some(vec![ApnRelativeCapacity {
                t: APN_REL_CAP,
                length: 14,
                ins: 0,
                relative_cap: 100,
                name: "test.net.com".to_string(),
            }]),
        }),
        overload_control_info: Some(OverloadControlInfo {
            t: OVERLOAD_CNTRL,
            length: 35,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            validity: EpcTimer {
                t: EPC_TIMER,
                length: EPC_TIMER_LENGTH as u16,
                ins: 0,
                timer_unit: 3,
                timer_value: 31,
            },
            list: Some(vec![Apn {
                t: APN,
                length: 13,
                ins: 0,
                name: "test.net.com".to_string(),
            }]),
        }),
        ..ModifyAccessBearersResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
