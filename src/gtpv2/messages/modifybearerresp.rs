use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MODIFY_BEARER_RESP: u8 = 35;

// Definition of GTPv2-C Modify Bearer Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyBearerResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub msisdn: Option<Msisdn>,
    pub linked_ebi: Option<Ebi>,
    pub apn_restriction: Option<ApnRestriction>,
    pub pco: Option<Pco>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub cra: Option<ChangeReportingAction>,
    pub csg_ira: Option<CSGInformationReportingAction>,
    pub henb_info_report: Option<HenbInfoReporting>,
    pub charging_gw_name: Option<Fqdn>,
    pub charging_gw_ip: Option<IpAddress>,
    pub pgw_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub recovery: Option<Recovery>,
    pub sgw_ldn: Option<Ldn>,
    pub pgw_ldn: Option<Ldn>,
    pub indication: Option<Indication>,
    pub praa: Option<PresenceReportingAreaAction>,
    pub load_control: Vec<LoadControl>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub charging_id: Option<ChargingId>,
    pub pgw_change_info: Option<PgwChangeInfo>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ModifyBearerResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MODIFY_BEARER_RESP,
            teid: Some(0),
            ..Default::default()
        };
        ModifyBearerResponse {
            header: hdr,
            cause: Cause::default(),
            msisdn: None,
            linked_ebi: None,
            apn_restriction: None,
            pco: None,
            bearer_ctxs: vec![],
            cra: None,
            csg_ira: None,
            henb_info_report: None,
            charging_gw_name: None,
            charging_gw_ip: None,
            pgw_fqcsid: None,
            sgw_fqcsid: None,
            recovery: None,
            sgw_ldn: None,
            pgw_ldn: None,
            indication: None,
            praa: None,
            load_control: vec![],
            overload_info: vec![],
            charging_id: None,
            pgw_change_info: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ModifyBearerResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyBearerResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_BEARER_RESP {
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

        if let Some(i) = self.msisdn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apn_restriction.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.cra.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.csg_ira.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_info_report.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.charging_gw_name.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.charging_gw_ip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.praa.clone() {
            elements.push(i.into())
        };

        self.load_control
            .iter()
            .for_each(|x| elements.push(InformationElement::LoadControlInfo(x.clone())));

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.charging_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_change_info.clone() {
            elements.push(InformationElement::PgwChangeInfo(i))
        };

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
                InformationElement::Msisdn(j) => {
                    if let (0, true) = (j.ins, self.msisdn.is_none()) {
                        self.msisdn = Some(j.clone())
                    };
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.linked_ebi.is_none()) {
                        self.linked_ebi = Some(j.clone())
                    };
                }
                InformationElement::ApnRestriction(j) => {
                    if let (0, true) = (j.ins, self.apn_restriction.is_none()) {
                        self.apn_restriction = Some(j.clone())
                    };
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j.clone())
                    };
                }
                InformationElement::BearerContext(j) => {
                    if j.ins < 2 {
                        self.bearer_ctxs.push(j.clone())
                    };
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
                InformationElement::Fqdn(j) => {
                    if let (0, true) = (j.ins, self.charging_gw_name.is_none()) {
                        self.charging_gw_name = Some(j.clone())
                    };
                }
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.charging_gw_ip.is_none()) {
                        self.charging_gw_ip = Some(j.clone())
                    };
                }
                InformationElement::Fqcsid(j) => {
                    // 2 instances
                    match (j.ins, self.pgw_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.pgw_fqcsid = Some(j.clone()),
                        (1, _, true) => self.sgw_fqcsid = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::Ldn(j) => {
                    // 2 instances
                    match (j.ins, self.sgw_ldn.is_none(), self.pgw_ldn.is_none()) {
                        (0, true, _) => self.sgw_ldn = Some(j.clone()),
                        (1, _, true) => self.pgw_ldn = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::PresenceReportingAreaAction(j) => {
                    if let (0, true) = (j.ins, self.praa.is_none()) {
                        self.praa = Some(j.clone())
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
                InformationElement::ChargingId(j) => {
                    if let (0, true) = (j.ins, self.charging_id.is_none()) {
                        self.charging_id = Some(j.clone())
                    };
                }
                InformationElement::PgwChangeInfo(j) => {
                    if let (0, true) = (j.ins, self.pgw_change_info.is_none()) {
                        self.pgw_change_info = Some(j.clone())
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
fn test_modify_bearer_resp_unmarshal() {
    let encoded: [u8; 68] = [
        0x48, 0x23, 0x00, 0x40, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, 0x58, 0x01, 0x02, 0x93, 0x56, 0xf0,
        0x49, 0x00, 0x01, 0x00, 0x05, 0x7f, 0x00, 0x01, 0x00, 0x00, 0x5d, 0x00, 0x13, 0x00, 0x02,
        0x00, 0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x5e, 0x00, 0x04, 0x00, 0x01,
        0x76, 0x4f, 0xbb, 0x03, 0x00, 0x01, 0x00, 0x08,
    ];
    /*
    let encoded:[u8;68] = [
        0x48, 0x23, 0x00, 0x40, 0xa4, 0x78, /* :JH#.@.x */
        0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, /* ..K).... */
        0x02, 0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, /* ........ */
        0x08, 0x49, 0x00, 0x01, 0x00, 0x05, 0x4c, 0x00, /* .I....L. */
        0x08, 0x00, 0x88, 0x22, 0x58, 0x01, 0x02, 0x93, /* ..."X... */
        0x56, 0xf0, 0x5d, 0x00, 0x13, 0x00, 0x02, 0x00, /* V.]..... */
        0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, 0x00, /* ....I... */
        0x05, 0x5e, 0x00, 0x04, 0x00, 0x01, 0x76, 0x4f, /* .^....vO */
        0xbb, 0x7f, 0x00, 0x01, 0x00, 0x00
    ]; */
    let decoded = ModifyBearerResponse {
        header: Gtpv2Header {
            msgtype: MODIFY_BEARER_RESP,
            piggyback: false,
            message_prio: None,
            length: 64,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 16,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        recovery: Some(Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 8,
        }),
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        msisdn: Some(Msisdn {
            t: MSISDN,
            length: 8,
            ins: 0,
            msisdn: "882285102039650".to_string(),
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 19,
            ins: 0,
            cause: Some(Cause {
                t: CAUSE,
                length: 2,
                ins: 0,
                value: 16,
                pce: false,
                bce: false,
                cs: false,
                offend_ie_type: None,
            }),
            tft: None,
            charging_id: Some(ChargingId {
                t: CHARGINGID,
                length: 4,
                ins: 0,
                charging_id: 24530875,
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
                value: 5,
            },
            fteids: vec![],
            bearer_qos: None,
            ..BearerContext::default()
        }],
        apn_restriction: Some(ApnRestriction {
            t: APNRESTRICTION,
            length: 1,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }),
        ..ModifyBearerResponse::default()
    };

    let message = ModifyBearerResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_modify_bearer_resp_marshal() {
    let encoded: [u8; 68] = [
        0x48, 0x23, 0x00, 0x40, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, 0x58, 0x01, 0x02, 0x93, 0x56, 0xf0,
        0x49, 0x00, 0x01, 0x00, 0x05, 0x7f, 0x00, 0x01, 0x00, 0x00, 0x5d, 0x00, 0x13, 0x00, 0x02,
        0x00, 0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x5e, 0x00, 0x04, 0x00, 0x01,
        0x76, 0x4f, 0xbb, 0x03, 0x00, 0x01, 0x00, 0x08,
    ];
    let decoded = ModifyBearerResponse {
        header: Gtpv2Header {
            msgtype: MODIFY_BEARER_RESP,
            piggyback: false,
            message_prio: None,
            length: 64,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 16,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        recovery: Some(Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 8,
        }),
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        msisdn: Some(Msisdn {
            t: MSISDN,
            length: 8,
            ins: 0,
            msisdn: "882285102039650".to_string(),
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 19,
            ins: 0,
            cause: Some(Cause {
                t: CAUSE,
                length: 2,
                ins: 0,
                value: 16,
                pce: false,
                bce: false,
                cs: false,
                offend_ie_type: None,
            }),
            tft: None,
            charging_id: Some(ChargingId {
                t: CHARGINGID,
                length: 4,
                ins: 0,
                charging_id: 24530875,
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
                value: 5,
            },
            fteids: vec![],
            bearer_qos: None,
            ..BearerContext::default()
        }],
        apn_restriction: Some(ApnRestriction {
            t: APNRESTRICTION,
            length: 1,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }),
        ..ModifyBearerResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
