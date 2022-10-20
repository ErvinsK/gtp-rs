use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const UPD_BEARER_REQ:u8 = 97;

// Definition of GTPv2-C Update Bearer Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateBearerRequest {
    pub header:Gtpv2Header,
    pub bearer_ctxs:Vec<BearerContext>,
    pub pti:Option<Pti>,
    pub pco:Option<Pco>,
    pub apnambr:ApnAmbr,
    pub cra: Option<ChangeReportingAction>,
    pub csg_ira: Option<CSGInformationReportingAction>,
    pub henb_info_report:Option<HenbInfoReporting>,
    pub indication:Option<Indication>,    
    pub pgw_fqcsid:Option<Fqcsid>,
    pub sgw_fqcsid:Option<Fqcsid>,
    pub praa:Option<PresenceReportingAreaAction>,
    pub load_control:Vec<LoadControl>, 
    pub overload_info:Vec<OverloadControlInfo>,
    pub nbifom:Option<Fcontainer>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for UpdateBearerRequest {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = UPD_BEARER_REQ;
        hdr.teid = Some(0);
        UpdateBearerRequest {
            header:hdr,
            bearer_ctxs:vec!(),
            pti:None,
            pco:None,
            apnambr:ApnAmbr::default(),            
            cra:None,
            csg_ira:None,
            henb_info_report:None,
            indication:None,  
            pgw_fqcsid:None,
            sgw_fqcsid:None,
            praa:None,
            load_control:vec!(), 
            overload_info:vec!(),
            nbifom:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for UpdateBearerRequest {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = UpdateBearerRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UPD_BEARER_REQ {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            match InformationElement::decoder(&buffer[12..]) {
                Ok(i) => {
                    match message.from_vec(i) {
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

    fn to_vec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));
        
        match self.pti.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        
        elements.push(self.apnambr.clone().into());

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
        match self.indication.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }     
        match self.pgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.sgw_fqcsid.clone() {
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

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:bool=false;
        for e in elements.iter() {
            match e {
                InformationElement::BearerContext(j) => {
                    match j.ins {
                        0 => {
                            self.bearer_ctxs.push(j.clone());
                        },
                        _ => (),
                    }
                }
                InformationElement::Pti(j) => {
                    match (j.ins, self.pti.is_none()) {
                        (0, true) => self.pti = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Pco(j) => {
                    match (j.ins, self.pco.is_none()) {
                        (0, true) => self.pco = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::ApnAmbr(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.apnambr, mandatory) = (j.clone(), true),
                        (_,_) => (),
                    }
                },
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
                InformationElement::Indication(j) => {  
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
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
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory, self.bearer_ctxs.is_empty()) {
            (false,true) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)),
            (false,false) => Err(GTPV2Error::MessageMandatoryIEMissing(APNAMBR)),
            (true,true) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)), 
            (true,false) => Ok(true),
        }
    }
}

#[test]
fn test_update_bearer_req_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;164] = [
        0x48, 0x61, 0x00, 0xa0, 0x09, 0x09, 0xa4, 0x56,
        0x00, 0x00, 0x2f, 0x00, 0x5d, 0x00, 0x34, 0x00, 
        0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 
        0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 
        0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x5e, 0x00, 0x04, 0x00, 
        0x01, 0x62, 0x9c, 0xc4, 0x64, 0x00, 0x01, 0x00, 
        0xfa, 0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 
        0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 
        0x08, 0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 
        0x61, 0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0xc3, 
        0x50, 0x00, 0x02, 0x49, 0xf0, 0x84, 0x00, 0x07, 
        0x00, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff, 
        0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04, 0x00, 
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 
        0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 
        0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 
        0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 
        0x00, 0x01, 0x00, 0x7e,
    ];
    let mut decoded = UpdateBearerRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:UPD_BEARER_REQ,
            piggyback:false,
            message_prio:None, 
            length:160, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.bearer_ctxs = vec!(
            BearerContext { 
                t: 93, 
                length: 52, 
                ins: 0,
                cause: None,
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
                ebi: Ebi { t: EBI, length: 1, ins: 0, value: 0 },
                fteids: vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None }),
                bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
    });
    decoded.pti = Some (
        Pti { t: PTI, length: PTI_LENGTH as u16, ins: 0, pti: 0xfa }
    );
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
    });
    decoded.apnambr = ApnAmbr {
        t:APNAMBR,
        length:APNAMBR_LENGTH as u16,
        ins: 0,
        ambr_ul:  50000,
        ambr_dl: 150000,
    };
    decoded.pgw_fqcsid = Some (
        Fqcsid {  t:FQCSID,
            length:7, 
            ins:0, 
            nodeid: NodeId::V4(Ipv4Addr::new(139,7,133,184)),
            csid: vec!(0xffff) 
        }
    );
    decoded.overload_info = vec!(
        OverloadControlInfo {
                t: OVERLOAD_CNTRL, 
                length: 18, 
                ins: 0, 
                sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
                metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
                validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:31 },
                list: None,
        },
        OverloadControlInfo {
            t: OVERLOAD_CNTRL, 
            length: 18, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee22 },
            metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:30 },
            list: None,
    },   
    );
    let message = UpdateBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_update_bearer_req_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;164] = [
        0x48, 0x61, 0x00, 0xa0, 0x09, 0x09, 0xa4, 0x56,
        0x00, 0x00, 0x2f, 0x00, 0x5d, 0x00, 0x34, 0x00, 
        0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 
        0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e, 0x99, 
        0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x5e, 0x00, 0x04, 0x00, 
        0x01, 0x62, 0x9c, 0xc4, 0x64, 0x00, 0x01, 0x00, 
        0xfa, 0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 
        0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 
        0x08, 0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 
        0x61, 0x48, 0x00, 0x08, 0x00, 0x00, 0x00, 0xc3, 
        0x50, 0x00, 0x02, 0x49, 0xf0, 0x84, 0x00, 0x07, 
        0x00, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff, 
        0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04, 0x00, 
        0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 
        0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 0x00, 
        0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 
        0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 
        0x00, 0x01, 0x00, 0x7e,
    ];
    let mut decoded = UpdateBearerRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:UPD_BEARER_REQ,
            piggyback:false,
            message_prio:None, 
            length:160, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.bearer_ctxs = vec!(
            BearerContext { 
                t: 93, 
                length: 52, 
                ins: 0,
                cause: None,
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
                ebi: Ebi { t: EBI, length: 1, ins: 0, value: 0 },
                fteids: vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None }),
                bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
    });
    decoded.pti = Some (
        Pti { t: PTI, length: PTI_LENGTH as u16, ins: 0, pti: 0xfa }
    );
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
    });
    decoded.apnambr = ApnAmbr {
        t:APNAMBR,
        length:APNAMBR_LENGTH as u16,
        ins: 0,
        ambr_ul:  50000,
        ambr_dl: 150000,
    };
    decoded.pgw_fqcsid = Some (
        Fqcsid {  t:FQCSID,
            length:7, 
            ins:0, 
            nodeid: NodeId::V4(Ipv4Addr::new(139,7,133,184)),
            csid: vec!(0xffff) 
        }
    );
    decoded.overload_info = vec!(
        OverloadControlInfo {
                t: OVERLOAD_CNTRL, 
                length: 18, 
                ins: 0, 
                sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
                metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
                validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:31 },
                list: None,
        },
        OverloadControlInfo {
            t: OVERLOAD_CNTRL, 
            length: 18, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee22 },
            metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:30 },
            list: None,
    },   
    );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
