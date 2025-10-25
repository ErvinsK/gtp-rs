use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const RELEASE_ACCESS_BRS_RESP: u8 = 171;

// Definition of GTPv2-C Release Access Bearers Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAccessBearersResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub indication: Option<Indication>,
    pub load_control: Vec<LoadControl>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ReleaseAccessBearersResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: RELEASE_ACCESS_BRS_RESP,
            teid: Some(0),
            ..Default::default()
        };
        ReleaseAccessBearersResponse {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            indication: None,
            load_control: vec![],
            overload_info: vec![],
            private_ext: vec![],
        }
    }
}

impl Messages for ReleaseAccessBearersResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ReleaseAccessBearersResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != RELEASE_ACCESS_BRS_RESP {
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

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        self.load_control
            .iter()
            .for_each(|x| elements.push(InformationElement::LoadControlInfo(x.clone())));

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

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
                    };
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::LoadControlInfo(j) => {
                    if j.ins < 3 {
                        self.load_control.push(j.clone())
                    };
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 2 {
                        self.overload_info.push(j.clone())
                    };
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
fn test_release_access_bearers_resp_unmarshal() {
    let encoded: [u8; 116] = [
        0x48, 0xab, 0x00, 0x70, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x08, 0x4d, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00,
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04,
        0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x12,
        0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c,
        0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee,
        0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let decoded = ReleaseAccessBearersResponse {
        header: Gtpv2Header {
            msgtype: RELEASE_ACCESS_BRS_RESP,
            piggyback: false,
            message_prio: None,
            length: 112,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            value: 16,
            ..Cause::default()
        },
        recovery: Some(Recovery {
            recovery: 8,
            ..Recovery::default()
        }),
        indication: Some(Indication {
            sgwci: true,
            ..Indication::default()
        }),
        load_control: vec![LoadControl {
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
        }],
        overload_info: vec![
            OverloadControlInfo {
                t: OVERLOAD_CNTRL,
                length: 18,
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
                list: None,
            },
            OverloadControlInfo {
                t: OVERLOAD_CNTRL,
                length: 18,
                ins: 1,
                sqn: Sqn {
                    t: SQN,
                    length: SQN_LENGTH as u16,
                    ins: 0,
                    sqn: 0xffaaee22,
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
                    timer_value: 30,
                },
                list: None,
            },
        ],
        ..ReleaseAccessBearersResponse::default()
    };
    let message = ReleaseAccessBearersResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_release_access_bearers_resp_marshal() {
    let encoded: [u8; 116] = [
        0x48, 0xab, 0x00, 0x70, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x08, 0x4d, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00,
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04,
        0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x12,
        0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c,
        0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee,
        0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let decoded = ReleaseAccessBearersResponse {
        header: Gtpv2Header {
            msgtype: RELEASE_ACCESS_BRS_RESP,
            piggyback: false,
            message_prio: None,
            length: 112,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            value: 16,
            ..Cause::default()
        },
        recovery: Some(Recovery {
            recovery: 8,
            ..Recovery::default()
        }),
        indication: Some(Indication {
            sgwci: true,
            ..Indication::default()
        }),
        load_control: vec![LoadControl {
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
        }],
        overload_info: vec![
            OverloadControlInfo {
                t: OVERLOAD_CNTRL,
                length: 18,
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
                list: None,
            },
            OverloadControlInfo {
                t: OVERLOAD_CNTRL,
                length: 18,
                ins: 1,
                sqn: Sqn {
                    t: SQN,
                    length: SQN_LENGTH as u16,
                    ins: 0,
                    sqn: 0xffaaee22,
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
                    timer_value: 30,
                },
                list: None,
            },
        ],
        ..ReleaseAccessBearersResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
