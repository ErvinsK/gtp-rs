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

// Common traits of GTPU Messages

pub trait Messages {
    fn marshal (self, buffer: &mut Vec<u8>);
    fn unmarshal (header:GtpuHeader, buffer:&[u8]) -> Result<Self, GTPUError> where Self:Sized;
    //fn len (&self) -> usize;
}

// G-PDU message

#[derive(Debug, Clone)]
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

impl Messages for Gpdu {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        buffer.extend(self.tpdu);
        set_length(buffer);
    }

    fn unmarshal (header: GtpuHeader, buffer: &[u8]) -> Result<Gpdu, GTPUError> {
        let mut message = Gpdu::default();
        message.header = header;
        message.tpdu = buffer.to_vec();
        Ok(message)
    }
}

// Echo Request message

#[derive(Debug, Clone)]
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

impl Messages for EchoRequest {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        if let Some(i) = self.private_extension {
            i.marshal(buffer);
        }
        set_length(buffer);
    }

    fn unmarshal (header: GtpuHeader, buffer: &[u8]) -> Result<EchoRequest, GTPUError> {
        let mut message = EchoRequest::default();
        message.header = header;
        message.private_extension = PrivateExtension::unmarshal(buffer);
        Ok(message)
    }
}


// Echo Response message 

#[derive(Debug, Clone)]
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

impl Messages for EchoResponse {

    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.recovery.marshal(buffer);
        if let Some(i) = self.private_extension {
            i.marshal(buffer);
        }
        set_length(buffer);
    }

    fn unmarshal(header: GtpuHeader, buffer: &[u8]) -> Result<EchoResponse, GTPUError> {
        let mut message = EchoResponse::default();
        message.header=header;
        match Recovery::unmarshal(buffer) {
            Some(i) => message.recovery=i,
            None => return Err(GTPUError::MandatoryIEMissing),
        }
        message.private_extension=PrivateExtension::unmarshal(&buffer[message.recovery.len()-1..]);
        Ok(message)
    }
}
// Supported Extension Headers Notification message 

#[derive(Debug, Clone)]
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

impl Messages for SupportedExtensionHeadersNotification {

    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.list.marshal(buffer);
        set_length(buffer);
    }

    fn unmarshal(header: GtpuHeader, buffer: &[u8]) -> Result<SupportedExtensionHeadersNotification, GTPUError> {
        let mut message = SupportedExtensionHeadersNotification::default();
        message.header = header;
        match ExtensionHeaderTypeList::unmarshal(buffer) {
            Some(i) => message.list = i,
            None => return Err(GTPUError::MandatoryIEMissing),
        } 
        Ok(message)
    }
}
// Error Indication message

#[derive(Debug, Clone)]
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

impl Messages for ErrorIndication {

    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.teid.marshal(buffer);
        self.peer.marshal(buffer);
        if let Some(i) = self.private_extension {
            i.marshal(buffer);
        }
        set_length(buffer);
    }

    fn unmarshal(header: GtpuHeader, buffer: &[u8]) -> Result<ErrorIndication, GTPUError> {
        let mut message = ErrorIndication::default();
        let mut cursor = 0;
        message.header = header;
        match Teid::unmarshal(buffer) {
            Some(i) => message.teid=i,
            None => return Err(GTPUError::MandatoryIEMissing),
        }
        cursor += message.teid.len()-1;
        match GTPUPeerAddress::unmarshal(&buffer[cursor..]) {
            Some(i) => message.peer=i,
            None => return Err(GTPUError::MandatoryIEMissing),
        }
        cursor += message.peer.len()-1;
        message.private_extension = PrivateExtension::unmarshal(&buffer[cursor..]);
        Ok(message)
    }
}

// End Marker message

#[derive(Debug, Clone)]
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

impl Messages for EndMarker {

    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        if let Some(i) = self.private_extension {
            i.marshal(buffer);
        }
        set_length(buffer);
    }

    fn unmarshal(header: GtpuHeader, buffer: &[u8]) -> Result<EndMarker,GTPUError> {
        let mut message = EndMarker::default();
        message.header=header;
        message.private_extension=PrivateExtension::unmarshal(buffer);
        Ok(message)
    }
}
// Enum of GTP-U messages 

#[derive(Debug, Clone)]
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

// Marshal GTP-U Message 

    pub fn marshal (self, buffer: &mut Vec<u8>) {

        match self {
            GTPUMessage::Gpdu(i) => i.marshal(buffer),
            GTPUMessage::EchoRequest(i) => i.marshal(buffer),
            GTPUMessage::EchoResponse(i) => i.marshal(buffer),
            GTPUMessage::SupportedExtensionHeadersNotification(i) => i.marshal(buffer),
            GTPUMessage::ErrorIndication(i) => i.marshal(buffer),
            GTPUMessage::EndMarker(i) => i.marshal(buffer),
        }
    }
// Parse GTP-U Message from byte slice

    pub fn unmarshal (buffer: &[u8]) -> Result<GTPUMessage, GTPUError> {
        
        let header:GtpuHeader;

        match GtpuHeader::unmarshal(&buffer[0..]) {
           Ok(x) => header = x,
           Err(i) => return Err(i),
        }
        
        let offset:usize;
        
        if (header.length as usize) <= buffer[7..].len() {
            offset = header.header_offset();
        } else {
            return Err(GTPUError::MessageLengthError);
        }
        
        match header.msgtype {
            G_PDU => {
                match Gpdu::unmarshal(header, &buffer[offset..]) {
                    Ok(i) => Ok(GTPUMessage::Gpdu(i)),
                    Err(i) => Err(i),
                }
            }
            ECHO_REQUEST => {
                if header.sequence_number_flag == true {
                    match EchoRequest::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok (GTPUMessage::EchoRequest(i)),
                        Err(i) => Err(i),
                    } 
                } else {
                   Err(GTPUError::MandatoryHeaderFlagError)
                }                  
            }
            ECHO_RESPONSE => {
                if header.sequence_number_flag == true {
                    match EchoResponse::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok (GTPUMessage::EchoResponse(i)),
                        Err(i) => Err(i),
                    }  
                } else {
                    Err(GTPUError::MandatoryHeaderFlagError)
                }
            }
            ERROR_INDICATION => {
                if header.sequence_number_flag == true {
                    match ErrorIndication::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok (GTPUMessage::ErrorIndication(i)),
                        Err(i) => Err(i),
                    }
                } else {
                    Err(GTPUError::MandatoryHeaderFlagError)
                }
            }
            SUPPORT_EXTENSION_HEADERS_NOTIFICATION => {
                if header.sequence_number_flag == true {
                    match SupportedExtensionHeadersNotification::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok(GTPUMessage::SupportedExtensionHeadersNotification(i)),
                        Err(i) => Err(i),
                    }
                } else {
                    Err(GTPUError::MandatoryHeaderFlagError)
                }
            }
            END_MARKER => {
                if header.sequence_number_flag == false {    
                    match EndMarker::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok(GTPUMessage::EndMarker(i)),
                        Err(i) => Err(i),
                    }
                } else {
                    Err(GTPUError::MandatoryHeaderFlagError)
                }
            }
            _ => Err(GTPUError::MessageNotSupported)      
        }
    }
}

fn set_length (buffer: &mut Vec<u8>) {
        let size = ((buffer.len()-8) as u16).to_be_bytes();
        buffer[2]=size[0];
        buffer[3]=size[1];             
} 
