use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const MODIFY_BEARER_CMD:u8 = 64;

// Definition of GTPv2-C Modify Bearer Command Message

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyBearerCommand {
    pub header:Gtpv2Header,
    pub apnambr: ApnAmbr,
    pub bearer_ctxs: Vec<BearerContext>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub fteid_control:Option<Fteid>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for ModifyBearerCommand {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = MODIFY_BEARER_CMD;
        hdr.teid = Some(0);
        ModifyBearerCommand {
            header: hdr,
            apnambr: ApnAmbr::default(),
            bearer_ctxs: vec!(),
            overload_info: vec!(),
            fteid_control: None,
            private_ext: vec!(),
        }
    }
}

impl Messages for ModifyBearerCommand {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyBearerCommand::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_BEARER_CMD {
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
        
        elements.push(self.apnambr.clone().into());

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        match self.fteid_control.clone() {
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
                InformationElement::ApnAmbr(j) => {
                    match (j.ins, mandatory[0]) {
                        (0, false) => (self.apnambr, mandatory[0]) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::BearerContext(j) => {  
                    match (j.ins, mandatory[1]) {
                        (0,false) => {
                            self.bearer_ctxs.push(j.clone());
                            mandatory[1] = true;
                        },
                        _ => (),
                    }
                }, 
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<3 => self.overload_info.push(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Fteid(j) => {  
                    match (j.ins, self.fteid_control.is_none()) {
                        (0, true) => self.fteid_control = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1]) {
            (false,false) => Err(GTPV2Error::MessageMandatoryIEMissing(APNAMBR)),
            (false,true) => Err(GTPV2Error::MessageMandatoryIEMissing(APNAMBR)),
            (true,false) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)), 
            (true,true) => Ok(true),
        }
    }   
}

#[test]
fn test_modify_bearer_command_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;116] = [
        0x48,0x40,0x00,0x70,0x00,0x00,0x00,0x00,
        0x00,0x00,0x68,0x00,0x48,0x00,0x08,0x00,
        0x00,0x00,0xc3,0x50,0x00,0x02,0x49,0xf0,
        0x5d,0x00,0x1f,0x00,0x49,0x00,0x01,0x00,
        0x05,0x50,0x00,0x16,0x00,0x64,0x09,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0xb4,0x00,0x12,0x00,0xb7,
        0x00,0x04,0x00,0xff,0xaa,0xee,0x11,0xb6,
        0x00,0x01,0x00,0x60,0x9c,0x00,0x01,0x00,
        0x7f,0xb4,0x00,0x12,0x01,0xb7,0x00,0x04,
        0x00,0xff,0xaa,0xee,0x22,0xb6,0x00,0x01,
        0x00,0x60,0x9c,0x00,0x01,0x00,0x7e,0x57,
        0x00,0x09,0x00,0x86,0x06,0xd1,0x82,0x4c,
        0xc1,0xfe,0x8b,0x2d,
    ];
    let mut decoded = ModifyBearerCommand::default();
    decoded.header = Gtpv2Header {
            msgtype:MODIFY_BEARER_CMD,
            piggyback:false,
            message_prio:None, 
            length:112, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.apnambr = ApnAmbr {
        t:APNAMBR,
        length:APNAMBR_LENGTH as u16,
        ins: 0,
        ambr_ul:  50000,
        ambr_dl: 150000,
    };
    let mut bearer = BearerContext::default();
    bearer.ebi = Ebi {
        t: EBI,
        length: EBI_LENGTH as u16,
        ins: 0,
        value: 5,
    };
    bearer.bearer_qos = Some(
        BearerQos { t: BEARERQOS, length: BEARERQOS_LENGTH as u16, ins: 0, pre_emption_vulnerability: 0, priority_level: 9, pre_emption_capability: 1, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }
    );
    bearer.length = 31;
    decoded.bearer_ctxs = vec!(bearer);
    decoded.fteid_control = Some(
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193,254,139,45)),
            ipv6:None
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
    let message = ModifyBearerCommand::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_modify_bearer_command_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;116] = [
        0x48,0x40,0x00,0x70,0x00,0x00,0x00,0x00,
        0x00,0x00,0x68,0x00,0x48,0x00,0x08,0x00,
        0x00,0x00,0xc3,0x50,0x00,0x02,0x49,0xf0,
        0x5d,0x00,0x1f,0x00,0x49,0x00,0x01,0x00,
        0x05,0x50,0x00,0x16,0x00,0x64,0x09,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0xb4,0x00,0x12,0x00,0xb7,
        0x00,0x04,0x00,0xff,0xaa,0xee,0x11,0xb6,
        0x00,0x01,0x00,0x60,0x9c,0x00,0x01,0x00,
        0x7f,0xb4,0x00,0x12,0x01,0xb7,0x00,0x04,
        0x00,0xff,0xaa,0xee,0x22,0xb6,0x00,0x01,
        0x00,0x60,0x9c,0x00,0x01,0x00,0x7e,0x57,
        0x00,0x09,0x00,0x86,0x06,0xd1,0x82,0x4c,
        0xc1,0xfe,0x8b,0x2d,
    ];
    let mut decoded = ModifyBearerCommand::default();
    decoded.header = Gtpv2Header {
            msgtype:MODIFY_BEARER_CMD,
            piggyback:false,
            message_prio:None, 
            length:112, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.apnambr = ApnAmbr {
        t:APNAMBR,
        length:APNAMBR_LENGTH as u16,
        ins: 0,
        ambr_ul:  50000,
        ambr_dl: 150000,
    };
    let mut bearer = BearerContext::default();
    bearer.ebi = Ebi {
        t: EBI,
        length: EBI_LENGTH as u16,
        ins: 0,
        value: 5,
    };
    bearer.bearer_qos = Some(
        BearerQos { t: BEARERQOS, length: BEARERQOS_LENGTH as u16, ins: 0, pre_emption_vulnerability: 0, priority_level: 9, pre_emption_capability: 1, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }
    );
    decoded.bearer_ctxs = vec!(bearer);
    decoded.fteid_control = Some(
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193,254,139,45)),
            ipv6:None
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
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer,encoded);
}
