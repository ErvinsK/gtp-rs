use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

// Definition of GTPv1-C GGSN initiated Update PDP Context Request

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePDPContextRequestGGSN {
    pub header: Gtpv1Header,
    pub imsi: Option<Imsi>,
    pub recovery: Option<Recovery>,
    pub nsapi: Nsapi,
    pub end_user_address: Option<EndUserAddress>,
    pub pco: Option<Pco>,
    pub qos: Option<Qos>,
    pub tft: Option<Tft>,
    pub common_flags: Option<CommonFlags>,
    pub apn_restriction: Option<ApnRestriction>,
    pub ms_info_change: Option<MSInfoChangeReportingAction>,
    pub dtf: Option<DirectTunnelFlags>,
    pub bearer_ctrl_mode: Option<BearerControlMode>,
    pub evolved_alloc: Option<EvolvedAllocationRetentionI>,
    pub ext_common_flags: Option<ExtendedCommonFlags>,
    pub user_csg_info: Option<CSGInformationReportingAction>,
    pub apn_ambr: Option<ApnAmbr>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for UpdatePDPContextRequestGGSN {
    fn default() -> UpdatePDPContextRequestGGSN {
        let hdr = Gtpv1Header {
            msgtype: UPDATE_PDP_CONTEXT_REQUEST,
            ..Default::default()
        };
        UpdatePDPContextRequestGGSN {
            header: hdr,
            imsi: None,
            recovery: None,
            nsapi: Nsapi::default(),
            end_user_address: None,
            pco: None,
            qos: None,
            tft: None,
            common_flags: None,
            apn_restriction: None,
            ms_info_change: None,
            dtf: None,
            bearer_ctrl_mode: None,
            evolved_alloc: None,
            ext_common_flags: None,
            user_csg_info: None,
            apn_ambr: None,
            private_extension: None,
        }
    }
}

impl Messages for UpdatePDPContextRequestGGSN {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal IMSI IE

        if let Some(i) = self.imsi {
            i.marshal(buffer)
        };

        // Marshal Recovery IE

        if let Some(i) = self.recovery {
            i.marshal(buffer)
        };

        // Marshal NSAPI IE

        self.nsapi.marshal(buffer);

        // Marshal End User Address IE

        if let Some(i) = self.end_user_address {
            i.marshal(buffer)
        };

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal QoS IE

        if let Some(i) = self.qos {
            i.marshal(buffer)
        };

        // Marshal TFT IE

        if let Some(i) = self.tft {
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

        // Marshal MS Info Change Reporting IE

        if let Some(i) = self.ms_info_change {
            i.marshal(buffer)
        };

        // Marshal DTF IE

        if let Some(i) = self.dtf {
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

        // Marshal User CSG Information IE

        if let Some(i) = self.user_csg_info {
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

        let mut message = UpdatePDPContextRequestGGSN::default();

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
                                        message.user_csg_info = Some(i);
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
            if msg_hash.contains_key(&NSAPI) {
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
fn update_pdp_ctx_req_ggsn_unmarshal_test() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 78] = [
        0x32, 0x12, 0x00, 0x46, 0x10, 0x2b, 0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, 0x41,
        0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x0e, 0xbf, 0x14, 0x05, 0x80, 0x00, 0x16, 0xf1, 0x8d,
        0x64, 0x75, 0x82, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, 0x1f, 0x93, 0x96, 0x86,
        0x86, 0x74, 0x83, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, 0x01, 0x40, 0xbf,
        0x00, 0x01, 0x64,
    ];
    let decoded = UpdatePDPContextRequestGGSN {
        header: Gtpv1Header {
            msgtype: 18,
            length: 70,
            teid: 271310627,
            sequence_number: Some(49286),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            t: 2,
            imsi: "901405102776539".to_string(),
        }),
        recovery: Some(Recovery { t: 14, value: 191 }),
        nsapi: Nsapi { t: 20, value: 5 },
        end_user_address: Some(EndUserAddress {
            t: END_USER_ADDRESS,
            length: 22,
            pdp_type_org: IETF,
            pdp_type_nbr: IPV46,
            ipv4: Some(Ipv4Addr::new(100, 117, 130, 53)),
            ipv6: Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xffff, 0xffff)),
        }),
        pco: None,
        qos: Some(Qos {
            t: 135,
            length: 17,
            arp: 3,
            qos: vec![
                35, 115, 31, 147, 150, 134, 134, 116, 131, 255, 255, 0, 0, 0, 0, 0,
            ],
        }),
        tft: None,
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
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
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
        private_extension: None,
    };
    assert_eq!(
        UpdatePDPContextRequestGGSN::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn update_pdp_ctx_req_ggsn_marshal_test() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 78] = [
        0x32, 0x12, 0x00, 0x46, 0x10, 0x2b, 0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, 0x41,
        0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x0e, 0xbf, 0x14, 0x05, 0x80, 0x00, 0x16, 0xf1, 0x8d,
        0x64, 0x75, 0x82, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, 0x1f, 0x93, 0x96, 0x86,
        0x86, 0x74, 0x83, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, 0x01, 0x40, 0xbf,
        0x00, 0x01, 0x64,
    ];
    let decoded = UpdatePDPContextRequestGGSN {
        header: Gtpv1Header {
            msgtype: 18,
            length: 70,
            teid: 271310627,
            sequence_number: Some(49286),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            t: 2,
            imsi: "901405102776539".to_string(),
        }),
        recovery: Some(Recovery { t: 14, value: 191 }),
        nsapi: Nsapi { t: 20, value: 5 },
        end_user_address: Some(EndUserAddress {
            t: END_USER_ADDRESS,
            length: 22,
            pdp_type_org: IETF,
            pdp_type_nbr: IPV46,
            ipv4: Some(Ipv4Addr::new(100, 117, 130, 53)),
            ipv6: Some(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xffff, 0xffff)),
        }),
        pco: None,
        qos: Some(Qos {
            t: 135,
            length: 17,
            arp: 3,
            qos: vec![
                35, 115, 31, 147, 150, 134, 134, 116, 131, 255, 255, 0, 0, 0, 0, 0,
            ],
        }),
        tft: None,
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
        apn_restriction: None,
        ms_info_change: None,
        bearer_ctrl_mode: None,
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
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn update_pdp_ctx_req_ggsn_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 78] = [
        0x32, 0x12, 0x00, 0x46, 0x10, 0x2b, 0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, 0x41,
        0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x14, 0x05, 0x0e, 0xbf, 0x80, 0x00, 0x16, 0xf1, 0x8d,
        0x64, 0x75, 0x82, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, 0x1f, 0x93, 0x96, 0x86,
        0x86, 0x74, 0x83, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, 0x01, 0x40, 0xbf,
        0x00, 0x01, 0x64,
    ];
    assert_eq!(
        UpdatePDPContextRequestGGSN::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn update_pdp_ctx_req_ggsn_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 76] = [
        0x32, 0x12, 0x00, 0x44, 0x10, 0x2b, 0xdf, 0x23, 0xc0, 0x86, 0x00, 0x00, 0x02, 0x09, 0x41,
        0x50, 0x01, 0x72, 0x67, 0x35, 0xf9, 0x0e, 0xbf, 0x80, 0x00, 0x16, 0xf1, 0x8d, 0x64, 0x75,
        0x82, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xff, 0xff, 0x87, 0x00, 0x11, 0x03, 0x23, 0x73, 0x1f, 0x93, 0x96, 0x86, 0x86, 0x74,
        0x83, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x00, 0x01, 0x40, 0xbf, 0x00, 0x01,
        0x64,
    ];
    assert_eq!(
        UpdatePDPContextRequestGGSN::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
