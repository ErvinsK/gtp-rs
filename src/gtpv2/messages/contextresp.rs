use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CTX_RESP: u8 = 131;

// Definition of GTPv2-C Context Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub imsi: Option<Imsi>,
    pub mmctx: Option<MmContext>,
    pub ue_eps_pdn: Vec<PdnConnections>,
    pub fteid_control: Option<Fteid>,
    pub sgw_fteid: Option<Fteid>,
    pub fqdns: Vec<Fqdn>,
    pub indication: Option<Indication>,
    pub trace_info: Option<TraceInformation>,
    pub ip_addresses: Vec<IpAddress>,
    pub rfsp_indexes: Vec<RfspIndex>,
    pub ue_time_zone: Option<UeTimeZone>,
    pub mme_sgsn_ldn: Option<Ldn>,
    pub mdt_cfg: Option<MdtConfiguration>,
    pub uci: Option<Uci>,
    pub monitor_event_info: Vec<MonitoringEventInformation>,
    pub monitor_even_ext_info: Vec<MonitoringEventExtensionInfo>,
    pub ue_usage_type: Option<IntegerNumber>,
    pub scef_pdn_connections: Vec<ScefPdnConnections>,
    pub rat_type: Option<RatType>,
    pub srv_plmn_rate_cntrl: Option<ServingPlmnRateControl>,
    pub mo_exception_data_cntr: Option<Counter>,
    pub rem_running_srvc_gap_timer: Option<IntegerNumber>,
    pub ext_trace_info: Option<ExtendedTraceInformation>,
    pub add_rrm_policy_index: Vec<AdditionalRrmPolicyIndex>,
    pub iwk_scef_id: Option<NodeIdentifier>,
    pub alt_imsi: Option<AlternativeImsi>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ContextResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: CTX_RESP,
            teid: Some(0),
            ..Default::default()
        };
        ContextResponse {
            header: hdr,
            cause: Cause::default(),
            imsi: None,
            mmctx: None,
            ue_eps_pdn: vec![],
            fteid_control: None,
            sgw_fteid: None,
            fqdns: vec![],
            indication: None,
            trace_info: None,
            ip_addresses: vec![],
            rfsp_indexes: vec![],
            ue_time_zone: None,
            mme_sgsn_ldn: None,
            mdt_cfg: None,
            uci: None,
            monitor_event_info: vec![],
            monitor_even_ext_info: vec![],
            ue_usage_type: None,
            scef_pdn_connections: vec![],
            rat_type: None,
            srv_plmn_rate_cntrl: None,
            mo_exception_data_cntr: None,
            rem_running_srvc_gap_timer: None,
            ext_trace_info: None,
            add_rrm_policy_index: vec![],
            iwk_scef_id: None,
            alt_imsi: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ContextResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ContextResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CTX_RESP {
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

        elements.push(self.cause.clone().into());

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mmctx.clone() {
            elements.push(i.into())
        };

        self.ue_eps_pdn
            .iter()
            .for_each(|x| elements.push(InformationElement::PdnConnections(Box::new(x.clone()))));

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.sgw_fteid.clone() {
            elements.push(i.into());
        }

        self.fqdns
            .iter()
            .for_each(|x| elements.push(InformationElement::Fqdn(x.clone())));

        if let Some(i) = self.indication.clone() {
            elements.push(InformationElement::Indication(i));
        }

        if let Some(i) = self.trace_info.clone() {
            elements.push(i.into());
        }

        self.ip_addresses
            .iter()
            .for_each(|x| elements.push(InformationElement::IpAddress(x.clone())));

        self.rfsp_indexes
            .iter()
            .for_each(|x| elements.push(InformationElement::RfspIndex(x.clone())));

        if let Some(i) = self.ue_time_zone.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mme_sgsn_ldn.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mdt_cfg.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.uci.clone() {
            elements.push(i.into());
        }

        self.monitor_event_info
            .iter()
            .for_each(|x| elements.push(InformationElement::MonitoringEventInformation(x.clone())));

        self.monitor_even_ext_info.iter().for_each(|x| {
            elements.push(InformationElement::MonitoringEventExtensionInfo(x.clone()))
        });

        if let Some(i) = self.ue_usage_type.clone() {
            elements.push(i.into());
        }

        self.scef_pdn_connections
            .iter()
            .for_each(|x| elements.push(InformationElement::ScefPdnConnections(x.clone())));

        if let Some(i) = self.rat_type.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.srv_plmn_rate_cntrl.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mo_exception_data_cntr.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.rem_running_srvc_gap_timer.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.ext_trace_info.clone() {
            elements.push(i.into());
        }

        self.add_rrm_policy_index
            .iter()
            .for_each(|x| elements.push(InformationElement::AdditionalRrmPolicyIndex(x.clone())));

        if let Some(i) = self.iwk_scef_id.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.alt_imsi.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        self.cause = j;
                        mandatory = true;
                    }
                }
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j);
                    }
                }
                InformationElement::MmContext(j) => {
                    if let (0, true) = (j.get_ins(), self.mmctx.is_none()) {
                        self.mmctx = Some(j);
                    }
                }
                InformationElement::PdnConnections(j) => {
                    if j.ins == 0 {
                        self.ue_eps_pdn.push(*j);
                    }
                }
                InformationElement::Fteid(j) => {
                    match (
                        j.ins,
                        self.fteid_control.is_none(),
                        self.sgw_fteid.is_none(),
                    ) {
                        (0, true, _) => self.fteid_control = Some(j),
                        (1, _, true) => self.sgw_fteid = Some(j),
                        (_, _, _) => (),
                    }
                }
                InformationElement::Fqdn(j) => {
                    if j.ins < 3 {
                        self.fqdns.push(j);
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j);
                    }
                }
                InformationElement::TraceInformation(j) => {
                    if let (0, true) = (j.ins, self.trace_info.is_none()) {
                        self.trace_info = Some(j);
                    }
                }
                InformationElement::IpAddress(j) => {
                    if j.ins < 2 {
                        self.ip_addresses.push(j);
                    }
                }
                InformationElement::RfspIndex(j) => {
                    if j.ins < 2 {
                        self.rfsp_indexes.push(j);
                    }
                }
                InformationElement::UeTimeZone(j) => {
                    if let (0, true) = (j.ins, self.ue_time_zone.is_none()) {
                        self.ue_time_zone = Some(j);
                    }
                }
                InformationElement::Ldn(j) => {
                    if let (0, true) = (j.ins, self.mme_sgsn_ldn.is_none()) {
                        self.mme_sgsn_ldn = Some(j);
                    }
                }
                InformationElement::MdtConfiguration(j) => {
                    if let (0, true) = (j.ins, self.mdt_cfg.is_none()) {
                        self.mdt_cfg = Some(j);
                    }
                }
                InformationElement::Uci(j) => {
                    if let (0, true) = (j.ins, self.uci.is_none()) {
                        self.uci = Some(j);
                    }
                }
                InformationElement::MonitoringEventInformation(j) => {
                    if j.ins == 0 {
                        self.monitor_event_info.push(j);
                    }
                }
                InformationElement::MonitoringEventExtensionInfo(j) => {
                    if j.ins == 0 {
                        self.monitor_even_ext_info.push(j);
                    }
                }
                InformationElement::IntegerNumber(j) => {
                    match (
                        j.ins,
                        self.ue_usage_type.is_none(),
                        self.rem_running_srvc_gap_timer.is_none(),
                    ) {
                        (0, true, _) => self.ue_usage_type = Some(j),
                        (1, _, true) => self.rem_running_srvc_gap_timer = Some(j),
                        (_, _, _) => (),
                    }
                }
                InformationElement::ScefPdnConnections(j) => {
                    if j.ins == 0 {
                        self.scef_pdn_connections.push(j);
                    }
                }
                InformationElement::RatType(j) => {
                    if let (0, true) = (j.ins, self.rat_type.is_none()) {
                        self.rat_type = Some(j);
                    }
                }
                InformationElement::ServingPlmnRateControl(j) => {
                    if let (0, true) = (j.ins, self.srv_plmn_rate_cntrl.is_none()) {
                        self.srv_plmn_rate_cntrl = Some(j);
                    }
                }
                InformationElement::Counter(j) => {
                    if let (0, true) = (j.ins, self.mo_exception_data_cntr.is_none()) {
                        self.mo_exception_data_cntr = Some(j);
                    }
                }
                InformationElement::ExtendedTraceInformation(j) => {
                    if let (0, true) = (j.ins, self.ext_trace_info.is_none()) {
                        self.ext_trace_info = Some(j);
                    }
                }
                InformationElement::AdditionalRrmPolicyIndex(j) => {
                    if j.ins < 2 {
                        self.add_rrm_policy_index.push(j);
                    }
                }
                InformationElement::NodeIdentifier(j) => {
                    if let (0, true) = (j.ins, self.iwk_scef_id.is_none()) {
                        self.iwk_scef_id = Some(j);
                    }
                }
                InformationElement::AlternativeImsi(j) => {
                    if let (0, true) = (j.ins, self.alt_imsi.is_none()) {
                        self.alt_imsi = Some(j);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE))
        }
    }
}

