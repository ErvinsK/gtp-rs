use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DELETE_SESSION_REQ: u8 = 36;

// Definition of GTPv2-C Delete Session Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteSessionRequest {
    pub header: Gtpv2Header,
    pub cause: Option<Cause>,
    pub linked_ebi: Option<Ebi>,
    pub uli: Option<Uli>,
    pub indication: Option<Indication>,
    pub pco: Option<Pco>,
    pub orig_node: Option<NodeType>,
    pub fteid_control: Option<Fteid>,
    pub uetimezone: Option<UeTimeZone>,
    pub uli_timestamp: Option<UliTimestamp>,
    pub ran_nas_cause: Option<RanNasCause>,
    pub twan_id: Option<TwanId>,
    pub twan_id_timestamp: Option<TwanIdTimeStamp>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub wlan_loc: Option<TwanId>,
    pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    pub ue_localip: Option<IpAddress>,
    pub ue_udpport: Option<PortNumber>,
    pub epco: Option<Epco>,
    pub ue_tcpport: Option<PortNumber>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DeleteSessionRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: DELETE_SESSION_REQ,
            teid: Some(0),
            ..Default::default()
        };
        DeleteSessionRequest {
            header: hdr,
            cause: None,
            linked_ebi: None,
            uli: None,
            indication: None,
            pco: None,
            orig_node: None,
            fteid_control: None,
            uetimezone: None,
            uli_timestamp: None,
            ran_nas_cause: None,
            twan_id: None,
            twan_id_timestamp: None,
            overload_info: vec![],
            wlan_loc: None,
            wlan_loc_timestamp: None,
            ue_localip: None,
            ue_udpport: None,
            epco: None,
            ue_tcpport: None,
            secondary_rat_usage_report: vec![],
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DeleteSessionRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteSessionRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_SESSION_REQ {
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

        if let Some(i) = self.cause.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.orig_node.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uetimezone.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli_timestamp.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ran_nas_cause.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.twan_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.twan_id_timestamp.clone() {
            elements.push(i.into())
        };

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.wlan_loc.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.wlan_loc_timestamp.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_localip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_udpport.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_tcpport.clone() {
            elements.push(i.into())
        };

        self.secondary_rat_usage_report.iter().for_each(|x| {
            elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone()))
        });

        if let Some(i) = self.pscellid.clone() {
            elements.push(i.into())
        };
        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, true) = (j.ins, self.cause.is_none()) {
                        self.cause = Some(j)
                    };
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.linked_ebi.is_none()) {
                        self.linked_ebi = Some(j)
                    };
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.uli.is_none()) {
                        self.uli = Some(j)
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j)
                    };
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j)
                    };
                }
                InformationElement::NodeType(j) => {
                    if let (0, true) = (j.ins, self.orig_node.is_none()) {
                        self.orig_node = Some(j)
                    };
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j)
                    };
                }
                InformationElement::UeTimeZone(j) => {
                    if let (0, true) = (j.ins, self.uetimezone.is_none()) {
                        self.uetimezone = Some(j)
                    };
                }
                InformationElement::UliTimestamp(j) => {
                    if let (0, true) = (j.ins, self.uli_timestamp.is_none()) {
                        self.uli_timestamp = Some(j)
                    };
                }
                InformationElement::RanNasCause(j) => {
                    if let (0, true) = (j.ins, self.ran_nas_cause.is_none()) {
                        self.ran_nas_cause = Some(j)
                    };
                }
                InformationElement::TwanId(j) => {
                    // 2 instances
                    match (j.ins, self.twan_id.is_none(), self.wlan_loc.is_none()) {
                        (0, true, _) => self.twan_id = Some(j),
                        (1, _, true) => self.wlan_loc = Some(j),
                        _ => (),
                    }
                }
                InformationElement::TwanIdTimeStamp(j) => {
                    // 2 instances
                    match (
                        j.ins,
                        self.twan_id_timestamp.is_none(),
                        self.wlan_loc_timestamp.is_none(),
                    ) {
                        (0, true, _) => self.twan_id_timestamp = Some(j),
                        (1, _, true) => self.wlan_loc_timestamp = Some(j),
                        _ => (),
                    }
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 3 {
                        self.overload_info.push(j)
                    };
                }
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.ue_localip.is_none()) {
                        self.ue_localip = Some(j)
                    };
                }
                InformationElement::PortNumber(j) => {
                    // 2 instances
                    match (j.ins, self.ue_udpport.is_none(), self.ue_tcpport.is_none()) {
                        (0, true, _) => self.ue_udpport = Some(j),
                        (1, _, true) => self.ue_tcpport = Some(j),
                        _ => (),
                    }
                }
                InformationElement::Epco(j) => {
                    if let (0, true) = (j.ins, self.epco.is_none()) {
                        self.epco = Some(j)
                    };
                }
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j)
                    };
                }
                InformationElement::PSCellId(j) => {
                    if let (0, true) = (j.ins, self.pscellid.is_none()) {
                        self.pscellid = Some(j)
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_delete_session_req_unmarshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 91] = [
        0x48, 0x24, 0x00, 0x57, 0x9a, 0x33, /* ..H$.W.3 */
        0xde, 0xaf, 0x47, 0x82, 0x1c, 0x00, 0x49, 0x00, /* ..G...I. */
        0x01, 0x00, 0x05, 0x56, 0x00, 0x0d, 0x00, 0x18, /* ...V.... */
        0x13, 0x00, 0x14, 0x27, 0x00, 0x13, 0x00, 0x14, /* ...'.... */
        0x0b, 0xa6, 0xb8, 0x86, 0xff, 0x00, 0x0a, 0x00, /* ........ */
        0x00, 0x4a, 0x01, 0x00, 0x05, 0x05, 0x01, 0x02, /* .J...... */
        0x00, 0x1a, 0x57, 0x00, 0x19, 0x00, 0xc6, 0x82, /* ..W..... */
        0x15, 0x62, 0x1c, 0xa6, 0x89, 0xf9, 0xd7, 0x26, /* .b.....& */
        0x00, 0x03, 0x00, 0x20, 0x20, 0x1e, 0xff, 0x00, /* ...  ... */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xaa, /* ........ */
        0x00, 0x04, 0x00, 0xe5, 0xce, 0x77, 0xf3, 0xac, /* .....w.. */
        0x00, 0x02, 0x00, 0x10, 0x1a,
    ];
    let decoded = DeleteSessionRequest {
        header: Gtpv2Header {
            msgtype: DELETE_SESSION_REQ,
            piggyback: false,
            message_prio: None,
            length: 87,
            teid: Some(0x9a33deaf),
            sqn: 0x47821c,
        },
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 310,
                    mnc: 410,
                    mnc_is_three_digits: true,
                    tac: 0x2700,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 310,
                    mnc: 410,
                    mnc_is_three_digits: true,
                    eci: 195475590,
                }),
            ],
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 6,
            teid: 0x8215621c,
            ipv4: Some(Ipv4Addr::new(166, 137, 249, 215)),
            ipv6: Some(Ipv6Addr::new(0x2600, 0x300, 0x2020, 0x1eff, 0, 0, 0, 0x7)),
        }),
        uli_timestamp: Some(UliTimestamp {
            t: ULI_TIMESTAMP,
            length: ULI_TIMESTAMP_LENGTH as u16,
            ins: 0,
            timestamp: 0xe5ce77f3,
        }),
        ran_nas_cause: Some(RanNasCause {
            t: RAN_NAS_CAUSE,
            length: 2,
            ins: 0,
            cause: CauseValue::S1ap(S1APCause::RadioLayer(26)),
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 10,
            ins: 0,
            enterprise_id: 0x4a,
            value: vec![0x01, 0x00, 0x05, 0x05, 0x01, 0x02, 0x00, 0x1a],
        }],
        ..DeleteSessionRequest::default()
    };
    let message = DeleteSessionRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_delete_session_req_marshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 91] = [
        0x48, 0x24, 0x00, 0x57, 0x9a, 0x33, /* ..H$.W.3 */
        0xde, 0xaf, 0x47, 0x82, 0x1c, 0x00, 0x49, 0x00, /* ..G...I. */
        0x01, 0x00, 0x05, 0x56, 0x00, 0x0d, 0x00, 0x18, /* ...V.... */
        0x13, 0x00, 0x14, 0x27, 0x00, 0x13, 0x00, 0x14, /* ...'.... */
        0x0b, 0xa6, 0xb8, 0x86, 0x57, 0x00, 0x19, 0x00, 0xc6, 0x82, /* ..W..... */
        0x15, 0x62, 0x1c, 0xa6, 0x89, 0xf9, 0xd7, 0x26, /* .b.....& */
        0x00, 0x03, 0x00, 0x20, 0x20, 0x1e, 0xff, 0x00, /* ...  ... */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xaa, /* ........ */
        0x00, 0x04, 0x00, 0xe5, 0xce, 0x77, 0xf3, 0xac, /* .....w.. */
        0x00, 0x02, 0x00, 0x10, 0x1a, 0xff, 0x00, 0x0a, 0x00, /* ........ */
        0x00, 0x4a, 0x01, 0x00, 0x05, 0x05, 0x01, 0x02, /* .J...... */
        0x00, 0x1a,
    ];
    let decoded = DeleteSessionRequest {
        header: Gtpv2Header {
            msgtype: DELETE_SESSION_REQ,
            piggyback: false,
            message_prio: None,
            length: 87,
            teid: Some(0x9a33deaf),
            sqn: 0x47821c,
        },
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 310,
                    mnc: 410,
                    mnc_is_three_digits: true,
                    tac: 0x2700,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 310,
                    mnc: 410,
                    mnc_is_three_digits: true,
                    eci: 195475590,
                }),
            ],
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 6,
            teid: 0x8215621c,
            ipv4: Some(Ipv4Addr::new(166, 137, 249, 215)),
            ipv6: Some(Ipv6Addr::new(0x2600, 0x300, 0x2020, 0x1eff, 0, 0, 0, 0x7)),
        }),
        uli_timestamp: Some(UliTimestamp {
            t: ULI_TIMESTAMP,
            length: ULI_TIMESTAMP_LENGTH as u16,
            ins: 0,
            timestamp: 0xe5ce77f3,
        }),
        ran_nas_cause: Some(RanNasCause {
            t: RAN_NAS_CAUSE,
            length: 2,
            ins: 0,
            cause: CauseValue::S1ap(S1APCause::RadioLayer(26)),
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 10,
            ins: 0,
            enterprise_id: 0x4a,
            value: vec![0x01, 0x00, 0x05, 0x05, 0x01, 0x02, 0x00, 0x1a],
        }],
        ..DeleteSessionRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
