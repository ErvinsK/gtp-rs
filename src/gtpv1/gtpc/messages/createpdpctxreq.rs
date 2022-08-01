use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;


// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const CREATE_PDP_CONTEXT_REQUEST:u8 = 16;

// Definition of GTPv1-C Create PDP Context Request

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
    pub msisdn:Option<Msisdn>,
    pub qos:Qos,
    pub tft:Option<Tft>,
    pub trigger_id:Option<TriggerId>,
    pub omc_id:Option<OmcId>,
    pub common_flags:Option<CommonFlags>,
    pub apn_restriction:Option<ApnRestriction>,
    pub rat_type:Option<RatType>,
    pub uli:Option<Uli>,
    pub ms_timezone:Option<MsTimeZone>,
    pub imei:Option<Imei>,
    pub camel_cic:Option<CamelChargingInfoContainer>,
    pub add_trace_info:Option<AdditionalTraceInfo>,
    pub correlation_id:Option<CorrelationId>,
    pub evolved_alloc:Option<EvolvedAllocationRetentionI>,
    pub ext_common_flags:Option<ExtendedCommonFlags>,
    pub user_csg_info:Option<Uci>,
    pub apn_ambr:Option<ApnAmbr>,
    pub signalling_prio:Option<Spi>,
    pub cnose:Option<CnOperatorSelectionEntity>,
    pub mapped_ue_usage_type:Option<MappedUeUsageType>,
    pub up_func_selection_flags:Option<UpFunctionSelectionIndicationFlags>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for CreatePDPContextRequest {
    fn default() -> CreatePDPContextRequest {
        let mut hdr = Gtpv1Header::default();
        hdr.msgtype = CREATE_PDP_CONTEXT_REQUEST;
        CreatePDPContextRequest {
            header: hdr,
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
            msisdn:None,
            qos:Qos::default(),
            tft:None,
            trigger_id:None,
            omc_id:None,
            common_flags:None,
            apn_restriction:None,
            rat_type:None,
            uli:None,
            ms_timezone:None,
            imei:None,
            camel_cic:None,
            add_trace_info:None,
            correlation_id:None,
            evolved_alloc:None,
            ext_common_flags:None,
            user_csg_info:None,
            apn_ambr:None,
            signalling_prio:None,
            cnose:None,
            mapped_ue_usage_type:None,
            up_func_selection_flags:None, 
            private_extension: None,
        }
    }
}

impl Messages for CreatePDPContextRequest {

    fn marshal (self, buffer: &mut Vec<u8>) {
    
        // Marshal header

            self.header.marshal(buffer);
        
        // Marshal IMSI IE

            match self.imsi {
                Some(i) => {
                    marshal_IE(i, buffer);
                },
                None => (),
            }

        // Marshal RAI IE

        match self.rai {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Recovery IE

        match self.recovery {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Selection Mode IE

        match self.selectionmode {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal TEID Data IE

        self.teid_data.marshal(buffer);

        // Marshal TEID Control IE

        match self.teid_control {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal NSAPI IE 

        self.nsapi.marshal(buffer);

        // Marshal Linked NSAPI IE

        match self.linked_nsapi {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Charging Characteristics IE

        match self.charging_char {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Trace Reference IE 

        match self.trace_ref {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Trace Type IE 

        match self.trace_type {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal End User Address IE

        match self.end_user_address {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal APN IE

        match self.apn {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal PCO IE

        match self.pco {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal SGSN Address for Signalling IE

        marshal_IE(self.sgsn_ip_control, buffer);
        
        // Marshal SGSN Address for User plane IE

        marshal_IE(self.sgsn_ip_user,buffer);

        // Marshal MSISDN IE

        match self.msisdn {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal QoS IE

        self.qos.marshal(buffer);

        // Marshal TFT IE

        match self.tft {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Trigger ID IE

        match self.trigger_id {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal OMC Id IE

        match self.omc_id {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Common Flags IE

        match self.common_flags {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal APN Restriction IE

        match self.apn_restriction {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal RAT Type IE

        match self.rat_type {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal ULI IE

        match self.uli {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal MS Time Zone IE

        match self.ms_timezone {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal IMEI(SV) IE

        match self.imei {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal CAMEL Charging Information Container IE

        match self.camel_cic {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Additional Trace Info IE

        match self.add_trace_info {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Correlation-ID IE

        match self.correlation_id {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Evolved Allocation/Retention Priority I IE

        match self.evolved_alloc {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Extended Common Flags IE

        match self.ext_common_flags {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal User CSG Information IE

        match self.user_csg_info {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal APN-AMBR IE

        match self.apn_ambr {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Signalling Priority Indication IE

        match self.signalling_prio {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal CN Operator Selection Entity IE

        match self.cnose {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Mapped UE Usage Type IE

        match self.mapped_ue_usage_type {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal UP Function Selection Indication Flags IE

        match self.up_func_selection_flags {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }

        // Marshal Private Extension IE
        
        match self.private_extension {
            Some(i) => {
                marshal_IE(i, buffer);
            },
            None => (),
        }
        
        set_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut message = CreatePDPContextRequest::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }
        if message.header.length as usize <= buffer.len() {
            let mut cursor = message.header.get_header_size();
            match Recovery::unmarshal(&buffer[cursor..]) {
                Ok(i) => message.recovery=i,
                Err(_) => return Err(GTPV1Error::MandatoryIEMissing),
            }
            cursor+=message.recovery.len();
            match PrivateExtension::unmarshal(&buffer[cursor..]) {
                Ok(i) => message.private_extension=Some(i),
                Err(_)=> (),
            }
            Ok(message)    
        } else {
            Err(GTPV1Error::MessageLengthError)
        }    
    }       
}



