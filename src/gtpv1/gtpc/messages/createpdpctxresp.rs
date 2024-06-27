use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const CREATE_PDP_CONTEXT_RESPONSE: u8 = 17;

// Definition of GTPv1-C Create PDP Context Response

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePDPContextResponse {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub reordering_req: Option<ReorderingRequired>,
    pub recovery: Option<Recovery>,
    pub teid_data: Option<Teid>,
    pub teid_control: Option<Teid>,
    pub nsapi: Option<Nsapi>,
    pub charging_id: Option<ChargingID>,
    pub end_user_address: Option<EndUserAddress>,
    pub pco: Option<Pco>,
    pub ggsn_ip_control: Option<GsnAddress>,
    pub ggsn_ip_user: Option<GsnAddress>,
    pub alt_ggsn_ip_control: Option<GsnAddress>,
    pub alt_ggsn_ip_user: Option<GsnAddress>,
    pub qos: Option<Qos>,
    pub charging_gw_addr: Option<ChargingGWAddress>,
    pub alt_charging_gw_addr: Option<ChargingGWAddress>,
    pub common_flags: Option<CommonFlags>,
    pub apn_restriction: Option<ApnRestriction>,
    pub ms_info_change: Option<MSInfoChangeReportingAction>,
    pub bearer_ctrl_mode: Option<BearerControlMode>,
    pub evolved_alloc: Option<EvolvedAllocationRetentionI>,
    pub ext_common_flags: Option<ExtendedCommonFlags>,
    pub csg_info_report: Option<CSGInformationReportingAction>,
    pub apn_ambr: Option<ApnAmbr>,
    pub ggsn_backoff_time: Option<GGSNBackOffTime>,
    pub ext_common_flags_ii: Option<ExtendedCommonFlagsII>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for CreatePDPContextResponse {
    fn default() -> CreatePDPContextResponse {
        let hdr = Gtpv1Header {
            msgtype: CREATE_PDP_CONTEXT_RESPONSE,
            ..Default::default()
        };
        CreatePDPContextResponse {
            header: hdr,
            cause: Cause::default(),
            reordering_req: None,
            recovery: None,
            teid_data: None,
            teid_control: None,
            nsapi: None,
            charging_id: None,
            end_user_address: None,
            pco: None,
            ggsn_ip_control: None,
            ggsn_ip_user: None,
            alt_ggsn_ip_control: None,
            alt_ggsn_ip_user: None,
            qos: None,
            charging_gw_addr: None,
            alt_charging_gw_addr: None,
            common_flags: None,
            apn_restriction: None,
            ms_info_change: None,
            bearer_ctrl_mode: None,
            evolved_alloc: None,
            ext_common_flags: None,
            csg_info_report: None,
            apn_ambr: None,
            ggsn_backoff_time: None,
            ext_common_flags_ii: None,
            private_extension: None,
        }
    }
}

impl Messages for CreatePDPContextResponse {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Cause IE

        self.cause.marshal(buffer);

        // Marshal Reordering Required IE

        if let Some(i) = self.reordering_req {
            i.marshal(buffer)
        };

        // Marshal Recovery IE

        if let Some(i) = self.recovery {
            i.marshal(buffer)
        };

        // Marshal TEID Data I IE

        if let Some(i) = self.teid_data {
            i.marshal(buffer)
        };

        // Marshal TEID Control IE

        if let Some(i) = self.teid_control {
            i.marshal(buffer)
        };

        // Marshal NSAPI IE

        if let Some(i) = self.nsapi {
            i.marshal(buffer)
        };

        // Marshal Charging ID IE

        if let Some(i) = self.charging_id {
            i.marshal(buffer)
        };

        // Marshal End User Address IE

        if let Some(i) = self.end_user_address {
            i.marshal(buffer)
        };

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal GGSN Address for Signalling IE

        if let Some(i) = self.ggsn_ip_control {
            i.marshal(buffer)
        };

        // Marshal GGSN Address for User plane IE

        if let Some(i) = self.ggsn_ip_user {
            i.marshal(buffer)
        };

        // Marshal Alternative GGSN Address for Signalling IE

        if let Some(i) = self.alt_ggsn_ip_control {
            i.marshal(buffer)
        };

        // Marshal Alternative GGSN Address for User plane IE

        if let Some(i) = self.alt_ggsn_ip_user {
            i.marshal(buffer)
        };

        // Marshal QoS IE

        if let Some(i) = self.qos {
            i.marshal(buffer)
        };

        // Marshal Charging GW Address IE

        if let Some(i) = self.charging_gw_addr {
            i.marshal(buffer)
        };

        // Marshal Alternative Charging GW Address IE

        if let Some(i) = self.alt_charging_gw_addr {
            i.marshal(buffer)
        };

        // Marshal Common Flags IE

        if let Some(i) = self.common_flags {
            i.marshal(buffer)
        };

        // Marshal APN Restriction IE

        if let Some(i) = self.apn_restriction {
            i.marshal(buffer)
        };

        // Marshal MS Info Change Reporting Action IE

        if let Some(i) = self.ms_info_change {
            i.marshal(buffer)
        };

        // Marshal Bearer Control Mode IE

        if let Some(i) = self.bearer_ctrl_mode {
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

        // Marshal CSG Information Reporting Action IE

        if let Some(i) = self.csg_info_report {
            i.marshal(buffer)
        };

        // Marshal APN-AMBR IE

        if let Some(i) = self.apn_ambr {
            i.marshal(buffer)
        };

        // Marshal GGSN Back-Off Time IE

        if let Some(i) = self.ggsn_backoff_time {
            i.marshal(buffer)
        };

        // Marshal Extended Common Flags II IE

        if let Some(i) = self.ext_common_flags_ii {
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

        let mut message = CreatePDPContextResponse::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_PDP_CONTEXT_RESPONSE {
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
                        CAUSE => match Cause::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.cause = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        REORDERING_REQUIRED => {
                            match ReorderingRequired::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.reordering_req = Some(i);
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
                                    message.teid_data = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
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
                                    message.nsapi = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        CHARGING_ID => match ChargingID::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.charging_id = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        END_USER_ADDRESS => match EndUserAddress::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.end_user_address = Some(i);
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
                                    message.ggsn_ip_control = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap();
                                    match n {
                                        1 => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.ggsn_ip_user = Some(i);
                                        }
                                        2 => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.alt_ggsn_ip_control = Some(i);
                                        }
                                        3 => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.alt_ggsn_ip_user = Some(i);
                                        }
                                        _ => {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                        }
                                    }
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        QOS => match Qos::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.qos = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        CHARGING_GW_ADDRESS => {
                            match ChargingGWAddress::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.charging_gw_addr = Some(i);
                                    } else {
                                        let n = *msg_hash.get(&buffer[cursor]).unwrap();
                                        if n < 2 {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                            message.alt_charging_gw_addr = Some(i);
                                        } else {
                                            msg_hash.insert(buffer[cursor], n + 1);
                                            increment = buffer[cursor];
                                            cursor += i.len();
                                        }
                                    }
                                }
                                Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                            }
                        }
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
                        APNRESTRICTION => match ApnRestriction::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.apn_restriction = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        MSINFO_CHANGE => {
                            match MSInfoChangeReportingAction::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.ms_info_change = Some(i);
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
                        BEARER_CONTROL_MODE => {
                            match BearerControlMode::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.bearer_ctrl_mode = Some(i);
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
                        CSG_INFO_REPORT => {
                            match CSGInformationReportingAction::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.csg_info_report = Some(i);
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
                        GGSN_BACKOFF => match GGSNBackOffTime::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.ggsn_backoff_time = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        EXTCOMMONFLAGS_II => {
                            match ExtendedCommonFlagsII::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.ext_common_flags_ii = Some(i);
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
            if msg_hash.contains_key(&CAUSE) {
                Ok(message)
            } else {
                Err(GTPV1Error::MessageMandatoryIEMissing)
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]

fn create_pdp_ctx_resp_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 94] = [
        0x32, 0x11, 0x00, 0x56, 0x70, 0x0b, /* ..2..Vp. */
        0x0c, 0x60, 0x74, 0x17, 0x00, 0x00, 0x01, 0x80, /* .`t..... */
        0x08, 0xfe, 0x0e, 0x06, 0x10, 0xf3, 0xc3, 0xe7, /* ........ */
        0xf9, 0x11, 0x1f, 0x4b, 0xf2, 0xf4, 0x7f, 0x05, /* ...K.... */
        0xeb, 0x6b, 0xb3, 0x80, 0x00, 0x06, 0xf1, 0x21, /* .k.....! */
        0x0a, 0xdb, 0x3b, 0x30, 0x84, 0x00, 0x14, 0x80, /* ..;0.... */
        0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, /* .!...... */
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x08, /* ........ */
        0x08, 0x04, 0x04, 0x85, 0x00, 0x04, 0x3e, 0x99, /* ......>. */
        0x89, 0x41, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4b, 0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, /* K....... */
        0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40, /* s...D..@ */
    ];
    let decoded = CreatePDPContextResponse {
        header: Gtpv1Header {
            msgtype: 17,
            length: 86,
            teid: 1879772256,
            sequence_number: Some(29719),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        reordering_req: Some(ReorderingRequired { t: 8, req: false }),
        recovery: Some(Recovery { t: 14, value: 6 }),
        teid_data: Some(Teid {
            t: 16,
            teid: 4089702393,
        }),
        teid_control: Some(Teid {
            t: 17,
            teid: 525071092,
        }),
        nsapi: None,
        charging_id: Some(ChargingID {
            t: 127,
            value: 99314611,
        }),
        end_user_address: Some(EndUserAddress {
            t: 128,
            length: 6,
            pdp_type_org: 1,
            pdp_type_nbr: 33,
            ipv4: Some(Ipv4Addr::new(10, 219, 59, 48)),
            ipv6: None,
        }),
        pco: Some(Pco {
            t: 132,
            length: 20,
            pco: vec![
                128, 128, 33, 16, 2, 0, 0, 16, 129, 6, 8, 8, 8, 8, 131, 6, 8, 8, 4, 4,
            ],
        }),
        ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 65)),
        }),
        ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 75)),
        }),
        alt_ggsn_ip_control: None,
        alt_ggsn_ip_user: None,
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![27, 147, 31, 115, 150, 151, 151, 68, 251, 16, 64],
        }),
        charging_gw_addr: None,
        alt_charging_gw_addr: None,
        common_flags: None,
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
        evolved_alloc: None,
        ext_common_flags: None,
        csg_info_report: None,
        apn_ambr: None,
        ggsn_backoff_time: None,
        ext_common_flags_ii: None,
        private_extension: None,
    };
    assert_eq!(
        CreatePDPContextResponse::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn create_pdp_ctx_response_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 94] = [
        0x32, 0x11, 0x00, 0x56, 0x70, 0x0b, /* ..2..Vp. */
        0x0c, 0x60, 0x74, 0x17, 0x00, 0x00, 0x01, 0x80, /* .`t..... */
        0x08, 0xfe, 0x0e, 0x06, 0x10, 0xf3, 0xc3, 0xe7, /* ........ */
        0xf9, 0x11, 0x1f, 0x4b, 0xf2, 0xf4, 0x7f, 0x05, /* ...K.... */
        0xeb, 0x6b, 0xb3, 0x80, 0x00, 0x06, 0xf1, 0x21, /* .k.....! */
        0x0a, 0xdb, 0x3b, 0x30, 0x84, 0x00, 0x14, 0x80, /* ..;0.... */
        0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, /* .!...... */
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x08, /* ........ */
        0x08, 0x04, 0x04, 0x85, 0x00, 0x04, 0x3e, 0x99, /* ......>. */
        0x89, 0x41, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4b, 0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, /* K....... */
        0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40, /* s...D..@ */
    ];
    let decoded = CreatePDPContextResponse {
        header: Gtpv1Header {
            msgtype: 17,
            length: 86,
            teid: 1879772256,
            sequence_number: Some(29719),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        reordering_req: Some(ReorderingRequired { t: 8, req: false }),
        recovery: Some(Recovery { t: 14, value: 6 }),
        teid_data: Some(Teid {
            t: 16,
            teid: 4089702393,
        }),
        teid_control: Some(Teid {
            t: 17,
            teid: 525071092,
        }),
        nsapi: None,
        charging_id: Some(ChargingID {
            t: 127,
            value: 99314611,
        }),
        end_user_address: Some(EndUserAddress {
            t: 128,
            length: 6,
            pdp_type_org: 1,
            pdp_type_nbr: 33,
            ipv4: Some(Ipv4Addr::new(10, 219, 59, 48)),
            ipv6: None,
        }),
        pco: Some(Pco {
            t: 132,
            length: 20,
            pco: vec![
                128, 128, 33, 16, 2, 0, 0, 16, 129, 6, 8, 8, 8, 8, 131, 6, 8, 8, 4, 4,
            ],
        }),
        ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 65)),
        }),
        ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 75)),
        }),
        alt_ggsn_ip_control: None,
        alt_ggsn_ip_user: None,
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![27, 147, 31, 115, 150, 151, 151, 68, 251, 16, 64],
        }),
        charging_gw_addr: None,
        alt_charging_gw_addr: None,
        common_flags: None,
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
        evolved_alloc: None,
        ext_common_flags: None,
        csg_info_report: None,
        apn_ambr: None,
        ggsn_backoff_time: None,
        ext_common_flags_ii: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn create_pdp_ctx_resp_unmarshal_with_repetitive_fields_ggsn_addr_charging_gw() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 122] = [
        0x32, 0x11, 0x00, 0x72, 0x70, 0x0b, /* ..2..Vp. */
        0x0c, 0x60, 0x74, 0x17, 0x00, 0x00, 0x01, 0x80, /* .`t..... */
        0x08, 0xfe, 0x0e, 0x06, 0x10, 0xf3, 0xc3, 0xe7, /* ........ */
        0xf9, 0x11, 0x1f, 0x4b, 0xf2, 0xf4, 0x7f, 0x05, /* ...K.... */
        0xeb, 0x6b, 0xb3, 0x80, 0x00, 0x06, 0xf1, 0x21, /* .k.....! */
        0x0a, 0xdb, 0x3b, 0x30, 0x84, 0x00, 0x14, 0x80, /* ..;0.... */
        0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, /* .!...... */
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x08, /* ........ */
        0x08, 0x04, 0x04, 0x85, 0x00, 0x04, 0x3e, 0x99, /* ......>. */
        0x89, 0x41, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4b, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4c, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4d, 0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, /* K....... */
        0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40, /* s...D..@ */
        0xfb, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4e, 0xfb, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4f,
    ];
    let decoded = CreatePDPContextResponse {
        header: Gtpv1Header {
            msgtype: 17,
            length: 114,
            teid: 1879772256,
            sequence_number: Some(29719),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        reordering_req: Some(ReorderingRequired { t: 8, req: false }),
        recovery: Some(Recovery { t: 14, value: 6 }),
        teid_data: Some(Teid {
            t: 16,
            teid: 4089702393,
        }),
        teid_control: Some(Teid {
            t: 17,
            teid: 525071092,
        }),
        nsapi: None,
        charging_id: Some(ChargingID {
            t: 127,
            value: 99314611,
        }),
        end_user_address: Some(EndUserAddress {
            t: 128,
            length: 6,
            pdp_type_org: 1,
            pdp_type_nbr: 33,
            ipv4: Some(Ipv4Addr::new(10, 219, 59, 48)),
            ipv6: None,
        }),
        pco: Some(Pco {
            t: 132,
            length: 20,
            pco: vec![
                128, 128, 33, 16, 2, 0, 0, 16, 129, 6, 8, 8, 8, 8, 131, 6, 8, 8, 4, 4,
            ],
        }),
        ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 65)),
        }),
        ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 75)),
        }),
        alt_ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 76)),
        }),
        alt_ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 77)),
        }),
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![27, 147, 31, 115, 150, 151, 151, 68, 251, 16, 64],
        }),
        charging_gw_addr: Some(ChargingGWAddress {
            t: CHARGING_GW_ADDRESS,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 78)),
        }),
        alt_charging_gw_addr: Some(ChargingGWAddress {
            t: CHARGING_GW_ADDRESS,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 79)),
        }),
        common_flags: None,
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
        evolved_alloc: None,
        ext_common_flags: None,
        csg_info_report: None,
        apn_ambr: None,
        ggsn_backoff_time: None,
        ext_common_flags_ii: None,
        private_extension: None,
    };
    assert_eq!(
        CreatePDPContextResponse::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn create_pdp_ctx_resp_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 94] = [
        0x32, 0x11, 0x00, 0x56, 0x70, 0x0b, /* ..2..Vp. */
        0x0c, 0x60, 0x74, 0x17, 0x00, 0x00, /* .`t..... */
        0x08, 0xfe, 0x01, 0x80, 0x0e, 0x06, 0x10, 0xf3, 0xc3, 0xe7, /* ........ */
        0xf9, 0x11, 0x1f, 0x4b, 0xf2, 0xf4, 0x7f, 0x05, /* ...K.... */
        0xeb, 0x6b, 0xb3, 0x80, 0x00, 0x06, 0xf1, 0x21, /* .k.....! */
        0x0a, 0xdb, 0x3b, 0x30, 0x84, 0x00, 0x14, 0x80, /* ..;0.... */
        0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, /* .!...... */
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x08, /* ........ */
        0x08, 0x04, 0x04, 0x85, 0x00, 0x04, 0x3e, 0x99, /* ......>. */
        0x89, 0x41, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4b, 0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, /* K....... */
        0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40, /* s...D..@ */
    ];
    assert_eq!(
        CreatePDPContextResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn create_pdp_ctx_resp_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 92] = [
        0x32, 0x11, 0x00, 0x54, 0x70, 0x0b, /* ..2..Vp. */
        0x0c, 0x60, 0x74, 0x17, 0x00, 0x00, /* .`t..... */
        0x08, 0xfe, 0x0e, 0x06, 0x10, 0xf3, 0xc3, 0xe7, /* ........ */
        0xf9, 0x11, 0x1f, 0x4b, 0xf2, 0xf4, 0x7f, 0x05, /* ...K.... */
        0xeb, 0x6b, 0xb3, 0x80, 0x00, 0x06, 0xf1, 0x21, /* .k.....! */
        0x0a, 0xdb, 0x3b, 0x30, 0x84, 0x00, 0x14, 0x80, /* ..;0.... */
        0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, /* .!...... */
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x08, /* ........ */
        0x08, 0x04, 0x04, 0x85, 0x00, 0x04, 0x3e, 0x99, /* ......>. */
        0x89, 0x41, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4b, 0x87, 0x00, 0x0c, 0x03, 0x1b, 0x93, 0x1f, /* K....... */
        0x73, 0x96, 0x97, 0x97, 0x44, 0xfb, 0x10, 0x40, /* s...D..@ */
    ];
    assert_eq!(
        CreatePDPContextResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
