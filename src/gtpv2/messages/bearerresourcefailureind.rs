use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const BEARER_RSRC_FAIL: u8 = 69;

// Definition of GTPv2-C Bearer Resource Failure Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerResourceFailureInd {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub linked_ebi: Ebi,
    pub pti: Pti,
    pub indication: Option<Indication>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub recovery: Option<Recovery>,
    pub nbifom: Option<Fcontainer>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for BearerResourceFailureInd {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: BEARER_RSRC_FAIL,
            teid: Some(0),
            ..Default::default()
        };
        BearerResourceFailureInd {
            header: hdr,
            cause: Cause::default(),
            linked_ebi: Ebi::default(),
            pti: Pti::default(),
            indication: None,
            overload_info: vec![],
            recovery: None,
            nbifom: None,
            private_ext: vec![],
        }
    }
}

impl Messages for BearerResourceFailureInd {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = BearerResourceFailureInd::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != BEARER_RSRC_FAIL {
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

        elements.push(self.linked_ebi.clone().into());

        elements.push(self.pti.clone().into());

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));
        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 3] = [false; 3];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        (self.cause, mandatory[0]) = (j.clone(), true)
                    };
                }
                InformationElement::Ebi(j) => {
                    if let (0, false) = (j.ins, mandatory[1]) {
                        (self.linked_ebi, mandatory[1]) = (j.clone(), true)
                    };
                }
                InformationElement::Pti(j) => {
                    if let (0, false) = (j.ins, mandatory[2]) {
                        (self.pti, mandatory[2]) = (j.clone(), true)
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 2 {
                        self.overload_info.push(j.clone())
                    };
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::Fcontainer(j) => {
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1], mandatory[2]) {
            (false, _, _) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (true, false, _) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (true, true, false) => Err(GTPV2Error::MessageMandatoryIEMissing(PTI)),
            (true, true, true) => Ok(true),
        }
    }
}

#[test]
fn test_bearer_failure_ind_unmarshal() {
    let encoded: [u8; 72] = [
        0x48, 0x45, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x4d, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x64, 0x00, 0x01, 0x00, 0xff, 0xb4, 0x00,
        0x12, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60,
        0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa,
        0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let decoded = BearerResourceFailureInd {
        header: Gtpv2Header {
            msgtype: BEARER_RSRC_FAIL,
            piggyback: false,
            message_prio: None,
            length: 68,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 77,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        linked_ebi: Ebi {
            t: 73,
            length: 1,
            ins: 0,
            value: 5,
        },
        pti: Pti {
            t: PTI,
            length: 1,
            ins: 0,
            pti: 0xff,
        },
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
        ..BearerResourceFailureInd::default()
    };
    let message = BearerResourceFailureInd::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_bearer_failure_ind_marshal() {
    let encoded: [u8; 72] = [
        0x48, 0x45, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x4d, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x64, 0x00, 0x01, 0x00, 0xff, 0xb4, 0x00,
        0x12, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60,
        0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa,
        0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let decoded = BearerResourceFailureInd {
        header: Gtpv2Header {
            msgtype: BEARER_RSRC_FAIL,
            piggyback: false,
            message_prio: None,
            length: 68,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 77,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        linked_ebi: Ebi {
            t: 73,
            length: 1,
            ins: 0,
            value: 5,
        },
        pti: Pti {
            t: PTI,
            length: 1,
            ins: 0,
            pti: 0xff,
        },
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
        ..BearerResourceFailureInd::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
