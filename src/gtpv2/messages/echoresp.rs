use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const ECHO_RESPONSE:u8 = 2;

// Definition of GTPv2-C Echo Response Message

#[derive(Debug, Clone, PartialEq)]
pub struct EchoResponse {
    pub header:Gtpv2Header,
    pub recovery: Recovery,
    pub sending_node_features: Option<NodeFeatures>,
    pub private_ext:Option<PrivateExtension>,
}

impl Default for EchoResponse {
    fn default() -> EchoResponse {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = ECHO_RESPONSE;
        EchoResponse {
            header: hdr,
            recovery: Recovery::default(),
            sending_node_features:None,
            private_ext: None,
        }
    }
}

impl Messages for EchoResponse {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.recovery.marshal(buffer);
        match self.sending_node_features {
            Some(i) => i.marshal(buffer),
            None => (),
        }
        match self.private_ext {
            Some(i) => i.marshal(buffer),
            None => (),
        }
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = EchoResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ECHO_RESPONSE {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            let ies:Vec<InformationElement>;
            match InformationElement::decoder(&buffer[8..]) {
                Ok(i) => ies = i,
                Err(j) => return Err(j),
            }
            let mut flag = false;
            for i in ies.iter() {
                match i {
                    InformationElement::Recovery(j) => {
                        if j.ins == 0 {
                            message.recovery = j.clone();
                            flag = true;
                        }
                    },
                    InformationElement::NodeFeatures(j) => {
                        if j.ins == 0 {
                            message.sending_node_features = Some(j.clone());
                        }
                    },
                    InformationElement::PrivateExtension(j) => {
                        if j.ins == 0 {
                            message.private_ext = Some(j.clone());
                        }
                    }
                    _ => (),
                }
            }
            if flag {
                Ok(message)
            } else {
                Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY))
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }
}

#[test]
fn test_echo_resp_unmarshal () {
    let encoded:[u8;20] = [0x40, 0x02, 0x00, 0x0f, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x21, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff];
    let decoded = EchoResponse { 
        header: Gtpv2Header {
            msgtype:ECHO_RESPONSE,
            piggyback:false,
            message_prio:None, 
            length:15, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 33 },
        sending_node_features: None,
        private_ext: Some(PrivateExtension { t: PRIVATE_EXT, length:3, ins: 0, enterprise_id: 0x0a, value: vec!(0xff) }) } ;
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_resp_no_mandatory_ie_unmarshal () {
    let encoded:[u8;15] = [0x40, 0x02, 0x00, 0x0b, 0x2d, 0xcc, 0x38, 0x00, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff];
    assert_eq!(EchoResponse::unmarshal(&encoded),Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY)));
}

#[test]
fn test_echo_resp_marshal () {
    let encoded:[u8;13] = [0x40, 0x02, 0x00, 0x09, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x21];
    let decoded = EchoResponse { 
        header: Gtpv2Header {
            msgtype:ECHO_RESPONSE,
            piggyback:false,
            message_prio:None, 
            length:9, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 33 },
        sending_node_features: None,
        private_ext: None } ;
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
