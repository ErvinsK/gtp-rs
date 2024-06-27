use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const UPDATE_PDP_CONTEXT_RESPONSE: u8 = 19;

// Definition of GTPv1-C Update PDP Context Response

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePDPContextResponse {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub teid_data: Option<Teid>,
    pub teid_control: Option<Teid>,
    pub charging_id: Option<ChargingID>,
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
    pub csg_info_report: Option<CSGInformationReportingAction>,
    pub apn_ambr: Option<ApnAmbr>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for UpdatePDPContextResponse {
    fn default() -> UpdatePDPContextResponse {
        let hdr = Gtpv1Header {
            msgtype: UPDATE_PDP_CONTEXT_RESPONSE,
            ..Default::default()
        };
        UpdatePDPContextResponse {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            teid_data: None,
            teid_control: None,
            charging_id: None,
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
            csg_info_report: None,
            apn_ambr: None,
            private_extension: None,
        }
    }
}

impl Messages for UpdatePDPContextResponse {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Cause IE

        self.cause.marshal(buffer);

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

        // Marshal Charging ID IE

        if let Some(i) = self.charging_id {
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

        // Marshal CSG Information Reporting Action IE

        if let Some(i) = self.csg_info_report {
            i.marshal(buffer)
        };

        // Marshal APN-AMBR IE

        if let Some(i) = self.apn_ambr {
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

        let mut message = UpdatePDPContextResponse::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UPDATE_PDP_CONTEXT_RESPONSE {
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
fn update_pdp_ctx_resp_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 60] = [
        0x32, 0x13, 0x00, 0x34, 0x37, 0x38, /* ..2..478 */
        0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x01, 0x80, /* .z...... */
        0x0e, 0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x11, /* .....I.. */
        0x09, 0x86, 0xbb, 0x9f, 0x7f, 0x03, 0x94, 0x38, /* .......8 */
        0x7d, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x5f, /* }...>.._ */
        0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, /* ...>..`. */
        0x00, 0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, /* ......q. */
        0x87, 0x87, 0x74, 0xfa, 0xff, 0xff,
    ];
    let decoded = UpdatePDPContextResponse {
        header: Gtpv1Header {
            msgtype: 19,
            length: 52,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        recovery: Some(Recovery { t: 14, value: 5 }),
        teid_data: Some(Teid {
            t: 16,
            teid: 2794932724,
        }),
        teid_control: Some(Teid {
            t: 17,
            teid: 159824799,
        }),
        charging_id: Some(ChargingID {
            t: 127,
            value: 60045437,
        }),
        pco: None,
        ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 95)),
        }),
        ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        }),
        alt_ggsn_ip_control: None,
        alt_ggsn_ip_user: None,
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![19, 131, 31, 113, 150, 135, 135, 116, 250, 255, 255],
        }),
        charging_gw_addr: None,
        alt_charging_gw_addr: None,
        common_flags: None,
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
        evolved_alloc: None,
        csg_info_report: None,
        apn_ambr: None,
        private_extension: None,
    };
    assert_eq!(
        UpdatePDPContextResponse::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn update_pdp_ctx_response_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 60] = [
        0x32, 0x13, 0x00, 0x34, 0x37, 0x38, /* ..2..478 */
        0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x01, 0x80, /* .z...... */
        0x0e, 0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x11, /* .....I.. */
        0x09, 0x86, 0xbb, 0x9f, 0x7f, 0x03, 0x94, 0x38, /* .......8 */
        0x7d, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x5f, /* }...>.._ */
        0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, /* ...>..`. */
        0x00, 0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, /* ......q. */
        0x87, 0x87, 0x74, 0xfa, 0xff, 0xff,
    ];
    let decoded = UpdatePDPContextResponse {
        header: Gtpv1Header {
            msgtype: 19,
            length: 52,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        recovery: Some(Recovery { t: 14, value: 5 }),
        teid_data: Some(Teid {
            t: 16,
            teid: 2794932724,
        }),
        teid_control: Some(Teid {
            t: 17,
            teid: 159824799,
        }),
        charging_id: Some(ChargingID {
            t: 127,
            value: 60045437,
        }),
        pco: None,
        ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 95)),
        }),
        ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        }),
        alt_ggsn_ip_control: None,
        alt_ggsn_ip_user: None,
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![19, 131, 31, 113, 150, 135, 135, 116, 250, 255, 255],
        }),
        charging_gw_addr: None,
        alt_charging_gw_addr: None,
        common_flags: None,
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
        evolved_alloc: None,
        csg_info_report: None,
        apn_ambr: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn create_pdp_ctx_resp_unmarshal_with_repetitive_fields_ggsn_addr_charging_gw() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 88] = [
        0x32, 0x13, 0x00, 0x50, 0x37, 0x38, /* ..2..478 */
        0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x01, 0x80, /* .z...... */
        0x0e, 0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x11, /* .....I.. */
        0x09, 0x86, 0xbb, 0x9f, 0x7f, 0x03, 0x94, 0x38, /* .......8 */
        0x7d, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x5f, /* }...>.._ */
        0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x61, 0x85,
        0x00, 0x04, 0x3e, 0x99, 0x89, 0x62, 0x87, /* ...>..`. */
        0x00, 0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, /* ......q. */
        0x87, 0x87, 0x74, 0xfa, 0xff, 0xff, 0xfb, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4e, 0xfb, 0x00, 0x04, 0x3e, 0x99, 0x89, /* .A...>.. */
        0x4f,
    ];
    let decoded = UpdatePDPContextResponse {
        header: Gtpv1Header {
            msgtype: 19,
            length: 80,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause { t: 1, value: 128 },
        recovery: Some(Recovery { t: 14, value: 5 }),
        teid_data: Some(Teid {
            t: 16,
            teid: 2794932724,
        }),
        teid_control: Some(Teid {
            t: 17,
            teid: 159824799,
        }),
        charging_id: Some(ChargingID {
            t: 127,
            value: 60045437,
        }),
        pco: None,
        ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 95)),
        }),
        ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        }),
        alt_ggsn_ip_control: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 97)),
        }),
        alt_ggsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 98)),
        }),
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![19, 131, 31, 113, 150, 135, 135, 116, 250, 255, 255],
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
        csg_info_report: None,
        apn_ambr: None,
        private_extension: None,
    };
    assert_eq!(
        UpdatePDPContextResponse::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn update_pdp_ctx_resp_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 60] = [
        0x32, 0x13, 0x00, 0x34, 0x37, 0x38, /* ..2..478 */
        0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x01, 0x80, /* .z...... */
        0x0e, 0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x11, /* .....I.. */
        0x09, 0x86, 0xbb, 0x9f, 0x7f, 0x03, 0x94, 0x38, /* .......8 */
        0x7d, 0x87, /* ...>..`. */
        0x00, 0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, /* ......q. */
        0x87, 0x87, 0x74, 0xfa, 0xff, 0xff, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89,
        0x5f, /* }...>.._ */
        0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60,
    ];
    assert_eq!(
        UpdatePDPContextResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn update_pdp_ctx_resp_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 58] = [
        0x32, 0x13, 0x00, 0x32, 0x37, 0x38, /* ..2..478 */
        0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, /* .z...... */
        0x0e, 0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x11, /* .....I.. */
        0x09, 0x86, 0xbb, 0x9f, 0x7f, 0x03, 0x94, 0x38, /* .......8 */
        0x7d, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x5f, /* }...>.._ */
        0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, /* ...>..`. */
        0x00, 0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, /* ......q. */
        0x87, 0x87, 0x74, 0xfa, 0xff, 0xff,
    ];
    assert_eq!(
        UpdatePDPContextResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
