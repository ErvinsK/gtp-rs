use std::collections::HashMap;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;


// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const CREATE_PDP_CONTEXT_RESPONSE:u8 = 17;

// Definition of GTPv1-C Create PDP Context Response

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePDPContextResponse {
    pub header:Gtpv1Header,
    pub cause:Cause,
    pub reordering_req:Option<ReorderingRequired>,
    pub recovery:Option<Recovery>,
    pub teid_data:Option<Teid>,
    pub teid_control:Option<Teid>,
    pub nsapi:Option<Nsapi>,
    pub charging_id:Option<ChargingID>,
    pub end_user_address:Option<EndUserAddress>,
    pub pco:Option<Pco>,
    pub sgsn_ip_control:Option<GsnAddress>,
    pub sgsn_ip_user:Option<GsnAddress>,
    pub alt_sgsn_ip_control:Option<GsnAddress>,
    pub alt_sgsn_ip_user:Option<GsnAddress>,
    pub qos:Option<Qos>,
    pub charging_gw_addr:Option<ChargingGWAddress>,
    pub alt_charging_gw_addr:Option<ChargingGWAddress>,
    pub common_flags:Option<CommonFlags>,
    pub apn_restriction:Option<ApnRestriction>,
    pub ms_info_change:Option<MSInfoChangeReportingAction>,
    pub bearer_ctrl_mode:Option<BearerControlMode>,
    pub evolved_alloc:Option<EvolvedAllocationRetentionI>,
    pub ext_common_flags:Option<ExtendedCommonFlags>,
    pub csg_info_report:Option<CSGInformationReportingAction>,
    pub apn_ambr:Option<ApnAmbr>,
    pub ggsn_backoff_time:Option<GGSNBackOffTime>,
    pub ext_common_flags_ii:Option<ExtendedCommonFlagsII>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for CreatePDPContextResponse {
    fn default() -> CreatePDPContextResponse {
        let mut hdr = Gtpv1Header::default();
        hdr.msgtype = CREATE_PDP_CONTEXT_RESPONSE;
        CreatePDPContextResponse {
            header: hdr,
            cause:Cause::default(),
            reordering_req:None,
            recovery:None,
            teid_data:None,
            teid_control:None,
            nsapi:None,
            charging_id:None,
            end_user_address:None,
            pco:None,
            sgsn_ip_control:None,
            sgsn_ip_user:None,
            alt_sgsn_ip_control:None,
            alt_sgsn_ip_user:None,
            qos:None,
            charging_gw_addr:None,
            alt_charging_gw_addr:None,
            common_flags:None,
            apn_restriction:None,
            ms_info_change:None,
            bearer_ctrl_mode:None,
            evolved_alloc:None,
            ext_common_flags:None,
            csg_info_report:None,
            apn_ambr:None,
            ggsn_backoff_time:None,
            ext_common_flags_ii:None,
            private_extension: None,
        }
    }
}

/*
impl Messages for CreatePDPContextRequest {

    fn marshal (self, buffer: &mut Vec<u8>) {
    
        // Marshal header

            self.header.marshal(buffer);
               
        // Marshal IMSI IE

            match self.imsi {
                Some(i) => {
                    i.marshal(buffer);
                },
                None => (),
            }

        // Marshal RAI IE

        match self.rai {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Recovery IE

        match self.recovery {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Selection Mode IE

        match self.selectionmode {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal TEID Data IE

        self.teid_data.marshal(buffer);

        // Marshal TEID Control IE

        match self.teid_control {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal NSAPI IE 

        self.nsapi.marshal(buffer);

        // Marshal Linked NSAPI IE

        match self.linked_nsapi {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Charging Characteristics IE

        match self.charging_char {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Trace Reference IE 

        match self.trace_ref {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Trace Type IE 

        match self.trace_type {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal End User Address IE

        match self.end_user_address {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal APN IE

        match self.apn {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal PCO IE

        match self.pco {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal SGSN Address for Signalling IE

        self.sgsn_ip_control.marshal(buffer);
        
        // Marshal SGSN Address for User plane IE

        self.sgsn_ip_user.marshal(buffer);

        // Marshal MSISDN IE

        match self.msisdn {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal QoS IE

        self.qos.marshal(buffer);

        // Marshal TFT IE

        match self.tft {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Trigger ID IE

        match self.trigger_id {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal OMC Id IE

        match self.omc_id {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Common Flags IE

        match self.common_flags {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal APN Restriction IE

        match self.apn_restriction {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal RAT Type IE

        match self.rat_type {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal ULI IE

        match self.uli {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal MS Time Zone IE

        match self.ms_timezone {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal IMEI(SV) IE

        match self.imei {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal CAMEL Charging Information Container IE

        match self.camel_cic {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Additional Trace Info IE

        match self.add_trace_info {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Correlation-ID IE

        match self.correlation_id {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Evolved Allocation/Retention Priority I IE

        match self.evolved_alloc {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Extended Common Flags IE

        match self.ext_common_flags {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal User CSG Information IE

        match self.user_csg_info {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal APN-AMBR IE

        match self.apn_ambr {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Signalling Priority Indication IE

        match self.signalling_prio {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal CN Operator Selection Entity IE

        match self.cnose {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Mapped UE Usage Type IE

        match self.mapped_ue_usage_type {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal UP Function Selection Indication Flags IE

        match self.up_func_selection_flags {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        // Marshal Private Extension IE
        
        match self.private_extension {
            Some(i) => {
                i.marshal(buffer);
            },
            None => (),
        }

        set_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV1Error> {
        
        let mut msg_hash:HashMap<u8,u8> = HashMap::new();

        let mut message = CreatePDPContextRequest::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }
        if (message.header.length+8) as usize <= buffer.len() {
            
            let mut cursor = message.header.get_header_size();
            let mut increment:u8=0;
            loop {
                if buffer[cursor]>=increment {    
                    match buffer[cursor] {
                                IMSI => { 
                                    match Imsi::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => { 
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.imsi= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                }, 
                                RAI => {
                                    match Rai::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.rai= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                RECOVERY => {
                                    match Recovery::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.recovery= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                SELECTION_MODE => {
                                    match SelectionMode::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.selectionmode= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                TEID_DATA => {
                                    match Teid::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.teid_data= i;
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageMandatoryIEMissing), 
                                    }
                                },
                                TEID_CONTROL=> {
                                    match Teid::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.teid_control= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                NSAPI => {
                                    match Nsapi::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.nsapi= i;
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap();
                                                if n<2 {
                                                    msg_hash.insert(buffer[cursor], n+1);
                                                    increment = buffer[cursor];
                                                    cursor+=i.len();
                                                    message.linked_nsapi=Some(i);
                                                } else {
                                                    msg_hash.insert(buffer[cursor], n+1);
                                                    increment = buffer[cursor];
                                                    cursor+=i.len();
                                                }
                                            }   
                                        },
                                        Err (_) => {
                                            if increment != NSAPI {
                                                return Err(GTPV1Error::MessageMandatoryIEMissing);
                                            } else {
                                                return Err(GTPV1Error::MessageOptionalIEIncorrect);
                                            }
                                        }
                                    }
                                },
                                CHARGING_CHARACTERISTICS => {
                                    match ChargingCharacteristics::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.charging_char= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                TRACE_REFERENCE => {
                                    match TraceReference::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.trace_ref= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                TRACE_TYPE => {
                                    match TraceType::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.trace_type= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                END_USER_ADDRESS => {
                                    match EndUserAddress::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.end_user_address= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                APN => {
                                    match Apn::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.apn= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                PCO => {
                                    match Pco::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.pco= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                GSN_ADDRESS=> {
                                    match GsnAddress::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.sgsn_ip_control= i;
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap();
                                                if n<2 {
                                                    msg_hash.insert(buffer[cursor], n+1);
                                                    increment = buffer[cursor];
                                                    cursor+=i.len();
                                                    message.sgsn_ip_user=i;
                                                } else {
                                                    msg_hash.insert(buffer[cursor], n+1);
                                                    increment = buffer[cursor];
                                                    cursor+=i.len();
                                                }
                                            }   
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageMandatoryIEMissing), 
                                    }
                                },
                                MSISDN => {
                                    match Msisdn::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.msisdn= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                QOS => {
                                    match Qos::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.qos= i;
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageMandatoryIEMissing), 
                                    }
                                },
                                TFT => {
                                    match Tft::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.tft= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                TRIGGERID => {
                                    match TriggerId::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.trigger_id= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                OMCID => {
                                    match OmcId::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.omc_id= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                COMMONFLAGS => {
                                    match CommonFlags::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.common_flags= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                APNRESTRICTION => {
                                    match ApnRestriction::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.apn_restriction = Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                RATTYPE => {
                                    match RatType::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.rat_type= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                ULI => {
                                    match Uli::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.uli= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                MSTIMEZONETYPE => {
                                    match MsTimeZone::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.ms_timezone = Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                IMEI => {
                                    match Imei::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.imei= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                CAMELCIC => {
                                    match CamelChargingInfoContainer::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.camel_cic= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                ADDITIONALTRACEINFO => {
                                    match AdditionalTraceInfo::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.add_trace_info= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                CORRELATIONID => {
                                    match CorrelationId::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.correlation_id= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                EVOLVEDALLOCRETENTIONI => {
                                    match EvolvedAllocationRetentionI::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.evolved_alloc= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                EXTCOMMONFLAGS => {
                                    match ExtendedCommonFlags::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.ext_common_flags= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                UCI => match Uci::unmarshal(&buffer[cursor..]) {
                                    Ok(i) => {
                                        if !msg_hash.contains_key(&buffer[cursor]) {
                                            increment = buffer[cursor];
                                            msg_hash.insert(buffer[cursor], 1);
                                            cursor+=i.len();
                                            message.user_csg_info= Some(i);
                                        } else {
                                            let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                            msg_hash.insert(buffer[cursor], n);
                                            increment = buffer[cursor];
                                            cursor+=i.len();
                                        }
                                    },
                                    Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                },
                                APNAMBR => {
                                    match ApnAmbr::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.apn_ambr= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                SPI => match Spi::unmarshal(&buffer[cursor..]) {
                                    Ok(i) => {
                                        if !msg_hash.contains_key(&buffer[cursor]) {
                                            increment = buffer[cursor];
                                            msg_hash.insert(buffer[cursor], 1);
                                            cursor+=i.len();
                                            message.signalling_prio= Some(i);
                                        } else {
                                            let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                            msg_hash.insert(buffer[cursor], n);
                                            increment = buffer[cursor];
                                            cursor+=i.len();
                                        }
                                    },
                                    Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                },
                                CNOSE => {
                                    match CnOperatorSelectionEntity::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.cnose= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                MUEUT => {
                                    match MappedUeUsageType::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.mapped_ue_usage_type= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                UPFSIF => {
                                    match UpFunctionSelectionIndicationFlags::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.up_func_selection_flags= Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                PRIVATE_EXTENSION => {
                                    match PrivateExtension::unmarshal(&buffer[cursor..]) {
                                        Ok(i) => {
                                            if !msg_hash.contains_key(&buffer[cursor]) {
                                                increment = buffer[cursor];
                                                msg_hash.insert(buffer[cursor], 1);
                                                cursor+=i.len();
                                                message.private_extension = Some(i);
                                            } else {
                                                let n = *msg_hash.get(&buffer[cursor]).unwrap()+1;
                                                msg_hash.insert(buffer[cursor], n);
                                                increment = buffer[cursor];
                                                cursor+=i.len();
                                            }
                                        },
                                        Err (_) => return Err(GTPV1Error::MessageOptionalIEIncorrect), 
                                    }
                                },
                                _ => return Err(GTPV1Error::MessageInvalidMessageFormat),
                            }
                        } else {
                            return Err(GTPV1Error::MessageInvalidMessageFormat);
                        }
                        if cursor>=buffer.len() {
                            match (msg_hash.get(&TEID_DATA), msg_hash.get(&NSAPI), msg_hash.get(&GSN_ADDRESS), msg_hash.get(&QOS)) {
                                (Some(_),Some(_),Some(_), Some(_)) => return Ok(message),
                                _ => return Err(GTPV1Error::MessageMandatoryIEMissing),
                            }
                        }
                }
            } else {
                return Err(GTPV1Error::MessageLengthError);
            }                 
        }

}

#[test]

fn create_pdp_ctx_unmarshal_test() {
    use std::{net::{IpAddr, Ipv4Addr}};
    let encoded:[u8;175]= [
    0x32, 0x10, 0x00, 0xa7, 0x00, 0x00, /* ..2..... */
    0x00, 0x00, 0x5a, 0xfc, 0xff, 0x00, 0x02, 0x09, /* ..Z..... */
    0x41, 0x50, 0x01, 0x71, 0x44, 0x45, 0xf6, 0x03, /* AP.qDE.. */
    0x13, 0x00, 0x62, 0xff, 0xfe, 0xff, 0x0e, 0x0e, /* ..b..... */
    0x0f, 0xfc, 0x10, 0x00, 0x04, 0x72, 0xd5, 0x11, /* .....r.. */
    0xd7, 0x08, 0x61, 0x02, 0x14, 0x05, 0x80, 0x00, /* ..a..... */
    0x02, 0xf1, 0x21, 0x83, 0x00, 0x0d, 0x03, 0x69, /* ..!....i */
    0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, 0x65, 0x03, /* ot.1nce. */
    0x6e, 0x65, 0x74, 0x84, 0x00, 0x20, 0x80, 0x80, /* net.. .. */
    0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, /* !....... */
    0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, /* ........ */
    0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x0a, 0x00, /* ........ */
    0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x85, 0x00, /* ........ */
    0x04, 0xac, 0x39, 0x2b, 0xcc, 0x85, 0x00, 0x04, /* ..9+.... */
    0xac, 0x39, 0x2b, 0xcd, 0x86, 0x00, 0x09, 0x91, /* .9+..... */
    0x88, 0x22, 0x58, 0x01, 0x71, 0x44, 0x45, 0xf6, /* ."X.qDE. */
    0x87, 0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, /* .....c.s */
    0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00, /* .sst.... */
    0x00, 0x00, 0x97, 0x00, 0x01, 0x02, 0x98, 0x00, /* ........ */
    0x08, 0x00, 0x13, 0x00, 0x62, 0x53, 0x17, 0x04, /* ....bS.. */
    0x27, 0x99, 0x00, 0x02, 0x00, 0x00, 0x9a, 0x00, /* '....... */
    0x08, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10, /* .h..0.d. */
    0x10];
    let decoded = CreatePDPContextRequest {
         header: Gtpv1Header { msgtype: 16, length: 167, teid: 0, sequence_number: Some(23292), npdu_number: None, extension_headers: None }, 
         imsi: Some(Imsi { t: 2, imsi: "901405101744546".to_string() }),
         rai: Some(Rai { t: 3, mcc: 310, mnc: 260, lac: 65534, rac: 255 }),
         recovery: Some(Recovery { t: 14, value: 14 }), 
         selectionmode: Some(SelectionMode { t: 15, value: 0 }), 
         teid_data: Teid { t: 16, teid: 291541 },
         teid_control: Some(Teid { t: 17, teid: 3607650562 }),
         nsapi: Nsapi { t: 20, value: 5 },
         linked_nsapi: None,
         charging_char: None,
         trace_ref: None,
         trace_type: None,
         end_user_address: Some(EndUserAddress { t: 128, length: 2, pdp_type_org: 1, pdp_type_nbr: 33, ipv4: None, ipv6: None }),
         apn: Some(Apn { t: 131, length: 13, name: "iot.1nce.net".to_string() }),
         pco: Some(Pco { t: 132, length: 32, pco: vec!(128, 128, 33, 16, 1, 0, 0, 16, 129, 6, 0, 0, 0, 0, 131, 6, 0, 0, 0, 0, 0, 13, 0, 0, 10, 0, 0, 5, 0, 0, 17, 0) }),
         sgsn_ip_control: GsnAddress { t: 133, length: 4, ip: IpAddr::V4(Ipv4Addr::new(172,57,43,204)) },
         sgsn_ip_user: GsnAddress { t: 133, length: 4, ip: IpAddr::V4(Ipv4Addr::new(172,57,43,205)) },
         msisdn: Some(Msisdn { t: 134, length: 9, extension: 1, number_nature: 1, number_plan: 1, msisdn: "882285101744546".to_string() }),
         qos: Qos { t: 135, length: 15, arp: 3, qos: vec!(27, 99, 31, 115, 150, 115, 115, 116, 255, 255, 255, 0, 0, 0) },
         tft: None, 
         trigger_id: None, 
         omc_id: None, 
         common_flags: None, 
         apn_restriction: None, 
         rat_type: Some(RatType { t: 151, length: 1, rat_type: Rat::Geran }),
         uli: Some(Uli { t: 152, length: 8, mcc: 310, mnc: 260, lac: 21271, loc: Location::Ci(1063) }),
         ms_timezone: Some(MsTimeZone { t: 153, length: 2, time_zone: 0, dst: 0 }), 
         imei: Some(Imei { t: 154, length: 8, imei: "8699510319460101".to_string() }),
         camel_cic: None,
         add_trace_info: None,
         correlation_id: None, 
         evolved_alloc: None,
         ext_common_flags: None,
         user_csg_info: None,
         apn_ambr: None,
         signalling_prio: None,
         cnose: None,
         mapped_ue_usage_type: None,
         up_func_selection_flags: None,
         private_extension: None };
    assert_eq!(CreatePDPContextRequest::unmarshal(&encoded).unwrap(),decoded);
}

