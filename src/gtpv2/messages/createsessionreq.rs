use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_SESSION_REQ: u8 = 32;

// Definition of GTPv2-C Create Session Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSessionRequest {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub msisdn: Option<Msisdn>,
    pub mei: Option<Mei>,
    pub uli: Option<Uli>,
    pub servingnetwork: Option<ServingNetwork>,
    pub rattype: RatType,
    pub indication: Option<Indication>,
    pub fteid_control: Fteid,
    pub pgw_addr_control: Option<Fteid>,
    pub apn: Apn,
    pub selectionmode: Option<SelectionMode>,
    pub pdntype: Option<PdnType>,
    pub paa: Option<PdnAddressAllocation>,
    pub max_apnrestriction: Option<ApnRestriction>,
    pub apnambr: Option<Ambr>,
    pub linked_ebi: Option<Ebi>,
    pub twmi: Option<Twmi>,
    pub pco: Option<Pco>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub traceinfo: Option<TraceInformation>,
    pub recovery: Option<Recovery>,
    pub mme_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub epdg_fqcsid: Option<Fqcsid>,
    pub twan_fqcsid: Option<Fqcsid>,
    pub uetimezone: Option<UeTimeZone>,
    pub uci: Option<Uci>,
    pub chargingchar: Option<ChargingCharacteristics>,
    pub ldns: Vec<Ldn>,
    pub spi: Option<Spi>,
    pub ue_localip: Option<IpAddress>,
    pub ue_udpport: Option<PortNumber>,
    pub apco: Option<Apco>,
    pub henb_localip: Option<IpAddress>,
    pub henb_udpport: Option<PortNumber>,
    pub mme_id: Option<IpAddress>,
    pub twan_id: Option<TwanId>,
    pub epdg_ip: Option<IpAddress>,
    pub cnose: Option<CnOperatorSelectionEntity>,
    pub prai: Option<PresenceReportingAreaInformation>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub origination_timestamp: Option<MilliSecondTimeStamp>,
    pub max_waittime: Option<IntegerNumber>,
    pub wlan_loc: Option<TwanId>,
    pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    pub nbifom: Option<Fcontainer>,
    pub remote_ue_ctx_connected: Vec<RemoteUeContext>,
    pub aaaserver_id: Option<NodeIdentifier>,
    pub epco: Option<Epco>,
    pub srv_plmn_rate_cntrl: Option<ServingPlmnRateControl>,
    pub mo_exception_data_counter: Option<Counter>,
    pub ue_tcpport: Option<PortNumber>,
    pub mapped_ue_usage_type: Option<MappedUeUsageType>,
    pub uli_for_sgw: Option<Uli>,
    pub sgwu_node: Option<Fqdn>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub up_function_selection_flags: Option<UpFunctionSelectionIndicationFlags>,
    pub apn_rate_control_status: Option<ApnRateControlStatus>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateSessionRequest {
    fn default() -> CreateSessionRequest {
        let hdr = Gtpv2Header {
            msgtype: CREATE_SESSION_REQ,
            teid: Some(0),
            ..Default::default()
        };
        CreateSessionRequest {
            header: hdr,
            imsi: None,
            msisdn: None,
            mei: None,
            uli: None,
            servingnetwork: None,
            rattype: RatType::default(),
            indication: None,
            fteid_control: Fteid::default(),
            pgw_addr_control: None,
            apn: Apn::default(),
            selectionmode: None,
            pdntype: None,
            paa: None,
            max_apnrestriction: None,
            apnambr: None,
            linked_ebi: None,
            twmi: None,
            pco: None,
            bearer_ctxs: vec![],
            traceinfo: None,
            recovery: None,
            mme_fqcsid: None,
            sgw_fqcsid: None,
            epdg_fqcsid: None,
            twan_fqcsid: None,
            uetimezone: None,
            uci: None,
            chargingchar: None,
            ldns: vec![],
            spi: None,
            ue_localip: None,
            ue_udpport: None,
            apco: None,
            henb_localip: None,
            henb_udpport: None,
            mme_id: None,
            twan_id: None,
            epdg_ip: None,
            cnose: None,
            prai: None,
            overload_info: vec![],
            origination_timestamp: None,
            max_waittime: None,
            wlan_loc: None,
            wlan_loc_timestamp: None,
            nbifom: None,
            remote_ue_ctx_connected: vec![],
            aaaserver_id: None,
            epco: None,
            srv_plmn_rate_cntrl: None,
            mo_exception_data_counter: None,
            ue_tcpport: None,
            mapped_ue_usage_type: None,
            uli_for_sgw: None,
            sgwu_node: None,
            secondary_rat_usage_report: vec![],
            up_function_selection_flags: None,
            apn_rate_control_status: None,
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CreateSessionRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateSessionRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_SESSION_REQ {
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

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.msisdn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mei.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.servingnetwork.clone() {
            elements.push(i.into())
        };

        elements.push(InformationElement::RatType(self.rattype.clone()));

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        elements.push(InformationElement::Fteid(self.fteid_control.clone()));

        if let Some(i) = self.pgw_addr_control.clone() {
            elements.push(i.into())
        };

        elements.push(InformationElement::Apn(self.apn.clone()));

        if let Some(i) = self.selectionmode.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pdntype.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.paa.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.max_apnrestriction.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apnambr.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.twmi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.traceinfo.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epdg_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.twan_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uetimezone.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uci.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.chargingchar.clone() {
            elements.push(i.into())
        };

        self.ldns
            .iter()
            .for_each(|x| elements.push(InformationElement::Ldn(x.clone())));

        if let Some(i) = self.spi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_localip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_udpport.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_localip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_udpport.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.twan_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epdg_ip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.cnose.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.prai.clone() {
            elements.push(i.into())
        };

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.origination_timestamp.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.max_waittime.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.wlan_loc.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.wlan_loc_timestamp.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into())
        };

        self.remote_ue_ctx_connected
            .iter()
            .for_each(|x| elements.push(InformationElement::RemoteUeContext(x.clone())));

        if let Some(i) = self.aaaserver_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.srv_plmn_rate_cntrl.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mo_exception_data_counter.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_tcpport.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mapped_ue_usage_type.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli_for_sgw.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgwu_node.clone() {
            elements.push(i.into())
        };

        self.secondary_rat_usage_report.iter().for_each(|x| {
            elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone()))
        });

        if let Some(i) = self.up_function_selection_flags.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apn_rate_control_status.clone() {
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
        let mut mandatory: Vec<u8> = vec![];
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone())
                    };
                }
                InformationElement::Msisdn(j) => {
                    if let (0, true) = (j.ins, self.msisdn.is_none()) {
                        self.msisdn = Some(j.clone())
                    };
                }
                InformationElement::Mei(j) => {
                    if let (0, true) = (j.ins, self.mei.is_none()) {
                        self.mei = Some(j.clone())
                    };
                }
                InformationElement::Uli(j) => {
                    // Two instances
                    match (j.ins, self.uli.is_none(), self.uli_for_sgw.is_none()) {
                        (0, true, _) => self.uli = Some(j.clone()),
                        (1, _, true) => self.uli_for_sgw = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::ServingNetwork(j) => {
                    if let (0, true) = (j.ins, self.servingnetwork.is_none()) {
                        self.servingnetwork = Some(j.clone())
                    };
                }
                InformationElement::RatType(j) => {
                    if let (0, None) = (j.ins, mandatory.iter().find(|&&x| x == RATTYPE)) {
                        mandatory.push(j.t);
                        self.rattype = j.clone();
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::Fteid(j) => {
                    // 2 instances
                    match (
                        j.ins,
                        mandatory.iter().find(|&&x| x == FTEID),
                        self.pgw_addr_control.is_none(),
                    ) {
                        (0, None, _) => {
                            mandatory.push(j.t);
                            self.fteid_control = j.clone();
                        }
                        (1, _, true) => self.pgw_addr_control = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Apn(j) => {
                    if let (0, None) = (j.ins, mandatory.iter().find(|&&x| x == APN)) {
                        mandatory.push(j.t);
                        self.apn = j.clone();
                    }
                }
                InformationElement::SelectionMode(j) => {
                    if let (0, true) = (j.ins, self.selectionmode.is_none()) {
                        self.selectionmode = Some(j.clone())
                    };
                }
                InformationElement::PdnType(j) => {
                    if let (0, true) = (j.ins, self.pdntype.is_none()) {
                        self.pdntype = Some(j.clone())
                    };
                }
                InformationElement::PdnAddressAllocation(j) => {
                    if let (0, true) = (j.ins, self.paa.is_none()) {
                        self.paa = Some(j.clone())
                    };
                }
                InformationElement::ApnRestriction(j) => {
                    if let (0, true) = (j.ins, self.max_apnrestriction.is_none()) {
                        self.max_apnrestriction = Some(j.clone())
                    };
                }
                InformationElement::ApnAmbr(j) => {
                    if let (0, true) = (j.ins, self.apnambr.is_none()) {
                        self.apnambr = Some(j.clone())
                    };
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.linked_ebi.is_none()) {
                        self.linked_ebi = Some(j.clone())
                    };
                }
                InformationElement::Twmi(j) => {
                    if let (0, true) = (j.ins, self.twmi.is_none()) {
                        self.twmi = Some(j.clone())
                    };
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j.clone())
                    };
                }
                InformationElement::BearerContext(j) => match j.ins {
                    0 => {
                        mandatory.push(j.t);
                        self.bearer_ctxs.push(j.clone());
                    }
                    1 => self.bearer_ctxs.push(j.clone()),
                    _ => (),
                },
                InformationElement::TraceInformation(j) => {
                    if let (0, true) = (j.ins, self.traceinfo.is_none()) {
                        self.traceinfo = Some(j.clone())
                    };
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::Fqcsid(j) => {
                    // 4 instances
                    match (
                        j.ins,
                        self.mme_fqcsid.is_none(),
                        self.sgw_fqcsid.is_none(),
                        self.epdg_fqcsid.is_none(),
                        self.twan_fqcsid.is_none(),
                    ) {
                        (0, true, _, _, _) => self.mme_fqcsid = Some(j.clone()),
                        (1, _, true, _, _) => self.sgw_fqcsid = Some(j.clone()),
                        (2, _, _, true, _) => self.epdg_fqcsid = Some(j.clone()),
                        (3, _, _, _, true) => self.twan_fqcsid = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::UeTimeZone(j) => {
                    if let (0, true) = (j.ins, self.uetimezone.is_none()) {
                        self.uetimezone = Some(j.clone())
                    };
                }
                InformationElement::Uci(j) => {
                    if let (0, true) = (j.ins, self.uci.is_none()) {
                        self.uci = Some(j.clone())
                    };
                }
                InformationElement::ChargingCharacteristics(j) => {
                    if let (0, true) = (j.ins, self.chargingchar.is_none()) {
                        self.chargingchar = Some(j.clone())
                    };
                }
                InformationElement::Ldn(j) => self.ldns.push(j.clone()),
                InformationElement::Spi(j) => {
                    if let (0, true) = (j.ins, self.spi.is_none()) {
                        self.spi = Some(j.clone())
                    };
                }
                InformationElement::IpAddress(j) => {
                    // four ins
                    match (
                        j.ins,
                        self.ue_localip.is_none(),
                        self.henb_localip.is_none(),
                        self.mme_id.is_none(),
                        self.epdg_ip.is_none(),
                    ) {
                        (0, true, _, _, _) => self.ue_localip = Some(j.clone()),
                        (1, _, true, _, _) => self.henb_localip = Some(j.clone()),
                        (2, _, _, true, _) => self.mme_id = Some(j.clone()),
                        (3, _, _, _, true) => self.epdg_ip = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::PortNumber(j) => {
                    // three ins
                    match (
                        j.ins,
                        self.ue_udpport.is_none(),
                        self.henb_udpport.is_none(),
                        self.ue_tcpport.is_none(),
                    ) {
                        (0, true, _, _) => self.ue_udpport = Some(j.clone()),
                        (1, _, true, _) => self.henb_udpport = Some(j.clone()),
                        (2, _, _, true) => self.ue_tcpport = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Apco(j) => {
                    if let (0, true) = (j.ins, self.apco.is_none()) {
                        self.apco = Some(j.clone())
                    };
                }
                InformationElement::TwanId(j) => {
                    match (j.ins, self.twan_id.is_none(), self.wlan_loc.is_none()) {
                        (0, true, _) => self.twan_id = Some(j.clone()),
                        (1, _, true) => self.wlan_loc = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::CnOperatorSelectionEntity(j) => {
                    if let (0, true) = (j.ins, self.cnose.is_none()) {
                        self.cnose = Some(j.clone())
                    };
                }
                InformationElement::PresenceReportingAreaInformation(j) => {
                    if let (0, true) = (j.ins, self.prai.is_none()) {
                        self.prai = Some(j.clone())
                    };
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 3 {
                        self.overload_info.push(j.clone())
                    };
                }
                InformationElement::MilliSecondTimeStamp(j) => {
                    if let (0, true) = (j.ins, self.origination_timestamp.is_none()) {
                        self.origination_timestamp = Some(j.clone())
                    };
                }
                InformationElement::IntegerNumber(j) => {
                    if let (0, true) = (j.ins, self.max_waittime.is_none()) {
                        self.max_waittime = Some(j.clone())
                    };
                }
                InformationElement::TwanIdTimeStamp(j) => {
                    if let (0, true) = (j.ins, self.wlan_loc_timestamp.is_none()) {
                        self.wlan_loc_timestamp = Some(j.clone())
                    };
                }
                InformationElement::Fcontainer(j) => {
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j.clone())
                    };
                }
                InformationElement::RemoteUeContext(j) => {
                    if j.ins == 0 {
                        self.remote_ue_ctx_connected.push(j.clone());
                    }
                }
                InformationElement::NodeIdentifier(j) => {
                    if let (0, true) = (j.ins, self.aaaserver_id.is_none()) {
                        self.aaaserver_id = Some(j.clone())
                    };
                }
                InformationElement::Epco(j) => {
                    if let (0, true) = (j.ins, self.epco.is_none()) {
                        self.epco = Some(j.clone())
                    };
                }
                InformationElement::ServingPlmnRateControl(j) => {
                    if let (0, true) = (j.ins, self.srv_plmn_rate_cntrl.is_none()) {
                        self.srv_plmn_rate_cntrl = Some(j.clone())
                    };
                }
                InformationElement::Counter(j) => {
                    if let (0, true) = (j.ins, self.mo_exception_data_counter.is_none()) {
                        self.mo_exception_data_counter = Some(j.clone())
                    };
                }
                InformationElement::MappedUeUsageType(j) => {
                    if let (0, true) = (j.ins, self.mapped_ue_usage_type.is_none()) {
                        self.mapped_ue_usage_type = Some(j.clone())
                    };
                }
                InformationElement::Fqdn(j) => {
                    if let (0, true) = (j.ins, self.sgwu_node.is_none()) {
                        self.sgwu_node = Some(j.clone())
                    };
                }
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j.clone());
                    }
                }
                InformationElement::UpFunctionSelectionIndicationFlags(j) => {
                    if let (0, true) = (j.ins, self.up_function_selection_flags.is_none()) {
                        self.up_function_selection_flags = Some(j.clone())
                    };
                }
                InformationElement::ApnRateControlStatus(j) => {
                    if let (0, true) = (j.ins, self.apn_rate_control_status.is_none()) {
                        self.apn_rate_control_status = Some(j.clone())
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
        match (
            mandatory.iter().find(|&&x| x == RATTYPE),
            mandatory.iter().find(|&&x| x == FTEID),
            mandatory.iter().find(|&&x| x == APN),
            mandatory.iter().find(|&&x| x == BEARER_CTX),
        ) {
            (None, _, _, _) => Err(GTPV2Error::MessageMandatoryIEMissing(RATTYPE)),
            (_, None, _, _) => Err(GTPV2Error::MessageMandatoryIEMissing(FTEID)),
            (_, _, None, _) => Err(GTPV2Error::MessageMandatoryIEMissing(APN)),
            (_, _, _, None) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)),
            (Some(_), Some(_), Some(_), Some(_)) => Ok(true),
        }
    }
}

#[test]
fn test_create_session_req_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 266] = [
        0x48, 0x20, 0x01, 0x06, 0x00, 0x00, /* ..H .... */
        0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x01, 0x00, /* ....h... */
        0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, /* ...AP..7 */
        0x78, 0xf4, 0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, /* x.L...." */
        0x58, 0x01, 0x01, 0x37, 0x78, 0xf4, 0x4b, 0x00, /* X..7x.K. */
        0x08, 0x00, 0x68, 0x10, 0x59, 0x50, 0x46, 0x53, /* ..h.YPFS */
        0x18, 0x70, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x62, /* .pV....b */
        0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x01, /* ....b... */
        0xba, 0x40, 0x02, 0x53, 0x00, 0x03, 0x00, 0x62, /* .@.S...b */
        0xf2, 0x10, 0x52, 0x00, 0x01, 0x00, 0x06, 0x57, /* ..R....W */
        0x00, 0x09, 0x00, 0x86, 0x06, 0xd1, 0x82, 0x4c, /* .......L */
        0xc1, 0xfe, 0x8b, 0x2d, 0x47, 0x00, 0x20, 0x00, /* ...-G. . */
        0x03, 0x69, 0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, /* .iot.1nc */
        0x65, 0x03, 0x6e, 0x65, 0x74, 0x06, 0x6d, 0x6e, /* e.net.mn */
        0x63, 0x30, 0x34, 0x30, 0x06, 0x6d, 0x63, 0x63, /* c040.mcc */
        0x39, 0x30, 0x31, 0x04, 0x67, 0x70, 0x72, 0x73, /* 901.gprs */
        0x80, 0x00, 0x01, 0x00, 0x00, 0x63, 0x00, 0x01, /* .....c.. */
        0x00, 0x01, 0x4f, 0x00, 0x05, 0x00, 0x01, 0x00, /* ..O..... */
        0x00, 0x00, 0x00, 0x7f, 0x00, 0x01, 0x00, 0x00, /* ........ */
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, 0xe8, /* H....... */
        0x00, 0x00, 0x03, 0xe8, 0x4e, 0x00, 0x23, 0x00, /* ....N.#. */
        0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, /* ..!..... */
        0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, /* ........ */
        0x03, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x05, 0x00, /* ........ */
        0x00, 0x10, 0x00, 0x5d, 0x00, 0x2c, 0x00, 0x49, /* ...].,.I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x84, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, /* ....L... */
        0x2d, 0x50, 0x00, 0x16, 0x00, 0x6c, 0x09, 0x00, /* -P...l.. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0xbb, /* ........ */
        0x72, 0x00, 0x02, 0x00, 0x02, 0x01, 0x5f, 0x00, /* r....._. */
        0x02, 0x00, 0x08, 0x00,
    ];
    let decoded = CreateSessionRequest {
        header: Gtpv2Header {
            msgtype: CREATE_SESSION_REQ,
            piggyback: false,
            message_prio: None,
            length: 262,
            teid: Some(0),
            sqn: 0x68,
        },
        imsi: Some(Imsi {
            t: IMSI,
            length: 8,
            ins: 0,
            imsi: "901405101073874".to_string(),
        }),
        msisdn: Some(Msisdn {
            t: MSISDN,
            length: 8,
            ins: 0,
            msisdn: "882285101073874".to_string(),
        }),
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8601950564358107".to_string(),
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
        rattype: RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        },
        fteid_control: Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        },
        apn: Apn {
            t: APN,
            length: 32,
            ins: 0,
            name: "iot.1nce.net.mnc040.mcc901.gprs".to_string(),
        },
        selectionmode: Some(SelectionMode {
            t: SELECTION_MODE,
            length: 1,
            ins: 0,
            mode: 0,
        }),
        pdntype: Some(PdnType {
            t: PDNTYPE,
            length: 1,
            ins: 0,
            pdn_type: Pdn::Ipv4,
        }),
        paa: Some(PdnAddressAllocation {
            t: PAA,
            length: 5,
            ins: 0,
            ip: PdnAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }),
        max_apnrestriction: Some(ApnRestriction {
            t: APNRESTRICTION,
            length: 1,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }),
        apnambr: Some(Ambr {
            t: AMBR,
            length: 8,
            ins: 0,
            ambr_ul: 1000,
            ambr_dl: 1000,
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
        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 44,
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
                t: 73,
                length: 1,
                ins: 0,
                value: 5,
            },
            fteids: vec![Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 4,
                teid: 114393676,
                ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
                ipv6: None,
            }],
            bearer_qos: Some(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 1,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
            ..BearerContext::default()
        }],
        recovery: Some(Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 187,
        }),
        uetimezone: Some(UeTimeZone {
            t: UETIMEZONE,
            length: 2,
            ins: 0,
            time_zone: 2,
            dst: 1,
        }),
        chargingchar: Some(ChargingCharacteristics {
            t: CHARGINGCHAR,
            length: 2,
            ins: 0,
            charging_char: 0x0800,
        }),
        ..CreateSessionRequest::default()
    };

    let message = CreateSessionRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_session_req_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 266] = [
        0x48, 0x20, 0x01, 0x06, 0x00, 0x00, /* ..H .... */
        0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x01, 0x00, /* ....h... */
        0x08, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, /* ...AP..7 */
        0x78, 0xf4, 0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, /* x.L...." */
        0x58, 0x01, 0x01, 0x37, 0x78, 0xf4, 0x4b, 0x00, /* X..7x.K. */
        0x08, 0x00, 0x68, 0x10, 0x59, 0x50, 0x46, 0x53, /* ..h.YPFS */
        0x18, 0x70, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x62, /* .pV....b */
        0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x01, /* ....b... */
        0xba, 0x40, 0x02, 0x53, 0x00, 0x03, 0x00, 0x62, /* .@.S...b */
        0xf2, 0x10, 0x52, 0x00, 0x01, 0x00, 0x06, 0x57, /* ..R....W */
        0x00, 0x09, 0x00, 0x86, 0x06, 0xd1, 0x82, 0x4c, /* .......L */
        0xc1, 0xfe, 0x8b, 0x2d, 0x47, 0x00, 0x20, 0x00, /* ...-G. . */
        0x03, 0x69, 0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, /* .iot.1nc */
        0x65, 0x03, 0x6e, 0x65, 0x74, 0x06, 0x6d, 0x6e, /* e.net.mn */
        0x63, 0x30, 0x34, 0x30, 0x06, 0x6d, 0x63, 0x63, /* c040.mcc */
        0x39, 0x30, 0x31, 0x04, 0x67, 0x70, 0x72, 0x73, /* 901.gprs */
        0x80, 0x00, 0x01, 0x00, 0x00, 0x63, 0x00, 0x01, /* .....c.. */
        0x00, 0x01, 0x4f, 0x00, 0x05, 0x00, 0x01, 0x00, /* ..O..... */
        0x00, 0x00, 0x00, 0x7f, 0x00, 0x01, 0x00, 0x00, /* ........ */
        0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, 0xe8, /* H....... */
        0x00, 0x00, 0x03, 0xe8, 0x4e, 0x00, 0x23, 0x00, /* ....N.#. */
        0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, /* ..!..... */
        0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, /* ........ */
        0x03, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x05, 0x00, /* ........ */
        0x00, 0x10, 0x00, 0x5d, 0x00, 0x2c, 0x00, 0x49, /* ...].,.I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x84, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, /* ....L... */
        0x2d, 0x50, 0x00, 0x16, 0x00, 0x6c, 0x09, 0x00, /* -P...l.. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0xbb, /* ........ */
        0x72, 0x00, 0x02, 0x00, 0x02, 0x01, 0x5f, 0x00, /* r....._. */
        0x02, 0x00, 0x08, 0x00,
    ];
    let decoded = CreateSessionRequest {
        header: Gtpv2Header {
            msgtype: CREATE_SESSION_REQ,
            piggyback: false,
            message_prio: None,
            length: 262,
            teid: Some(0),
            sqn: 0x68,
        },
        imsi: Some(Imsi {
            t: IMSI,
            length: 8,
            ins: 0,
            imsi: "901405101073874".to_string(),
        }),
        msisdn: Some(Msisdn {
            t: MSISDN,
            length: 8,
            ins: 0,
            msisdn: "882285101073874".to_string(),
        }),
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8601950564358107".to_string(),
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
        rattype: RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        },
        fteid_control: Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        },
        apn: Apn {
            t: APN,
            length: 32,
            ins: 0,
            name: "iot.1nce.net.mnc040.mcc901.gprs".to_string(),
        },
        selectionmode: Some(SelectionMode {
            t: SELECTION_MODE,
            length: 1,
            ins: 0,
            mode: 0,
        }),
        pdntype: Some(PdnType {
            t: PDNTYPE,
            length: 1,
            ins: 0,
            pdn_type: Pdn::Ipv4,
        }),
        paa: Some(PdnAddressAllocation {
            t: PAA,
            length: 5,
            ins: 0,
            ip: PdnAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
        }),
        max_apnrestriction: Some(ApnRestriction {
            t: APNRESTRICTION,
            length: 1,
            ins: 0,
            restriction_type: Restriction::NoApnRestriction,
        }),
        apnambr: Some(Ambr {
            t: AMBR,
            length: 8,
            ins: 0,
            ambr_ul: 1000,
            ambr_dl: 1000,
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
        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 44,
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
                t: 73,
                length: 1,
                ins: 0,
                value: 5,
            },
            fteids: vec![Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 4,
                teid: 114393676,
                ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
                ipv6: None,
            }],
            bearer_qos: Some(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 1,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
            ..BearerContext::default()
        }],
        recovery: Some(Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 187,
        }),
        uetimezone: Some(UeTimeZone {
            t: UETIMEZONE,
            length: 2,
            ins: 0,
            time_zone: 2,
            dst: 1,
        }),
        chargingchar: Some(ChargingCharacteristics {
            t: CHARGINGCHAR,
            length: 2,
            ins: 0,
            charging_char: 0x0800,
        }),
        ..CreateSessionRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
