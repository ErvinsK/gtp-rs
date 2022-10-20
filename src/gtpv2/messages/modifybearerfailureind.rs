use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const MODIFY_BEARER_FAIL_IND:u8 = 65;

// Definition of GTPv2-C Modify Bearer Failure Indication Message

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyBearerFailureInd {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub recovery:Option<Recovery>,
    pub indication:Option<Indication>,
    pub overload_info:Vec<OverloadControlInfo>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for ModifyBearerFailureInd {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = MODIFY_BEARER_FAIL_IND;
        hdr.teid = Some(0);
        ModifyBearerFailureInd {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            indication: None,
            overload_info: vec!(),
            private_ext: vec!(),
        }
    }
}

impl Messages for ModifyBearerFailureInd {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyBearerFailureInd::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_BEARER_FAIL_IND {
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
                Err(j) => return Err(j),
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
        match self.indication.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone()))); 

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:bool=false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::Recovery(j) => {  
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Indication(j) => {  
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<2 => self.overload_info.push(j.clone()),
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
fn test_modify_bearer_failure_unmarshal () {
    let encoded:[u8;78] = [
        0x48,0x41,0x00,0x4a,0x00,0x00,0x00,0x00,
        0x00,0x00,0x68,0x00,0x02,0x00,0x02,0x00,
        0x0e,0x00,0x03,0x00,0x01,0x00,0x04,0x4d,
        0x00,0x07,0x00,0x01,0x00,0x00,0x00,0x00,
        0x00,0x01,0xb4,0x00,0x12,0x00,0xb7,0x00,
        0x04,0x00,0xff,0xaa,0xee,0x11,0xb6,0x00,
        0x01,0x00,0x60,0x9c,0x00,0x01,0x00,0x7f,
        0xb4,0x00,0x12,0x01,0xb7,0x00,0x04,0x00,
        0xff,0xaa,0xee,0x22,0xb6,0x00,0x01,0x00,
        0x60,0x9c,0x00,0x01,0x00,0x7e,
    ];
    let mut decoded = ModifyBearerFailureInd::default();
    decoded.header = Gtpv2Header {
            msgtype:MODIFY_BEARER_FAIL_IND,
            piggyback:false,
            message_prio:None, 
            length:74, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.cause = Cause{
                t:CAUSE,
                length:2,
                ins:0,
                value:14,
                pce:false,
                bce:false,
                cs:false,
                offend_ie_type:None,
    };
    let mut i = Indication::default();
    i.tspcmi = true;
    i.sgwci = true;
    decoded.indication = Some (i);
    decoded.recovery = Some (
        Recovery{ t: RECOVERY, length: RECOVERY_LENGTH as u16, ins:0, recovery:4 }
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
    let message = ModifyBearerFailureInd::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_modify_bearer_failure_marshal () {
    let encoded:[u8;78] = [
        0x48,0x41,0x00,0x4a,0x00,0x00,0x00,0x00,
        0x00,0x00,0x68,0x00,0x02,0x00,0x02,0x00,
        0x0e,0x00,0x03,0x00,0x01,0x00,0x04,0x4d,
        0x00,0x07,0x00,0x01,0x00,0x00,0x00,0x00,
        0x00,0x01,0xb4,0x00,0x12,0x00,0xb7,0x00,
        0x04,0x00,0xff,0xaa,0xee,0x11,0xb6,0x00,
        0x01,0x00,0x60,0x9c,0x00,0x01,0x00,0x7f,
        0xb4,0x00,0x12,0x01,0xb7,0x00,0x04,0x00,
        0xff,0xaa,0xee,0x22,0xb6,0x00,0x01,0x00,
        0x60,0x9c,0x00,0x01,0x00,0x7e,
    ];
    let mut decoded = ModifyBearerFailureInd::default();
    decoded.header = Gtpv2Header {
            msgtype:MODIFY_BEARER_FAIL_IND,
            piggyback:false,
            message_prio:None, 
            length:74, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.cause = Cause{
                t:CAUSE,
                length:2,
                ins:0,
                value:14,
                pce:false,
                bce:false,
                cs:false,
                offend_ie_type:None,
    };
    let mut i = Indication::default();
    i.tspcmi = true;
    i.sgwci = true;
    decoded.indication = Some (i);
    decoded.recovery = Some (
        Recovery{ t: RECOVERY, length: RECOVERY_LENGTH as u16, ins:0, recovery:4 }
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
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer,encoded);
}