#[test]
fn create_pdp_ctx_marshal_test() {
    use std::{net::{IpAddr, Ipv4Addr}};
    let encoded:[u8;175]= [
    0x32, 0x10, 0x00, 0xa7, 0x00, 0x00, /* ..2..... */
    0x00, 0x00, 0x5a, 0xfc, 0x00, 0x00, 0x02, 0x09, /* ..Z..... */
    0x41, 0x50, 0x01, 0x71, 0x44, 0x45, 0xf6, 0x03, /* AP.qDE.. */
    0x13, 0x00, 0x62, 0xff, 0xfe, 0xff, 0x0e, 0x0e, /* ..b..... */
    0x0f, 0xfc, 0x10, 0x00, 0x04, 0x72, 0xd5, 0x11, /* .....r.. */
    0xd7, 0x08, 0x61, 0x02, 0x14, 0x05, 0x80, 0x00, /* ..a..... */
    0x02, 0xf1, 0x21, 0x83, 0x00, 0x0d, 0x03, 0x69, /* ..!....i */
    0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, 0x65, 0x03, /* ot.1nce. */
    0x6e, 0x65, 0x74, 0x84, 0x00, 0x20, 0x80, 0x80, /* net.. .. */
    0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, /* !....... */
    0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, /* ........ */
    0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x0a, 0x00, /* ........ */
    0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x85, 0x00, /* ........ */
    0x04, 0xac, 0x39, 0x2b, 0xcc, 0x85, 0x00, 0x04, /* ..9+.... */
    0xac, 0x39, 0x2b, 0xcc, 0x86, 0x00, 0x09, 0x91, /* .9+..... */
    0x88, 0x22, 0x58, 0x01, 0x71, 0x44, 0x45, 0xf6, /* ."X.qDE. */
    0x87, 0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, /* .....c.s */
    0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00, /* .sst.... */
    0x00, 0x00, 0x97, 0x00, 0x01, 0x02, 0x98, 0x00, /* ........ */
    0x08, 0x00, 0x13, 0x00, 0x62, 0x53, 0x17, 0x04, /* ....bS.. */
    0x27, 0x99, 0x00, 0x02, 0x00, 0x00, 0x9a, 0x00, /* '....... */
    0x08, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10, /* .h..0.d. */
    0x10];
    let decoded = CreatePDPContextRequest {
         header: Gtpv1Header { msgtype: 16, length: 167, teid: 0, sequence_number: Some(23292), npdu_number: None, extension_headers: None }, 
         imsi: Some(Imsi { t: 2, imsi: "901405101744546".to_string() }),
         rai: Some(Rai { t: 3, mcc: 310, mnc: 260, lac: 65534, rac: 255 }),
         recovery: Some(Recovery { t: 14, value: 14 }), 
         selectionmode: Some(SelectionMode { t: 15, value: 0 }), 
         teid_data: Teid { t: 16, teid: 291541 },
         teid_control: Some(Teid { t: 17, teid: 3607650562 }),
         nsapi: Nsapi { t: 20, value: 5 },
         linked_nsapi: None,
         charging_char: None,
         trace_ref: None,
         trace_type: None,
         end_user_address: Some(EndUserAddress { t: 128, length: 2, pdp_type_org: 1, pdp_type_nbr: 33, ipv4: None, ipv6: None }),
         apn: Some(Apn { t: 131, length: 13, name: "iot.1nce.net".to_string() }),
         pco: Some(Pco { t: 132, length: 32, pco: vec!(128, 128, 33, 16, 1, 0, 0, 16, 129, 6, 0, 0, 0, 0, 131, 6, 0, 0, 0, 0, 0, 13, 0, 0, 10, 0, 0, 5, 0, 0, 17, 0) }),
         sgsn_ip_control: GsnAddress { t: 133, length: 4, ip: IpAddr::V4(Ipv4Addr::new(172,57,43,204)) },
         sgsn_ip_user: GsnAddress { t: 133, length: 4, ip: IpAddr::V4(Ipv4Addr::new(172,57,43,204)) },
         msisdn: Some(Msisdn { t: 134, length: 9, extension: 1, number_nature: 1, number_plan: 1, msisdn: "882285101744546".to_string() }),
         qos: Qos { t: 135, length: 15, arp: 3, qos: vec!(27, 99, 31, 115, 150, 115, 115, 116, 255, 255, 255, 0, 0, 0) },
         tft: None, 
         trigger_id: None, 
         omc_id: None, 
         common_flags: None, 
         apn_restriction: None, 
         rat_type: Some(RatType { t: 151, length: 1, rat_type: Rat::Geran }),
         uli: Some(Uli { t: 152, length: 8, mcc: 310, mnc: 260, lac: 21271, loc: Location::Ci(1063) }),
         ms_timezone: Some(MsTimeZone { t: 153, length: 2, time_zone: 0, dst: 0 }), 
         imei: Some(Imei { t: 154, length: 8, imei: "8699510319460101".to_string() }),
         camel_cic: None,
         add_trace_info: None,
         correlation_id: None, 
         evolved_alloc: None,
         ext_common_flags: None,
         user_csg_info: None,
         apn_ambr: None,
         signalling_prio: None,
         cnose: None,
         mapped_ue_usage_type: None,
         up_func_selection_flags: None,
         private_extension: None };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn create_pdp_ctx_unmarshal_with_linked_nsapi_test() {
    use std::{net::{IpAddr, Ipv4Addr}};
    let encoded:[u8;177]= [
    0x32, 0x10, 0x00, 0xa9, 0x00, 0x00, /* ..2..... */
    0x00, 0x00, 0x5a, 0xfc, 0xff, 0x00, 0x02, 0x09, /* ..Z..... */
    0x41, 0x50, 0x01, 0x71, 0x44, 0x45, 0xf6, 0x03, /* AP.qDE.. */
    0x13, 0x00, 0x62, 0xff, 0xfe, 0xff, 0x0e, 0x0e, /* ..b..... */
    0x0f, 0xfc, 0x10, 0x00, 0x04, 0x72, 0xd5, 0x11, /* .....r.. */
    0xd7, 0x08, 0x61, 0x02, 0x14, 0x05, 0x14, 0x01, 0x80, 0x00, /* ..a..... */
    0x02, 0xf1, 0x21, 0x83, 0x00, 0x0d, 0x03, 0x69, /* ..!....i */
    0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, 0x65, 0x03, /* ot.1nce. */
    0x6e, 0x65, 0x74, 0x84, 0x00, 0x20, 0x80, 0x80, /* net.. .. */
    0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, /* !....... */
    0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, /* ........ */
    0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x0a, 0x00, /* ........ */
    0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x85, 0x00, /* ........ */
    0x04, 0xac, 0x39, 0x2b, 0xcc, 0x85, 0x00, 0x04, /* ..9+.... */
    0xac, 0x39, 0x2b, 0xcc, 0x86, 0x00, 0x09, 0x91, /* .9+..... */
    0x88, 0x22, 0x58, 0x01, 0x71, 0x44, 0x45, 0xf6, /* ."X.qDE. */
    0x87, 0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, /* .....c.s */
    0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00, /* .sst.... */
    0x00, 0x00, 0x97, 0x00, 0x01, 0x02, 0x98, 0x00, /* ........ */
    0x08, 0x00, 0x13, 0x00, 0x62, 0x53, 0x17, 0x04, /* ....bS.. */
    0x27, 0x99, 0x00, 0x02, 0x00, 0x00, 0x9a, 0x00, /* '....... */
    0x08, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10, /* .h..0.d. */
    0x10];
    let decoded = CreatePDPContextRequest {
         header: Gtpv1Header { msgtype: 16, length: 169, teid: 0, sequence_number: Some(23292), npdu_number: None, extension_headers: None }, 
         imsi: Some(Imsi { t: 2, imsi: "901405101744546".to_string() }),
         rai: Some(Rai { t: 3, mcc: 310, mnc: 260, lac: 65534, rac: 255 }),
         recovery: Some(Recovery { t: 14, value: 14 }), 
         selectionmode: Some(SelectionMode { t: 15, value: 0 }), 
         teid_data: Teid { t: 16, teid: 291541 },
         teid_control: Some(Teid { t: 17, teid: 3607650562 }),
         nsapi: Nsapi { t: 20, value: 5 },
         linked_nsapi: Some(Nsapi { t:20, value: 1}),
         charging_char: None,
         trace_ref: None,
         trace_type: None,
         end_user_address: Some(EndUserAddress { t: 128, length: 2, pdp_type_org: 1, pdp_type_nbr: 33, ipv4: None, ipv6: None }),
         apn: Some(Apn { t: 131, length: 13, name: "iot.1nce.net".to_string() }),
         pco: Some(Pco { t: 132, length: 32, pco: vec!(128, 128, 33, 16, 1, 0, 0, 16, 129, 6, 0, 0, 0, 0, 131, 6, 0, 0, 0, 0, 0, 13, 0, 0, 10, 0, 0, 5, 0, 0, 17, 0) }),
         sgsn_ip_control: GsnAddress { t: 133, length: 4, ip: IpAddr::V4(Ipv4Addr::new(172,57,43,204)) },
         sgsn_ip_user: GsnAddress { t: 133, length: 4, ip: IpAddr::V4(Ipv4Addr::new(172,57,43,204)) },
         msisdn: Some(Msisdn { t: 134, length: 9, extension: 1, number_nature: 1, number_plan: 1, msisdn: "882285101744546".to_string() }),
         qos: Qos { t: 135, length: 15, arp: 3, qos: vec!(27, 99, 31, 115, 150, 115, 115, 116, 255, 255, 255, 0, 0, 0) },
         tft: None, 
         trigger_id: None, 
         omc_id: None, 
         common_flags: None, 
         apn_restriction: None, 
         rat_type: Some(RatType { t: 151, length: 1, rat_type: Rat::Geran }),
         uli: Some(Uli { t: 152, length: 8, mcc: 310, mnc: 260, lac: 21271, loc: Location::Ci(1063) }),
         ms_timezone: Some(MsTimeZone { t: 153, length: 2, time_zone: 0, dst: 0 }), 
         imei: Some(Imei { t: 154, length: 8, imei: "8699510319460101".to_string() }),
         camel_cic: None,
         add_trace_info: None,
         correlation_id: None, 
         evolved_alloc: None,
         ext_common_flags: None,
         user_csg_info: None,
         apn_ambr: None,
         signalling_prio: None,
         cnose: None,
         mapped_ue_usage_type: None,
         up_func_selection_flags: None,
         private_extension: None };
    assert_eq!(CreatePDPContextRequest::unmarshal(&encoded).unwrap(),decoded);
}

#[test]

fn create_pdp_ctx_wrong_ie_order_unmarshal_test() {
    let encoded:[u8;175]= [
    0x32, 0x10, 0x00, 0xa7, 0x00, 0x00, /* ..2..... */
    0x00, 0x00, 0x5a, 0xfc, 0xff, 0x00, 0x02, 0x09, /* ..Z..... */
    0x41, 0x50, 0x01, 0x71, 0x44, 0x45, 0xf6, 0x0e, 0x0e, 0x03, /* AP.qDE.. */
    0x13, 0x00, 0x62, 0xff, 0xfe, 0xff, /* ..b..... */
    0x0f, 0xfc, 0x10, 0x00, 0x04, 0x72, 0xd5, 0x11, /* .....r.. */
    0xd7, 0x08, 0x61, 0x02, 0x14, 0x05, 0x80, 0x00, /* ..a..... */
    0x02, 0xf1, 0x21, 0x83, 0x00, 0x0d, 0x03, 0x69, /* ..!....i */
    0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, 0x65, 0x03, /* ot.1nce. */
    0x6e, 0x65, 0x74, 0x84, 0x00, 0x20, 0x80, 0x80, /* net.. .. */
    0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, /* !....... */
    0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, /* ........ */
    0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x0a, 0x00, /* ........ */
    0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x85, 0x00, /* ........ */
    0x04, 0xac, 0x39, 0x2b, 0xcd, 0x85, 0x00, 0x04, /* ..9+.... */
    0xac, 0x39, 0x2b, 0xcc, 0x86, 0x00, 0x09, 0x91, /* .9+..... */
    0x88, 0x22, 0x58, 0x01, 0x71, 0x44, 0x45, 0xf6, /* ."X.qDE. */
    0x87, 0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, /* .....c.s */
    0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00, /* .sst.... */
    0x00, 0x00, 0x97, 0x00, 0x01, 0x02, 0x98, 0x00, /* ........ */
    0x08, 0x00, 0x13, 0x00, 0x62, 0x53, 0x17, 0x04, /* ....bS.. */
    0x27, 0x99, 0x00, 0x02, 0x00, 0x00, 0x9a, 0x00, /* '....... */
    0x08, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10, /* .h..0.d. */
    0x10];
    assert_eq!(CreatePDPContextRequest::unmarshal(&encoded),Err(GTPV1Error::MessageInvalidMessageFormat));
}

#[test]

fn create_pdp_ctx_missing_mandatory_ie_unmarshal_test() {
    let encoded:[u8;170]= [
        0x32, 0x10, 0x00, 0xa2, 0x00, 0x00, /* ..2..... */
        0x00, 0x00, 0x5a, 0xfc, 0xff, 0x00, 0x02, 0x09, /* ..Z..... */
        0x41, 0x50, 0x01, 0x71, 0x44, 0x45, 0xf6, 0x03, /* AP.qDE.. */
        0x13, 0x00, 0x62, 0xff, 0xfe, 0xff, 0x0e, 0x0e, /* ..b..... */
        0x0f, 0xfc, 0x11, /* .....r.. */
        0xd7, 0x08, 0x61, 0x02, 0x14, 0x05, 0x80, 0x00, /* ..a..... */
        0x02, 0xf1, 0x21, 0x83, 0x00, 0x0d, 0x03, 0x69, /* ..!....i */
        0x6f, 0x74, 0x04, 0x31, 0x6e, 0x63, 0x65, 0x03, /* ot.1nce. */
        0x6e, 0x65, 0x74, 0x84, 0x00, 0x20, 0x80, 0x80, /* net.. .. */
        0x21, 0x10, 0x01, 0x00, 0x00, 0x10, 0x81, 0x06, /* !....... */
        0x00, 0x00, 0x00, 0x00, 0x83, 0x06, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x0a, 0x00, /* ........ */
        0x00, 0x05, 0x00, 0x00, 0x11, 0x00, 0x85, 0x00, /* ........ */
        0x04, 0xac, 0x39, 0x2b, 0xcd, 0x85, 0x00, 0x04, /* ..9+.... */
        0xac, 0x39, 0x2b, 0xcc, 0x86, 0x00, 0x09, 0x91, /* .9+..... */
        0x88, 0x22, 0x58, 0x01, 0x71, 0x44, 0x45, 0xf6, /* ."X.qDE. */
        0x87, 0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, /* .....c.s */
        0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00, /* .sst.... */
        0x00, 0x00, 0x97, 0x00, 0x01, 0x02, 0x98, 0x00, /* ........ */
        0x08, 0x00, 0x13, 0x00, 0x62, 0x53, 0x17, 0x04, /* ....bS.. */
        0x27, 0x99, 0x00, 0x02, 0x00, 0x00, 0x9a, 0x00, /* '....... */
        0x08, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10, /* .h..0.d. */
        0x10];
    assert_eq!(CreatePDPContextRequest::unmarshal(&encoded),Err(GTPV1Error::MessageMandatoryIEMissing));
}
*/