// Bearer Context IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Bearer Context IE Type

pub const PDN_CONN: u8 = 109;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdnConnections {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub apn: Apn,
    pub apn_restriction: Option<ApnRestriction>,
    pub selection_mode: Option<SelectionMode>,
    pub ipv4: Option<IpAddress>,
    pub ipv6: Option<IpAddress>,
    pub linked_ebi: Ebi,
    pub pgw_addr_control: Fteid,
    pub pgw_node_name: Option<Fqdn>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub apn_ambr: Ambr,
    pub charging_char: Option<ChargingCharacteristics>,
    pub change_reporting_action: Option<ChangeReportingAction>,
    pub csg_info_reporting_action: Option<CSGInformationReportingAction>,
    pub henb_info_reporting: Option<HenbInfoReporting>,
    pub indication: Option<Indication>,
    pub signalling_priority_indication: Option<Spi>,
    pub change_to_report_flags: Option<ChangeToReportFlags>,
    pub local_home_network_id: Option<Fqdn>,
    pub praa: Option<PresenceReportingAreaAction>,
    pub wlan_offloadability: Option<WlanOffloadIndication>,
    pub remote_ue_ctxs: Vec<RemoteUeContext>,
    pub pdn_type: Option<PdnType>,
    pub hdr_compr_config: Option<HeaderCompressionConfiguration>,
    pub pgw_change_info: Option<PgwChangeInfo>,
    pub up_security_policy: Option<UpSecurityPolicy>,
}

impl Default for PdnConnections {
    fn default() -> Self {
        PdnConnections {
            t: PDN_CONN,
            length: 0,
            ins: 0,
            apn: Apn::default(),
            apn_restriction: None,
            selection_mode: None,
            ipv4: None,
            ipv6: None,
            linked_ebi: Ebi::default(),
            pgw_addr_control: Fteid::default(),
            pgw_node_name: None,
            bearer_ctxs: vec![],
            apn_ambr: Ambr::default(),
            charging_char: None,
            change_reporting_action: None,
            csg_info_reporting_action: None,
            henb_info_reporting: None,
            indication: None,
            signalling_priority_indication: None,
            change_to_report_flags: None,
            local_home_network_id: None,
            praa: None,
            wlan_offloadability: None,
            remote_ue_ctxs: vec![],
            pdn_type: None,
            hdr_compr_config: None,
            pgw_change_info: None,
            up_security_policy: None,
        }
    }
}

