use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MODIFY_BEARER_FAIL_IND: u8 = 65;

// Definition of GTPv2-C Modify Bearer Failure Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyBearerFailureInd {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub indication: Option<Indication>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ModifyBearerFailureInd {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MODIFY_BEARER_FAIL_IND,
            teid: Some(0),
            ..Default::default()
        };
        ModifyBearerFailureInd {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            indication: None,
            overload_info: vec![],
            private_ext: vec![],
        }
    }
}

impl Messages for ModifyBearerFailureInd {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyBearerFailureInd::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_BEARER_FAIL_IND {
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
            elements.push(i.into());
        }
        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: bool = false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.cause, mandatory) = (j.clone(), true);
                    }
                }
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
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 2 {
                        self.overload_info.push(j.clone());
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
fn test_modify_bearer_failure_unmarshal() {
    let encoded: [u8; 81] = [
        0x48, 0x41, 0x00, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x0e, 0x00, 0x03, 0x00, 0x01, 0x00, 0x04, 0x4d, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04, 0x00,
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4,
        0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x22, 0xb6, 0x00, 0x01, 0x00,
        0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let decoded = ModifyBearerFailureInd {
        header: Gtpv2Header {
            msgtype: MODIFY_BEARER_FAIL_IND,
            piggyback: false,
            message_prio: None,
            length: 77,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 14,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        indication: Some(Indication {
            tspcmi: true,
            sgwci: true,
            ..Indication::default()
        }),
        recovery: Some(Recovery {
            t: RECOVERY,
            length: RECOVERY_LENGTH as u16,
            ins: 0,
            recovery: 4,
        }),
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
        ..ModifyBearerFailureInd::default()
    };
    let message = ModifyBearerFailureInd::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_modify_bearer_failure_marshal() {
    let encoded: [u8; 81] = [
        0x48, 0x41, 0x00, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x0e, 0x00, 0x03, 0x00, 0x01, 0x00, 0x04, 0x4d, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04, 0x00,
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4,
        0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x22, 0xb6, 0x00, 0x01, 0x00,
        0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let decoded = ModifyBearerFailureInd {
        header: Gtpv2Header {
            msgtype: MODIFY_BEARER_FAIL_IND,
            piggyback: false,
            message_prio: None,
            length: 77,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 14,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        indication: Some(Indication {
            tspcmi: true,
            sgwci: true,
            ..Indication::default()
        }),
        recovery: Some(Recovery {
            t: RECOVERY,
            length: RECOVERY_LENGTH as u16,
            ins: 0,
            recovery: 4,
        }),
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
        ..ModifyBearerFailureInd::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
