use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const ECHO_REQUEST:u8 = 1;

// Definition of GTPv2-C Echo Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct EchoRequest {
    pub header:Gtpv2Header,
    pub recovery: Recovery,
    pub sending_node_features: Option<NodeFeatures>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for EchoRequest {
    fn default() -> EchoRequest {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = ECHO_REQUEST;
        EchoRequest {
            header: hdr,
            recovery: Recovery::default(),
            sending_node_features:None,
            private_ext: vec!(),
        }
    }
}

impl Messages for EchoRequest {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = EchoRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ECHO_REQUEST {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            let ies:Vec<InformationElement>;
            match InformationElement::decoder(&buffer[8..]) {
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
        
        elements.push(InformationElement::Recovery(self.recovery.clone()));
        
        match self.sending_node_features.clone() {
            Some(i) => elements.push(InformationElement::NodeFeatures(i)),
            None => (),
        }
    
        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));    
        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:bool=false;
        for e in elements.iter() {
            match e {
                InformationElement::Recovery(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => {
                            mandatory=true;
                            self.recovery = j.clone();
                        },
                        _ => (),
                    }
                },
                InformationElement::NodeFeatures(j) => {
                    match (j.ins, self.sending_node_features.is_none()) {
                        (0, true) => self.sending_node_features = Some(j.clone()),
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
            Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY))
        }
    }
}


#[test]
fn test_echo_req_unmarshal () {
    let encoded:[u8;20] = [0x40, 0x01, 0x00, 0x0f, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x0c, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff];
    let decoded:EchoRequest = EchoRequest { 
        header: Gtpv2Header {
            msgtype:ECHO_REQUEST,
            piggyback:false,
            message_prio:None, 
            length:15, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 12 },
        sending_node_features: None,
        private_ext: vec!(PrivateExtension { t: PRIVATE_EXT, length:3, ins: 0, enterprise_id: 0x0a, value: vec!(0xff) }) } ;
    assert_eq!(EchoRequest::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_req_no_mandatory_ie_unmarshal () {
    let encoded:[u8;15] = [0x40, 0x01, 0x00, 0x0b, 0x2d, 0xcc, 0x38, 0x00, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff];
    assert_eq!(EchoRequest::unmarshal(&encoded),Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY)));
}

#[test]
fn test_echo_req_marshal () {
    let encoded:[u8;13] = [0x40, 0x01, 0x00, 0x09, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x0c];
    let decoded:EchoRequest = EchoRequest { 
        header: Gtpv2Header {
            msgtype:ECHO_REQUEST,
            piggyback:false,
            message_prio:None, 
            length:9, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 12 },
        sending_node_features: None,
        private_ext: vec!() } ;
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
