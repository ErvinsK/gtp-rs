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
    // pub bearer_0: BearerContext
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
    // pub sgwu_node: Option<Fqdn>,
    // pub secondary_rat_usage_report: Option<SecondaryRatUsageDataReport>,
    // pub up_function_selection_flags: Option<UpFunctionSelectionIndicationFlags>,
    // pub apn_rate_control_status: Option<ApnRateControlStatus>,
    pub private_ext:Option<PrivateExtension>,
}

impl Default for EchoRequest {
    fn default() -> EchoRequest {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = ECHO_REQUEST;
        EchoRequest {
            header: hdr,
            recovery: Recovery::default(),
            sending_node_features:None,
            private_ext: None,
        }
    }
}

impl Messages for EchoRequest {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.recovery.marshal(buffer);
        match self.sending_node_features {
            Some(i) => i.marshal(buffer),
            None => (),
        }
        match self.private_ext {
            Some(i) => i.marshal(buffer),
            None => (),
        }
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = EchoRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ECHO_REQUEST {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            let ies:Vec<InformationElement>;
            match InformationElement::decoder(&buffer[8..]) {
                Ok(i) => ies = i,
                Err(j) => return Err(j),
            }
            let mut flag = false;
            for i in ies.iter() {
                match i {
                    InformationElement::Recovery(j) => {
                        if j.ins == 0 {
                            message.recovery = j.clone();
                            flag = true;
                        }
                    },
                    InformationElement::NodeFeatures(j) => {
                        if j.ins == 0 {
                            message.sending_node_features = Some(j.clone());
                        }
                    },
                    InformationElement::PrivateExtension(j) => {
                        if j.ins == 0 {
                            message.private_ext = Some(j.clone());
                        }
                    }
                    _ => (),
                }
            }
            if flag {
                Ok(message)
            } else {
                Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY))
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }
}

#[test]
fn test_echo_req_unmarshal () {
    let encoded:[u8;20] = [0x40, 0x01, 0x00, 0x0f, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x0c, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff];
    let decoded:EchoRequest = EchoRequest { 
        header: Gtpv2Header {
            msgtype:ECHO_REQUEST,
            piggyback:false,
            message_prio:None, 
            length:15, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 12 },
        sending_node_features: None,
        private_ext: Some(PrivateExtension { t: PRIVATE_EXT, length:3, ins: 0, enterprise_id: 0x0a, value: vec!(0xff) }) } ;
    assert_eq!(EchoRequest::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_req_no_mandatory_ie_unmarshal () {
    let encoded:[u8;15] = [0x40, 0x01, 0x00, 0x0b, 0x2d, 0xcc, 0x38, 0x00, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff];
    assert_eq!(EchoRequest::unmarshal(&encoded),Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY)));
}

#[test]
fn test_echo_req_marshal () {
    let encoded:[u8;13] = [0x40, 0x01, 0x00, 0x09, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x0c];
    let decoded:EchoRequest = EchoRequest { 
        header: Gtpv2Header {
            msgtype:ECHO_REQUEST,
            piggyback:false,
            message_prio:None, 
            length:9, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 12 },
        sending_node_features: None,
        private_ext: None } ;
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
