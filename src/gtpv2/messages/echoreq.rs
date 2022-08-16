use crate::gtpv2::header::*;
use crate::gtpv2::messages::commons::*;
use crate::gtpv2::errors::*;
use crate::gtpv2::messages::ies::*;
//use crate::gtpv2::utils::*;


// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const ECHO_REQUEST:u8 = 1;

// Definition of GTPv2-C Echo Request Message

#[derive(Debug, Clone, PartialEq)]
pub struct EchoRequest {
    pub header:Gtpv2Header,
    pub recovery: Recovery,
    pub private_ext:Option<PrivateExtension>,
}

impl Default for EchoRequest {
    fn default() -> EchoRequest {
        let mut hdr = Gtpv1Header::default();
        hdr.msgtype = ECHO_REQUEST;
        EchoRequest {
            header: hdr,
            private_ext: None,
        }
    }
}

impl Messages for EchoRequest {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
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
        let mut message = EchoRequest::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ECHO_REQUEST {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)<buffer.len() {
            match PrivateExtension::unmarshal(&buffer[message.header.get_header_size()..]) {
                Ok(i) => message.private_ext=Some(i),
                Err(_)=> (),
            }
        }
        Ok(message)
    }
}

#[test]
fn test_echo_req_unmarshal () {
    let encoded:[u8;12] = [0x32, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00];
    let decoded:EchoRequest = EchoRequest { header: Gtpv1Header {msgtype:ECHO_REQUEST, length:4, teid:0, sequence_number:Some(18890), npdu_number: None, extension_headers:None}, private_ext: None };
    assert_eq!(EchoRequest::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn test_echo_req_marshal () {
    let encoded:[u8;12] = [0x32, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00];
    let decoded:EchoRequest = EchoRequest { header: Gtpv1Header {msgtype:ECHO_REQUEST, length:4, teid:0, sequence_number:Some(18890), npdu_number: None, extension_headers:None}, private_ext: None };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

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