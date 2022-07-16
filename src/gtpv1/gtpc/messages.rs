use crate::gtpv1::header::*;
use crate::gtpv1::errors::*;
use crate::gtpv1::messages::*;
use crate::gtpv1::utils::*;
use crate::gtpv1::gtpc::ies::*;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

// Definition of GTPv1-C Messages

// Version Not Supported message 

#[derive(Debug, Clone)]
pub struct VersionNotSupported {
    pub header:Gtpv1Header,
}

impl Default for VersionNotSupported {
    fn default() -> VersionNotSupported {
        VersionNotSupported {
            header: Gtpv1Header::new(),
        }
    }
}

impl Messages for VersionNotSupported {

    fn marshal (self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        set_length(buffer);
    }

    fn unmarshal (header: Gtpv1Header, buffer: &[u8]) -> Result<VersionNotSupported, GTPV1Error> {
        let mut message = VersionNotSupported::default();
        message.header = header;
        Ok(message)
    }
}

// Create PDP Context Request message

#[derive(Debug, Clone)]
pub struct CreatePDPContextRequest {
    pub header:Gtpv1Header,
    pub imsi:Option<Imsi>,
    pub rai:Option<Rai>,
    pub recovery:Option<Recovery>,
    pub selectionmode:Option<SelectionMode>,
    pub teid_data:Teid,
    pub teid_control:Option<Teid>,
    pub nsapi:Nsapi,
    pub linked_nsapi:Option<Nsapi>,
    pub charging_char:Option<ChargingCharacteristics>,
    pub trace_ref:Option<TraceReference>,
    pub trace_type:Option<TraceType>,
    pub end_user_address:Option<EndUserAddress>,
    pub apn:Option<Apn>,
    pub pco:Option<Pco>,
    pub sgsn_ip_control:GsnAddress,
    pub sgsn_ip_user:GsnAddress,
//    pub msisdn:Option<Msisdn>,
//    pub qos:Qos,
//    pub tft:Option<Tft>,
//    pub trigger_id:Option<TriggerId>,
//    pub omc_id:Option<OmcId>,
//    pub common_flags:Option<CommonFlags>,
//    pub apn_restrictions:Option<ApnRestrictions>,
//    pub rat_type:Option<RatType>,
//    pub uli:Option<Uli>,
//    pub ms_timezone:Option<MsTimeZone>,
//    pub imei_sv:Option<ImeiSV>,
//    pub camel_cic:Option<CamelCic>,
//    pub add_trace_info:Option<AdditionalTraceInfo>,
//    pub correlation_id:Option<CorrelationId>,
//    pub evolved_alloc:Option<EvolvedAllocationRetention>,
//    pub ext_common_flags:Option<ExtendedCommonFlags>,
//    pub user_csg_info:Option<UserCsgInfo>,
//    pub apn_ambr:Option<ApnAmbr>,
//    pub signalling_prio:Option<SignallingPriorityIndication>,
//    pub cn_operator_selection:Option<CnOperatorSelectionEntity>,
//    pub mapped_ue_usage_type:Option<MappedUeUsageType>,
//    pub up_func_selection_flags:Option<UpFunctionSelectionIndicationFlags>,
//    pub private_extension: Option<PrivateExtension>,
}

impl Default for CreatePDPContextRequest {
    fn default() -> CreatePDPContextRequest {
        CreatePDPContextRequest {
            header: Gtpv1Header::new(),
            imsi:None,
            rai:None,
            recovery:None,
            selectionmode:None,
            teid_data:Teid::default(),
            teid_control:None,
            nsapi:Nsapi::default(),
            linked_nsapi:None,
            charging_char:None,
            trace_ref:None,
            trace_type:None,
            end_user_address:None,
            apn:None,
            pco:None,
            sgsn_ip_control:GsnAddress::default(),
            sgsn_ip_user:GsnAddress::default(),
/*          msisdn:None,
            qos:Qos::default(),
            tft:None,
            trigger_id:None,
            omc_id:None,
            common_flags:None,
            apn_restrictions:None,
            rat_type:None,
            uli:None,
            ms_timezone:None,
            imei_sv:None,
            camel_cic:None,
            add_trace_info:None,
            correlation_id:None,
            evolved_alloc:None,
            ext_common_flags:None,
            user_csg_info:None,
            apn_ambr:None,
            signalling_prio:None,
            cn_operator_selection:None,
            mapped_ue_usage_type:None,
            up_func_selection_flags:None, 
            private_extension: None, */
        }
    }
}
/*
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

    fn unmarshal(header: Gtpv1Header, buffer: &[u8]) -> Result<ErrorIndication, GTPV1Error> {
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
    pub header:Gtpv1Header,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for EndMarker {
    fn default() -> EndMarker {
        EndMarker {
            header: Gtpv1Header::new(),
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

    fn unmarshal(header: Gtpv1Header, buffer: &[u8]) -> Result<EndMarker,GTPV1Error> {
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
        
        let header:Gtpv1Header;

        match Gtpv1Header::unmarshal(&buffer[0..]) {
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
*/