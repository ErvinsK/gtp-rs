use crate::gtpv1::header::*;
use crate::gtpv1::errors::*;
use crate::gtpv1::messages::*;
use crate::gtpv1::gtpu::ies::*;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

// Definition of GTP-U Messages

pub const ECHO_REQUEST:u8 = 1;
pub const ECHO_RESPONSE:u8 = 2;
pub const ERROR_INDICATION:u8 = 26;
pub const SUPPORT_EXTENSION_HEADERS_NOTIFICATION:u8 = 31;
pub const END_MARKER:u8 = 254;
pub const G_PDU:u8 = 255;

// G-PDU message

#[derive(Debug, Clone)]
pub struct Gpdu {
    pub header:GtpuHeader,
    pub tpdu:Vec<u8>,
}

impl Default for Gpdu {
    fn default() -> Gpdu {
        Gpdu {
            header: GtpuHeader::new(),
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

    fn unmarshal (header: GtpuHeader, buffer: &[u8]) -> Result<Gpdu, GTPV1Error> {
        let mut message = Gpdu::default();
        message.header = header;
        message.tpdu = buffer.to_vec();
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
            header: GtpuHeader::new(),
            teid: Teid::default(),
            peer: GTPUPeerAddress::default(),
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

    fn unmarshal(header: GtpuHeader, buffer: &[u8]) -> Result<ErrorIndication, GTPV1Error> {
        let mut message = ErrorIndication::default();
        let mut cursor = 0;
        message.header = header;
        match Teid::unmarshal(buffer) {
            Some(i) => message.teid=i,
            None => return Err(GTPV1Error::MandatoryIEMissing),
        }
        cursor += message.teid.len()-1;
        match GTPUPeerAddress::unmarshal(&buffer[cursor..]) {
            Some(i) => message.peer=i,
            None => return Err(GTPV1Error::MandatoryIEMissing),
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
            header: GtpuHeader::new(),
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

    fn unmarshal(header: GtpuHeader, buffer: &[u8]) -> Result<EndMarker,GTPV1Error> {
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

    pub fn unmarshal (buffer: &[u8]) -> Result<GTPUMessage, GTPV1Error> {
        
        let header:GtpuHeader;

        match GtpuHeader::unmarshal(&buffer[0..]) {
           Ok(x) => header = x,
           Err(i) => return Err(i),
        }
        
        let offset:usize;
        
        if (header.length as usize) <= buffer[7..].len() {
            offset = header.header_offset();
        } else {
            return Err(GTPV1Error::MessageLengthError);
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
                   Err(GTPV1Error::MandatoryHeaderFlagError)
                }                  
            }
            ECHO_RESPONSE => {
                if header.sequence_number_flag == true {
                    match EchoResponse::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok (GTPUMessage::EchoResponse(i)),
                        Err(i) => Err(i),
                    }  
                } else {
                    Err(GTPV1Error::MandatoryHeaderFlagError)
                }
            }
            ERROR_INDICATION => {
                if header.sequence_number_flag == true {
                    match ErrorIndication::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok (GTPUMessage::ErrorIndication(i)),
                        Err(i) => Err(i),
                    }
                } else {
                    Err(GTPV1Error::MandatoryHeaderFlagError)
                }
            }
            SUPPORT_EXTENSION_HEADERS_NOTIFICATION => {
                if header.sequence_number_flag == true {
                    match SupportedExtensionHeadersNotification::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok(GTPUMessage::SupportedExtensionHeadersNotification(i)),
                        Err(i) => Err(i),
                    }
                } else {
                    Err(GTPV1Error::MandatoryHeaderFlagError)
                }
            }
            END_MARKER => {
                if header.sequence_number_flag == false {    
                    match EndMarker::unmarshal(header, &buffer[offset..]) {
                        Ok(i) => Ok(GTPUMessage::EndMarker(i)),
                        Err(i) => Err(i),
                    }
                } else {
                    Err(GTPV1Error::MandatoryHeaderFlagError)
                }
            }
            _ => Err(GTPV1Error::MessageNotSupported)      
        }
    }
}

