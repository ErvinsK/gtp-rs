use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const DELETE_BEARER_FAIL:u8 = 67;

// Definition of GTPv2-C Delete Bearer Failure Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteBearerFailureInd {
    pub header:Gtpv2Header,
    pub cause: Cause,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>, 
    pub indication: Option<Indication>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for DeleteBearerFailureInd {
    fn default() -> Self {
        let hdr = Gtpv2Header{
            msgtype:DELETE_BEARER_FAIL,
            teid:Some(0),
            ..Default::default()};
        DeleteBearerFailureInd {
            header: hdr,
            cause: Cause::default(),
            bearer_ctxs: vec!(),
            recovery: None,
            indication: None,
            overload_info: vec!(),
            private_ext: vec!(),
        }
    }
}

impl Messages for DeleteBearerFailureInd {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteBearerFailureInd::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_BEARER_FAIL {
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
        
        elements.push(self.cause.clone().into());

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }
        
        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));    

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory=false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.cause, mandatory) = (j.clone(), true);
                    }
                },
                InformationElement::BearerContext(j) => {  
                    if let 0 = j.ins {
                        self.bearer_ctxs.push(j.clone());
                    }
                }, 
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone());
                    }
                },
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
                    }
                },
                InformationElement::OverloadControlInfo(j) => {  
                    if j.ins < 2 {
                        self.overload_info.push(j.clone());
                    }
                }, 
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory, self.bearer_ctxs.is_empty()) {
            (false,false) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (true,true) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)),
            (false,true) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)), 
            (true,false) => Ok(true),
        }
    }   
}

#[test]
fn test_delete_bearer_failure_ind_unmarshal () {
    let encoded:[u8;77] = [
        0x48, 0x43, 0x00, 0x49, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x4d, 0x00, 0x5d, 0x00, 0x0b, 0x00, 0x02, 0x00, 
        0x02, 0x00, 0x4d, 0x00, 0x49, 0x00, 0x01, 0x00, 
        0x05, 0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04, 
        0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 
        0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 
        0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 
        0xaa, 0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 
        0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let mut decoded = DeleteBearerFailureInd::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_BEARER_FAIL,
            piggyback:false,
            message_prio:None, 
            length:73, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:77,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: BEARER_CTX, 
            length: 11, 
            ins: 0,
            cause: Some(
                Cause{
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:77,
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
            fteids: vec!(),
            bearer_qos: None,
            });
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
    let message = DeleteBearerFailureInd::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_delete_bearer_failure_ind_marshal () {
    let encoded:[u8;77] = [
        0x48, 0x43, 0x00, 0x49, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x4d, 0x00, 0x5d, 0x00, 0x0b, 0x00, 0x02, 0x00, 
        0x02, 0x00, 0x4d, 0x00, 0x49, 0x00, 0x01, 0x00, 
        0x05, 0xb4, 0x00, 0x12, 0x00, 0xb7, 0x00, 0x04, 
        0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 
        0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0xb4, 
        0x00, 0x12, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 
        0xaa, 0xee, 0x22, 0xb6, 0x00, 0x01, 0x00, 0x60, 
        0x9c, 0x00, 0x01, 0x00, 0x7e,
    ];
    let mut decoded = DeleteBearerFailureInd::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_BEARER_FAIL,
            piggyback:false,
            message_prio:None, 
            length:73, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:77,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: BEARER_CTX, 
            length: 11, 
            ins: 0,
            cause: Some(
                Cause{
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:77,
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
            fteids: vec!(),
            bearer_qos: None,
            });
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
