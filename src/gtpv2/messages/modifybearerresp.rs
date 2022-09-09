use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const MODIFY_BEARER_RESP:u8 = 35;

// Definition of GTPv2-C Modify Bearer Response Message

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyBearerResponse {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub msisdn: Option<Msisdn>,
    pub linked_ebi:Option<Ebi>,
    pub apn_restriction:Option<ApnRestriction>,
    pub pco:Option<Pco>,
    pub bearer_ctxs:Vec<BearerContext>,
    pub cra:Option<ChangeReportingAction>,
    pub csg_ira:Option<CSGInformationReportingAction>,
    pub henb_info_report:Option<HenbInfoReporting>,
    pub charging_gw_name:Option<Fqdn>,
    pub charging_gw_ip:Option<IpAddress>,
    pub pgw_fqcsid:Option<Fqcsid>,
    pub sgw_fqcsid:Option<Fqcsid>,
    pub recovery:Option<Recovery>,
    pub sgw_ldn:Option<Ldn>,
    pub pgw_ldn:Option<Ldn>,
    pub indication:Option<Indication>,
    pub praa:Option<PresenceReportingAreaAction>,
    pub load_control:Vec<LoadControl>, 
    pub overload_info:Vec<OverloadControlInfo>,
    pub charging_id:Option<ChargingId>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for ModifyBearerResponse {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = MODIFY_BEARER_RESP;
        hdr.teid = Some(0);
        ModifyBearerResponse {
            header:hdr,
            cause:Cause::default(),
            msisdn:None,
            linked_ebi:None,
            apn_restriction:None,
            pco:None,
            bearer_ctxs:vec!(),
            cra:None,
            csg_ira:None,
            henb_info_report:None,
            charging_gw_name:None,
            charging_gw_ip:None,
            pgw_fqcsid:None,
            sgw_fqcsid:None,
            recovery:None,
            sgw_ldn:None,
            pgw_ldn:None,
            indication:None,
            praa:None,
            load_control:vec!(), 
            overload_info:vec!(),
            charging_id:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for ModifyBearerResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyBearerResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_BEARER_RESP {
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

        match self.msisdn.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.apn_restriction.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }  

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

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
        match self.recovery.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.sgw_ldn.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pgw_ldn.clone() {
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

        match self.charging_id.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }    

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory=false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::Msisdn(j) => {
                    match (j.ins, self.msisdn.is_none()) {
                        (0, true) => self.msisdn = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ApnRestriction(j) => {
                    match (j.ins, self.apn_restriction.is_none()) {
                        (0, true) => self.apn_restriction = Some(j.clone()),
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
                        k if k<2 => self.bearer_ctxs.push(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::ChangeReportingAction(j) => {
                    match (j.ins, self.cra.is_none()) {
                        (0, true) => self.cra = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::CSGInformationReportingAction(j) => {
                    match (j.ins, self.csg_ira.is_none()) {
                        (0, true) => self.csg_ira = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::HenbInfoReporting(j) => {
                    match (j.ins, self.henb_info_report.is_none()) {
                        (0, true) => self.henb_info_report = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fqdn(j) => {
                    match (j.ins, self.charging_gw_name.is_none()) {
                        (0, true) => self.charging_gw_name = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::IpAddress(j) => {
                    match (j.ins, self.charging_gw_ip.is_none()) {
                        (0, true) => self.charging_gw_ip = Some(j.clone()),
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
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j.clone()),
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
                InformationElement::ChargingId(j) => {  
                    match (j.ins, self.charging_id.is_none()) {
                        (0, true) => self.charging_id = Some(j.clone()),
                        _ => (),
                    }
                },
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
fn test_modify_bearer_resp_unmarshal () {
    let encoded:[u8;68] = [
        0x48, 0x23, 0x00, 0x40, 0xa4, 0x78, 0x95, 0x80,
        0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, 
        0x58, 0x01, 0x02, 0x93, 0x56, 0xf0, 0x49, 0x00, 
        0x01, 0x00, 0x05, 0x7f, 0x00, 0x01, 0x00, 0x00, 
        0x5d, 0x00, 0x13, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x5e, 
        0x00, 0x04, 0x00, 0x01, 0x76, 0x4f, 0xbb, 0x03, 
        0x00, 0x01, 0x00, 0x08,
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
    let mut decoded = ModifyBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:MODIFY_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:64, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
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
            recovery:8,
        }    
    );
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.msisdn = Some (
        Msisdn {
            t:MSISDN,
            length:8,
            ins:0,
            msisdn: "882285102039650".to_string(),
        });
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: BEARER_CTX, 
            length: 19, 
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
                    charging_id: 24530875,
                }
            ),
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: None,
            bearer_qos:None,
            });
    decoded.apn_restriction = Some (
        ApnRestriction {
            t:APNRESTRICTION,
            length:1,
            ins:0,
            restriction_type: Restriction::NoApnRestriction,
        });
    
    let message = ModifyBearerResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_modify_bearer_resp_marshal () {
    let encoded:[u8;68] = [
        0x48, 0x23, 0x00, 0x40, 0xa4, 0x78, 0x95, 0x80,
        0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x4c, 0x00, 0x08, 0x00, 0x88, 0x22, 
        0x58, 0x01, 0x02, 0x93, 0x56, 0xf0, 0x49, 0x00, 
        0x01, 0x00, 0x05, 0x7f, 0x00, 0x01, 0x00, 0x00, 
        0x5d, 0x00, 0x13, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x5e, 
        0x00, 0x04, 0x00, 0x01, 0x76, 0x4f, 0xbb, 0x03, 
        0x00, 0x01, 0x00, 0x08,
    ];
    let mut decoded = ModifyBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:MODIFY_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:64, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
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
            recovery:8,
        }    
    );
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.msisdn = Some (
        Msisdn {
            t:MSISDN,
            length:8,
            ins:0,
            msisdn: "882285102039650".to_string(),
        });
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: BEARER_CTX, 
            length: 19, 
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
                    charging_id: 24530875,
                }
            ),
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: None,
            bearer_qos:None,
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
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
