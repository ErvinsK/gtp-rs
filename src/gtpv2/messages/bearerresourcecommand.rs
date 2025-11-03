use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const BEARER_RSRC_CMD: u8 = 68;

// Definition of GTPv2-C Bearer Resource Command Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerResourceCommand {
    pub header: Gtpv2Header,
    pub linked_ebi: Ebi,
    pub pti: Pti,
    pub flow_qos: Option<FlowQos>,
    pub tad: Option<TrafficAggregateDescription>,
    pub rattype: Option<RatType>,
    pub servingnetwork: Option<ServingNetwork>,
    pub uli: Option<Uli>,
    pub ebi: Option<Ebi>,
    pub indication: Option<Indication>,
    pub sgsn_fteid: Option<Fteid>,
    pub rnc_fteid: Option<Fteid>,
    pub pco: Option<Pco>,
    pub spi: Option<Spi>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub nbifom: Option<Fcontainer>,
    pub epco: Option<Epco>,
    pub fteid_control: Option<Fteid>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for BearerResourceCommand {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: BEARER_RSRC_CMD,
            teid: Some(0),
            ..Default::default()
        };
        BearerResourceCommand {
            header: hdr,
            linked_ebi: Ebi::default(),
            pti: Pti::default(),
            flow_qos: None,
            tad: None,
            rattype: None,
            servingnetwork: None,
            uli: None,
            ebi: None,
            indication: None,
            sgsn_fteid: None,
            rnc_fteid: None,
            pco: None,
            spi: None,
            overload_info: vec![],
            nbifom: None,
            epco: None,
            fteid_control: None,
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for BearerResourceCommand {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = BearerResourceCommand::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != BEARER_RSRC_CMD {
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

        elements.push(self.linked_ebi.clone().into());

        elements.push(self.pti.clone().into());

        if let Some(i) = self.flow_qos.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.tad.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.rattype.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.servingnetwork.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ebi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgsn_fteid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.rnc_fteid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.spi.clone() {
            elements.push(i.into())
        };

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epco.clone() {
            elements.push(i.into())
        };
        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };
        if let Some(i) = self.pscellid.clone() {
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
                InformationElement::Ebi(j) => match (j.ins, mandatory[0], self.ebi.is_none()) {
                    (0, false, _) => (self.linked_ebi, mandatory[0]) = (j.clone(), true),
                    (1, _, true) => self.ebi = Some(j.clone()),
                    _ => (),
                },
                InformationElement::Pti(j) => {
                    if let (0, false) = (j.ins, mandatory[1]) {
                        (self.pti, mandatory[1]) = (j.clone(), true)
                    };
                }
                InformationElement::FlowQos(j) => {
                    if let (0, true) = (j.ins, self.flow_qos.is_none()) {
                        self.flow_qos = Some(j.clone())
                    };
                }
                InformationElement::TrafficAggregateDescription(j) => {
                    if let (0, true) = (j.ins, self.tad.is_none()) {
                        self.tad = Some(j.clone())
                    };
                }
                InformationElement::RatType(j) => {
                    if let (0, true) = (j.ins, self.rattype.is_none()) {
                        self.rattype = Some(j.clone())
                    };
                }
                InformationElement::ServingNetwork(j) => {
                    if let (0, true) = (j.ins, self.servingnetwork.is_none()) {
                        self.servingnetwork = Some(j.clone())
                    };
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.uli.is_none()) {
                        self.uli = Some(j.clone())
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::Fteid(j) => {
                    // 3 instances
                    match (
                        j.ins,
                        self.sgsn_fteid.is_none(),
                        self.rnc_fteid.is_none(),
                        self.fteid_control.is_none(),
                    ) {
                        (0, true, _, _) => self.sgsn_fteid = Some(j.clone()),
                        (1, _, true, _) => self.rnc_fteid = Some(j.clone()),
                        (2, _, _, true) => self.fteid_control = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j.clone())
                    };
                }
                InformationElement::Spi(j) => {
                    if let (0, true) = (j.ins, self.spi.is_none()) {
                        self.spi = Some(j.clone())
                    };
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 2 {
                        self.overload_info.push(j.clone())
                    };
                }
                InformationElement::Fcontainer(j) => {
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j.clone())
                    };
                }
                InformationElement::Epco(j) => {
                    if let (0, true) = (j.ins, self.epco.is_none()) {
                        self.epco = Some(j.clone())
                    };
                }
                InformationElement::PSCellId(j) => {
                    if let (0, true) = (j.ins, self.pscellid.is_none()) {
                        self.pscellid = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1]) {
            (false, _) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (_, false) => Err(GTPV2Error::MessageMandatoryIEMissing(PTI)),
            (true, true) => Ok(true),
        }
    }
}

