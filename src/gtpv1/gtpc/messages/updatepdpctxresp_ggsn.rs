use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

// Definition of GTPv1-C GGSN-initiated Update PDP Context Response

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePDPContextResponseGGSN {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub teid_data: Option<Teid>,
    pub pco: Option<Pco>,
    pub sgsn_ip_user: Option<GsnAddress>,
    pub qos: Option<Qos>,
    pub uli: Option<Uli>,
    pub ms_timezone: Option<MsTimeZone>,
    pub dtf: Option<DirectTunnelFlags>,
    pub evolved_alloc: Option<EvolvedAllocationRetentionI>,
    pub apn_ambr: Option<ApnAmbr>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for UpdatePDPContextResponseGGSN {
    fn default() -> UpdatePDPContextResponseGGSN {
        let hdr = Gtpv1Header {
            msgtype: UPDATE_PDP_CONTEXT_RESPONSE,
            ..Default::default()
        };
        UpdatePDPContextResponseGGSN {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            teid_data: None,
            pco: None,
            sgsn_ip_user: None,
            qos: None,
            uli: None,
            ms_timezone: None,
            dtf: None,
            evolved_alloc: None,
            apn_ambr: None,
            private_extension: None,
        }
    }
}

impl Messages for UpdatePDPContextResponseGGSN {
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

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal SGSN Address for User plane IE

        if let Some(i) = self.sgsn_ip_user {
            i.marshal(buffer)
        };

        // Marshal QoS IE

        if let Some(i) = self.qos {
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

        // Marshal Direct Tunnel Flags IE

        if let Some(i) = self.dtf {
            i.marshal(buffer)
        };

        // Marshal Evolved Allocation/Retention Priority I IE

        if let Some(i) = self.evolved_alloc {
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

        let mut message = UpdatePDPContextResponseGGSN::default();

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
                                    message.sgsn_ip_user = Some(i);
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
fn update_pdp_ctx_resp_ggsn_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 59] = [
        0x32, 0x13, 0x00, 0x33, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x01, 0x80, 0x0e,
        0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, 0x00,
        0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, 0x87, 0x87, 0x74, 0xfa, 0xff, 0xff, 0x98, 0x00,
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, 0xa9, 0x99, 0x00, 0x02, 0x02, 0x00,
    ];
    let decoded = UpdatePDPContextResponseGGSN {
        header: Gtpv1Header {
            msgtype: 19,
            length: 51,
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
        pco: None,
        sgsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        }),
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![19, 131, 31, 113, 150, 135, 135, 116, 250, 255, 255],
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
        dtf: None,
        evolved_alloc: None,
        apn_ambr: None,
        private_extension: None,
    };
    assert_eq!(
        UpdatePDPContextResponseGGSN::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn update_pdp_ctx_response_ggsn_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 59] = [
        0x32, 0x13, 0x00, 0x33, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x01, 0x80, 0x0e,
        0x05, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, 0x00,
        0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, 0x87, 0x87, 0x74, 0xfa, 0xff, 0xff, 0x98, 0x00,
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, 0xa9, 0x99, 0x00, 0x02, 0x02, 0x00,
    ];
    let decoded = UpdatePDPContextResponseGGSN {
        header: Gtpv1Header {
            msgtype: 19,
            length: 51,
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
        pco: None,
        sgsn_ip_user: Some(GsnAddress {
            t: 133,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(62, 153, 137, 96)),
        }),
        qos: Some(Qos {
            t: 135,
            length: 12,
            arp: 3,
            qos: vec![19, 131, 31, 113, 150, 135, 135, 116, 250, 255, 255],
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
        dtf: None,
        evolved_alloc: None,
        apn_ambr: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn update_pdp_ctx_resp_ggsn_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 59] = [
        0x32, 0x13, 0x00, 0x33, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x0e, 0x05, 0x01,
        0x80, 0x10, 0xa6, 0x97, 0x49, 0xf4, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, 0x00,
        0x0c, 0x03, 0x13, 0x83, 0x1f, 0x71, 0x96, 0x87, 0x87, 0x74, 0xfa, 0xff, 0xff, 0x98, 0x00,
        0x08, 0x01, 0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, 0xa9, 0x99, 0x00, 0x02, 0x02, 0x00,
    ];
    assert_eq!(
        UpdatePDPContextResponseGGSN::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn update_pdp_ctx_resp_ggsn_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 57] = [
        0x32, 0x13, 0x00, 0x31, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x0e, 0x05, 0x10,
        0xa6, 0x97, 0x49, 0xf4, 0x85, 0x00, 0x04, 0x3e, 0x99, 0x89, 0x60, 0x87, 0x00, 0x0c, 0x03,
        0x13, 0x83, 0x1f, 0x71, 0x96, 0x87, 0x87, 0x74, 0xfa, 0xff, 0xff, 0x98, 0x00, 0x08, 0x01,
        0x22, 0xf6, 0x01, 0x06, 0x54, 0x3c, 0xa9, 0x99, 0x00, 0x02, 0x02, 0x00,
    ];
    assert_eq!(
        UpdatePDPContextResponseGGSN::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
