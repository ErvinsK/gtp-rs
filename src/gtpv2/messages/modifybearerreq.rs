use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MODIFY_BEARER_REQ: u8 = 34;

// Definition of GTPv2-C Modify Bearer Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyBearerRequest {
    pub header: Gtpv2Header,
    pub mei: Option<Mei>,
    pub uli: Option<Uli>,
    pub servingnetwork: Option<ServingNetwork>,
    pub rattype: Option<RatType>,
    pub indication: Option<Indication>,
    pub fteid_control: Option<Fteid>,
    pub apnambr: Option<Ambr>,
    pub delay_dl_pnr: Option<DelayValue>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub uetimezone: Option<UeTimeZone>,
    pub mme_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub uci: Option<Uci>,
    pub ue_localip: Option<IpAddress>,
    pub ue_udpport: Option<PortNumber>,
    pub mme_ldn: Option<Ldn>,
    pub sgw_ldn: Option<Ldn>,
    pub henb_localip: Option<IpAddress>,
    pub henb_udpport: Option<PortNumber>,
    pub mme_id: Option<IpAddress>,
    pub cnose: Option<CnOperatorSelectionEntity>,
    pub prai: Option<PresenceReportingAreaInformation>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub srv_plmn_rate_cntrl: Option<ServingPlmnRateControl>,
    pub mo_exception_data_counter: Option<Counter>,
    pub imsi: Option<Imsi>,
    pub uli_for_sgw: Option<Uli>,
    pub wlan_loc: Option<TwanId>,
    pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ModifyBearerRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MODIFY_BEARER_REQ,
            teid: Some(0),
            ..Default::default()
        };
        ModifyBearerRequest {
            header: hdr,
            mei: None,
            uli: None,
            servingnetwork: None,
            rattype: None,
            indication: None,
            fteid_control: None,
            apnambr: None,
            delay_dl_pnr: None,
            bearer_ctxs: vec![],
            recovery: None,
            uetimezone: None,
            mme_fqcsid: None,
            sgw_fqcsid: None,
            uci: None,
            ue_localip: None,
            ue_udpport: None,
            mme_ldn: None,
            sgw_ldn: None,
            henb_localip: None,
            henb_udpport: None,
            mme_id: None,
            cnose: None,
            prai: None,
            overload_info: vec![],
            srv_plmn_rate_cntrl: None,
            mo_exception_data_counter: None,
            imsi: None,
            uli_for_sgw: None,
            wlan_loc: None,
            wlan_loc_timestamp: None,
            secondary_rat_usage_report: vec![],
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ModifyBearerRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ModifyBearerRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MODIFY_BEARER_REQ {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        let offset = message.header.length as usize + MANDATORY_HDR_LENGTH;

        if buffer.len() >= offset {
            match InformationElement::decoder(&buffer[MAX_HEADER_LENGTH..offset]) {
                Ok(i) => match message.fromvec(i) {
                    Ok(_) => Ok(message),
                    Err(j) => Err(j),
                },
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }

    fn tovec(&self) -> Vec<InformationElement> {
        let mut elements: Vec<InformationElement> = vec![];

        if let Some(i) = self.mei.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.servingnetwork.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.rattype.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apnambr.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.delay_dl_pnr.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uetimezone.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uci.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_localip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ue_udpport.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_localip.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.henb_udpport.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.cnose.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.prai.clone() {
            elements.push(i.into())
        };

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.srv_plmn_rate_cntrl.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mo_exception_data_counter.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.uli_for_sgw.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.wlan_loc.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.wlan_loc_timestamp.clone() {
            elements.push(i.into())
        };

        self.secondary_rat_usage_report.iter().for_each(|x| {
            elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone()))
        });

        if let Some(i) = self.pscellid.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.iter() {
            match e {
                InformationElement::Mei(j) => {
                    if let (0, true) = (j.ins, self.mei.is_none()) {
                        self.mei = Some(j.clone())
                    };
                }
                InformationElement::Uli(j) => {
                    // Two instances
                    match (j.ins, self.uli.is_none(), self.uli_for_sgw.is_none()) {
                        (0, true, _) => self.uli = Some(j.clone()),
                        (1, _, true) => self.uli_for_sgw = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::ServingNetwork(j) => {
                    if let (0, true) = (j.ins, self.servingnetwork.is_none()) {
                        self.servingnetwork = Some(j.clone())
                    };
                }
                InformationElement::RatType(j) => {
                    if let (0, true) = (j.ins, self.rattype.is_none()) {
                        self.rattype = Some(j.clone())
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone())
                    };
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone())
                    };
                }
                InformationElement::ApnAmbr(j) => {
                    if let (0, true) = (j.ins, self.apnambr.is_none()) {
                        self.apnambr = Some(j.clone())
                    };
                }
                InformationElement::DelayValue(j) => {
                    if let (0, true) = (j.ins, self.delay_dl_pnr.is_none()) {
                        self.delay_dl_pnr = Some(j.clone())
                    };
                }
                InformationElement::BearerContext(j) => {
                    if j.ins < 2 {
                        self.bearer_ctxs.push(j.clone())
                    };
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::UeTimeZone(j) => {
                    if let (0, true) = (j.ins, self.uetimezone.is_none()) {
                        self.uetimezone = Some(j.clone())
                    };
                }
                InformationElement::Fqcsid(j) => {
                    // 2 instances
                    match (j.ins, self.mme_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.mme_fqcsid = Some(j.clone()),
                        (1, _, true) => self.sgw_fqcsid = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Uci(j) => {
                    if let (0, true) = (j.ins, self.uci.is_none()) {
                        self.uci = Some(j.clone())
                    };
                }
                InformationElement::IpAddress(j) => {
                    // three ins
                    match (
                        j.ins,
                        self.henb_localip.is_none(),
                        self.ue_localip.is_none(),
                        self.mme_id.is_none(),
                    ) {
                        (0, true, _, _) => self.henb_localip = Some(j.clone()),
                        (1, _, true, _) => self.ue_localip = Some(j.clone()),
                        (2, _, _, true) => self.mme_id = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::PortNumber(j) => {
                    // two ins
                    match (
                        j.ins,
                        self.henb_udpport.is_none(),
                        self.ue_udpport.is_none(),
                    ) {
                        (0, true, _) => self.henb_udpport = Some(j.clone()),
                        (1, _, true) => self.ue_udpport = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::Ldn(j) => {
                    // two ins
                    match (j.ins, self.mme_ldn.is_none(), self.sgw_ldn.is_none()) {
                        (0, true, _) => self.mme_ldn = Some(j.clone()),
                        (1, _, true) => self.sgw_ldn = Some(j.clone()),
                        _ => (),
                    }
                }
                InformationElement::CnOperatorSelectionEntity(j) => {
                    if let (0, true) = (j.ins, self.cnose.is_none()) {
                        self.cnose = Some(j.clone())
                    };
                }
                InformationElement::PresenceReportingAreaInformation(j) => {
                    if let (0, true) = (j.ins, self.prai.is_none()) {
                        self.prai = Some(j.clone())
                    };
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 3 {
                        self.overload_info.push(j.clone())
                    };
                }
                InformationElement::ServingPlmnRateControl(j) => {
                    if let (0, true) = (j.ins, self.srv_plmn_rate_cntrl.is_none()) {
                        self.srv_plmn_rate_cntrl = Some(j.clone())
                    };
                }
                InformationElement::Counter(j) => {
                    if let (0, true) = (j.ins, self.mo_exception_data_counter.is_none()) {
                        self.mo_exception_data_counter = Some(j.clone())
                    };
                }
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone())
                    };
                }
                InformationElement::TwanId(j) => {
                    if let (0, true) = (j.ins, self.wlan_loc.is_none()) {
                        self.wlan_loc = Some(j.clone())
                    };
                }
                InformationElement::TwanIdTimeStamp(j) => {
                    if let (0, true) = (j.ins, self.wlan_loc_timestamp.is_none()) {
                        self.wlan_loc_timestamp = Some(j.clone())
                    };
                }
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j.clone());
                    }
                }
                InformationElement::PSCellId(j) => {
                    if let (0, true) = (j.ins, self.pscellid.is_none()) {
                        self.pscellid = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_modify_bearer_req_unmarshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 153] = [
        0x48, 0x22, 0x00, 0x95, 0xe6, 0x4d, /* UZH"...M */
        0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x4b, 0x00, /* ..&...K. */
        0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 0x01, 0x50, /* ..hI)P.P */
        0x94, 0x70, 0x52, 0x00, 0x01, 0x00, 0x06, 0x56, /* .pR....V */
        0x00, 0x0d, 0x00, 0x18, 0x32, 0xf4, 0x02, 0x0d, /* ....2... */
        0x59, 0x32, 0xf4, 0x02, 0x00, 0xc5, 0x58, 0x02, /* Y2....X. */
        0x53, 0x00, 0x03, 0x00, 0x32, 0xf4, 0x02, 0x57, /* S...2..W */
        0x00, 0x19, 0x00, 0xc6, 0x23, 0xed, 0x38, 0x20, /* ....#.8  */
        0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, /* ....*.JE */
        0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x27, 0x48, 0x00, 0x08, 0x00, /* ...'H... */
        0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe8, /* ........ */
        0x03, 0x00, 0x01, 0x00, 0x16, 0x72, 0x00, 0x02, /* .....r.. */
        0x00, 0x00, 0x00, 0x5d, 0x00, 0x22, 0x00, 0x49, /* ...].".I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x19, 0x01, /* ....W... */
        0xc4, 0x23, 0xed, 0x38, 0x25, 0xd9, 0xab, 0x8d, /* .#.8%... */
        0xf3, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, /* .*.JE... */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x28, 0xff, 0x00, 0x06, 0x00, 0x07, /* ........ */
        0xdb, 0x07, 0x00, 0x01, 0x00, /* ( */
    ];
    let decoded = ModifyBearerRequest {
        header: Gtpv2Header {
            msgtype: MODIFY_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 149,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8694920510054907".to_string(),
        }),
        rattype: Some(RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        }),
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 234,
                    mnc: 20,
                    mnc_is_three_digits: false,
                    tac: 0x0d59,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 234,
                    mnc: 20,
                    mnc_is_three_digits: false,
                    eci: 12933122,
                }),
            ],
        }),
        servingnetwork: Some(ServingNetwork {
            t: SERVINGNW,
            length: 3,
            ins: 0,
            mcc: 234,
            mnc: 20,
            mnc_is_three_digits: false,
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 6,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        apnambr: Some(Ambr {
            t: AMBR,
            length: 8,
            ins: 0,
            ambr_ul: 1000,
            ambr_dl: 1000,
        }),
        recovery: Some(Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 22,
        }),
        uetimezone: Some(UeTimeZone {
            t: UETIMEZONE,
            length: 2,
            ins: 0,
            time_zone: 0,
            dst: 0,
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 34,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: None,
            bearer_flags: None,
            pco: None,
            apco: None,
            epco: None,
            max_packet_loss: None,
            ran_nas_cause: None,
            ebi: Ebi {
                t: EBI,
                length: 1,
                ins: 0,
                value: 5,
            },
            fteids: vec![Fteid {
                t: FTEID,
                length: 25,
                ins: 1,
                interface: 4,
                teid: 0x23ed3825,
                ipv4: Some(Ipv4Addr::new(217, 171, 141, 243)),
                ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x28)),
            }],
            bearer_qos: None,
            ..BearerContext::default()
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ModifyBearerRequest::default()
    };
    let message = ModifyBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_modify_bearer_req_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 121] = [
        0x48, 0x22, 0x00, 0x75, 0x8b, 0x29, /* ..H".u.) */
        0x26, 0xfd, 0x00, 0x00, 0xc3, 0x00, 0x4b, 0x00, /* &.....K. */
        0x08, 0x00, 0x68, 0x35, 0x43, 0x40, 0x80, 0x96, /* ..h5C@.. */
        0x73, 0x32, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x02, /* s2V..... */
        0xf6, 0x10, 0x0c, 0x3f, 0x02, 0xf6, 0x10, 0x02, /* ...?.... */
        0xc0, 0xe6, 0x06, 0x53, 0x00, 0x03, 0x00, 0x02, /* ...S.... */
        0xf6, 0x10, 0x52, 0x00, 0x01, 0x00, 0x06, 0x57, /* ..R....W */
        0x00, 0x09, 0x00, 0x86, 0x15, 0xe9, 0xe7, 0xcc, /* ........ */
        0xd5, 0xb5, 0x3c, 0x70, 0x48, 0x00, 0x08, 0x00, /* ..<pH... */
        0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe8, /* ........ */
        0x5d, 0x00, 0x12, 0x00, 0x49, 0x00, 0x01, 0x00, /* ]...I... */
        0x05, 0x57, 0x00, 0x09, 0x01, 0x84, 0x15, 0xe9, /* .W...... */
        0xe7, 0xcc, 0xd5, 0xb5, 0x3c, 0x70, 0x03, 0x00, /* ....<p.. */
        0x01, 0x00, 0x15, 0x72, 0x00, 0x02, /* .....r.. */
        0x00, 0x01, 0x00, 0xff, 0x00, 0x06, 0x00, 0x07, /* ........ */
        0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ModifyBearerRequest {
        header: Gtpv2Header {
            msgtype: MODIFY_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 117,
            teid: Some(0x8b2926fd),
            sqn: 0xc3,
        },
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8653340408693723".to_string(),
        }),
        rattype: Some(RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        }),
        uli: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 206,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    tac: 0x0c3f,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 206,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    eci: 46196230,
                }),
            ],
        }),
        servingnetwork: Some(ServingNetwork {
            t: SERVINGNW,
            length: 3,
            ins: 0,
            mcc: 206,
            mnc: 1,
            mnc_is_three_digits: false,
        }),
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x15e9e7cc,
            ipv4: Some(Ipv4Addr::new(213, 181, 60, 112)),
            ipv6: None,
        }),
        apnambr: Some(Ambr {
            t: AMBR,
            length: 8,
            ins: 0,
            ambr_ul: 1000,
            ambr_dl: 1000,
        }),
        recovery: Some(Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 21,
        }),
        uetimezone: Some(UeTimeZone {
            t: UETIMEZONE,
            length: 2,
            ins: 0,
            time_zone: 1,
            dst: 0,
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 34,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: None,
            bearer_flags: None,
            pco: None,
            apco: None,
            epco: None,
            max_packet_loss: None,
            ran_nas_cause: None,
            ebi: Ebi {
                t: EBI,
                length: 1,
                ins: 0,
                value: 5,
            },
            fteids: vec![Fteid {
                t: FTEID,
                length: 25,
                ins: 1,
                interface: 4,
                teid: 0x15e9e7cc,
                ipv4: Some(Ipv4Addr::new(213, 181, 60, 112)),
                ipv6: None,
            }],
            bearer_qos: None,
            ..BearerContext::default()
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ModifyBearerRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
