use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const UPDATE_PDP_CONTEXT_REQUEST: u8 = 18;

// Definition of GTPv1-C Update PDP Context Request

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePDPContextRequest {
    pub header: Gtpv1Header,
    pub imsi: Option<Imsi>,
    pub rai: Option<Rai>,
    pub recovery: Option<Recovery>,
    pub teid_data: Teid,
    pub teid_control: Option<Teid>,
    pub nsapi: Nsapi,
    pub trace_ref: Option<TraceReference>,
    pub trace_type: Option<TraceType>,
    pub pco: Option<Pco>,
    pub sgsn_ip_control: GsnAddress,
    pub sgsn_ip_user: GsnAddress,
    pub alt_sgsn_ip_control: Option<GsnAddress>,
    pub alt_sgsn_ip_user: Option<GsnAddress>,
    pub qos: Qos,
    pub tft: Option<Tft>,
    pub trigger_id: Option<TriggerId>,
    pub omc_id: Option<OmcId>,
    pub common_flags: Option<CommonFlags>,
    pub rat_type: Option<RatType>,
    pub uli: Option<Uli>,
    pub ms_timezone: Option<MsTimeZone>,
    pub imei: Option<Imei>,
    pub add_trace_info: Option<AdditionalTraceInfo>,
    pub dtf: Option<DirectTunnelFlags>,
    pub evolved_alloc: Option<EvolvedAllocationRetentionI>,
    pub ext_common_flags: Option<ExtendedCommonFlags>,
    pub user_csg_info: Option<Uci>,
    pub apn_ambr: Option<ApnAmbr>,
    pub signalling_prio: Option<Spi>,
    pub cnose: Option<CnOperatorSelectionEntity>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for UpdatePDPContextRequest {
    fn default() -> UpdatePDPContextRequest {
        let hdr = Gtpv1Header {
            msgtype: UPDATE_PDP_CONTEXT_REQUEST,
            ..Default::default()
        };
        UpdatePDPContextRequest {
            header: hdr,
            imsi: None,
            rai: None,
            recovery: None,
            teid_data: Teid::default(),
            teid_control: None,
            nsapi: Nsapi::default(),
            trace_ref: None,
            trace_type: None,
            pco: None,
            sgsn_ip_control: GsnAddress::default(),
            sgsn_ip_user: GsnAddress::default(),
            alt_sgsn_ip_control: None,
            alt_sgsn_ip_user: None,
            qos: Qos::default(),
            tft: None,
            trigger_id: None,
            omc_id: None,
            common_flags: None,
            rat_type: None,
            uli: None,
            ms_timezone: None,
            imei: None,
            add_trace_info: None,
            dtf: None,
            evolved_alloc: None,
            ext_common_flags: None,
            user_csg_info: None,
            apn_ambr: None,
            signalling_prio: None,
            cnose: None,
            private_extension: None,
        }
    }
}

impl Messages for UpdatePDPContextRequest {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal IMSI IE

        if let Some(i) = self.imsi {
            i.marshal(buffer)
        };

        // Marshal RAI IE

        if let Some(i) = self.rai {
            i.marshal(buffer)
        };

        // Marshal Recovery IE

        if let Some(i) = self.recovery {
            i.marshal(buffer)
        };

        // Marshal TEID Data IE

        self.teid_data.marshal(buffer);

        // Marshal TEID Control IE

        if let Some(i) = self.teid_control {
            i.marshal(buffer)
        };

        // Marshal NSAPI IE

        self.nsapi.marshal(buffer);

        // Marshal Trace Reference IE

        if let Some(i) = self.trace_ref {
            i.marshal(buffer)
        };

        // Marshal Trace Type IE

        if let Some(i) = self.trace_type {
            i.marshal(buffer)
        };

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal SGSN Address for Signalling IE

        self.sgsn_ip_control.marshal(buffer);

        // Marshal SGSN Address for User plane IE

        self.sgsn_ip_user.marshal(buffer);

        // Marshal Alternative SGSN Address for Signalling IE

        if let Some(i) = self.alt_sgsn_ip_control {
            i.marshal(buffer)
        };

        // Marshal Alternative SGSN Address for User plane IE

        if let Some(i) = self.alt_sgsn_ip_user {
            i.marshal(buffer)
        };

        // Marshal QoS IE

        self.qos.marshal(buffer);

        // Marshal TFT IE

        if let Some(i) = self.tft {
            i.marshal(buffer)
        };

        // Marshal Trigger ID IE

        if let Some(i) = self.trigger_id {
            i.marshal(buffer)
        };

        // Marshal OMC Id IE

        if let Some(i) = self.omc_id {
            i.marshal(buffer)
        };

        // Marshal Common Flags IE

        if let Some(i) = self.common_flags {
            i.marshal(buffer)
        };

        // Marshal RAT Type IE

        if let Some(i) = self.rat_type {
            i.marshal(buffer)
        };

        // Marshal ULI IE

        if let Some(i) = self.uli {
            i.marshal(buffer)
        };

        // Marshal MS Time Zone IE

        if let Some(i) = self.ms_timezone {
            i.marshal(buffer)
        };

        // Marshal IMEI(SV) IE

        if let Some(i) = self.imei {
            i.marshal(buffer)
        };

        // Marshal Additional Trace Info IE

        if let Some(i) = self.add_trace_info {
            i.marshal(buffer)
        };

        // Marshal Direct Tunnel Flags IE

        if let Some(i) = self.dtf {
            i.marshal(buffer)
        };

        // Marshal Evolved Allocation/Retention Priority I IE

        if let Some(i) = self.evolved_alloc {
            i.marshal(buffer)
        };

        // Marshal Extended Common Flags IE

        if let Some(i) = self.ext_common_flags {
            i.marshal(buffer)
        };

        // Marshal User CSG Information IE

        if let Some(i) = self.user_csg_info {
            i.marshal(buffer)
        };

        // Marshal APN-AMBR IE

        if let Some(i) = self.apn_ambr {
            i.marshal(buffer)
        };

        // Marshal Signalling Priority Indication IE

        if let Some(i) = self.signalling_prio {
            i.marshal(buffer)
        };

        // Marshal CN Operator Selection Entity IE

        if let Some(i) = self.cnose {
            i.marshal(buffer)
        };

        // Marshal Private Extension IE

        if let Some(i) = self.private_extension {
            i.marshal(buffer)
        };

        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = UpdatePDPContextRequest::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UPDATE_PDP_CONTEXT_REQUEST {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length + 8) as usize <= buffer.len() {
            let mut cursor = message.header.len();
            let mut increment: u8 = 0;
            loop {
                if cursor >= buffer.len() {
                    break;
                }
                if buffer[cursor] >= increment {
                    match buffer[cursor] {
                        IMSI => match Imsi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.imsi = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        RAI => match Rai::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.rai = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        RECOVERY => match Recovery::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.recovery = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        TEID_DATA => match Teid::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.teid_data = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        TEID_CONTROL => match Teid::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.teid_control = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        NSAPI => match Nsapi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.nsapi = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        TRACE_REFERENCE => match TraceReference::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.trace_ref = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        TRACE_TYPE => match TraceType::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.trace_type = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        PCO => match Pco::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.pco = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        GSN_ADDRESS => match GsnAddress::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.sgsn_ip_control = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap();
                                    match n {
                                        1 => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.sgsn_ip_user = i;
                                        }
                                        2 => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.alt_sgsn_ip_control = Some(i);
                                        }
                                        3 => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.alt_sgsn_ip_user = Some(i);
                                        }
                                        _ => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                        }
                                    }
                                }
                            }
                            Err(_) => match *msg_hash.get(&buffer[cursor]).unwrap() {
                                i if i < 3 => return Err(GTPV1Error::MessageMandatoryIEMissing),
                                _ => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                            },
                        },
                        QOS => match Qos::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.qos = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        TFT => match Tft::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.tft = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        TRIGGERID => match TriggerId::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.trigger_id = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        OMCID => match OmcId::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.omc_id = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        COMMONFLAGS => match CommonFlags::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.common_flags = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        RATTYPE => match RatType::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.rat_type = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        ULI => match Uli::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.uli = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        MSTIMEZONETYPE => match MsTimeZone::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.ms_timezone = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        IMEI => match Imei::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.imei = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        ADDITIONALTRACEINFO => {
                            match AdditionalTraceInfo::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.add_trace_info = Some(i);
                                    } else {
                                        let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                        msg_hash.insert(buffer[cursor], n);
                                        increment = buffer[cursor];
                                        cursor += i.len();
                                    }
                                }
                                Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                            }
                        }
                        DTF => match DirectTunnelFlags::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.dtf = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        EVOLVEDALLOCRETENTIONI => {
                            match EvolvedAllocationRetentionI::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.evolved_alloc = Some(i);
                                    } else {
                                        let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                        msg_hash.insert(buffer[cursor], n);
                                        increment = buffer[cursor];
                                        cursor += i.len();
                                    }
                                }
                                Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                            }
                        }
                        EXTCOMMONFLAGS => match ExtendedCommonFlags::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.ext_common_flags = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        UCI => match Uci::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.user_csg_info = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        APNAMBR => match ApnAmbr::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.apn_ambr = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        SPI => match Spi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.signalling_prio = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        CNOSE => match CnOperatorSelectionEntity::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.cnose = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        PRIVATE_EXTENSION => match PrivateExtension::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.private_extension = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        _ => return Err(GTPV1Error::MessageInvalidMessageFormat),
                    }
                } else {
                    return Err(GTPV1Error::MessageInvalidMessageFormat);
                }
            }
            match (
                msg_hash.get(&TEID_DATA),
                msg_hash.get(&NSAPI),
                msg_hash.get(&GSN_ADDRESS),
                msg_hash.get(&QOS),
            ) {
                (Some(_), Some(_), Some(_), Some(_)) => Ok(message),
                _ => Err(GTPV1Error::MessageMandatoryIEMissing),
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn update_pdp_ctx_req_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 104] = [
        0x32, 0x12, 0x00, 0x60, 0x10, 0x2b, /* 6.2..`.+ */
        0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, /* .#...... */
        0x41, 0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x03, /* AP.rg5.. */
        0x22, 0xf6, 0x01, 0xff, 0xfe, 0xff, 0x0e, 0xbf, /* "....... */
        0x10, 0x2f, 0x3c, 0x40, 0xc7, 0x11, 0x2f, 0x3c, /* ./<@../< */
        0x40, 0xc8, 0x14, 0x05, 0x85, 0x00, 0x04, 0x3e, /* @......> */
        0xd9, 0xc8, 0x04, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x28, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, /* .(....#s */
        0x1f, 0x93, 0x96, 0x86, 0x86, 0x74, 0x83, 0xff, /* .....t.. */
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, /* ........ */
        0x01, 0x40, 0x97, 0x00, 0x01, 0x01, 0x98, 0x00, /* .@...... */
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, /* .."...T< */
        0xa9, 0x99, 0x00, 0x02, 0x02, 0x20, 0xbf, 0x00, /* ..... .. */
        0x01, 0x64,
    ];
    let decoded = UpdatePDPContextRequest {
        header: Gtpv1Header {
            msgtype: 18,
            length: 96,
            teid: 271310627,
            sequence_number: Some(49286),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            t: 2,
            imsi: "901405102776539".to_string(),
        }),
        rai: Some(Rai {
            t: 3,
            mcc: 226,
            mnc: 10,
            mnc_is_three_digits: false,
            lac: 65534,
            rac: 255,
        }),
        recovery: Some(Recovery { t: 14, value: 191 }),
        teid_data: Teid {
            t: 16,
            teid: 792477895,
        },
        teid_control: Some(Teid {
            t: 17,
            teid: 792477896,
        }),
        nsapi: Nsapi { t: 20, value: 5 },
        trace_ref: None,
        trace_type: None,
        pco: None,
        sgsn_ip_control: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 4)),
        },
        sgsn_ip_user: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 40)),
        },
        alt_sgsn_ip_control: None,
        alt_sgsn_ip_user: None,
        qos: Qos {
            t: 135,
            length: 17,
            arp: 3,
            qos: vec![
                35, 115, 31, 147, 150, 134, 134, 116, 131, 255, 255, 0, 0, 0, 0, 0,
            ],
        },
        tft: None,
        trigger_id: None,
        omc_id: None,
        common_flags: Some(CommonFlags {
            t: 148,
            length: 1,
            dual_addr_bearer: false,
            upgrade_qos_support: true,
            nrsn: false,
            no_qos_negotiation: false,
            mbms_counting_info: false,
            ran_procedures_ready: false,
            mbms_service_type: false,
            prohibit_payload_compr: false,
        }),
        rat_type: Some(RatType {
            t: 151,
            length: 1,
            rat_type: Rat::Utran,
        }),
        uli: Some(Uli {
            t: 152,
            length: 8,
            mcc: 226,
            mnc: 10,
            mnc_is_three_digits: false,
            lac: 1620,
            loc: Location::Sac(15529),
        }),
        ms_timezone: Some(MsTimeZone {
            t: 153,
            length: 2,
            time_zone: 2,
            dst: 0,
        }),
        imei: None,
        add_trace_info: None,
        dtf: None,
        evolved_alloc: Some(EvolvedAllocationRetentionI {
            t: 191,
            length: 1,
            pre_emption_vulnerability: 0,
            priority_level: 9,
            pre_emption_capability: 1,
        }),
        ext_common_flags: None,
        user_csg_info: None,
        apn_ambr: None,
        signalling_prio: None,
        cnose: None,
        private_extension: None,
    };
    assert_eq!(
        UpdatePDPContextRequest::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn update_pdp_ctx_req_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 104] = [
        0x32, 0x12, 0x00, 0x60, 0x10, 0x2b, /* 6.2..`.+ */
        0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, /* .#...... */
        0x41, 0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x03, /* AP.rg5.. */
        0x22, 0xf6, 0x01, 0xff, 0xfe, 0xff, 0x0e, 0xbf, /* "....... */
        0x10, 0x2f, 0x3c, 0x40, 0xc7, 0x11, 0x2f, 0x3c, /* ./<@../< */
        0x40, 0xc8, 0x14, 0x05, 0x85, 0x00, 0x04, 0x3e, /* @......> */
        0xd9, 0xc8, 0x04, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x28, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, /* .(....#s */
        0x1f, 0x93, 0x96, 0x86, 0x86, 0x74, 0x83, 0xff, /* .....t.. */
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, /* ........ */
        0x01, 0x40, 0x97, 0x00, 0x01, 0x01, 0x98, 0x00, /* .@...... */
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, /* .."...T< */
        0xa9, 0x99, 0x00, 0x02, 0x02, 0x00, 0xbf, 0x00, /* ..... .. */
        0x01, 0x64,
    ];
    let decoded = UpdatePDPContextRequest {
        header: Gtpv1Header {
            msgtype: 18,
            length: 96,
            teid: 271310627,
            sequence_number: Some(49286),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            t: 2,
            imsi: "901405102776539".to_string(),
        }),
        rai: Some(Rai {
            t: 3,
            mcc: 226,
            mnc: 10,
            mnc_is_three_digits: false,
            lac: 65534,
            rac: 255,
        }),
        recovery: Some(Recovery { t: 14, value: 191 }),
        teid_data: Teid {
            t: 16,
            teid: 792477895,
        },
        teid_control: Some(Teid {
            t: 17,
            teid: 792477896,
        }),
        nsapi: Nsapi { t: 20, value: 5 },
        trace_ref: None,
        trace_type: None,
        pco: None,
        sgsn_ip_control: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 4)),
        },
        sgsn_ip_user: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 40)),
        },
        alt_sgsn_ip_control: None,
        alt_sgsn_ip_user: None,
        qos: Qos {
            t: 135,
            length: 17,
            arp: 3,
            qos: vec![
                35, 115, 31, 147, 150, 134, 134, 116, 131, 255, 255, 0, 0, 0, 0, 0,
            ],
        },
        tft: None,
        trigger_id: None,
        omc_id: None,
        common_flags: Some(CommonFlags {
            t: 148,
            length: 1,
            dual_addr_bearer: false,
            upgrade_qos_support: true,
            nrsn: false,
            no_qos_negotiation: false,
            mbms_counting_info: false,
            ran_procedures_ready: false,
            mbms_service_type: false,
            prohibit_payload_compr: false,
        }),
        rat_type: Some(RatType {
            t: 151,
            length: 1,
            rat_type: Rat::Utran,
        }),
        uli: Some(Uli {
            t: 152,
            length: 8,
            mcc: 226,
            mnc: 10,
            mnc_is_three_digits: false,
            lac: 1620,
            loc: Location::Sac(15529),
        }),
        ms_timezone: Some(MsTimeZone {
            t: 153,
            length: 2,
            time_zone: 2,
            dst: 0,
        }),
        imei: None,
        add_trace_info: None,
        dtf: None,
        evolved_alloc: Some(EvolvedAllocationRetentionI {
            t: 191,
            length: 1,
            pre_emption_vulnerability: 0,
            priority_level: 9,
            pre_emption_capability: 1,
        }),
        ext_common_flags: None,
        user_csg_info: None,
        apn_ambr: None,
        signalling_prio: None,
        cnose: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn update_pdp_ctx_req_unmarshal_with_alt_sgsn_addr_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 118] = [
        0x32, 0x12, 0x00, 0x6E, 0x10, 0x2b, /* 6.2..`.+ */
        0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, /* .#...... */
        0x41, 0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x03, /* AP.rg5.. */
        0x22, 0xf6, 0x01, 0xff, 0xfe, 0xff, 0x0e, 0xbf, /* "....... */
        0x10, 0x2f, 0x3c, 0x40, 0xc7, 0x11, 0x2f, 0x3c, /* ./<@../< */
        0x40, 0xc8, 0x14, 0x05, 0x85, 0x00, 0x04, 0x3e, /* @......> */
        0xd9, 0xc8, 0x04, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x28, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x29, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x2a, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, /* .(....#s */
        0x1f, 0x93, 0x96, 0x86, 0x86, 0x74, 0x83, 0xff, /* .....t.. */
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, /* ........ */
        0x01, 0x40, 0x97, 0x00, 0x01, 0x01, 0x98, 0x00, /* .@...... */
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, /* .."...T< */
        0xa9, 0x99, 0x00, 0x02, 0x02, 0x00, 0xbf, 0x00, /* ..... .. */
        0x01, 0x64,
    ];
    let decoded = UpdatePDPContextRequest {
        header: Gtpv1Header {
            msgtype: 18,
            length: 110,
            teid: 271310627,
            sequence_number: Some(49286),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            t: 2,
            imsi: "901405102776539".to_string(),
        }),
        rai: Some(Rai {
            t: 3,
            mcc: 226,
            mnc: 10,
            mnc_is_three_digits: false,
            lac: 65534,
            rac: 255,
        }),
        recovery: Some(Recovery { t: 14, value: 191 }),
        teid_data: Teid {
            t: 16,
            teid: 792477895,
        },
        teid_control: Some(Teid {
            t: 17,
            teid: 792477896,
        }),
        nsapi: Nsapi { t: 20, value: 5 },
        trace_ref: None,
        trace_type: None,
        pco: None,
        sgsn_ip_control: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 4)),
        },
        sgsn_ip_user: GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 40)),
        },
        alt_sgsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 41)),
        }),
        alt_sgsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 217, 200, 42)),
        }),
        qos: Qos {
            t: 135,
            length: 17,
            arp: 3,
            qos: vec![
                35, 115, 31, 147, 150, 134, 134, 116, 131, 255, 255, 0, 0, 0, 0, 0,
            ],
        },
        tft: None,
        trigger_id: None,
        omc_id: None,
        common_flags: Some(CommonFlags {
            t: 148,
            length: 1,
            dual_addr_bearer: false,
            upgrade_qos_support: true,
            nrsn: false,
            no_qos_negotiation: false,
            mbms_counting_info: false,
            ran_procedures_ready: false,
            mbms_service_type: false,
            prohibit_payload_compr: false,
        }),
        rat_type: Some(RatType {
            t: 151,
            length: 1,
            rat_type: Rat::Utran,
        }),
        uli: Some(Uli {
            t: 152,
            length: 8,
            mcc: 226,
            mnc: 10,
            mnc_is_three_digits: false,
            lac: 1620,
            loc: Location::Sac(15529),
        }),
        ms_timezone: Some(MsTimeZone {
            t: 153,
            length: 2,
            time_zone: 2,
            dst: 0,
        }),
        imei: None,
        add_trace_info: None,
        dtf: None,
        evolved_alloc: Some(EvolvedAllocationRetentionI {
            t: 191,
            length: 1,
            pre_emption_vulnerability: 0,
            priority_level: 9,
            pre_emption_capability: 1,
        }),
        ext_common_flags: None,
        user_csg_info: None,
        apn_ambr: None,
        signalling_prio: None,
        cnose: None,
        private_extension: None,
    };
    assert_eq!(
        UpdatePDPContextRequest::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]

