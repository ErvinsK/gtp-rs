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
    // pub twmi: Option<Twmi>
    pub pco: Option<Pco>,
    pub bearer_ctx_created: GroupedIe,
    // pub bearer_1: Option<BearerContext>
    pub traceinfo: Option<TraceInformation>,
    pub recovery: Option<Recovery>,
    // pub mme_fqcsid: Option<Fqcsid>,
    // pub sgw_fqcsid: Option<Fqcsid>,
    // pub epdg_fqcsid: Option<Fqcsid>,
    // pub twan_fqcsid: Option<Fqcsid>,
    pub uetimezone: Option<UeTimeZone>,
    pub uci: Option<Uci>,
    pub chargingchar: Option<ChargingCharacteristics>,
    // pub mme_ldn: Option<Ldn>,
    // pub sgw_ldn: Option<Ldn>,
    // pub epdg_ldn: Option<Ldn>,
    // pub twan_ldn: Option<Ldn>,
    // pub spi: Option<Spi>,
    pub ue_localip: Option<IpAddress>,
    pub ue_udpport: Option<PortNumber>,
    // pub apco: Option<AddPco>,
    pub henb_localip: Option<IpAddress>,
    pub henb_udpport: Option<PortNumber>,
    pub mme_id: Option<IpAddress>,
    // pub twan_id: Option<TwanIdentifier>,
    pub epdg_ip: Option<IpAddress>,
    // pub cnose: Option<CNOperatorSelectionEntity>,
    // pub presence_reporting: Option<PresenceReportingAreaInformation>,
    // pub mme_overload: Option<OverloadControlInfo>,
    // pub sgw_overload: Option<OverloadControlInfo>,
    // pub twan_epdg_overload: Option<OverloadControlInfo>,
    // pub origination_timestamp: Option<MillisecondTimeStamp>,
    // pub max_waittime: Option<IntegerNumber>,
    // pub wlan_loc: Option<TwanId>,
    // pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    // pub nbifom: Option<Fcontainer>,
    // pub remote_ue_ctx_connected: Option<RemoteUeContext>,
    // pub aaaserver_id: Option<NodeIdentifier>,
    // pub epco: Option<Epco>,
    // pub serv_plmn_ratecontrol: Option<ServingPlmnRateControl>,
    // pub mo_exception_data_counter: Option<Counter>,
    pub ue_tcpport: Option<PortNumber>,
    // pub mappedue_usage_type: Option<MappedUeUsageType>,
    pub uli_for_sgw: Option<Uli>,
    pub sgwu_node: Option<Fqdn>,
    // pub secondary_rat_usage_report: Option<SecondaryRatUsageDataReport>,
    // pub up_function_selection_flags: Option<UpFunctionSelectionIndicationFlags>,
    // pub apn_rate_control_status: Option<ApnRateControlStatus>,
    pub private_ext:Option<PrivateExtension>,
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
            // twmi: None
            pco: None,
            bearer_ctx_created: GroupedIe::default(),
            // pub bearer_1: None,
            traceinfo: None,
            recovery: None,
            // mme_fqcsid: None,
            // sgw_fqcsid: None,
            // epdg_fqcsid: None,
            // twan_fqcsid: None,
            uetimezone: None,
            uci: None,
            chargingchar: None,
            // mme_ldn: None,
            // sgw_ldn: None,
            // epdg_ldn: None,
            // twan_ldn: None,
            // spi: None,
            ue_localip: None,
            ue_udpport: None,
            // apco: None,
            henb_localip: None,
            henb_udpport: None,
            mme_id: None,
            // twan_id: None,
            epdg_ip: None,
            // cnose: None,
            // presence_reporting: None,
            // mme_overload: None,
            // sgw_overload: None,
            // twan_epdg_overload: None,
            // origination_timestamp: None,
            // max_waittime: None,
            // wlan_loc: None,
            // wlan_loc_timestamp: None,
            // nbifom: None,
            // remote_ue_ctx_connected: None,
            // aaaserver_id: None,
            // epco: None,
            // serv_plmn_ratecontrol: None,
            // mo_exception_data_counter: None,
            ue_tcpport: None,
            // mapped_ue_usage_type: None,
            uli_for_sgw: None,
            sgwu_node: None,
            // secondary_rat_usage_report: None,
            // up_function_selection_flags: None,
            // apn_rate_control_status: None,
            private_ext: None,
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
            Some(i) => elements.push(InformationElement::Ebi(i)),
            None => (),
        }
            // twmi: None
        match self.pco.clone() {
            Some(i) => elements.push(InformationElement::Pco(i)),
            None => (),
        }    
        
        elements.push(InformationElement::BearerContext(self.bearer_ctx_created.clone()));
        
            // pub bearer_1: None,
        match self.traceinfo.clone() {
            Some(i) => elements.push(InformationElement::TraceInformation(i)),
            None => (),
        }
        match self.recovery.clone() {
            Some(i) => elements.push(InformationElement::Recovery(i)),
            None => (),
        }
            // mme_fqcsid: None,
            // sgw_fqcsid: None,
            // epdg_fqcsid: None,
            // twan_fqcsid: None,
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
            // mme_ldn: None,
            // sgw_ldn: None,
            // epdg_ldn: None,
            // twan_ldn: None,
            // spi: None,
        match self.ue_localip.clone() {
            Some(i) => elements.push(InformationElement::IpAddress(i)),
            None => (),
        }
        match self.ue_udpport.clone() {
            Some(i) => elements.push(InformationElement::PortNumber(i)),
            None => (),
        }
            // apco: None,
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
            // twan_id: None,
        match self.epdg_ip.clone() {
            Some(i) => elements.push(InformationElement::IpAddress(i)),
            None => (),
        }
            // cnose: None,
            // presence_reporting: None,
            // mme_overload: None,
            // sgw_overload: None,
            // twan_epdg_overload: None,
            // origination_timestamp: None,
            // max_waittime: None,
            // wlan_loc: None,
            // wlan_loc_timestamp: None,
            // nbifom: None,
            // remote_ue_ctx_connected: None,
            // aaaserver_id: None,
            // epco: None,
            // serv_plmn_ratecontrol: None,
            // mo_exception_data_counter: None,
        match self.ue_tcpport.clone() {
            Some(i) => elements.push(InformationElement::PortNumber(i)),
            None => (),
        }
            // mapped_ue_usage_type: None,
        match self.uli_for_sgw.clone() {
            Some(i) => elements.push(InformationElement::Uli(i)),
            None => (),
        }
        match self.sgwu_node.clone() {
            Some(i) => elements.push(InformationElement::Fqdn(i)),
            None => (),
        }   
            // secondary_rat_usage_report: None,
            // up_function_selection_flags: None,
            // apn_rate_control_status: None,
        match self.private_ext.clone() {
            Some(i) => elements.push(InformationElement::PrivateExtension(i)),
            None => (),
        }
    elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:Vec<u8>=vec!();
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if j.ins == 0 {
                        self.imsi = Some(j.clone());
                    }
                },
                InformationElement::Msisdn(j) => {
                    if j.ins == 0 {
                        self.msisdn = Some(j.clone());
                    }
                },
                InformationElement::Mei(j) => {
                    if j.ins == 0 {
                        self.mei = Some(j.clone());
                    }
                },
                InformationElement::Uli(j) => { // Two instances
                    match j.ins {
                        0 => self.uli = Some(j.clone()),
                        1 => self.uli_for_sgw = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::ServingNetwork(j) => {
                    if j.ins == 0 {
                        self.servingnetwork = Some(j.clone());
                    }
                },
                InformationElement::RatType(j) => {
                    if j.ins == 0 {
                        mandatory.push(j.t);
                        self.rattype = j.clone();
                    }
                },
                InformationElement::Indication(j) => {
                    if j.ins == 0 {
                        self.indication = Some(j.clone());
                    }
                },
                InformationElement::Fteid(j) => {  // 2 instances
                    match j.ins {
                        0 => {
                            mandatory.push(j.t);
                            self.fteid_control = j.clone();
                        },
                        1 => self.pgw_addr_control = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Apn(j) => {
                    if j.ins == 0 {
                        mandatory.push(j.t);
                        self.apn = j.clone();
                    }
                },
                InformationElement::SelectionMode(j) => {
                    if j.ins == 0 {
                        self.selectionmode = Some(j.clone());
                    }
                },
                InformationElement::PdnType(j) => {
                    if j.ins == 0 {
                        self.pdntype = Some(j.clone());
                    }
                },
                InformationElement::PdnAddressAllocation(j) => {
                    if j.ins == 0 {
                        self.paa = Some(j.clone());
                    }
                },
                InformationElement::ApnRestriction(j) => {
                    if j.ins == 0 {
                        self.max_apnrestriction = Some(j.clone());
                    }
                },
                InformationElement::ApnAmbr(j) => {
                    if j.ins == 0 {
                        self.apnambr = Some(j.clone());
                    }
                },
                InformationElement::Ebi(j) => {
                    if j.ins == 0 {
                        self.linked_ebi = Some(j.clone());
                    }
                },
                // pub twmi: Option<Twmi>
                InformationElement::Pco(j) => {
                    if j.ins == 0 {
                        self.pco = Some(j.clone());
                    }
                },
                InformationElement::BearerContext(j) => {
                    match j.ins {
                        0 => self.bearer_ctx_created = j.clone(),
                        _ => (),
                    }
                }
                // pub bearer_0: BearerContext
                // pub bearer_1: Option<BearerContext>
                InformationElement::TraceInformation(j) => {
                    if j.ins == 0 {
                        self.traceinfo = Some(j.clone());
                    }
                },
                InformationElement::Recovery(j) => {
                    if j.ins == 0 {
                        self.recovery = Some(j.clone());
                    }
                },
                // pub mme_fqcsid: Option<Fqcsid>,
                // pub sgw_fqcsid: Option<Fqcsid>,
                // pub epdg_fqcsid: Option<Fqcsid>,
                // pub twan_fqcsid: Option<Fqcsid>,
                InformationElement::UeTimeZone(j) => {
                    if j.ins == 0 {
                        self.uetimezone = Some(j.clone());
                    }
                },
                InformationElement::Uci(j) => {
                    if j.ins == 0 {
                        self.uci = Some(j.clone());
                    }
                },
                InformationElement::ChargingCharacteristics(j) => {
                    if j.ins == 0 {
                        self.chargingchar = Some(j.clone());
                    }
                },
                // pub mme_ldn: Option<Ldn>,
                // pub sgw_ldn: Option<Ldn>,
                // pub epdg_ldn: Option<Ldn>,
                // pub twan_ldn: Option<Ldn>,
                // pub spi: Option<Spi>,
                InformationElement::IpAddress(j) => {   // four ins
                    match j.ins {
                        0 => self.ue_localip = Some(j.clone()),
                        1 => self.henb_localip = Some(j.clone()),
                        2 => self.mme_id = Some(j.clone()),
                        3 => self.epdg_ip = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PortNumber(j) => {  // three ins
                    match j.ins {
                        0 => self.ue_udpport = Some(j.clone()),
                        1 => self.henb_udpport = Some(j.clone()),
                        2 => self.ue_tcpport = Some(j.clone()),
                        _ => (),
                    }
                }, 
                // pub apco: Option<AddPco>,
                // pub twan_id: Option<TwanIdentifier>,
                // pub cnose: Option<CNOperatorSelectionEntity>,
                // pub presence_reporting: Option<PresenceReportingAreaInformation>,
                // pub mme_overload: Option<OverloadControlInfo>,
                // pub sgw_overload: Option<OverloadControlInfo>,
                // pub twan_epdg_overload: Option<OverloadControlInfo>,
                // pub origination_timestamp: Option<MillisecondTimeStamp>,
                // pub max_waittime: Option<IntegerNumber>,
                // pub wlan_loc: Option<TwanId>,
                // pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
                // pub nbifom: Option<Fcontainer>,
                // pub remote_ue_ctx_connected: Option<RemoteUeContext>,
                // pub aaaserver_id: Option<NodeIdentifier>,
                // pub epco: Option<Epco>,
                // pub serv_plmn_ratecontrol: Option<ServingPlmnRateControl>,
                // pub mo_exception_data_counter: Option<Counter>,
                // pub mappedue_usage_type: Option<MappedUeUsageType>,
                InformationElement::Fqdn(j) => {
                    if j.ins == 0 {
                        self.sgwu_node = Some(j.clone());
                    }
                },
                // pub secondary_rat_usage_report: Option<SecondaryRatUsageDataReport>,
                // pub up_function_selection_flags: Option<UpFunctionSelectionIndicationFlags>,
                // pub apn_rate_control_status: Option<ApnRateControlStatus>,
                InformationElement::PrivateExtension(j) => self.private_ext = Some(j.clone()),
                _ => (),
            }
        }
        match ( mandatory.iter().find(|&&x| x==RATTYPE ), mandatory.iter().find(|&&x| x==FTEID ), mandatory.iter().find(|&&x| x==APN)) {
            (None, _, _) => Err(GTPV2Error::MessageMandatoryIEMissing(RATTYPE)),
            (_, None, _) => Err(GTPV2Error::MessageMandatoryIEMissing(FTEID)),
            (_, _, None) => Err(GTPV2Error::MessageMandatoryIEMissing(APN)),
            (Some(_),Some(_),Some(_)) => Ok(true),
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
    decoded.bearer_ctx_created = 
        GroupedIe { 
            t: 93, 
            length: 44, 
            ins: 0, 
            elements: vec!( 
                        InformationElement::Ebi(Ebi { t: 73, length: 1, ins: 0, value: 5 }), 
                        InformationElement::Fteid(Fteid { t: 87, length: 9, ins: 2, interface: 4, teid: 114393676, ipv4: Some(Ipv4Addr::new(193,254,139,45)), ipv6: None }), 
                        InformationElement::BearerQos(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 1, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 })
        )};
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
    decoded.bearer_ctx_created = 
        GroupedIe { 
            t: 93, 
            length: 44, 
            ins: 0, 
            elements: vec!( 
                        InformationElement::Ebi(Ebi { t: 73, length: 1, ins: 0, value: 5 }), 
                        InformationElement::Fteid(Fteid { t: 87, length: 9, ins: 2, interface: 4, teid: 114393676, ipv4: Some(Ipv4Addr::new(193,254,139,45)), ipv6: None }), 
                        InformationElement::BearerQos(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 1, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 })
        )};
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
