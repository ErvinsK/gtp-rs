use crate::header::{*};
use crate::errors::{*};
use crate::ies::{*};

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

// Definition of GTP-U Messages

pub const ECHO_REQUEST:u8 = 1;
pub const ECHO_RESPONSE:u8 = 2;
pub const ERROR_INDICATION:u8 = 26;
pub const SUPPORT_EXTENSION_HEADERS_NOTIFICATION:u8 = 31;
pub const END_MARKER:u8 = 254;
pub const G_PDU:u8 = 255;

// G-PDU message

#[derive(Debug)]
pub struct Gpdu {
    pub header:GtpuHeader,
    pub tpdu:Vec<u8>,
}

impl Default for Gpdu {
    fn default() -> Gpdu {
        Gpdu {
            header: crate::header::GtpuHeader::new(),
            tpdu:vec!(),
        }
    }
}

// Echo Request message

#[derive(Debug)]
pub struct EchoRequest {
    pub header:GtpuHeader,
    pub private_extension:Option<PrivateExtension>,
}

impl Default for EchoRequest {
    fn default() -> EchoRequest {
        EchoRequest {
            header: crate::header::GtpuHeader::new(),
            private_extension: None,
        }
    }
}

// Echo Response message 

#[derive(Debug)]
pub struct EchoResponse {
    pub header:GtpuHeader,
    pub recovery:Recovery,
    pub private_extension:Option<PrivateExtension>,
}

impl Default for EchoResponse {
    fn default() -> EchoResponse {
        EchoResponse {
            header: crate::header::GtpuHeader::new(),
            recovery: crate::ies::Recovery::default(),
            private_extension: None,
        }
    }
}
// Supported Extension Headers Notification message 

#[derive(Debug)]
pub struct SupportedExtensionHeadersNotification {
    pub header:GtpuHeader,
    pub list:ExtensionHeaderTypeList,
}

impl Default for SupportedExtensionHeadersNotification {
    fn default() -> SupportedExtensionHeadersNotification {
        SupportedExtensionHeadersNotification {
            header: crate::header::GtpuHeader::new(),
            list: crate::ies::ExtensionHeaderTypeList::default(),
        }
    }
}
// Error Indication message

#[derive(Debug)]
pub struct ErrorIndication {
    pub header:GtpuHeader,
    pub teid:Teid,
    pub peer:GTPUPeerAddress,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for ErrorIndication {
    fn default() -> ErrorIndication {
        ErrorIndication {
            header: crate::header::GtpuHeader::new(),
            teid: crate::ies::Teid::default(),
            peer: crate::ies::GTPUPeerAddress::default(),
            private_extension: None,
        }
    }
}

// End Marker message

#[derive(Debug)]
pub struct EndMarker {
    pub header:GtpuHeader,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for EndMarker {
    fn default() -> EndMarker {
        EndMarker {
            header: crate::header::GtpuHeader::new(),
            private_extension: None,
        }
    }
}
// Enum of GTP-U messages 

#[derive(Debug)]
pub enum GTPUMessage {
    Gpdu(Gpdu),
    EchoRequest(EchoRequest),
    EchoResponse(EchoResponse),
    SupportedExtensionHeadersNotification(SupportedExtensionHeadersNotification),
    ErrorIndication(ErrorIndication),
    EndMarker(EndMarker),
}

// Implementation of GTP-U Messages

impl GTPUMessage {

// Parse GTP-U Message from byte slice

    pub fn unmarshal (buffer: &[u8]) -> Result<GTPUMessage, GTPUError> {
        
        let header:GtpuHeader;

        match GtpuHeader::unmarshal(&buffer[0..]) {
           Ok(x) => header = x,
           Err(i) => return Err(i),
        }
        
        let mut offset:usize;

        match (header.sequence_number_flag, header.npdu_number_flag, header.extension_header_flag) {
            (false, false,  false) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = MIN_HEADER_LENGTH;
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
                }
            (true,  false,  false) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = MIN_HEADER_LENGTH+SQN_LENGTH+header.extension_headers_length();
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
            }
            (true,  true,   false) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = 12;
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
            }
            (true,  true,   true) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = 10+header.extension_headers_length();
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
            }
            (true,  false,  true) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = 9+header.extension_headers_length();
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
            }
            (false, true,   true) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = 8+header.extension_headers_length();
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
            }
            (false, false,  true) => {
                    if (header.length as usize) <= buffer[7..].len() {
                        offset = 7+header.extension_headers_length();
                    } else {
                        return Err(GTPUError::MessageLengthError);
                    }
            }
            (false, true,   false) => {
                if (header.length as usize) <= buffer[7..].len() {
                    offset = 9;
                } else {
                    return Err(GTPUError::MessageLengthError);
                }
            }
        }

        match header.msgtype {
            G_PDU => {
                let mut message = Gpdu::default();
                message.header = header;
                message.tpdu = buffer[offset..].to_vec();
                Ok(GTPUMessage::Gpdu(message))
            }
            ECHO_REQUEST => {
                let mut message = EchoRequest::default();
                message.header = header;
                message.private_extension = crate::ies::PrivateExtension::unmarshal(&buffer[offset..]);
                Ok(GTPUMessage::EchoRequest(message))    
            }
            ECHO_RESPONSE => {
                if let Some(recovery) = crate::ies::Recovery::unmarshal(&buffer[offset+1..]) {
                    let mut message = EchoResponse::default();
                    message.header = header;
                    message.recovery = recovery;
                    offset+=crate::ies::RECOVERY_LENGTH;
                    message.private_extension = crate::ies::PrivateExtension::unmarshal(&buffer[offset+1..]); 
                    Ok(GTPUMessage::EchoResponse(message))
                } else {
                    Err(GTPUError::MandatoryIEMissing)
                }
            }
            ERROR_INDICATION => {
                let mut message = ErrorIndication::default();
                message.header = header;
                if let Some(teid) = crate::ies::Teid::unmarshal(&buffer[offset+1..]) {
                    message.teid = teid;
                } else {
                    return Err(GTPUError::MandatoryIEMissing);
                }
                offset+=crate::ies::TEID_LENGTH;
                if let Some(peer) = crate::ies::GTPUPeerAddress::unmarshal(&buffer[offset+1..]) {
                    message.peer = peer;
                } else {
                    return Err(GTPUError::MandatoryIEMissing);
                }
                offset+=message.peer.length as usize;
                message.private_extension = crate::ies::PrivateExtension::unmarshal(&buffer[offset+1..]);
                Ok(GTPUMessage::ErrorIndication(message))
            }
            SUPPORT_EXTENSION_HEADERS_NOTIFICATION => {
                if let Some(list) = crate::ies::ExtensionHeaderTypeList::unmarshal(&buffer[offset+1..]) {
                    let mut message = SupportedExtensionHeadersNotification::default();
                    message.header = header;
                    message.list = list;
                    Ok(GTPUMessage::SupportedExtensionHeadersNotification(message))
                } else {
                    Err(GTPUError::MandatoryIEMissing)
                }
            }
            END_MARKER => {
                let mut message = EndMarker::default();
                message.header = header;
                message.private_extension = crate::ies::PrivateExtension::unmarshal(&buffer[offset+1..]);
                Ok(GTPUMessage::EndMarker(message))
            }
            _ => Err(GTPUError::MessageNotSupported)      
        }
    }
}
