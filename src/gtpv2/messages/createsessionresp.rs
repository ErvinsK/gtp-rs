use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_SESSION_RESP: u8 = 33;

// Definition of GTPv2-C Create Session Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSessionResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub cra: Option<ChangeReportingAction>,
    pub csg_ira: Option<CSGInformationReportingAction>,
    pub henb_info_report: Option<HenbInfoReporting>,
    pub fteid_control: Option<Fteid>,
    pub fteid_pgw: Option<Fteid>,
    pub paa: Option<PdnAddressAllocation>,
    pub apn_restriction: Option<ApnRestriction>,
    pub apn_ambr: Option<Ambr>,
    pub linked_ebi: Option<Ebi>,
    pub pco: Option<Pco>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub charging_gw_name: Option<Fqdn>,
    pub charging_gw_ip: Option<IpAddress>,
    pub pgw_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub sgw_ldn: Option<Ldn>,
    pub pgw_ldn: Option<Ldn>,
    pub pgw_backoff_time: Option<EpcTimer>,
    pub apco: Option<Apco>,
    pub twan_ip_params: Option<Ip4Cp>,
    pub indication: Option<Indication>,
    pub praa: Option<PresenceReportingAreaAction>,
    pub load_control: Vec<LoadControl>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub nbifom: Option<Fcontainer>,
    pub charging_id: Option<ChargingId>,
    pub epco: Option<Epco>,
    pub pgw_node_name: Option<Fqdn>,
    pub sgi_ptp_tunnel_addr: Option<SgiPtpTunnelAddress>,
    pub pgw_chng_info: Option<PgwChangeInfo>,
    pub alt_pgw_smf_fqdn: Vec<Fqdn>,
    pub alt_pgw_smf_ip: Vec<IpAddress>,
    pub upsp: Option<UpSecurityPolicy>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateSessionResponse {
    fn default() -> CreateSessionResponse {
        let hdr = Gtpv2Header {
            msgtype: CREATE_SESSION_RESP,
            teid: Some(0),
            ..Default::default()
        };
        CreateSessionResponse {
            header: hdr,
            cause: Cause::default(),
            cra: None,
            csg_ira: None,
            henb_info_report: None,
            fteid_control: None,
            fteid_pgw: None,
            paa: None,
            apn_restriction: None,
            apn_ambr: None,
            linked_ebi: None,
            pco: None,
            bearer_ctxs: vec![],
            recovery: None,
            charging_gw_name: None,
            charging_gw_ip: None,
            pgw_fqcsid: None,
            sgw_fqcsid: None,
            sgw_ldn: None,
            pgw_ldn: None,
            pgw_backoff_time: None,
            apco: None,
            twan_ip_params: None,
            indication: None,
            praa: None,
            load_control: vec![],
            overload_info: vec![],
            nbifom: None,
            charging_id: None,
            epco: None,
            pgw_node_name: None,
            sgi_ptp_tunnel_addr: None,
            pgw_chng_info: None,
            alt_pgw_smf_fqdn: vec![],
            alt_pgw_smf_ip: vec![],
            upsp: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CreateSessionResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateSessionResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_SESSION_RESP {
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

        if let Some(i) = self.cra.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.csg_ira.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_info_report.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_pgw.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.paa.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apn_restriction.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apn_ambr.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
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

        if let Some(i) = self.sgw_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_backoff_time.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.twan_ip_params.clone() {
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

        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.charging_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_node_name.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgi_ptp_tunnel_addr.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_chng_info.clone() {
            elements.push(InformationElement::PgwChangeInfo(i))
        };

        self.alt_pgw_smf_fqdn
            .iter()
            .for_each(|x| elements.push(x.clone().into()));

        self.alt_pgw_smf_ip
            .iter()
            .for_each(|x| elements.push(x.clone().into()));

        if let Some(i) = self.upsp.clone() {
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
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        (self.cause, mandatory[0]) = (j.clone(), true)
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
                InformationElement::Fteid(j) => {
                    // Two instances
                    match (
                        j.ins,
                        self.fteid_control.is_none(),
                        self.fteid_pgw.is_none(),
                    ) {
                        (0, true, _) => self.fteid_control = Some(j.clone()),
                        (1, _, true) => self.fteid_pgw = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::PdnAddressAllocation(j) => {
                    if let (0, true) = (j.ins, self.paa.is_none()) {
                        self.paa = Some(j.clone())
                    };
                }
                InformationElement::ApnRestriction(j) => {
                    if let (0, true) = (j.ins, self.apn_restriction.is_none()) {
                        self.apn_restriction = Some(j.clone())
                    };
                }
                InformationElement::ApnAmbr(j) => {
                    if let (0, true) = (j.ins, self.apn_ambr.is_none()) {
                        self.apn_ambr = Some(j.clone())
                    };
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.linked_ebi.is_none()) {
                        self.linked_ebi = Some(j.clone())
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
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
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
                InformationElement::Ldn(j) => {
                    // 2 instances
                    match (j.ins, self.sgw_ldn.is_none(), self.pgw_ldn.is_none()) {
                        (0, true, _) => self.sgw_ldn = Some(j.clone()),
                        (1, _, true) => self.pgw_ldn = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::EpcTimer(j) => {
                    if let (0, true) = (j.ins, self.pgw_backoff_time.is_none()) {
                        self.pgw_backoff_time = Some(j.clone())
                    };
                }
                InformationElement::Apco(j) => {
                    if let (0, true) = (j.ins, self.apco.is_none()) {
                        self.apco = Some(j.clone())
                    };
                }
                InformationElement::Ip4Cp(j) => {
                    if let (0, true) = (j.ins, self.twan_ip_params.is_none()) {
                        self.twan_ip_params = Some(j.clone())
                    };
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
                InformationElement::Fcontainer(j) => {
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j.clone())
                    };
                }
                InformationElement::ChargingId(j) => {
                    if let (0, true) = (j.ins, self.charging_id.is_none()) {
                        self.charging_id = Some(j.clone())
                    };
                }
                InformationElement::Epco(j) => {
                    if let (0, true) = (j.ins, self.epco.is_none()) {
                        self.epco = Some(j.clone())
                    };
                }
                InformationElement::Fqdn(j) => {
                    // 3 instances
                    match (
                        j.ins,
                        self.charging_gw_name.is_none(),
                        self.pgw_node_name.is_none(),
                    ) {
                        (0, true, _) => self.charging_gw_name = Some(j.clone()),
                        (1, _, true) => self.pgw_node_name = Some(j.clone()),
                        (3, _, _) => self.alt_pgw_smf_fqdn.push(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::SgiPtpTunnelAddress(j) => {
                    if let (0, true) = (j.ins, self.sgi_ptp_tunnel_addr.is_none()) {
                        self.sgi_ptp_tunnel_addr = Some(j.clone())
                    };
                }
                InformationElement::PgwChangeInfo(j) => {
                    if let (0, true) = (j.ins, self.pgw_chng_info.is_none()) {
                        self.pgw_chng_info = Some(j.clone())
                    };
                }
                InformationElement::IpAddress(j) => {
                    // 2 instances
                    match (j.ins, self.charging_gw_ip.is_none()) {
                        (0, true) => self.charging_gw_ip = Some(j.clone()),
                        (1, _) => self.alt_pgw_smf_ip.push(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::UpSecurityPolicy(j) => {
                    if let (0, true) = (j.ins, self.upsp.is_none()) {
                        self.upsp = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1]) {
            (false, false) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (false, true) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (true, false) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)),
            (true, true) => Ok(true),
        }
    }
}

#[test]
fn test_create_session_resp_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 148] = [
        0x48, 0x21, 0x00, 0x90, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x09, 0x01, 0x87, 0xb9, 0x7b, 0xbe, 0x07, 0x3e, 0x99, 0x89,
        0x4e, 0x4f, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xd8, 0x71, 0x5f, 0x7f, 0x00, 0x01, 0x00, 0x00,
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe8, 0x4e, 0x00, 0x14,
        0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
        0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 0x5d, 0x00, 0x3a, 0x00, 0x02, 0x00, 0x02, 0x00, 0x10,
        0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a,
        0x3e, 0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4, 0x03, 0x00, 0x01, 0x00, 0x11,
    ];
    let decoded = CreateSessionResponse {
        header: Gtpv2Header {
            msgtype: CREATE_SESSION_RESP,
            piggyback: false,
            message_prio: None,
            length: 144,
            teid: Some(0x0909a456),
            sqn: 0x2f,
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
            recovery: 17,
        }),
        apn_ambr: Some(Ambr {
            t: AMBR,
            length: 8,
            ins: 0,
            ambr_ul: 1000,
            ambr_dl: 1000,
        }),
        pco: Some(Pco {
            t: PCO,
            length: 20,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
                0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61,
            ],
        }),
        paa: Some(PdnAddressAllocation {
            t: PAA,
            length: 5,
            ins: 0,
            ip: PdnAddress::V4(Ipv4Addr::new(10, 216, 113, 95)),
        }),
        fteid_pgw: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 1,
            interface: 7,
            teid: 0xb97bbe07,
            ipv4: Some(Ipv4Addr::new(62, 153, 137, 78)),
            ipv6: None,
        }),

        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 58,
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
                value: 5,
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
        apn_restriction: Some(ApnRestriction {
            t: APNRESTRICTION,
            length: 1,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }),
        ..CreateSessionResponse::default()
    };

    let message = CreateSessionResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_session_resp_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 148] = [
        0x48, 0x21, 0x00, 0x90, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x09, 0x01, 0x87, 0xb9, 0x7b, 0xbe, 0x07, 0x3e, 0x99, 0x89,
        0x4e, 0x4f, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xd8, 0x71, 0x5f, 0x7f, 0x00, 0x01, 0x00, 0x00,
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe8, 0x4e, 0x00, 0x14,
        0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
        0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 0x5d, 0x00, 0x3a, 0x00, 0x02, 0x00, 0x02, 0x00, 0x10,
        0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a,
        0x3e, 0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4, 0x03, 0x00, 0x01, 0x00, 0x11,
    ];
    let decoded = CreateSessionResponse {
        header: Gtpv2Header {
            msgtype: CREATE_SESSION_RESP,
            piggyback: false,
            message_prio: None,
            length: 144,
            teid: Some(0x0909a456),
            sqn: 0x2f,
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
            recovery: 17,
        }),
        apn_ambr: Some(Ambr {
            t: AMBR,
            length: 8,
            ins: 0,
            ambr_ul: 1000,
            ambr_dl: 1000,
        }),
        pco: Some(Pco {
            t: PCO,
            length: 20,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
                0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61,
            ],
        }),
        paa: Some(PdnAddressAllocation {
            t: PAA,
            length: 5,
            ins: 0,
            ip: PdnAddress::V4(Ipv4Addr::new(10, 216, 113, 95)),
        }),
        fteid_pgw: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 1,
            interface: 7,
            teid: 0xb97bbe07,
            ipv4: Some(Ipv4Addr::new(62, 153, 137, 78)),
            ipv6: None,
        }),

        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 58,
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
                value: 5,
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
        apn_restriction: Some(ApnRestriction {
            t: APNRESTRICTION,
            length: 1,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }),
        ..CreateSessionResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