impl From<PdnConnections> for GroupedIe {
    fn from(i: PdnConnections) -> Self {
        GroupedIe {
            t: PDN_CONN,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for PdnConnections {
    fn from(i: GroupedIe) -> Self {
        let mut pdn_conn = PdnConnections::default();
        (pdn_conn.t, pdn_conn.length, pdn_conn.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            match j {
                InformationElement::Apn(k) => {
                    if k.ins == 0 {
                        pdn_conn.apn = k;
                    }
                }
                InformationElement::ApnRestriction(k) => {
                    if let (0, true) = (k.ins, pdn_conn.apn_restriction.is_none()) {
                        pdn_conn.apn_restriction = Some(k)
                    }
                }
                InformationElement::SelectionMode(k) => {
                    if let (0, true) = (k.ins, pdn_conn.selection_mode.is_none()) {
                        pdn_conn.selection_mode = Some(k)
                    }
                }
                InformationElement::IpAddress(k) => {
                    match (k.ins, pdn_conn.ipv4.is_none(), pdn_conn.ipv6.is_none()) {
                        (0, true, _) => pdn_conn.ipv4 = Some(k),
                        (1, _, true) => pdn_conn.ipv6 = Some(k),
                        _ => (),
                    }
                }
                InformationElement::Ebi(k) => {
                    if k.ins == 0 {
                        pdn_conn.linked_ebi = k;
                    }
                }
                InformationElement::Fteid(k) => {
                    if k.ins == 0 {
                        pdn_conn.pgw_addr_control = k;
                    }
                }
                InformationElement::Fqdn(k) => {
                    match (
                        k.ins,
                        pdn_conn.pgw_node_name.is_none(),
                        pdn_conn.local_home_network_id.is_none(),
                    ) {
                        (0, true, _) => pdn_conn.pgw_node_name = Some(k),
                        (1, _, true) => pdn_conn.local_home_network_id = Some(k),
                        _ => (),
                    }
                }
                InformationElement::BearerContext(k) => {
                    if k.ins == 0 {
                        pdn_conn.bearer_ctxs.push(k);
                    }
                }
                InformationElement::ApnAmbr(k) => {
                    if k.ins == 0 {
                        pdn_conn.apn_ambr = k;
                    }
                }
                InformationElement::ChargingCharacteristics(k) => {
                    if let (0, true) = (k.ins, pdn_conn.charging_char.is_none()) {
                        pdn_conn.charging_char = Some(k);
                    }
                }
                InformationElement::ChangeReportingAction(k) => {
                    if let (0, true) = (k.ins, pdn_conn.change_reporting_action.is_none()) {
                        pdn_conn.change_reporting_action = Some(k);
                    }
                }
                InformationElement::CSGInformationReportingAction(k) => {
                    if let (0, true) = (k.ins, pdn_conn.csg_info_reporting_action.is_none()) {
                        pdn_conn.csg_info_reporting_action = Some(k);
                    }
                }
                InformationElement::HenbInfoReporting(k) => {
                    if let (0, true) = (k.ins, pdn_conn.henb_info_reporting.is_none()) {
                        pdn_conn.henb_info_reporting = Some(k);
                    }
                }
                InformationElement::Indication(k) => {
                    if let (0, true) = (k.ins, pdn_conn.indication.is_none()) {
                        pdn_conn.indication = Some(k);
                    }
                }
                InformationElement::Spi(k) => {
                    if let (0, true) = (k.ins, pdn_conn.signalling_priority_indication.is_none()) {
                        pdn_conn.signalling_priority_indication = Some(k);
                    }
                }
                InformationElement::ChangeToReportFlags(k) => {
                    if let (0, true) = (k.ins, pdn_conn.change_to_report_flags.is_none()) {
                        pdn_conn.change_to_report_flags = Some(k);
                    }
                }
                InformationElement::PresenceReportingAreaAction(k) => {
                    if let (0, true) = (k.ins, pdn_conn.praa.is_none()) {
                        pdn_conn.praa = Some(k);
                    }
                }
                InformationElement::WlanOffloadIndication(k) => {
                    if let (0, true) = (k.ins, pdn_conn.wlan_offloadability.is_none()) {
                        pdn_conn.wlan_offloadability = Some(k);
                    }
                }
                InformationElement::RemoteUeContext(k) => {
                    if k.ins == 0 {
                        pdn_conn.remote_ue_ctxs.push(k);
                    }
                }
                InformationElement::PdnType(k) => {
                    if let (0, true) = (k.ins, pdn_conn.pdn_type.is_none()) {
                        pdn_conn.pdn_type = Some(k);
                    }
                }
                InformationElement::HeaderCompressionConfiguration(k) => {
                    if let (0, true) = (k.ins, pdn_conn.hdr_compr_config.is_none()) {
                        pdn_conn.hdr_compr_config = Some(k);
                    }
                }
                _ => (),
            }
        }
        pdn_conn
    }
}

impl IEs for PdnConnections {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie = GroupedIe::from(self.clone());
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: PdnConnections = match GroupedIe::unmarshal(buffer) {
            Ok(i) => PdnConnections::from(i),
            Err(j) => return Err(j),
        };
        Ok(data)
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

impl PdnConnections {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];

        v.push(self.apn.clone().into());

        if let Some(i) = self.apn_restriction.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.selection_mode.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.ipv4.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.ipv6.clone() {
            v.push(i.into());
        }

        v.push(self.linked_ebi.clone().into());

        v.push(self.pgw_addr_control.clone().into());

        if let Some(i) = self.pgw_node_name.clone() {
            v.push(i.into());
        }

        self.bearer_ctxs
            .iter()
            .for_each(|x| v.push(InformationElement::BearerContext(x.clone())));

        v.push(self.apn_ambr.clone().into());

        if let Some(i) = self.charging_char.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.change_reporting_action.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.csg_info_reporting_action.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.henb_info_reporting.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.indication.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.signalling_priority_indication.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.change_to_report_flags.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.local_home_network_id.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.praa.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.wlan_offloadability.clone() {
            v.push(i.into());
        }

        self.remote_ue_ctxs
            .iter()
            .for_each(|x| v.push(InformationElement::RemoteUeContext(x.clone())));

        if let Some(i) = self.pdn_type.clone() {
            v.push(i.into());
        }

        if let Some(i) = self.hdr_compr_config.clone() {
            v.push(i.into());
        }

        v
    }
}

#[test]
fn pdn_connection_ie_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 433] = [
        0x6d, 0x01, 0xad, 0x00, 0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e,
        0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0x7f, 0x00, 0x01, 0x00, 0x00, 0x80, 0x00, 0x01, 0x00,
        0x00, 0x4a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x10, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x00, 0x87, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07,
        0x85, 0xb8, 0x88, 0x00, 0x35, 0x00, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f,
        0x64, 0x65, 0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05,
        0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67,
        0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x5d,
        0x00, 0x34, 0x00, 0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4, 0x48, 0x00, 0x08, 0x00, 0x00,
        0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x5f, 0x00, 0x02, 0x00, 0xff, 0xff, 0x83, 0x00,
        0x01, 0x00, 0x04, 0x92, 0x00, 0x01, 0x00, 0x00, 0xa5, 0x00, 0x01, 0x00, 0x00, 0x4d, 0x00,
        0x0a, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9d, 0x00, 0x01,
        0x00, 0x01, 0xa7, 0x00, 0x01, 0x00, 0x02, 0x88, 0x00, 0x35, 0x01, 0x05, 0x74, 0x6f, 0x70,
        0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65,
        0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32,
        0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03,
        0x6f, 0x72, 0x67, 0x00, 0xb1, 0x00, 0x3e, 0x00, 0x01, 0xff, 0xff, 0xff, 0x11, 0x01, 0x01,
        0x01, 0x01, 0x01, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, 0x62,
        0xf2, 0x10, 0x0f, 0xff, 0xff, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x62, 0xf2, 0x10,
        0xff, 0xff, 0xaa, 0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, 0x62, 0xf2, 0x10, 0xff,
        0xff, 0xaa, 0xaa, 0x01, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, 0xb9, 0x00, 0x01, 0x00, 0x02,
        0xbf, 0x00, 0x29, 0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91,
        0x16, 0x78, 0xf3, 0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34,
        0x63, 0x00, 0x01, 0x00, 0x03, 0xc4, 0x00, 0x04, 0x00, 0x7f, 0x00, 0x00, 0xff,
    ];
    let decoded = PdnConnections {
        t: PDN_CONN,
        length: 429,
        ins: 0,
        apn: Apn {
            length: 13,
            name: "test.net.com".to_string(),
            ..Apn::default()
        },
        apn_restriction: Some(ApnRestriction {
            restriction_type: Restriction::NoApnRestriction,
            ..ApnRestriction::default()
        }),
        selection_mode: Some(SelectionMode {
            mode: 0,
            ..SelectionMode::default()
        }),
        ipv4: Some(IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..IpAddress::default()
        }),
        ipv6: Some(IpAddress {
            length: 16,
            ins: 1,
            ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            ..IpAddress::default()
        }),
        linked_ebi: Ebi {
            value: 5,
            ..Ebi::default()
        },
        pgw_addr_control: Fteid {
            length: 9,
            ins: 0,
            interface: 7,
            teid: 0x27892f70,
            ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
            ipv6: None,
            ..Fteid::default()
        },
        pgw_node_name: Some(Fqdn {
            length: 53,
            name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
            ..Fqdn::default()
        }),
        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 52,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: Some(ChargingId {
                t: CHARGINGID,
                length: 4,
                ins: 0,
                charging_id: 23239876,
            }),
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
                value: 0,
            },
            fteids: vec![Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 5,
                teid: 0x3b95985a,
                ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
                ipv6: None,
            }],
            bearer_qos: Some(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 0,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
            ..BearerContext::default()
        }],
        apn_ambr: Ambr {
            ambr_ul: 2000,
            ambr_dl: 8000,
            ..Ambr::default()
        },
        charging_char: Some(ChargingCharacteristics {
            charging_char: 0xffff,
            ..ChargingCharacteristics::default()
        }),
        change_reporting_action: Some(ChangeReportingAction {
            action: 4,
            ..ChangeReportingAction::default()
        }),
        csg_info_reporting_action: Some(CSGInformationReportingAction {
            action: 0,
            ..CSGInformationReportingAction::default()
        }),
        henb_info_reporting: Some(HenbInfoReporting {
            fti: false,
            ..HenbInfoReporting::default()
        }),
        indication: Some(Indication {
            crsi: true,
            ..Indication::default()
        }),
        signalling_priority_indication: Some(Spi {
            lapi: true,
            ..Spi::default()
        }),
        change_to_report_flags: Some(ChangeToReportFlags {
            tzcr: true,
            sncr: false,
            ..ChangeToReportFlags::default()
        }),
        local_home_network_id: Some(Fqdn {
            ins: 1,
            length: 53,
            name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
            ..Fqdn::default()
        }),
        praa: Some(PresenceReportingAreaAction {
            length: 62,
            inapra: false,
            action: 1,
            prai: 0xffffff,
            tai: vec![Tai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                tac: 0x0bd9,
            }],
            rai: vec![Rai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                rac: 0xaa,
            }],
            macro_enb: vec![MacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0x0fffff,
            }],
            home_enb: vec![MacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0x0fffff,
            }],
            ecgi: vec![Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983298,
            }],
            sai: vec![Sai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                sac: 0xdddd,
            }],
            cgi: vec![Cgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                ci: 0xaaaa,
            }],
            ext_macro_enb: vec![ExtMacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                smenb: false,
                ext_macro_id: 0x0fffff,
            }],
            ..PresenceReportingAreaAction::default()
        }),
        wlan_offloadability: Some(WlanOffloadIndication {
            eutran_ind: true,
            utran_ind: false,
            ..WlanOffloadIndication::default()
        }),
        remote_ue_ctxs: vec![RemoteUeContext {
            t: REMOTE_UE_CTX,
            length: 41,
            ins: 0,
            user_id: RemoteUserId {
                t: REMOTE_USR_ID,
                length: 28,
                ins: 0,
                imsi: "901405101961873".to_string(),
                msisdn: Some("882285100125112".to_string()),
                imei: Some("8676480401323003".to_string()),
            },
            ue_ip: Some(RemoteUeIpInformation {
                t: REMOTE_UE_IP,
                length: 5,
                ins: 0,
                ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
            }),
        }],
        pdn_type: Some(PdnType {
            pdn_type: Pdn::Ipv46,
            ..PdnType::default()
        }),
        hdr_compr_config: Some(HeaderCompressionConfiguration {
            rohc_profiles: vec![
                0x0000, 0x0002, 0x0003, 0x0004, 0x0006, 0x0102, 0x0103, 0x0104,
            ],
            max_cid: 0xff,
            ..HeaderCompressionConfiguration::default()
        }),
        ..PdnConnections::default()
    };
    let i = PdnConnections::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn pdn_connection_ie_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 433] = [
        0x6d, 0x01, 0xad, 0x00, 0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e,
        0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0x7f, 0x00, 0x01, 0x00, 0x00, 0x80, 0x00, 0x01, 0x00,
        0x00, 0x4a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x10, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x00, 0x87, 0x27, 0x89, 0x2f, 0x70, 0x8b, 0x07,
        0x85, 0xb8, 0x88, 0x00, 0x35, 0x00, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f,
        0x64, 0x65, 0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05,
        0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67,
        0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x5d,
        0x00, 0x34, 0x00, 0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x5e, 0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4, 0x48, 0x00, 0x08, 0x00, 0x00,
        0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x5f, 0x00, 0x02, 0x00, 0xff, 0xff, 0x83, 0x00,
        0x01, 0x00, 0x04, 0x92, 0x00, 0x01, 0x00, 0x00, 0xa5, 0x00, 0x01, 0x00, 0x00, 0x4d, 0x00,
        0x0a, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9d, 0x00, 0x01,
        0x00, 0x01, 0xa7, 0x00, 0x01, 0x00, 0x02, 0x88, 0x00, 0x35, 0x01, 0x05, 0x74, 0x6f, 0x70,
        0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65,
        0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32,
        0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03,
        0x6f, 0x72, 0x67, 0x00, 0xb1, 0x00, 0x3e, 0x00, 0x01, 0xff, 0xff, 0xff, 0x11, 0x01, 0x01,
        0x01, 0x01, 0x01, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, 0x62,
        0xf2, 0x10, 0x0f, 0xff, 0xff, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x62, 0xf2, 0x10,
        0xff, 0xff, 0xaa, 0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, 0x62, 0xf2, 0x10, 0xff,
        0xff, 0xaa, 0xaa, 0x01, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, 0xb9, 0x00, 0x01, 0x00, 0x02,
        0xbf, 0x00, 0x29, 0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91,
        0x16, 0x78, 0xf3, 0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34,
        0x63, 0x00, 0x01, 0x00, 0x03, 0xc4, 0x00, 0x04, 0x00, 0x7f, 0x00, 0x00, 0xff,
    ];
    let decoded = PdnConnections {
        t: PDN_CONN,
        length: 429,
        ins: 0,
        apn: Apn {
            length: 13,
            name: "test.net.com".to_string(),
            ..Apn::default()
        },
        apn_restriction: Some(ApnRestriction {
            restriction_type: Restriction::NoApnRestriction,
            ..ApnRestriction::default()
        }),
        selection_mode: Some(SelectionMode {
            mode: 0,
            ..SelectionMode::default()
        }),
        ipv4: Some(IpAddress {
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..IpAddress::default()
        }),
        ipv6: Some(IpAddress {
            length: 16,
            ins: 1,
            ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            ..IpAddress::default()
        }),
        linked_ebi: Ebi {
            value: 5,
            ..Ebi::default()
        },
        pgw_addr_control: Fteid {
            length: 9,
            ins: 0,
            interface: 7,
            teid: 0x27892f70,
            ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
            ipv6: None,
            ..Fteid::default()
        },
        pgw_node_name: Some(Fqdn {
            length: 53,
            name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
            ..Fqdn::default()
        }),
        bearer_ctxs: vec![BearerContext {
            t: 93,
            length: 52,
            ins: 0,
            cause: None,
            tft: None,
            charging_id: Some(ChargingId {
                t: CHARGINGID,
                length: 4,
                ins: 0,
                charging_id: 23239876,
            }),
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
                value: 0,
            },
            fteids: vec![Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 5,
                teid: 0x3b95985a,
                ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
                ipv6: None,
            }],
            bearer_qos: Some(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 0,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
            ..BearerContext::default()
        }],
        apn_ambr: Ambr {
            ambr_ul: 2000,
            ambr_dl: 8000,
            ..Ambr::default()
        },
        charging_char: Some(ChargingCharacteristics {
            charging_char: 0xffff,
            ..ChargingCharacteristics::default()
        }),
        change_reporting_action: Some(ChangeReportingAction {
            action: 4,
            ..ChangeReportingAction::default()
        }),
        csg_info_reporting_action: Some(CSGInformationReportingAction {
            action: 0,
            ..CSGInformationReportingAction::default()
        }),
        henb_info_reporting: Some(HenbInfoReporting {
            fti: false,
            ..HenbInfoReporting::default()
        }),
        indication: Some(Indication {
            crsi: true,
            ..Indication::default()
        }),
        signalling_priority_indication: Some(Spi {
            lapi: true,
            ..Spi::default()
        }),
        change_to_report_flags: Some(ChangeToReportFlags {
            tzcr: true,
            sncr: false,
            ..ChangeToReportFlags::default()
        }),
        local_home_network_id: Some(Fqdn {
            ins: 1,
            length: 53,
            name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
            ..Fqdn::default()
        }),
        praa: Some(PresenceReportingAreaAction {
            length: 62,
            inapra: false,
            action: 1,
            prai: 0xffffff,
            tai: vec![Tai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                tac: 0x0bd9,
            }],
            rai: vec![Rai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                rac: 0xaa,
            }],
            macro_enb: vec![MacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0x0fffff,
            }],
            home_enb: vec![MacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0x0fffff,
            }],
            ecgi: vec![Ecgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                eci: 28983298,
            }],
            sai: vec![Sai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                sac: 0xdddd,
            }],
            cgi: vec![Cgi {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0xffff,
                ci: 0xaaaa,
            }],
            ext_macro_enb: vec![ExtMacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                smenb: false,
                ext_macro_id: 0x0fffff,
            }],
            ..PresenceReportingAreaAction::default()
        }),
        wlan_offloadability: Some(WlanOffloadIndication {
            eutran_ind: true,
            utran_ind: false,
            ..WlanOffloadIndication::default()
        }),
        remote_ue_ctxs: vec![RemoteUeContext {
            t: REMOTE_UE_CTX,
            length: 41,
            ins: 0,
            user_id: RemoteUserId {
                t: REMOTE_USR_ID,
                length: 28,
                ins: 0,
                imsi: "901405101961873".to_string(),
                msisdn: Some("882285100125112".to_string()),
                imei: Some("8676480401323003".to_string()),
            },
            ue_ip: Some(RemoteUeIpInformation {
                t: REMOTE_UE_IP,
                length: 5,
                ins: 0,
                ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
            }),
        }],
        pdn_type: Some(PdnType {
            pdn_type: Pdn::Ipv46,
            ..PdnType::default()
        }),
        hdr_compr_config: Some(HeaderCompressionConfiguration {
            rohc_profiles: vec![
                0x0000, 0x0002, 0x0003, 0x0004, 0x0006, 0x0102, 0x0103, 0x0104,
            ],
            max_cid: 0xff,
            ..HeaderCompressionConfiguration::default()
        }),
        ..PdnConnections::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
