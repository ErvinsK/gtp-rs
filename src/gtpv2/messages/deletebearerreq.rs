use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const DELETE_BEARER_REQ:u8 = 99;

// Definition of GTPv2-C Delete Bearer Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteBearerRequest {
    pub header:Gtpv2Header,
    pub linked_ebi:Option<Ebi>,
    pub ebi:Option<Ebi>,
    pub bearer_ctxs:Vec<BearerContext>,
    pub pti:Option<Pti>,
    pub pco:Option<Pco>,
    pub pgw_fqcsid:Option<Fqcsid>,
    pub sgw_fqcsid:Option<Fqcsid>,
    pub cause:Option<Cause>,
    pub indication:Option<Indication>,
    pub load_control:Vec<LoadControl>, 
    pub overload_info:Vec<OverloadControlInfo>,
    pub nbifom:Option<Fcontainer>,
    pub apn_rate_control_status:Option<ApnRateControlStatus>,
    pub epco:Option<Epco>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for DeleteBearerRequest {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = DELETE_BEARER_REQ;
        hdr.teid = Some(0);
        DeleteBearerRequest {
            header:hdr,
            linked_ebi:None,
            ebi:None,            
            bearer_ctxs:vec!(),
            pti:None,
            pco:None,
            pgw_fqcsid:None,
            sgw_fqcsid:None,
            cause:None,
            indication:None,
            load_control:vec!(), 
            overload_info:vec!(),
            nbifom:None,
            apn_rate_control_status:None,
            epco:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for DeleteBearerRequest {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteBearerRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_BEARER_REQ {
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

        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        match self.pti.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }  
        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }  
        match self.pgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.sgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.cause.clone() {
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
        match self.apn_rate_control_status.clone() {
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
        for e in elements.into_iter() {
            match e {
                InformationElement::Ebi(j) => {  // 2 instances
                    match (j.ins, self.linked_ebi.is_none(), self.ebi.is_none()) {
                        (0, true, _) => self.linked_ebi = Some(j),
                        (1, _, true) => self.ebi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::BearerContext(j) => {
                    match j.ins {
                        0 => self.bearer_ctxs.push(j),
                        _ => (),
                    }
                }
                InformationElement::Pti(j) => {
                    match (j.ins, self.pti.is_none()) {
                        (0, true) => self.pti = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Pco(j) => {
                    match (j.ins, self.pco.is_none()) {
                        (0, true) => self.pco = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Fqcsid(j) => {  // 2 instances
                    match (j.ins, self.pgw_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.pgw_fqcsid = Some(j),
                        (1, _, true) => self.sgw_fqcsid = Some(j),
                        _ => (),
                    }
                }, 
                InformationElement::Cause(j) => {
                    match (j.ins, self.cause.is_none()) {
                        (0, true) => self.cause = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Indication(j) => {  
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j),
                        _ => (),
                    }
                }, 
                InformationElement::LoadControlInfo(j) => {  
                    match j.ins {
                        k if k<3 => self.load_control.push(j),
                        _ => (),
                    }
                }, 
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<2 => self.overload_info.push(j),
                        _ => (),
                    }
                }, 
                InformationElement::Fcontainer(j) => {  
                    match (j.ins, self.nbifom.is_none()) {
                        (0, true) => self.nbifom = Some(j),
                        _ => (),
                    }
                },
                InformationElement::ApnRateControlStatus(j) => {  
                    match (j.ins, self.apn_rate_control_status.is_none()) {
                        (0, true) => self.apn_rate_control_status = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Epco(j) => {  
                    match (j.ins, self.epco.is_none()) {
                        (0, true) => self.epco = Some(j),
                        _ => (),
                    }
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_delete_bearer_req_unmarshal () {
    let encoded:[u8;43] = [
        0x48, 0x63, 0x00, 0x27, 0xa4, 0x78, 0x95, 0x80,
        0x4b, 0x29, 0x1e, 0x00, 0x49, 0x00, 0x01, 0x00,
        0x05, 0x5d, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x02, 
        0x00, 0x08, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 
        0x64, 0x00, 0x01, 0x00, 0xfa, 0x02, 0x00, 0x02, 
        0x00, 0x08, 0x00,
    ];
    let mut decoded = DeleteBearerRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_BEARER_REQ,
            piggyback:false,
            message_prio:None, 
            length:39, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.cause = Some(
        Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:8,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    });
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.pti = Some (
        Pti {
            t:PTI,
            length:PTI_LENGTH as u16,
            ins: 0,
            pti: 0xfa,
        }
    );
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: BEARER_CTX, 
            length: 11, 
            ins: 0,
            cause: Some(
                Cause {
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:8,
                    pce:false,
                    bce:false,
                    cs:false,
                    offend_ie_type:None,
                }
            ),
            tft:None,
            charging_id:None,
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: None,
            bearer_qos:None,
            });
    let message = DeleteBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_delete_bearer_req_marshal () {
    let encoded:[u8;43] = [
        0x48, 0x63, 0x00, 0x27, 0xa4, 0x78, 0x95, 0x80,
        0x4b, 0x29, 0x1e, 0x00, 0x49, 0x00, 0x01, 0x00,
        0x05, 0x5d, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x02, 
        0x00, 0x08, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 
        0x64, 0x00, 0x01, 0x00, 0xfa, 0x02, 0x00, 0x02, 
        0x00, 0x08, 0x00,
    ];
    let mut decoded = DeleteBearerRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_BEARER_REQ,
            piggyback:false,
            message_prio:None, 
            length:39, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.cause = Some(
        Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:8,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    });
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.pti = Some (
        Pti {
            t:PTI,
            length:PTI_LENGTH as u16,
            ins: 0,
            pti: 0xfa,
        }
    );
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: BEARER_CTX, 
            length: 11, 
            ins: 0,
            cause: Some(
                Cause {
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:8,
                    pce:false,
                    bce:false,
                    cs:false,
                    offend_ie_type:None,
                }
            ),
            tft:None,
            charging_id:None,
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: None,
            bearer_qos:None,
            });
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
