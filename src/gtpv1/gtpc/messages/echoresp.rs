use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;


// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const ECHO_RESPONSE:u8 = 2;

// Definition of GTPv1-C Echo Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoResponse {
    pub header:Gtpv1Header,
    pub recovery:Recovery,
    pub private_ext:Option<PrivateExtension>,
}

impl Default for EchoResponse {
    fn default() -> EchoResponse {
        let mut hdr = Gtpv1Header::default();
        hdr.msgtype = ECHO_RESPONSE;
        EchoResponse {
            header: hdr,
            recovery: Recovery::default(),
            private_ext: None,
        }
    }
}

impl Messages for EchoResponse {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        {
            let mut buffer_ie:Vec<u8>=vec!();
            self.recovery.marshal(&mut buffer_ie);
            buffer.append(&mut buffer_ie);
        }
        match self.private_ext {
            Some(i) => {
                let mut buffer_ie:Vec<u8> = vec!();
                i.marshal(&mut buffer_ie);
                buffer.append(&mut buffer_ie);
            },
            None => (),
        }
        set_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut message = EchoResponse::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ECHO_RESPONSE {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if message.header.length as usize <= buffer.len() {
            let mut cursor = message.header.get_header_size();
            match Recovery::unmarshal(&buffer[cursor..]) {
                Ok(i) => message.recovery=i,
                Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
            }
            cursor+=message.recovery.len();
            match PrivateExtension::unmarshal(&buffer[cursor..]) {
                Ok(i) => message.private_ext=Some(i),
                Err(_)=> (),
            }
            Ok(message)    
        } else {
            Err(GTPV1Error::MessageLengthError)
        }    
    }       
}


#[test]
fn test_echo_resp_unmarshal () {
    let encoded:[u8;14] = [0x32, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0e, 0x26];
    let decoded:EchoResponse = EchoResponse { 
        header: Gtpv1Header {
            msgtype:ECHO_RESPONSE,
            length:6,
            teid:0,
            sequence_number:Some(63051),
            npdu_number: None,
            extension_headers:None},
            recovery: Recovery {
                t:RECOVERY,
                value:38},
            private_ext: None };
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_resp_marshal () {
    let encoded:[u8;14] = [0x32, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0e, 0x26];
    let decoded:EchoResponse = EchoResponse { 
        header: Gtpv1Header {
            msgtype:ECHO_RESPONSE,
            length:6,
            teid:0,
            sequence_number:Some(63051),
            npdu_number: None,
            extension_headers:None},
            recovery: Recovery {
                t:RECOVERY,
                value:38},
            private_ext: None };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn test_echo_resp_without_mandatory_ie_unmarshal () {
    let encoded:[u8;12] = [0x32, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00];
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap_err(), GTPV1Error::MessageMandatoryIEMissing);
}

#[test]
fn test_echo_resp_with_incorrect_mandatory_ie_unmarshal () {
    let encoded:[u8;14] = [0x32, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0f, 0x26];
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap_err(), GTPV1Error::MessageMandatoryIEMissing);
}

#[test]
fn test_echo_resp_with_private_ext_unmarshal () {
    let encoded:[u8;22] = [0x32, 0x02, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0e, 0x26, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    let decoded:EchoResponse = EchoResponse {
         header: Gtpv1Header {
            msgtype:ECHO_RESPONSE,
            length:14,
            teid:0,
            sequence_number:Some(63051),
            npdu_number: None,
            extension_headers:None},
            recovery: Recovery {
                t:RECOVERY,
                value:38 },
            private_ext: Some(PrivateExtension {
                t:PRIVATE_EXTENSION,
                length:5,
                extension_id:8,
                extension_value: vec![1,2,3]})};
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_resp_with_private_ext_marshal () {
    let encoded:[u8;22] = [0x32, 0x02, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0e, 0x26, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    let decoded:EchoResponse = EchoResponse {
        header: Gtpv1Header {
           msgtype:ECHO_RESPONSE,
           length:14,
           teid:0,
           sequence_number:Some(63051),
           npdu_number: None,
           extension_headers:None},
           recovery: Recovery {
               t:RECOVERY,
               value:38 },
           private_ext: Some(PrivateExtension {
               t:PRIVATE_EXTENSION,
               length:5,
               extension_id:8,
               extension_value: vec![1,2,3]})};
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn test_echo_resp_with_with_incorrect_mandatory_ie_and_private_ext_unmarshal () {
    let encoded:[u8;22] = [0x32, 0x02, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0f, 0x26, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap_err(), GTPV1Error::MessageMandatoryIEMissing);
}

#[test]
fn test_echo_resp_with_with_missing_mandatory_ie_and_private_ext_unmarshal () {
    let encoded:[u8;20] = [0x32, 0x02, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02, 0x03];
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap_err(), GTPV1Error::MessageMandatoryIEMissing);
}

#[test]
fn test_echo_resp_with_incorrect_private_ext_unmarshal () {
    let encoded:[u8;21] = [0x32, 0x02, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4b, 0x00, 0x00, 0x0e, 0x26, 0xff, 0x00, 0x05, 0x00, 0x08, 0x01, 0x02];
    let decoded:EchoResponse = EchoResponse {
         header: Gtpv1Header {
            msgtype:ECHO_RESPONSE,
            length:14,
            teid:0,
            sequence_number:Some(63051),
            npdu_number: None,
            extension_headers:None},
            recovery: Recovery {
                t:RECOVERY,
                value:38 },
            private_ext: None};
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap(),decoded);
}