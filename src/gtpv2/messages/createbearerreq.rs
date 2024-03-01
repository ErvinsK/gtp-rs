use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_BEARER_REQ: u8 = 95;

// Definition of GTPv2-C Create Bearer Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateBearerRequest {
    pub header: Gtpv2Header,
    pub pti: Option<Pti>,
    pub linked_ebi: Ebi,
    pub pco: Option<Pco>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub pgw_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub cra: Option<ChangeReportingAction>,
    pub csg_ira: Option<CSGInformationReportingAction>,
    pub henb_info_report: Option<HenbInfoReporting>,
    pub praa: Option<PresenceReportingAreaAction>,
    pub indication: Option<Indication>,
    pub load_control: Vec<LoadControl>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub nbifom: Option<Fcontainer>,
    pub pgw_change_info: Vec<PgwChangeInfo>,
    pub sender_fteid_cntrl_plane: Option<Fteid>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateBearerRequest {
    fn default() -> CreateBearerRequest {
        CreateBearerRequest {
            header: Gtpv2Header {
                msgtype: CREATE_BEARER_REQ,
                teid: Some(0),
                ..Default::default()
            },
            pti: None,
            linked_ebi: Ebi::default(),
            pco: None,
            bearer_ctxs: vec![],
            pgw_fqcsid: None,
            sgw_fqcsid: None,
            cra: None,
            csg_ira: None,
            henb_info_report: None,
            praa: None,
            indication: None,
            load_control: vec![],
            overload_info: vec![],
            nbifom: None,
            pgw_change_info: vec![],
            sender_fteid_cntrl_plane: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CreateBearerRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateBearerRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_BEARER_REQ {
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

        if let Some(i) = self.pti.clone() {
            elements.push(i.into())
        };

        elements.push(self.linked_ebi.clone().into());

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.pgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.cra.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.csg_ira.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_info_report.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.praa.clone() {
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

        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into())
        };

        self.pgw_change_info
            .iter()
            .for_each(|x| elements.push(InformationElement::PgwChangeInfo(x.clone())));

        if let Some(i) = self.sender_fteid_cntrl_plane.clone() {
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
                InformationElement::Pti(j) => {
                    if let (0, true) = (j.ins, self.pti.is_none()) {
                        self.pti = Some(j.clone())
                    };
                }
                InformationElement::Ebi(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        (self.linked_ebi, mandatory[0]) = (j.clone(), true)
                    };
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j.clone())
                    };
                }
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        mandatory[1] = true;
                        self.bearer_ctxs.push(j.clone());
                    }
                }
                InformationElement::Fqcsid(j) => {
                    // 2 instances
                    match (j.ins, self.pgw_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.pgw_fqcsid = Some(j.clone()),
                        (1, _, true) => self.sgw_fqcsid = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::ChangeReportingAction(j) => {
                    if let (0, true) = (j.ins, self.cra.is_none()) {
                        self.cra = Some(j.clone())
                    };
                }
                InformationElement::CSGInformationReportingAction(j) => {
                    if let (0, true) = (j.ins, self.csg_ira.is_none()) {
                        self.csg_ira = Some(j.clone())
                    };
                }
                InformationElement::HenbInfoReporting(j) => {
                    if let (0, true) = (j.ins, self.henb_info_report.is_none()) {
                        self.henb_info_report = Some(j.clone())
                    };
                }
                InformationElement::PresenceReportingAreaAction(j) => {
                    if let (0, true) = (j.ins, self.praa.is_none()) {
                        self.praa = Some(j.clone())
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
                InformationElement::Fcontainer(j) => {
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j.clone())
                    };
                }
                InformationElement::PgwChangeInfo(j) => {
                    if j.ins == 0 {
                        self.pgw_change_info.push(j.clone())
                    };
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.sender_fteid_cntrl_plane.is_none()) {
                        self.sender_fteid_cntrl_plane = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1]) {
            (false, false) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (false, true) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (true, false) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)),
            (true, true) => Ok(true),
        }
    }
}

#[test]
fn test_create_bearer_req_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 97] = [
        0x48, 0x5f, 0x00, 0x5d, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81,
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 0x5d, 0x00, 0x34, 0x00,
        0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e,
        0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e,
        0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateBearerRequest {
        header: Gtpv2Header {
            msgtype: CREATE_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 93,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        linked_ebi: Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 5,
        },
        pco: Some(Pco {
            t: PCO,
            length: 20,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
                0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61,
            ],
        }),

        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 52,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: Some(ChargingId {
                t: CHARGINGID,
                length: 4,
                ins: 0,
                charging_id: 23239876,
            }),
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
                value: 0,
            },
            fteids: vec![Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 5,
                teid: 0x3b95985a,
                ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
                ipv6: None,
            }],
            bearer_qos: Some(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 0,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
            ..BearerContext::default()
        }],
        ..CreateBearerRequest::default()
    };

    let message = CreateBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_bearer_req_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 97] = [
        0x48, 0x5f, 0x00, 0x5d, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81,
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 0x5d, 0x00, 0x34, 0x00,
        0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e,
        0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e,
        0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateBearerRequest {
        header: Gtpv2Header {
            msgtype: CREATE_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 93,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        linked_ebi: Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 5,
        },
        pco: Some(Pco {
            t: PCO,
            length: 20,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
                0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61,
            ],
        }),

        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 52,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: Some(ChargingId {
                t: CHARGINGID,
                length: 4,
                ins: 0,
                charging_id: 23239876,
            }),
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
                value: 0,
            },
            fteids: vec![Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 5,
                teid: 0x3b95985a,
                ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
                ipv6: None,
            }],
            bearer_qos: Some(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 0,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
            ..BearerContext::default()
        }],
        ..CreateBearerRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