fn update_pdp_ctx_req_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 104] = [
        0x32, 0x12, 0x00, 0x60, 0x10, 0x2b, /* 6.2..`.+ */
        0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, /* .#...... */
        0x41, 0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x03, /* AP.rg5.. */
        0x22, 0xf6, 0x01, 0xff, 0xfe, 0xff, 0x0e, 0xbf, /* "....... */
        0x11, 0x2f, 0x3c, 0x40, 0xc8, 0x10, 0x2f, 0x3c, 0x40, 0xc7, 0x14, 0x05, 0x85, 0x00, 0x04,
        0x3e, /* @......> */
        0xd9, 0xc8, 0x04, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x28, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, /* .(....#s */
        0x1f, 0x93, 0x96, 0x86, 0x86, 0x74, 0x83, 0xff, /* .....t.. */
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, /* ........ */
        0x01, 0x40, 0x97, 0x00, 0x01, 0x01, 0x98, 0x00, /* .@...... */
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, /* .."...T< */
        0xa9, 0x99, 0x00, 0x02, 0x02, 0x00, 0xbf, 0x00, /* ..... .. */
        0x01, 0x64,
    ];
    assert_eq!(
        UpdatePDPContextRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]

fn update_pdp_ctx_req_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 102] = [
        0x32, 0x12, 0x00, 0x5E, 0x10, 0x2b, /* 6.2..`.+ */
        0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, /* .#...... */
        0x41, 0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x03, /* AP.rg5.. */
        0x22, 0xf6, 0x01, 0xff, 0xfe, 0xff, 0x0e, 0xbf, /* "....... */
        0x10, 0x2f, 0x3c, 0x40, 0xc7, 0x11, 0x2f, 0x3c, /* ./<@../< */
        0x40, 0xc8, 0x85, 0x00, 0x04, 0x3e, /* @......> */
        0xd9, 0xc8, 0x04, 0x85, 0x00, 0x04, 0x3e, 0xd9, /* ......>. */
        0xc8, 0x28, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, /* .(....#s */
        0x1f, 0x93, 0x96, 0x86, 0x86, 0x74, 0x83, 0xff, /* .....t.. */
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, /* ........ */
        0x01, 0x40, 0x97, 0x00, 0x01, 0x01, 0x98, 0x00, /* .@...... */
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, /* .."...T< */
        0xa9, 0x99, 0x00, 0x02, 0x02, 0x00, 0xbf, 0x00, /* ..... .. */
        0x01, 0x64,
    ];
    assert_eq!(
        UpdatePDPContextRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
