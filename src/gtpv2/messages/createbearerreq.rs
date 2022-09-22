use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const CREATE_BEARER_REQ:u8 = 95;

// Definition of GTPv2-C Create Bearer Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct CreateBearerRequest {
    pub header:Gtpv2Header,
    pub pti:Option<Pti>,
    pub linked_ebi:Ebi,
    pub pco:Option<Pco>,
    pub bearer_ctxs:Vec<BearerContext>,
    pub pgw_fqcsid:Option<Fqcsid>,
    pub sgw_fqcsid:Option<Fqcsid>,
    pub cra: Option<ChangeReportingAction>,
    pub csg_ira: Option<CSGInformationReportingAction>,
    pub henb_info_report:Option<HenbInfoReporting>,
    pub praa:Option<PresenceReportingAreaAction>,
    pub indication:Option<Indication>,  
    pub load_control:Vec<LoadControl>, 
    pub overload_info:Vec<OverloadControlInfo>,
    pub nbifom:Option<Fcontainer>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for CreateBearerRequest {
    fn default() -> CreateBearerRequest {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = CREATE_BEARER_REQ;
        hdr.teid = Some(0);
        CreateBearerRequest {
            header:hdr,
            pti:None,
            linked_ebi:Ebi::default(),
            pco:None,
            bearer_ctxs:vec!(),
            pgw_fqcsid:None,
            sgw_fqcsid:None,
            cra: None,
            csg_ira: None,
            henb_info_report:None,
            praa:None,
            indication:None,  
            load_control:vec!(), 
            overload_info:vec!(),
            nbifom:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for CreateBearerRequest {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateBearerRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_BEARER_REQ {
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
        
        match self.pti.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        elements.push(self.linked_ebi.clone().into());

        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        match self.pgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.sgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
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

        match self.praa.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.indication.clone() {
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
        let mut mandatory:[bool;2]=[false,false];
        for e in elements.iter() {
            match e {
                InformationElement::Pti(j) => {
                    match (j.ins, self.pti.is_none()) {
                        (0, true) => self.pti = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, mandatory[0]) {
                        (0, false) => (self.linked_ebi, mandatory[0]) = (j.clone(), true),
                        (_,_) => (),
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
                        _ => (),
                    }
                }
                InformationElement::Fqcsid(j) => {  // 2 instances
                    match (j.ins, self.pgw_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.pgw_fqcsid = Some(j.clone()),
                        (1, _, true) => self.sgw_fqcsid = Some(j.clone()),
                        _ => (),
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
                InformationElement::PresenceReportingAreaAction(j) => {
                    match (j.ins, self.praa.is_none()) {
                        (0, true) => self.praa = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Indication(j) => {  
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
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
        match (mandatory[0], mandatory[1]) {
            (false,false) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (false,true) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (true,false) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)), 
            (true,true) => Ok(true),
        }
    }
}

#[test]
fn test_create_bearer_req_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;97] = [
        0x48,0x5f,0x00,0x5d,0x09,0x09,0xa4,0x56,
        0x00,0x00,0x2f,0x00,0x49,0x00,0x01,0x00,
        0x05,0x4e,0x00,0x14,0x00,0x80,0x80,0x21,
        0x10,0x02,0x00,0x00,0x10,0x81,0x06,0x08,
        0x08,0x08,0x08,0x83,0x06,0x0a,0x40,0xd0,
        0x61,0x5d,0x00,0x34,0x00,0x49,0x00,0x01,
        0x00,0x00,0x57,0x00,0x09,0x02,0x85,0x3b,
        0x95,0x98,0x5a,0x3e,0x99,0x89,0x55,0x50,
        0x00,0x16,0x00,0x2c,0x09,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x5e,0x00,0x04,0x00,0x01,0x62,0x9c,
        0xc4
    ];
    let mut decoded = CreateBearerRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_BEARER_REQ,
            piggyback:false,
            message_prio:None, 
            length:93, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.linked_ebi = Ebi{
        t:EBI,
        length:EBI_LENGTH as u16,
        ins:0,
        value:5,
    };
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
    
    let message = CreateBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_create_bearer_req_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;97] = [
        0x48,0x5f,0x00,0x5d,0x09,0x09,0xa4,0x56,
        0x00,0x00,0x2f,0x00,0x49,0x00,0x01,0x00,
        0x05,0x4e,0x00,0x14,0x00,0x80,0x80,0x21,
        0x10,0x02,0x00,0x00,0x10,0x81,0x06,0x08,
        0x08,0x08,0x08,0x83,0x06,0x0a,0x40,0xd0,
        0x61,0x5d,0x00,0x34,0x00,0x49,0x00,0x01,
        0x00,0x00,0x57,0x00,0x09,0x02,0x85,0x3b,
        0x95,0x98,0x5a,0x3e,0x99,0x89,0x55,0x50,
        0x00,0x16,0x00,0x2c,0x09,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x5e,0x00,0x04,0x00,0x01,0x62,0x9c,
        0xc4
    ];
    let mut decoded = CreateBearerRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_BEARER_REQ,
            piggyback:false,
            message_prio:None, 
            length:93, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.linked_ebi = Ebi{
        t:EBI,
        length:EBI_LENGTH as u16,
        ins:0,
        value:5,
    };
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
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}