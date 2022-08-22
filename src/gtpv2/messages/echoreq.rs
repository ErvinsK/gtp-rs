use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

//use crate::gtpv2::utils::*;


// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const ECHO_REQUEST:u8 = 1;

// Definition of GTPv2-C Echo Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct EchoRequest {
    pub header:Gtpv2Header,
    pub recovery: Recovery,
    pub sending_node_features: Option<NodeFeatures>,
    pub private_ext:Option<PrivateExtension>,
}

impl Default for EchoRequest {
    fn default() -> EchoRequest {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = ECHO_REQUEST;
        EchoRequest {
            header: hdr,
            recovery: Recovery::default(),
            sending_node_features:None,
            private_ext: None,
        }
    }
}

impl Messages for EchoRequest {

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
        set_length(buffer);
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
            let mut ies:Vec<InformationElement>=vec!();
            let mut cursor = MIN_HEADER_LENGTH;
            loop {
                if cursor>=(message.header.length as usize)+4 {
                    break;
                }
                match buffer[cursor] {
                    RECOVERY => {
                        match Recovery::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Recovery(i));
                            },
                            Err(_) => return Err(GTPV2Error::MessageInvalidLength(RECOVERY)),
                        }
                    },
                    NODEFEATURES => {
                        match NodeFeatures::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::NodeFeatures(i));
                            }
                            Err(_) => return Err(GTPV2Error::MessageInvalidLength(NODEFEATURES)),
                        }
                    },
                    PRIVATE_EXT => {
                        match PrivateExtension::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PrivateExtension(i));
                            },
                            Err(_) => return Err(GTPV2Error::MessageInvalidLength(PRIVATE_EXT)),
                        }
                    },
                    _ => cursor+=((u16::from_be_bytes([buffer[cursor+1],buffer[cursor+2]]))+4) as usize,
                }
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
fn test_echo_req_unmarshal () {
    let encoded:[u8;19] = [0x40, 0x01, 0x00, 0x0f, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x0c, 0xff, 0x00, 0x03, 0x00, 0x0a, 0xff];
    let decoded:EchoRequest = EchoRequest { 
        header: Gtpv2Header {
            msgtype:ECHO_REQUEST,
            piggyback:false,
            message_prio:None, 
            length:14, 
            teid:None, 
            sqn:0x2dcc38 },
        recovery: Recovery { t: RECOVERY, length: 1, ins: 0, recovery: 12 },
        sending_node_features: None,
        private_ext: None } ;
    println!("{:?}",EchoRequest::unmarshal(&encoded));
    //    assert_eq!(EchoRequest::unmarshal(&encoded).unwrap(),decoded);
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
        private_ext: None } ;
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
/*
#[test]
fn test_echo_req_with_private_ext_unmarshal () {
    let encoded:[u8;20] = [0x32, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    let decoded:EchoRequest = EchoRequest { header: Gtpv1Header {msgtype:ECHO_REQUEST, length:12, teid:0, sequence_number:Some(18890), npdu_number: None, extension_headers:None}, private_ext: Some(PrivateExtension { t:PRIVATE_EXTENSION, length:5, extension_id:8, extension_value: vec![1,2,3]})};
    assert_eq!(EchoRequest::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_req_with_private_ext_marshal () {
    let encoded:[u8;20] = [0x32, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    let decoded:EchoRequest = EchoRequest { header: Gtpv1Header {msgtype:ECHO_REQUEST, length:12, teid:0, sequence_number:Some(18890), npdu_number: None, extension_headers:None}, private_ext: Some(PrivateExtension { t:PRIVATE_EXTENSION, length:5, extension_id:8, extension_value: vec![1,2,3]})};
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn test_echo_req_with_incorrect_private_ext_unmarshal () {
    let encoded:[u8;19] = [0x32, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02];
    let decoded:EchoRequest = EchoRequest { header: Gtpv1Header {msgtype:ECHO_REQUEST, length:12, teid:0, sequence_number:Some(18890), npdu_number: None, extension_headers:None}, private_ext: None};
    assert_eq!(EchoRequest::unmarshal(&encoded).unwrap(),decoded);
}
*/