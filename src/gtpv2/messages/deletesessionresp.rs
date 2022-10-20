use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const DELETE_SESSION_RESP:u8 = 37;

// Definition of GTPv2-C Delete Session Response Message

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteSessionResponse {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub recovery:Option<Recovery>,
    pub pco:Option<Pco>,
    pub indication:Option<Indication>,
    pub load_control:Vec<LoadControl>, 
    pub overload_info:Vec<OverloadControlInfo>,
    pub epco:Option<Epco>,
    pub apn_rate_control_status:Option<ApnRateControlStatus>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for DeleteSessionResponse {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = DELETE_SESSION_RESP;
        hdr.teid = Some(0);
        DeleteSessionResponse {
            header:hdr,
            cause:Cause::default(),
            recovery:None,
            pco:None,
            indication:None,
            load_control:vec!(), 
            overload_info:vec!(),
            epco:None,
            apn_rate_control_status:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for DeleteSessionResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteSessionResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_SESSION_RESP {
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

        match self.recovery.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.indication.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.load_control.iter().for_each(|x| elements.push(InformationElement::LoadControlInfo(x.clone())));

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        match self.epco.clone() {
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
        let mut mandatory=false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory) = (j, true),
                        _ => (),
                    }
                },
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Pco(j) => {
                    match (j.ins, self.pco.is_none()) {
                        (0, true) => self.pco = Some(j),
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
                InformationElement::Epco(j) => {
                    match (j.ins, self.epco.is_none()) {
                        (0, true) => self.epco = Some(j),
                        _ => (),
                    }
                },
                InformationElement::ApnRateControlStatus(j) => {
                    match (j.ins, self.apn_rate_control_status.is_none()) {
                        (0, true) => self.apn_rate_control_status = Some(j),
                        _ => (),
                    }
                },
               
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
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
fn test_delete_session_resp_unmarshal () {
    let encoded:[u8;200] = [
        0x48, 0x25, 0x00, 0xc4, 0x02, 0x15, 0xfd, 0x34,
        0x00, 0x00, 0x70, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x08, 0x4e, 
        0x00, 0x23, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 
        0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 
        0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a, 0x00, 
        0x00, 0x05, 0x00, 0x00, 0x10, 0x00, 0xb5, 0x00, 
        0x1f, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 
        0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0xb8, 
        0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 
        0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 
        0x6d, 0xb5, 0x00, 0x1f, 0x01, 0xb7, 0x00, 0x04, 
        0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 
        0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 
        0x72, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 
        0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x12, 0x00, 
        0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 
        0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 
        0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 
        0x04, 0x00, 0xff, 0xaa, 0xee, 0x22, 0xb6, 0x00, 
        0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e, 
        0xcc, 0x00, 0x14, 0x00, 0x00, 0x00, 0x03, 0xe8, 
        0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x27, 0x10, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
    ];
    let mut decoded = DeleteSessionResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_SESSION_RESP,
            piggyback:false,
            message_prio:None, 
            length:196, 
            teid:Some(0x0215fd34), 
            sqn:0x70 };
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
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:35,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x05, 0x00, 
                    0x00, 0x10, 0x00),
        });
    decoded.apn_rate_control_status = Some (
        ApnRateControlStatus {
            t:APN_RATE_CNTRL,
            length:APN_RATE_CNTR_LENGTH as u16,
            ins:0,
            ul_packets_allowed: 1000,
            nmbr_add_exception_reports: 100,
            dl_packets_allowed: 10000,
            validity_time: 0xffffff,
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
    decoded.load_control = vec! (
        LoadControl { 
            t: LOAD_CNTRL, 
            length: 31, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
            load_metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            list: Some(vec!(
                ApnRelativeCapacity { t:APN_REL_CAP, length: 14, ins:0, relative_cap: 100, name: "test.net.com".to_string() }
            )),
            },
        LoadControl { 
            t: LOAD_CNTRL, 
            length: 31, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
            load_metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            list: Some(vec!(
                ApnRelativeCapacity { t:APN_REL_CAP, length: 14, ins:0, relative_cap: 100, name: "rest.net.com".to_string() }
            )),
            },
    );
    
    let message = DeleteSessionResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_delete_session_resp_marshal () {
    let encoded:[u8;200] = [
        0x48, 0x25, 0x00, 0xc4, 0x02, 0x15, 0xfd, 0x34,
        0x00, 0x00, 0x70, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x08, 0x4e, 
        0x00, 0x23, 0x00, 0x80, 0x80, 0x21, 0x10, 0x01, 
        0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 
        0x00, 0x83, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a, 0x00, 
        0x00, 0x05, 0x00, 0x00, 0x10, 0x00, 0xb5, 0x00, 
        0x1f, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 
        0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0xb8, 
        0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 
        0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 
        0x6d, 0xb5, 0x00, 0x1f, 0x01, 0xb7, 0x00, 0x04, 
        0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 
        0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 
        0x72, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 
        0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x12, 0x00, 
        0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 
        0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 
        0x00, 0x7f, 0xb4, 0x00, 0x12, 0x01, 0xb7, 0x00, 
        0x04, 0x00, 0xff, 0xaa, 0xee, 0x22, 0xb6, 0x00, 
        0x01, 0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7e, 
        0xcc, 0x00, 0x14, 0x00, 0x00, 0x00, 0x03, 0xe8, 
        0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x27, 0x10, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
    ];
    let mut decoded = DeleteSessionResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_SESSION_RESP,
            piggyback:false,
            message_prio:None, 
            length:196, 
            teid:Some(0x0215fd34), 
            sqn:0x70 };
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
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:35,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x03, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x05, 0x00, 
                    0x00, 0x10, 0x00),
        });
    decoded.apn_rate_control_status = Some (
        ApnRateControlStatus {
            t:APN_RATE_CNTRL,
            length:APN_RATE_CNTR_LENGTH as u16,
            ins:0,
            ul_packets_allowed: 1000,
            nmbr_add_exception_reports: 100,
            dl_packets_allowed: 10000,
            validity_time: 0xffffff,
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
    decoded.load_control = vec! (
        LoadControl { 
            t: LOAD_CNTRL, 
            length: 31, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
            load_metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            list: Some(vec!(
                ApnRelativeCapacity { t:APN_REL_CAP, length: 14, ins:0, relative_cap: 100, name: "test.net.com".to_string() }
            )),
            },
        LoadControl { 
            t: LOAD_CNTRL, 
            length: 31, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
            load_metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            list: Some(vec!(
                ApnRelativeCapacity { t:APN_REL_CAP, length: 14, ins:0, relative_cap: 100, name: "rest.net.com".to_string() }
            )),
            },
    );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
