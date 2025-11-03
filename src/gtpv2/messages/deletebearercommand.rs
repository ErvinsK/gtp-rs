use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DELETE_BEARER_CMD: u8 = 66;

// Definition of GTPv2-C Delete Bearer Command Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteBearerCommand {
    pub header: Gtpv2Header,
    pub bearer_ctxs: Vec<BearerContext>,
    pub uli: Option<Uli>,
    pub uli_timestamp: Option<UliTimestamp>,
    pub uetimezone: Option<UeTimeZone>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub fteid_control: Option<Fteid>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DeleteBearerCommand {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: DELETE_BEARER_CMD,
            teid: Some(0),
            ..Default::default()
        };

        DeleteBearerCommand {
            header: hdr,
            bearer_ctxs: vec![],
            uli: None,
            uli_timestamp: None,
            uetimezone: None,
            overload_info: vec![],
            fteid_control: None,
            secondary_rat_usage_report: vec![],
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DeleteBearerCommand {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteBearerCommand::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_BEARER_CMD {
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

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.uli.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.uli_timestamp.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.uetimezone.clone() {
            elements.push(i.into());
        }

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.fteid_control.clone() {
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
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        self.bearer_ctxs.push(j.clone());
                        mandatory = true;
                    }
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.uli.is_none()) {
                        self.uli = Some(j.clone());
                    }
                }
                InformationElement::UliTimestamp(j) => {
                    if let (0, true) = (j.ins, self.uli_timestamp.is_none()) {
                        self.uli_timestamp = Some(j.clone());
                    }
                }
                InformationElement::UeTimeZone(j) => {
                    if let (0, true) = (j.ins, self.uetimezone.is_none()) {
                        self.uetimezone = Some(j.clone());
                    }
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 2 {
                        self.overload_info.push(j.clone());
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone());
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
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX))
        }
    }
}

#[test]
fn test_delete_bearer_cmd_unmarshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 156] = [
        0x48, 0x42, 0x00, 0x98, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x5d, 0x00, 0x22,
        0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x19, 0x01, 0xc4, 0x23, 0xed, 0x38, 0x25,
        0xd9, 0xab, 0x8d, 0xf3, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x28, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x32, 0xf4, 0x02, 0x0d, 0x59,
        0x32, 0xf4, 0x02, 0x00, 0xc5, 0x58, 0x02, 0x72, 0x00, 0x02, 0x00, 0x00, 0x00, 0xb4, 0x00,
        0x12, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60,
        0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa,
        0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e, 0x57, 0x00, 0x19,
        0x00, 0xc6, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0xff, 0x00, 0x06, 0x00,
        0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = DeleteBearerCommand {
        header: Gtpv2Header {
            msgtype: DELETE_BEARER_CMD,
            piggyback: false,
            message_prio: None,
            length: 152,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 234,
                    mnc: 20,
                    mnc_is_three_digits: false,
                    tac: 0x0d59,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 234,
                    mnc: 20,
                    mnc_is_three_digits: false,
                    eci: 12933122,
                }),
            ],
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 6,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        uetimezone: Some(UeTimeZone {
            t: UETIMEZONE,
            length: 2,
            ins: 0,
            time_zone: 0,
            dst: 0,
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 34,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: None,
            bearer_flags: None,
            pco: None,
            apco: None,
            epco: None,
            max_packet_loss: None,
            ran_nas_cause: None,
            ebi: Ebi {
                t: EBI,
                length: 1,
                ins: 0,
                value: 5,
            },
            fteids: vec![Fteid {
                t: FTEID,
                length: 25,
                ins: 1,
                interface: 4,
                teid: 0x23ed3825,
                ipv4: Some(Ipv4Addr::new(217, 171, 141, 243)),
                ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x28)),
            }],
            bearer_qos: None,
            ..BearerContext::default()
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
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..DeleteBearerCommand::default()
    };
    let message = DeleteBearerCommand::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_delete_bearer_cmd_marshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 156] = [
        0x48, 0x42, 0x00, 0x98, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x5d, 0x00, 0x22,
        0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x19, 0x01, 0xc4, 0x23, 0xed, 0x38, 0x25,
        0xd9, 0xab, 0x8d, 0xf3, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x28, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x32, 0xf4, 0x02, 0x0d, 0x59,
        0x32, 0xf4, 0x02, 0x00, 0xc5, 0x58, 0x02, 0x72, 0x00, 0x02, 0x00, 0x00, 0x00, 0xb4, 0x00,
        0x12, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60,
        0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa,
        0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e, 0x57, 0x00, 0x19,
        0x00, 0xc6, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0xff, 0x00, 0x06, 0x00,
        0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = DeleteBearerCommand {
        header: Gtpv2Header {
            msgtype: DELETE_BEARER_CMD,
            piggyback: false,
            message_prio: None,
            length: 152,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 234,
                    mnc: 20,
                    mnc_is_three_digits: false,
                    tac: 0x0d59,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 234,
                    mnc: 20,
                    mnc_is_three_digits: false,
                    eci: 12933122,
                }),
            ],
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 6,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        uetimezone: Some(UeTimeZone {
            t: UETIMEZONE,
            length: 2,
            ins: 0,
            time_zone: 0,
            dst: 0,
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 34,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: None,
            bearer_flags: None,
            pco: None,
            apco: None,
            epco: None,
            max_packet_loss: None,
            ran_nas_cause: None,
            ebi: Ebi {
                t: EBI,
                length: 1,
                ins: 0,
                value: 5,
            },
            fteids: vec![Fteid {
                t: FTEID,
                length: 25,
                ins: 1,
                interface: 4,
                teid: 0x23ed3825,
                ipv4: Some(Ipv4Addr::new(217, 171, 141, 243)),
                ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x28)),
            }],
            bearer_qos: None,
            ..BearerContext::default()
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
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..DeleteBearerCommand::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