#[test]
fn test_bearer_resource_command_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 155] = [
        0x48, 0x44, 0x00, 0x97, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x64, 0x00, 0x01, 0x00, 0xff, 0x55, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x52, 0x00, 0x01, 0x00, 0x06, 0x53, 0x00, 0x03, 0x00, 0x62, 0xf2, 0x10, 0x56, 0x00, 0x0d,
        0x00, 0x18, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x4e,
        0x00, 0x23, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00,
        0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x0a, 0x00, 0x00, 0x05, 0x00, 0x00, 0x10, 0x00, 0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04,
        0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f,
        0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x22, 0xb6, 0x00, 0x01,
        0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e, 0x57, 0x00, 0x09, 0x02, 0x86, 0x06, 0xd1, 0x82,
        0x4c, 0xc1, 0xfe, 0x8b, 0x2d,
    ];
    let decoded = BearerResourceCommand {
        header: Gtpv2Header {
            msgtype: BEARER_RSRC_CMD,
            piggyback: false,
            message_prio: None,
            length: 151,
            teid: Some(0),
            sqn: 0x68,
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
        tad: Some(TrafficAggregateDescription {
            t: TAD,
            length: 4,
            ins: 0,
            tad: vec![0, 0, 0, 0],
        }),
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 262,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    tac: 0x0bd9,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 262,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    eci: 28983298,
                }),
            ],
        }),
        servingnetwork: Some(ServingNetwork {
            t: SERVINGNW,
            length: 3,
            ins: 0,
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
        }),
        rattype: Some(RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 2,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        }),
        pco: Some(Pco {
            t: PCO,
            length: 35,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
                0x83, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a,
                0x00, 0x00, 0x05, 0x00, 0x00, 0x10, 0x00,
            ],
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
        ..BearerResourceCommand::default()
    };
    let message = BearerResourceCommand::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_bearer_resource_command_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 155] = [
        0x48, 0x44, 0x00, 0x97, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x64, 0x00, 0x01, 0x00, 0xff, 0x55, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x52, 0x00, 0x01, 0x00, 0x06, 0x53, 0x00, 0x03, 0x00, 0x62, 0xf2, 0x10, 0x56, 0x00, 0x0d,
        0x00, 0x18, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x4e,
        0x00, 0x23, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00,
        0x00, 0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x0a, 0x00, 0x00, 0x05, 0x00, 0x00, 0x10, 0x00, 0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04,
        0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f,
        0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x22, 0xb6, 0x00, 0x01,
        0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e, 0x57, 0x00, 0x09, 0x02, 0x86, 0x06, 0xd1, 0x82,
        0x4c, 0xc1, 0xfe, 0x8b, 0x2d,
    ];
    let decoded = BearerResourceCommand {
        header: Gtpv2Header {
            msgtype: BEARER_RSRC_CMD,
            piggyback: false,
            message_prio: None,
            length: 151,
            teid: Some(0),
            sqn: 0x68,
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
        tad: Some(TrafficAggregateDescription {
            t: TAD,
            length: 4,
            ins: 0,
            tad: vec![0, 0, 0, 0],
        }),
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 262,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    tac: 0x0bd9,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 262,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    eci: 28983298,
                }),
            ],
        }),
        servingnetwork: Some(ServingNetwork {
            t: SERVINGNW,
            length: 3,
            ins: 0,
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
        }),
        rattype: Some(RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 2,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        }),
        pco: Some(Pco {
            t: PCO,
            length: 35,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
                0x83, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a,
                0x00, 0x00, 0x05, 0x00, 0x00, 0x10, 0x00,
            ],
        }),
        overload_info: vec![
            OverloadControlInfo {
                t: OVERLOAD_CNTRL,
                length: 22,
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
                length: 22,
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
        ..BearerResourceCommand::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer, encoded);
}