#[test]
fn test_ctx_resp_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 437] = [
        0x48, 0x83, 0x01, 0xb1, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x16, 0x00, 0x6b, 0x01, 0x88, 0x00, 0x98, 0x27, 0x81, 0x27, 0x00, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
        0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x02, 0x07, 0x08, 0x03, 0x03, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
        0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x03, 0x02, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
        0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a, 0x01, 0x02, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0x05,
        0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f,
        0x40, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03,
        0x04, 0x00, 0xad, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02,
        0x03, 0x04, 0x01, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02,
        0x00, 0x22, 0x00, 0x0c, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x61, 0x70, 0x6e, 0x2e, 0x63, 0x6f,
        0x6d, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02,
        0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x23, 0x00, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x32,
        0x2e, 0x61, 0x70, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56,
        0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x04, 0x01,
        0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02, 0x57, 0x00, 0x09, 0x00, 0x86, 0x27,
        0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07, 0x00,
        0x01, 0x00,
    ];
    let mmctx = MmContextEpsSecurityContextQuadruplets {
        t: MMCTXEPSSECCTXQ,
        length: 392,
        ins: 0,
        sec_mode: SecurityMode::EpsSecurityContextAndQuadruplets,
        ksi: 0,
        nas_integrity: NasIntegrityProtectionValues::NoIntegrity,
        nas_cipher: NasCipherValues::Eea1,
        nas_dl_count: 0x002700ff,
        nas_ul_count: 0x00ffffff,
        kasme: [0xff; 32],
        auth_quadruplets: Some(vec![AuthQuadruplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            autn: vec![0x03, 0x09, 0x0a],
            kasme: [
                0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        }]),
        auth_quintuplets: Some(vec![AuthQuintuplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            ck: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            ik: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            autn: vec![0x03, 0x09, 0x0a],
        }]),
        drx_params: Some([0x01, 0x02]),
        next_hop: Some([0xaa; 32]),
        ncc: Some(0x05),
        subscr_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        used_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        ue_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ms_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        mei: Some(vec![0x01, 0x02, 0x03, 0x04]),
        access_res: AccessRestrictionMM::from(0x00),
        old_eps_sec_ctx: Some(OldEpsSecurityContext {
            old_ksi: 5,
            old_ncc: Some(5),
            old_kasme: [0xff; 32],
            old_next_hop: Some([0xff; 32]),
        }),
        vdn_pref_ue_usage: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_for_paging: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ext_access_res: Some(ExtendedAccessRestrictionMM::from(0x03)),
        ue_add_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_nr_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        apn_rate_controls: Some(vec![
            ApnRateControlStatusMM {
                apn: "test.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
            ApnRateControlStatusMM {
                apn: "test2.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
        ]),
        core_nw_res: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_id: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ensct: Some(0x02),
    };
    let decoded = ContextResponse {
        header: Gtpv2Header {
            msgtype: CTX_RESP,
            piggyback: false,
            message_prio: None,
            length: 433,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            value: 0x16,
            ..Cause::default()
        },
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x27892f70,
            ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
            ipv6: None,
        }),
        mmctx: Some(mmctx.into()),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ContextResponse::default()
    };
    let message = ContextResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_ctx_resp_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 437] = [
        0x48, 0x83, 0x01, 0xb1, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x16, 0x00, 0x6b, 0x01, 0x88, 0x00, 0x98, 0x27, 0x81, 0x27, 0x00, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
        0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x02, 0x07, 0x08, 0x03, 0x03, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
        0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x03, 0x02, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
        0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a, 0x01, 0x02, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0x05,
        0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f,
        0x40, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03,
        0x04, 0x00, 0xad, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02,
        0x03, 0x04, 0x01, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02,
        0x00, 0x22, 0x00, 0x0c, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x61, 0x70, 0x6e, 0x2e, 0x63, 0x6f,
        0x6d, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02,
        0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x23, 0x00, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x32,
        0x2e, 0x61, 0x70, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56,
        0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x04, 0x01,
        0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02, 0x57, 0x00, 0x09, 0x00, 0x86, 0x27,
        0x89, 0x2f, 0x70, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07, 0x00,
        0x01, 0x00,
    ];
    let mmctx = MmContextEpsSecurityContextQuadruplets {
        t: MMCTXEPSSECCTXQ,
        length: 392,
        ins: 0,
        sec_mode: SecurityMode::EpsSecurityContextAndQuadruplets,
        ksi: 0,
        nas_integrity: NasIntegrityProtectionValues::NoIntegrity,
        nas_cipher: NasCipherValues::Eea1,
        nas_dl_count: 0x002700ff,
        nas_ul_count: 0x00ffffff,
        kasme: [0xff; 32],
        auth_quadruplets: Some(vec![AuthQuadruplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            autn: vec![0x03, 0x09, 0x0a],
            kasme: [
                0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        }]),
        auth_quintuplets: Some(vec![AuthQuintuplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            ck: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            ik: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            autn: vec![0x03, 0x09, 0x0a],
        }]),
        drx_params: Some([0x01, 0x02]),
        next_hop: Some([0xaa; 32]),
        ncc: Some(0x05),
        subscr_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        used_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        ue_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ms_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        mei: Some(vec![0x01, 0x02, 0x03, 0x04]),
        access_res: AccessRestrictionMM::from(0x00),
        old_eps_sec_ctx: Some(OldEpsSecurityContext {
            old_ksi: 5,
            old_ncc: Some(5),
            old_kasme: [0xff; 32],
            old_next_hop: Some([0xff; 32]),
        }),
        vdn_pref_ue_usage: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_for_paging: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ext_access_res: Some(ExtendedAccessRestrictionMM::from(0x03)),
        ue_add_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_nr_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        apn_rate_controls: Some(vec![
            ApnRateControlStatusMM {
                apn: "test.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
            ApnRateControlStatusMM {
                apn: "test2.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
        ]),
        core_nw_res: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_id: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ensct: Some(0x02),
    };
    let decoded = ContextResponse {
        header: Gtpv2Header {
            msgtype: CTX_RESP,
            piggyback: false,
            message_prio: None,
            length: 433,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Cause {
            value: 0x16,
            ..Cause::default()
        },
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 9,
            ins: 0,
            interface: 6,
            teid: 0x27892f70,
            ipv4: Some(Ipv4Addr::new(139, 7, 133, 184)),
            ipv6: None,
        }),
        mmctx: Some(mmctx.into()),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ContextResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
