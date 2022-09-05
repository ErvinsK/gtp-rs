use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const CREATE_SESSION_REQ:u8 = 32;

// Definition of GTPv2-C Create Session Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSessionRequest {
    pub header:Gtpv2Header,
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
    pub paa:Option<PdnAddressAllocation>,
    pub max_apnrestriction: Option<ApnRestriction>,
    pub apnambr: Option<ApnAmbr>,
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
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for CreateSessionRequest {
    fn default() -> CreateSessionRequest {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = CREATE_SESSION_REQ;
        hdr.teid = Some(0);
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
            bearer_ctxs: vec!(),
            traceinfo: None,
            recovery: None,
            mme_fqcsid: None,
            sgw_fqcsid: None,
            epdg_fqcsid: None,
            twan_fqcsid: None,
            uetimezone: None,
            uci: None,
            chargingchar: None,
            ldns: vec!(),
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
            overload_info:vec!(),
            origination_timestamp: None,
            max_waittime: None,
            wlan_loc: None,
            wlan_loc_timestamp: None,
            nbifom: None,
            remote_ue_ctx_connected: vec!(),
            aaaserver_id: None,
            epco: None,
            srv_plmn_rate_cntrl: None,
            mo_exception_data_counter: None,
            ue_tcpport: None,
            mapped_ue_usage_type: None,
            uli_for_sgw: None,
            sgwu_node: None,
            secondary_rat_usage_report: vec!(),
            up_function_selection_flags: None,
            apn_rate_control_status: None,
            private_ext: vec!(),
        }
    }
}

impl Messages for CreateSessionRequest {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateSessionRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_SESSION_REQ {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            let ies:Vec<InformationElement>;
            match InformationElement::decoder(&buffer[12..]) {
                Ok(i) => ies = i,
                Err(j) => return Err(j),
            }
            match message.from_vec(ies) {
                Ok(_) => Ok(message),
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }
}

impl CreateSessionRequest {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();
        match self.imsi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.msisdn.clone() {
            Some(i) => elements.push(InformationElement::Msisdn(i)),
            None => (),
        }
        match self.mei.clone() {
            Some(i) => elements.push(InformationElement::Mei(i)),
            None => (),
        }
        match self.uli.clone() {
            Some(i) => elements.push(InformationElement::Uli(i)),
            None => (),
        }
        match self.servingnetwork.clone() {
            Some(i) => elements.push(InformationElement::ServingNetwork(i)),
            None => (),
        }

        elements.push(InformationElement::RatType(self.rattype.clone()));
        
        match self.indication.clone() {
            Some(i) => elements.push(InformationElement::Indication(i)),
            None => (),
        }
        
        elements.push(InformationElement::Fteid(self.fteid_control.clone()));

        match self.pgw_addr_control.clone() {
            Some(i) => elements.push(InformationElement::Fteid(i)),
            None => (),
        }
        
        elements.push(InformationElement::Apn(self.apn.clone()));

        match self.selectionmode.clone() {
            Some(i) => elements.push(InformationElement::SelectionMode(i)),
            None => (),
        }
        match self.pdntype.clone() {
            Some(i) => elements.push(InformationElement::PdnType(i)),
            None => (),
        }
        match self.paa.clone() {
            Some(i) => elements.push(InformationElement::PdnAddressAllocation(i)),
            None => (),
        }
        match self.max_apnrestriction.clone() {
            Some(i) => elements.push(InformationElement::ApnRestriction(i)),
            None => (),
        }
        match self.apnambr.clone() {
            Some(i) => elements.push(InformationElement::ApnAmbr(i)),
            None => (),
        }
        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.twmi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pco.clone() {
            Some(i) => elements.push(InformationElement::Pco(i)),
            None => (),
        }    
        
        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        match self.traceinfo.clone() {
            Some(i) => elements.push(InformationElement::TraceInformation(i)),
            None => (),
        }
        match self.recovery.clone() {
            Some(i) => elements.push(InformationElement::Recovery(i)),
            None => (),
        }
        match self.mme_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.sgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.epdg_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.twan_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.uetimezone.clone() {
            Some(i) => elements.push(InformationElement::UeTimeZone(i)),
            None => (),
        }
        match self.uci.clone() {
            Some(i) => elements.push(InformationElement::Uci(i)),
            None => (),
        }
        match self.chargingchar.clone() {
            Some(i) => elements.push(InformationElement::ChargingCharacteristics(i)),
            None => (),
        }

