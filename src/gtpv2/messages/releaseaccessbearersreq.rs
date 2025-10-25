use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const RELEASE_ACCESS_BRS_REQ: u8 = 170;

// Definition of GTPv2-C Release Access Bearers Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAccessBearersRequest {
    pub header: Gtpv2Header,
    pub list_rabs: Vec<Ebi>,
    pub orig_node: Option<NodeType>,
    pub indication: Option<Indication>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ReleaseAccessBearersRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: RELEASE_ACCESS_BRS_REQ,
            teid: Some(0),
            ..Default::default()
        };
        ReleaseAccessBearersRequest {
            header: hdr,
            list_rabs: vec![],
            orig_node: None,
            indication: None,
            secondary_rat_usage_report: vec![],
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ReleaseAccessBearersRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ReleaseAccessBearersRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != RELEASE_ACCESS_BRS_REQ {
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

        self.list_rabs
            .iter()
            .for_each(|x| elements.push(InformationElement::Ebi(x.clone())));

        if let Some(i) = self.orig_node.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.indication.clone() {
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
                InformationElement::Ebi(j) => {
                    if j.ins == 0 {
                        self.list_rabs.push(j.clone());
                    }
                }
                InformationElement::NodeType(j) => {
                    if let (0, true) = (j.ins, self.orig_node.is_none()) {
                        self.orig_node = Some(j.clone());
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
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
fn test_release_access_bearers_req_unmarshal() {
    let encoded: [u8; 82] = [
        0x48, 0xaa, 0x00, 0x4e, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x49, 0x00, 0x01, 0x00, 0x06, 0x87, 0x00, 0x01, 0x00, 0x00, 0x4d, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0x00, 0x1b, 0x00,
        0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x06,
        0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ReleaseAccessBearersRequest {
        header: Gtpv2Header {
            msgtype: RELEASE_ACCESS_BRS_REQ,
            piggyback: false,
            message_prio: None,
            length: 78,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        list_rabs: vec![
            Ebi {
                t: EBI,
                length: EBI_LENGTH as u16,
                ins: 0,
                value: 5,
            },
            Ebi {
                t: EBI,
                length: EBI_LENGTH as u16,
                ins: 0,
                value: 6,
            },
        ],
        orig_node: Some(NodeType {
            node: Node::Mme,
            ..NodeType::default()
        }),
        indication: Some(Indication {
            arrl: true,
            ..Indication::default()
        }),
        secondary_rat_usage_report: vec![SecondaryRatUsageDataReport {
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
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ReleaseAccessBearersRequest::default()
    };
    let message = ReleaseAccessBearersRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_release_access_bearers_req_marshal() {
    let encoded: [u8; 82] = [
        0x48, 0xaa, 0x00, 0x4e, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x49, 0x00, 0x01, 0x00, 0x06, 0x87, 0x00, 0x01, 0x00, 0x00, 0x4d, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0x00, 0x1b, 0x00,
        0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x06,
        0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ReleaseAccessBearersRequest {
        header: Gtpv2Header {
            msgtype: RELEASE_ACCESS_BRS_REQ,
            piggyback: false,
            message_prio: None,
            length: 78,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        list_rabs: vec![
            Ebi {
                t: EBI,
                length: EBI_LENGTH as u16,
                ins: 0,
                value: 5,
            },
            Ebi {
                t: EBI,
                length: EBI_LENGTH as u16,
                ins: 0,
                value: 6,
            },
        ],
        orig_node: Some(NodeType {
            node: Node::Mme,
            ..NodeType::default()
        }),
        indication: Some(Indication {
            arrl: true,
            ..Indication::default()
        }),
        secondary_rat_usage_report: vec![SecondaryRatUsageDataReport {
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
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ReleaseAccessBearersRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
