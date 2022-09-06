use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const CREATE_SESSION_RESP:u8 = 33;

// Definition of GTPv2-C Create Session Response Message

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSessionResponse {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub cra:Option<ChangeReportingAction>,
    pub csg_ira:Option<CSGInformationReportingAction>,
    pub henb_info_report:Option<HenbInfoReporting>,
    pub fteid_control:Option<Fteid>,
    pub fteid_pgw:Option<Fteid>,
    pub paa:Option<PdnAddressAllocation>,
    pub apn_restriction:Option<ApnRestriction>,
    pub apn_ambr:Option<ApnAmbr>,
    pub linked_ebi:Option<Ebi>,
    pub pco:Option<Pco>,
    pub bearer_ctxs:Vec<BearerContext>,
    pub recovery:Option<Recovery>,
    pub charging_gw_name:Option<Fqdn>,
    pub charging_gw_ip:Option<IpAddress>,
    pub pgw_fqcsid:Option<Fqcsid>,
    pub sgw_fqcsid:Option<Fqcsid>,
    pub sgw_ldn:Option<Ldn>,
    pub pgw_ldn:Option<Ldn>,
    pub pgw_backoff_time:Option<EpcTimer>,
    pub apco:Option<Apco>,
    pub twan_ip_params:Option<Ip4Cp>,
    pub indication:Option<Indication>,
    pub praa:Option<PresenceReportingAreaAction>,
    pub load_control:Vec<LoadControl>, 
    pub overload_info:Vec<OverloadControlInfo>,
    pub nbifom:Option<Fcontainer>,
    pub charging_id:Option<ChargingId>,
    pub epco:Option<Epco>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for CreateSessionResponse {
    fn default() -> CreateSessionResponse {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = CREATE_SESSION_RESP;
        hdr.teid = Some(0);
        CreateSessionResponse {
            header:hdr,
            cause:Cause::default(),
            cra:None,
            csg_ira:None,
            henb_info_report:None,
            fteid_control:None,
            fteid_pgw:None,
            paa:None,
            apn_restriction:None,
            apn_ambr:None,
            linked_ebi:None,
            pco:None,
            bearer_ctxs:vec!(),
            recovery:None,
            charging_gw_name:None,
            charging_gw_ip:None,
            pgw_fqcsid:None,
            sgw_fqcsid:None,
            sgw_ldn:None,
            pgw_ldn:None,
            pgw_backoff_time:None,
            apco:None,
            twan_ip_params:None,
            indication:None,
            praa:None,
            load_control:vec!(), 
            overload_info:vec!(),
            nbifom:None,
            charging_id:None,
            epco:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for CreateSessionResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateSessionResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_SESSION_RESP {
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

    fn to_vec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();
        
        elements.push(self.cause.clone().into());

        match self.cra.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.csg_ira.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.henb_info_report.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.fteid_control.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.fteid_pgw.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.paa.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        
        match self.apn_restriction.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.apn_ambr.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }    

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        match self.recovery.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.charging_gw_name.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        } 

        match self.charging_gw_ip.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }

        match self.pgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.sgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }

        match self.sgw_ldn.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pgw_ldn.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.pgw_backoff_time.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.apco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.twan_ip_params.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.indication.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.praa.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.load_control.iter().for_each(|x| elements.push(InformationElement::LoadControlInfo(x.clone())));

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        match self.nbifom.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.charging_id.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }    
       
        match self.epco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        } 

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:[bool;2]=[false,false];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory[0]) {
                        (0, false) => (self.cause, mandatory[0]) = (j.clone(), true),
                        (_,_) => (),
                    }
                },
                InformationElement::ChangeReportingAction(j) => {
                    match (j.ins, self.cra.is_none()) {
                        (0, true) => self.cra = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::HenbInfoReporting(j) => {
                    match (j.ins, self.henb_info_report.is_none()) {
                        (0, true) => self.henb_info_report = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fteid(j) => { // Two instances
                    match (j.ins, self.fteid_control.is_none(), self.fteid_pgw.is_none()) {
                        (0, true, _) => self.fteid_control = Some(j.clone()),
                        (1, _, true) => self.fteid_pgw = Some(j.clone()),
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
                    match (j.ins, self.apn_restriction.is_none()) {
                        (0, true) => self.apn_restriction = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ApnAmbr(j) => {
                    match (j.ins, self.apn_ambr.is_none()) {
                        (0, true) => self.apn_ambr = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j.clone()),
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
                            mandatory[1]=true;
                            self.bearer_ctxs.push(j.clone());
                        },
                        _ => self.bearer_ctxs.push(j.clone()),
                    }
                }
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fqcsid(j) => {  // 2 instances
                    match (j.ins, self.pgw_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.pgw_fqcsid = Some(j.clone()),
                        (1, _, true) => self.sgw_fqcsid = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Ldn(j) => {  // 2 instances
                    match (j.ins, self.sgw_ldn.is_none(), self.pgw_ldn.is_none()) {
                        (0, true, _) => self.sgw_ldn = Some(j.clone()),
                        (1, _, true) => self.pgw_ldn = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::EpcTimer(j) => {
                    match (j.ins, self.pgw_backoff_time.is_none()) {
                        (0, true) => self.pgw_backoff_time = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Apco(j) => {  
                    match (j.ins, self.apco.is_none()) {
                        (0, true) => self.apco = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Ip4Cp(j) => {  
                    match (j.ins, self.twan_ip_params.is_none()) {
                        (0, true) => self.twan_ip_params = Some(j.clone()),
                        _ => (),
                    }
                },  
                InformationElement::Indication(j) => {  
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PresenceReportingAreaAction(j) => {  
                    match (j.ins, self.praa.is_none()) {
                        (0, true) => self.praa = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::LoadControlInfo(j) => {  
                    match j.ins {
                        k if k<3 => self.load_control.push(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<2 => self.overload_info.push(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Fcontainer(j) => {  
                    match (j.ins, self.nbifom.is_none()) {
                        (0, true) => self.nbifom = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ChargingId(j) => {  
                    match (j.ins, self.charging_id.is_none()) {
                        (0, true) => self.charging_id = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Epco(j) => {  
                    match (j.ins, self.epco.is_none()) {
                        (0, true) => self.epco = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1]) {
            (false,false) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (false,true) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (true,false) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)), 
            (true,true) => Ok(true),
        }
    }
}

#[test]
fn test_create_session_req_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;148] = [
        0x48, 0x21, 0x00, 0x90, 0x09, 0x09, /* .*H!.... */
        0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, /* .V../... */
        0x02, 0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, /* ........ */
        0x11, 0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, /* .H...... */
        0xe8, 0x00, 0x00, 0x03, 0xe8, 0x4e, 0x00, 0x14, /* .....N.. */
        0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, /* ...!.... */
        0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, /* ........ */
        0x06, 0x0a, 0x40, 0xd0, 0x61, 0x4f, 0x00, 0x05, /* ..@.aO.. */
        0x00, 0x01, 0x0a, 0xd8, 0x71, 0x5f, 0x57, 0x00, /* ....q_W. */
        0x09, 0x01, 0x87, 0xb9, 0x7b, 0xbe, 0x07, 0x3e, /* ....{..> */
        0x99, 0x89, 0x4e, 0x5d, 0x00, 0x3a, 0x00, 0x02, /* ..N].:.. */
        0x00, 0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, /* .....I.. */
        0x00, 0x05, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, /* ..P...,. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, /* .;..Z>.. */
        0x55, 0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, /* U^....b. */
        0xc4, 0x7f, 0x00, 0x01, 0x00, 0x00
    ];
    let mut decoded = CreateSessionResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_SESSION_RESP,
            piggyback:false,
            message_prio:None, 
            length:144, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:16,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.recovery = Some (
        Recovery {
            t:RECOVERY,
            length:1,
            ins:0,
            recovery:17,
        }    
    );
    decoded.apn_ambr = Some (
        ApnAmbr {
            t:APNAMBR,
            length:8,
            ins:0,
            ambr_ul:1000,
            ambr_dl:1000,
        }
    );
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
        });
    decoded.paa = Some (
        PdnAddressAllocation { t:PAA, length:5, ins:0, ip: PdnAddress::V4(Ipv4Addr::new(10,216,113,95)) }
    );
    decoded.fteid_pgw = Some(
        Fteid {
            t:FTEID,
            length:9,
            ins:1,
            interface:7,
            teid:0xb97bbe07,
            ipv4: Some(Ipv4Addr::new(62,153,137,78)),
            ipv6: None,
        }
    );
    
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: 93, 
            length: 58, 
            ins: 0,
            cause: Some(
                Cause {
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:16,
                    pce:false,
                    bce:false,
                    cs:false,
                    offend_ie_type:None,
                }
            ),
            tft:None,
            charging_id:Some(
                ChargingId {
                    t: CHARGINGID,
                    length:4,
                    ins: 0,
                    charging_id: 23239876,
                }
            ),
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: Some(vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None })),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    decoded.apn_restriction = Some (
        ApnRestriction {
            t:APNRESTRICTION,
            length:1,
            ins:0,
            restriction_type: Restriction::NoApnRestriction,
        });
    
    let message = CreateSessionResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_create_session_req_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;148] = [
        0x48, 0x21, 0x00, 0x90, 0x09, 0x09, /* .*H!.... */
        0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, /* .V../... */
        0x02, 0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, /* ........ */
        0x11, 0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0x03, /* .H...... */
        0xe8, 0x00, 0x00, 0x03, 0xe8, 0x4e, 0x00, 0x14, /* .....N.. */
        0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, /* ...!.... */
        0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, /* ........ */
        0x06, 0x0a, 0x40, 0xd0, 0x61, 0x4f, 0x00, 0x05, /* ..@.aO.. */
        0x00, 0x01, 0x0a, 0xd8, 0x71, 0x5f, 0x57, 0x00, /* ....q_W. */
        0x09, 0x01, 0x87, 0xb9, 0x7b, 0xbe, 0x07, 0x3e, /* ....{..> */
        0x99, 0x89, 0x4e, 0x5d, 0x00, 0x3a, 0x00, 0x02, /* ..N].:.. */
        0x00, 0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, /* .....I.. */
        0x00, 0x05, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, /* ..P...,. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 0x89, /* .;..Z>.. */
        0x55, 0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, /* U^....b. */
        0xc4, 0x7f, 0x00, 0x01, 0x00, 0x00
    ];
    let mut decoded = CreateSessionResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_SESSION_RESP,
            piggyback:false,
            message_prio:None, 
            length:144, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:16,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.recovery = Some (
        Recovery {
            t:RECOVERY,
            length:1,
            ins:0,
            recovery:17,
        }    
    );
    decoded.apn_ambr = Some (
        ApnAmbr {
            t:APNAMBR,
            length:8,
            ins:0,
            ambr_ul:1000,
            ambr_dl:1000,
        }
    );
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
        });
    decoded.paa = Some (
        PdnAddressAllocation { t:PAA, length:5, ins:0, ip: PdnAddress::V4(Ipv4Addr::new(10,216,113,95)) }
    );
    decoded.fteid_pgw = Some(
        Fteid {
            t:FTEID,
            length:9,
            ins:1,
            interface:7,
            teid:0xb97bbe07,
            ipv4: Some(Ipv4Addr::new(62,153,137,78)),
            ipv6: None,
        }
    );
    
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: 93, 
            length: 58, 
            ins: 0,
            cause: Some(
                Cause {
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:16,
                    pce:false,
                    bce:false,
                    cs:false,
                    offend_ie_type:None,
                }
            ),
            tft:None,
            charging_id:Some(
                ChargingId {
                    t: CHARGINGID,
                    length:4,
                    ins: 0,
                    charging_id: 23239876,
                }
            ),
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: Some(vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None })),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    decoded.apn_restriction = Some (
        ApnRestriction {
            t:APNRESTRICTION,
            length:1,
            ins:0,
            restriction_type: Restriction::NoApnRestriction,
        });
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