        self.ldns.iter().for_each(|x| elements.push(InformationElement::Ldn(x.clone())));
        
        match self.spi.clone() {
            Some(i) => elements.push(InformationElement::Spi(i)),
            None => (),
        }
        match self.ue_localip.clone() {
            Some(i) => elements.push(InformationElement::IpAddress(i)),
            None => (),
        }
        match self.ue_udpport.clone() {
            Some(i) => elements.push(InformationElement::PortNumber(i)),
            None => (),
        }
        match self.apco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.henb_localip.clone() {
            Some(i) => elements.push(InformationElement::IpAddress(i)),
            None => (),
        }
        match self.henb_udpport.clone() {
            Some(i) => elements.push(InformationElement::PortNumber(i)),
            None => (),
        }
        match self.mme_id.clone() {
            Some(i) => elements.push(InformationElement::IpAddress(i)),
            None => (),
        }
        match self.twan_id.clone() {
            Some(i) => elements.push(InformationElement::TwanId(i)),
            None => (),
        }
        match self.epdg_ip.clone() {
            Some(i) => elements.push(InformationElement::IpAddress(i)),
            None => (),
        }
        match self.cnose.clone() {
            Some(i) => elements.push(i.into()),
            None => (),

        }
        match self.prai.clone() {
            Some(i) => elements.push(i.into()),
            None => (),

        }
        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));
        match self.origination_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.max_waittime.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.wlan_loc.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.wlan_loc_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.nbifom.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        self.remote_ue_ctx_connected.iter().for_each(|x| elements.push(InformationElement::RemoteUeContext(x.clone())));
        match self.aaaserver_id.clone() {
            Some(i) => elements.push(InformationElement::NodeIdentifier(i)),
            None => (),
        }
        match self.epco.clone() {
            Some(i) => elements.push(InformationElement::Epco(i)),
            None => (),
        }
        match self.srv_plmn_rate_cntrl.clone() {
            Some(i) => elements.push(InformationElement::ServingPlmnRateControl(i)),
            None => (),
        }
        match self.mo_exception_data_counter.clone() {
            Some(i) => elements.push(InformationElement::Counter(i)),
            None => (),
        }
        match self.ue_tcpport.clone() {
            Some(i) => elements.push(InformationElement::PortNumber(i)),
            None => (),
        }
        match self.mapped_ue_usage_type.clone() {
            Some(i) => elements.push(InformationElement::MappedUeUsageType(i)),
            None => (),
        }
        match self.uli_for_sgw.clone() {
            Some(i) => elements.push(InformationElement::Uli(i)),
            None => (),
        }
        match self.sgwu_node.clone() {
            Some(i) => elements.push(InformationElement::Fqdn(i)),
            None => (),
        } 
        self.secondary_rat_usage_report.iter().for_each(|x| elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone())));
        match self.up_function_selection_flags.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }             
        match self.apn_rate_control_status.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        } 
        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));    
        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:Vec<u8>=vec!();
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    match (j.ins, self.imsi.is_none()) {
                        (0, true) => self.imsi = Some(j.clone()),
                        (_,_) => (),
                    }
                },
                InformationElement::Msisdn(j) => {
                    match (j.ins, self.msisdn.is_none()) {
                        (0, true) => self.msisdn = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Mei(j) => {
                    match (j.ins, self.mei.is_none()) {
                        (0, true) => self.mei = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Uli(j) => { // Two instances
                    match (j.ins, self.uli.is_none(), self.uli_for_sgw.is_none()) {
                        (0, true, _) => self.uli = Some(j.clone()),
                        (1, _, true) => self.uli_for_sgw = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::ServingNetwork(j) => {
                    match (j.ins, self.servingnetwork.is_none()) {
                        (0, true) => self.servingnetwork = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::RatType(j) => {
                    match (j.ins, mandatory.iter().find(|&&x| x==RATTYPE )) {
                        (0, None) => {
                            mandatory.push(j.t);
                            self.rattype = j.clone();
                        },
                        _ => (),
                    }
                },
                InformationElement::Indication(j) => {
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fteid(j) => {  // 2 instances
                    match (j.ins, mandatory.iter().find(|&&x| x==FTEID ), self.pgw_addr_control.is_none())  {
                        (0, None, _) => {
                            mandatory.push(j.t);
                            self.fteid_control = j.clone();
                        },
                        (1, _, true) => self.pgw_addr_control = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Apn(j) => {
                    match (j.ins, mandatory.iter().find(|&&x| x==APN )) {
                        (0, None) => {
                            mandatory.push(j.t);
                            self.apn = j.clone();
                        },
                        _ => (),
                    }
                },
                InformationElement::SelectionMode(j) => {
                    match (j.ins, self.selectionmode.is_none()) {
                        (0, true) => self.selectionmode = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PdnType(j) => {
                    match (j.ins, self.pdntype.is_none()) {
                        (0, true) => self.pdntype = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PdnAddressAllocation(j) => {
                    match (j.ins, self.paa.is_none()) {
                        (0, true) => self.paa = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ApnRestriction(j) => {
                    match (j.ins, self.max_apnrestriction.is_none()) {
                        (0, true) => self.max_apnrestriction = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ApnAmbr(j) => {
                    match (j.ins, self.apnambr.is_none()) {
                        (0, true) => self.apnambr = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Twmi(j) => {
                    match (j.ins, self.twmi.is_none()) {
                        (0, true) => self.twmi = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Pco(j) => {
                    match (j.ins, self.pco.is_none()) {
                        (0, true) => self.pco = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::BearerContext(j) => {
                    match j.ins {
                        0 => {
                            mandatory.push(j.t);
                            self.bearer_ctxs.push(j.clone());
                        },
                        1 => self.bearer_ctxs.push(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::TraceInformation(j) => {
                    match (j.ins, self.traceinfo.is_none()) {
                        (0, true) => self.traceinfo = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fqcsid(j) => {  // 4 instances
                    match (j.ins, self.mme_fqcsid.is_none(), self.sgw_fqcsid.is_none(), self.epdg_fqcsid.is_none(), self.twan_fqcsid.is_none()) {
                        (0, true, _, _, _) => self.mme_fqcsid = Some(j.clone()),
                        (1, _, true, _, _) => self.sgw_fqcsid = Some(j.clone()),
                        (2, _, _, true,_) => self.epdg_fqcsid = Some(j.clone()),
                        (3, _, _, _, true) => self.twan_fqcsid = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::UeTimeZone(j) => {
                    match (j.ins, self.uetimezone.is_none()) {
                        (0, true) => self.uetimezone = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Uci(j) => {
                    match (j.ins, self.uci.is_none()) {
                        (0, true) => self.uci = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ChargingCharacteristics(j) => {
                    match (j.ins, self.chargingchar.is_none()) {
                        (0, true) => self.chargingchar = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Ldn(j) => self.ldns.push(j.clone()),
                InformationElement::Spi(j) => {
                    match (j.ins, self.spi.is_none()) {
                        (0, true) => self.spi = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::IpAddress(j) => {   // four ins
                    match (j.ins, self.ue_localip.is_none(), self.henb_localip.is_none(), self.mme_id.is_none(), self.epdg_ip.is_none()) {
                        (0, true, _, _, _) => self.ue_localip = Some(j.clone()),
                        (1, _, true, _, _) => self.henb_localip = Some(j.clone()),
                        (2, _, _, true, _) => self.mme_id = Some(j.clone()),
                        (3, _, _, _, true) => self.epdg_ip = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PortNumber(j) => {  // three ins
                    match (j.ins, self.ue_udpport.is_none(), self.henb_udpport.is_none(), self.ue_tcpport.is_none()) {
                        (0, true, _, _) => self.ue_udpport = Some(j.clone()),
                        (1, _, true, _) => self.henb_udpport = Some(j.clone()),
                        (2, _, _, true) => self.ue_tcpport = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Apco(j) => {  
                    match (j.ins, self.apco.is_none()) {
                        (0, true) => self.apco = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::TwanId(j) => {  
                    match (j.ins, self.twan_id.is_none(), self.wlan_loc.is_none()) {
                        (0, true, _) => self.twan_id = Some(j.clone()),
                        (1, _, true) => self.wlan_loc = Some(j.clone()),
                        _ => (),
                    }
                },  
                InformationElement::CnOperatorSelectionEntity(j) => {  
                    match (j.ins, self.cnose.is_none()) {
                        (0, true) => self.cnose = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PresenceReportingAreaInformation(j) => {  
                    match (j.ins, self.prai.is_none()) {
                        (0, true) => self.prai = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<3 => self.overload_info.push(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::MilliSecondTimeStamp(j) => {  
                    match (j.ins, self.origination_timestamp.is_none()) {
                        (0, true) => self.origination_timestamp = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::IntegerNumber(j) => {  
                    match (j.ins, self.max_waittime.is_none()) {
                        (0, true) => self.max_waittime = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::TwanIdTimeStamp(j) => {  
                    match (j.ins, self.wlan_loc_timestamp.is_none()) {
                        (0, true) => self.wlan_loc_timestamp = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fcontainer(j) => {  
                    match (j.ins, self.nbifom.is_none()) {
                        (0, true) => self.nbifom = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::RemoteUeContext(j) => {  
                    if j.ins == 0 {
                        self.remote_ue_ctx_connected.push(j.clone());
                    }
                }, 
                InformationElement::NodeIdentifier(j) => {  
                    match (j.ins, self.aaaserver_id.is_none()) {
                        (0, true) => self.aaaserver_id = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Epco(j) => {  
                    match (j.ins, self.epco.is_none()) {
                        (0, true) => self.epco = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ServingPlmnRateControl(j) => {  
                    match (j.ins, self.srv_plmn_rate_cntrl.is_none()) {
                        (0, true) => self.srv_plmn_rate_cntrl = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Counter(j) => {  
                    match (j.ins, self.mo_exception_data_counter.is_none()) {
                        (0, true) => self.mo_exception_data_counter = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::MappedUeUsageType(j) => {  
                    match (j.ins, self.mapped_ue_usage_type.is_none()) {
                        (0, true) => self.mapped_ue_usage_type = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Fqdn(j) => {
                    match (j.ins, self.sgwu_node.is_none()) {
                        (0, true) => self.sgwu_node = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j.clone());
                    }
                },
                InformationElement::UpFunctionSelectionIndicationFlags(j) => {
                    match (j.ins, self.up_function_selection_flags.is_none()) {
                        (0, true) => self.up_function_selection_flags = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ApnRateControlStatus(j) => {
                    match (j.ins, self.apn_rate_control_status.is_none()) {
                        (0, true) => self.apn_rate_control_status = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match ( mandatory.iter().find(|&&x| x==RATTYPE ), mandatory.iter().find(|&&x| x==FTEID ), mandatory.iter().find(|&&x| x==APN), mandatory.iter().find(|&&x| x==BEARER_CTX)) {
            (None,_,_,_) => Err(GTPV2Error::MessageMandatoryIEMissing(RATTYPE)),
            (_,None,_,_) => Err(GTPV2Error::MessageMandatoryIEMissing(FTEID)),
            (_,_,None,_) => Err(GTPV2Error::MessageMandatoryIEMissing(APN)),
            (_,_,_,None) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)), 
            (Some(_),Some(_),Some(_),Some(_)) => Ok(true),
        }
    }
}

#[test]
fn test_create_session_req_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;266] = [
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
        0x72, 0x00, 0x02, 0x00, 0x80, 0x01, 0x5f, 0x00, /* r....._. */
        0x02, 0x00, 0x08, 0x00
    ];
    let mut decoded = CreateSessionRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_SESSION_REQ,
            piggyback:false,
            message_prio:None, 
            length:262, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.imsi = Some (
        Imsi {
                t:IMSI,
                length:8,
                ins:0,
                imsi:"901405101073874".to_string(),
            });
    decoded.msisdn = Some (
        Msisdn {
            t:MSISDN,
            length:8,
            ins:0,
            msisdn:"882285101073874".to_string(),
        });
    decoded.mei = Some (
        Mei {
            t:MEI,
            length:8,
            ins:0,
            mei:"8601950564358107".to_string(),
        });
    decoded.uli = Some (
        Uli {
            t:ULI,
            length:13,
            ins:0,
            loc: vec!(Location::Tai(Tai { mcc: 262, mnc:1, tac:0x0bd9}),Location::Ecgi(Ecgi{ mcc: 262, mnc:1, eci:28983298})),
        });
    decoded.servingnetwork = Some (
        ServingNetwork {
            t:SERVINGNW,
            length:3,
            ins:0,
            mcc:262,
            mnc:1,
        });
    decoded.rattype = 
        RatType {
            t:RATTYPE,
            length:1,
            ins:0,
            rat_type:Rat::Eutran,
        };
    decoded.fteid_control = 
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193,254,139,45)),
            ipv6:None
        };
    decoded.apn = 
        Apn {
            t:APN,
            length: 32,
            ins:0,
            name:"iot.1nce.net.mnc040.mcc901.gprs".to_string(),
        };
    decoded.selectionmode = Some (
        SelectionMode {
            t:SELECTION_MODE,
            length:1,
            ins:0,
            mode:0,
        });
    decoded.pdntype = Some (
        PdnType {
            t:PDNTYPE,
            length:1,
            ins:0,
            pdn_type:Pdn::Ipv4,
        });
    decoded.paa = Some (
        PdnAddressAllocation { t:PAA, length:5, ins:0, ip: PdnAddress::V4(Ipv4Addr::new(0,0,0,0)) }
    );
    decoded.max_apnrestriction = Some (
        ApnRestriction {
            t:APNRESTRICTION,
            length:1,
            ins:0,
            restriction_type:Restriction::NoApnRestriction,
        });
    decoded.apnambr = Some (
        ApnAmbr {
            t:APNAMBR,
            length:8,
            ins:0,
            ambr_ul:1000,
            ambr_dl:1000,
        });
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:35,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x05, 0x00, 
                    0x00, 0x10, 0x00),
        });
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: 93, 
            length: 44, 
            ins: 0,
            cause: None,
            tft:None,
            charging_id:None,
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: 73, length: 1, ins: 0, value: 5 },
            fteids: Some(vec!( Fteid { t: 87, length: 9, ins: 2, interface: 4, teid: 114393676, ipv4: Some(Ipv4Addr::new(193,254,139,45)), ipv6: None })),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 1, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    decoded.recovery = Some (
        Recovery {
            t:RECOVERY,
            length:1,
            ins:0,
            recovery:187,
        });
    decoded.uetimezone = Some (
        UeTimeZone {
            t:UETIMEZONE,
            length:2,
            ins:0,
            time_zone: 2,
            dst:1,
        });
    decoded.chargingchar = Some (
        ChargingCharacteristics {
            t:CHARGINGCHAR,
            length:2,
            ins:0,
            charging_char:0x0800,
        });
    
    let message = CreateSessionRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_create_session_req_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;266] = [
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
        0x72, 0x00, 0x02, 0x00, 0x80, 0x01, 0x5f, 0x00, /* r....._. */
        0x02, 0x00, 0x08, 0x00
    ];
    let mut decoded = CreateSessionRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_SESSION_REQ,
            piggyback:false,
            message_prio:None, 
            length:262, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.imsi = Some (
        Imsi {
                t:IMSI,
                length:8,
                ins:0,
                imsi:"901405101073874".to_string(),
            });
    decoded.msisdn = Some (
        Msisdn {
            t:MSISDN,
            length:8,
            ins:0,
            msisdn:"882285101073874".to_string(),
        });
    decoded.mei = Some (
        Mei {
            t:MEI,
            length:8,
            ins:0,
            mei:"8601950564358107".to_string(),
        });
    decoded.uli = Some (
        Uli {
            t:ULI,
            length:13,
            ins:0,
            loc: vec!(Location::Tai(Tai { mcc: 262, mnc:1, tac:0x0bd9}),Location::Ecgi(Ecgi{ mcc: 262, mnc:1, eci:28983298})),
        });
    decoded.servingnetwork = Some (
        ServingNetwork {
            t:SERVINGNW,
            length:3,
            ins:0,
            mcc:262,
            mnc:1,
        });
    decoded.rattype = 
        RatType {
            t:RATTYPE,
            length:1,
            ins:0,
            rat_type:Rat::Eutran,
        };
    decoded.fteid_control = 
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193,254,139,45)),
            ipv6:None
        };
    decoded.apn = 
        Apn {
            t:APN,
            length: 32,
            ins:0,
            name:"iot.1nce.net.mnc040.mcc901.gprs".to_string(),
        };
    decoded.selectionmode = Some (
        SelectionMode {
            t:SELECTION_MODE,
            length:1,
            ins:0,
            mode:0,
        });
    decoded.pdntype = Some (
        PdnType {
            t:PDNTYPE,
            length:1,
            ins:0,
            pdn_type:Pdn::Ipv4,
        });
    decoded.paa = Some (
        PdnAddressAllocation { t:PAA, length:5, ins:0, ip: PdnAddress::V4(Ipv4Addr::new(0,0,0,0)) }
    );
    decoded.max_apnrestriction = Some (
        ApnRestriction {
            t:APNRESTRICTION,
            length:1,
            ins:0,
            restriction_type:Restriction::NoApnRestriction,
        });
    decoded.apnambr = Some (
        ApnAmbr {
            t:APNAMBR,
            length:8,
            ins:0,
            ambr_ul:1000,
            ambr_dl:1000,
        });
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:35,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x05, 0x00, 
                    0x00, 0x10, 0x00),
        });
        decoded.bearer_ctxs = vec!( 
        BearerContext { 
            t: 93, 
            length: 44, 
            ins: 0,
            cause: None,
            tft:None,
            charging_id:None,
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: 73, length: 1, ins: 0, value: 5 },
            fteids: Some(vec!( Fteid { t: 87, length: 9, ins: 2, interface: 4, teid: 114393676, ipv4: Some(Ipv4Addr::new(193,254,139,45)), ipv6: None })),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 1, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    decoded.recovery = Some (
        Recovery {
            t:RECOVERY,
            length:1,
            ins:0,
            recovery:187,
        });
    decoded.uetimezone = Some (
        UeTimeZone {
            t:UETIMEZONE,
            length:2,
            ins:0,
            time_zone: 2,
            dst:1,
        });
    decoded.chargingchar = Some (
        ChargingCharacteristics {
            t:CHARGINGCHAR,
            length:2,
            ins:0,
            charging_char:0x0800,
        });
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
