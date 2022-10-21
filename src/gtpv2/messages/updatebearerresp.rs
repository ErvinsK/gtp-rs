use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const UPD_BEARER_RESP:u8 = 98;

// Definition of GTPv2-C Update Bearer Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateBearerResponse {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub bearer_ctxs:Vec<BearerContext>,
    pub pco:Option<Pco>,
    pub recovery:Option<Recovery>,
    pub mme_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub epdg_fqcsid: Option<Fqcsid>,
    pub twan_fqcsid: Option<Fqcsid>,
    pub indication: Option<Indication>,
    pub uetimezone: Option<UeTimeZone>,
    pub uli: Option<Uli>,
    pub twan_id: Option<TwanId>,
    pub overload_info:Vec<OverloadControlInfo>,
    pub prai:Option<PresenceReportingAreaInformation>,
    pub ip: Option<IpAddress>,   // Either MME ID IE (S11/S4/S5/S8) or UE Local IP IE (S2b) 
    pub wlan_loc: Option<TwanId>,
    pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    pub ue_udpport: Option<PortNumber>,
    pub nbifom:Option<Fcontainer>,
    pub ue_tcpport: Option<PortNumber>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for UpdateBearerResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header{
            msgtype:UPD_BEARER_RESP,
            teid:Some(0),
            ..Default::default()};
        UpdateBearerResponse {
            header:hdr,
            cause:Cause::default(),
            bearer_ctxs:vec!(),
            pco:None,
            recovery:None,
            mme_fqcsid:None,
            sgw_fqcsid:None,
            epdg_fqcsid:None,
            twan_fqcsid:None,
            indication:None,
            uetimezone:None,
            uli:None,
            twan_id:None,
            overload_info:vec!(),
            prai:None,
            ip:None,
            wlan_loc:None,
            wlan_loc_timestamp:None,
            ue_udpport:None,
            nbifom:None,
            ue_tcpport:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for UpdateBearerResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = UpdateBearerResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UPD_BEARER_RESP {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            match InformationElement::decoder(&buffer[12..]) {
                Ok(i) => {
                    match message.fromvec(i) {
                        Ok(_) => Ok(message),
                        Err(j) => Err(j),
                    }
                },
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }

    fn tovec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();
        
        elements.push(self.cause.clone().into());

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.pco.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.recovery.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.mme_fqcsid.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.epdg_fqcsid.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.twan_fqcsid.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.uetimezone.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.uli.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.twan_id.clone() {
            elements.push(i.into());
        }
        
        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.prai.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.ip.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.wlan_loc.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.wlan_loc_timestamp.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.ue_udpport.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.ue_tcpport.clone() {
            elements.push(i.into());
        }

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn fromvec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:[bool;2]=[false,false];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        (self.cause, mandatory[0]) = (j.clone(), true);
                    }
                },
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        mandatory[1]=true;
                        self.bearer_ctxs.push(j.clone());
                    }
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j.clone());
                    }
                },
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone());
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
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
                    }
                },
                InformationElement::UeTimeZone(j) => {
                    if let (0, true) = (j.ins, self.uetimezone.is_none()) {
                        self.uetimezone = Some(j.clone());
                    }
                },
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.uli.is_none()) {
                        self.uli = Some(j.clone());
                    }
                },
                InformationElement::TwanId(j) => {
                    match (j.ins, self.twan_id.is_none(), self.wlan_loc.is_none()) {
                        (0, true, _) => self.twan_id = Some(j.clone()),
                        (1, _, true) => self.wlan_loc = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::OverloadControlInfo(j) => {  
                    if j.ins < 3 {
                        self.overload_info.push(j.clone());
                    }
                }, 
                InformationElement::PresenceReportingAreaInformation(j) => {
                    if let (0, true) = (j.ins, self.prai.is_none()) {
                        self.prai = Some(j.clone());
                    }
                },
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.ip.is_none()) {
                        self.ip = Some(j.clone());
                    }
                },
                InformationElement::TwanIdTimeStamp(j) => {
                    if let (1, true) = (j.ins, self.wlan_loc_timestamp.is_none()) {
                        self.wlan_loc_timestamp = Some(j.clone());
                    }
                },
                InformationElement::PortNumber(j) => {
                    match (j.ins, self.ue_udpport.is_none(), self.ue_tcpport.is_none()) {
                        (0, true, _) => self.ue_udpport = Some(j.clone()),
                        (1, _, true) => self.ue_tcpport = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fcontainer(j) => {  
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j.clone());
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
fn test_update_bearer_resp_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;109] = [
        0x48, 0x62, 0x00, 0x69, 0x09, 0x09, 0xa4, 0x56,
        0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x5d, 0x00, 0x3a, 0x00, 0x02, 0x00, 
        0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 
        0x05, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x50, 0x00, 
        0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4, 
        0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 
        0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 
        0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 
        0x03, 0x00, 0x01, 0x00, 0x11
    ];
    let mut decoded = UpdateBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:UPD_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:105, 
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
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
        });
    
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
            ran_nas_cause:None,
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None }),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    
    let message = UpdateBearerResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_update_bearer_resp_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;109] = [
        0x48, 0x62, 0x00, 0x69, 0x09, 0x09, 0xa4, 0x56,
        0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x5d, 0x00, 0x3a, 0x00, 0x02, 0x00, 
        0x02, 0x00, 0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 
        0x05, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x50, 0x00, 
        0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4, 
        0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 
        0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 
        0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 
        0x03, 0x00, 0x01, 0x00, 0x11
    ];
    let mut decoded = UpdateBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:UPD_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:105, 
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
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
        });
    
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
            ran_nas_cause:None,
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None }),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
