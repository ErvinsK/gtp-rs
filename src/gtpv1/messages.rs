use crate::gtpv1::header::*;
use crate::gtpv1::errors::*;
use crate::gtpv1::utils::*;
use crate::gtpv1::gtpu::ies::*;

// According to 3GPP TS 29.060 V15.5.0 (2019-06) and 3GPP TS 29.281 V16.0.0 (2019-12)

// Definition of GTPv1 Messages

pub const ECHO_REQUEST:u8 = 1;
pub const ECHO_RESPONSE:u8 = 2;
pub const VERSION_NOT_SUPPORTED:u8 = 3;
pub const CREATE_PDP_CONTEXT_REQUEST:u8 = 16;
pub const CREATE_PDP_CONTEXT_RESPONSE:u8 = 17;
pub const UPDATE_PDP_CONTEXT_REQUEST:u8 = 18;
pub const UPDATE_PDP_CONTEXT_RESPONSE:u8 = 19;
pub const DELETE_PDP_CONTEXT_REQUEST:u8 = 20;
pub const DELETE_PDP_CONTEXT_RESPONSE:u8 = 21;
pub const ERROR_INDICATION:u8 = 26;
pub const SUPPORT_EXTENSION_HEADERS_NOTIFICATION:u8 = 31;
pub const END_MARKER:u8 = 254;
pub const G_PDU:u8 = 255;

// Common traits of GTPv1 Messages

pub trait Messages {
    fn marshal (self, buffer: &mut Vec<u8>);
    fn unmarshal (header:Gtpv1Header, buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized;
    //fn len (&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct EchoRequest {
    pub header:Gtpv1Header,
    pub private_extension:Option<PrivateExtension>,
}

impl Default for EchoRequest {
    fn default() -> EchoRequest {
        EchoRequest {
            header: Gtpv1Header::new(),
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

    fn unmarshal (header: Gtpv1Header, buffer: &[u8]) -> Result<EchoRequest, GTPV1Error> {
        let mut message = EchoRequest::default();
        message.header = header;
        message.private_extension = PrivateExtension::unmarshal(buffer);
        Ok(message)
    }
}


// Echo Response message 

#[derive(Debug, Clone)]
pub struct EchoResponse {
    pub header:Gtpv1Header,
    pub recovery:Recovery,
    pub private_extension:Option<PrivateExtension>,
}

impl Default for EchoResponse {
    fn default() -> EchoResponse {
        EchoResponse {
            header: Gtpv1Header::new(),
            recovery: Recovery::default(),
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

    fn unmarshal(header: Gtpv1Header, buffer: &[u8]) -> Result<EchoResponse, GTPV1Error> {
        let mut message = EchoResponse::default();
        message.header=header;
        match Recovery::unmarshal(buffer) {
            Some(i) => message.recovery=i,
            None => return Err(GTPV1Error::MandatoryIEMissing),
        }
        message.private_extension=PrivateExtension::unmarshal(&buffer[message.recovery.len()-1..]);
        Ok(message)
    }
}
// Supported Extension Headers Notification message 

#[derive(Debug, Clone)]
pub struct SupportedExtensionHeadersNotification {
    pub header:Gtpv1Header,
    pub list:ExtensionHeaderTypeList,
}

impl Default for SupportedExtensionHeadersNotification {
    fn default() -> SupportedExtensionHeadersNotification {
        SupportedExtensionHeadersNotification {
            header: Gtpv1Header::new(),
            list: ExtensionHeaderTypeList::default(),
        }
    }
}

impl Messages for SupportedExtensionHeadersNotification {

    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.list.marshal(buffer);
        set_length(buffer);
    }

    fn unmarshal(header: Gtpv1Header, buffer: &[u8]) -> Result<SupportedExtensionHeadersNotification, GTPV1Error> {
        let mut message = SupportedExtensionHeadersNotification::default();
        message.header = header;
        match ExtensionHeaderTypeList::unmarshal(buffer) {
            Some(i) => message.list = i,
            None => return Err(GTPV1Error::MandatoryIEMissing),
        } 
        Ok(message)
    }
}
