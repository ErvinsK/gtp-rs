use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CTX_REQ: u8 = 130;

// Definition of GTPv2-C Context Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextRequest {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub guti: Option<Guti>,
    pub rai: Option<Uli>,
    pub ptmsi: Option<Ptmsi>,
    pub ptmsi_signature: Option<PtmsiSignature>,
    pub complete_tau_req_msg: Option<CompleteRequestMessage>,
    pub fteid_control: Option<Fteid>,
    pub udp_src_port: Option<PortNumber>,
    pub rat_type: Option<RatType>,
    pub indication: Option<Indication>,
    pub hop_counter: Option<HopCounter>,
    pub target_plmnid: Option<ServingNetwork>,
    pub mme_sgsn_ldn: Option<Ldn>,
    pub sgsn_fqdn: Option<Fqdn>,
    pub mme_fqdn: Option<Fqdn>,
    pub sgsn_number: Option<NodeNumber>,
    pub sgsn_id: Option<NodeIdentifier>,
    pub mme_id: Option<NodeIdentifier>,
    pub ciot_optim_supp_ind: Option<CIoTOptimizationSupportIndication>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ContextRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: CTX_REQ,
            teid: Some(0),
            ..Default::default()
        };
        ContextRequest {
            header: hdr,
            imsi: None,
            guti: None,
            rai: None,
            ptmsi: None,
            ptmsi_signature: None,
            complete_tau_req_msg: None,
            fteid_control: None,
            udp_src_port: None,
            rat_type: None,
            indication: None,
            hop_counter: None,
            target_plmnid: None,
            mme_sgsn_ldn: None,
            sgsn_fqdn: None,
            mme_fqdn: None,
            sgsn_number: None,
            sgsn_id: None,
            mme_id: None,
            ciot_optim_supp_ind: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ContextRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ContextRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CTX_REQ {
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

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.guti.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.rai.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ptmsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ptmsi_signature.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.complete_tau_req_msg.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.udp_src_port.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.rat_type.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.hop_counter.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.target_plmnid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_sgsn_ldn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgsn_fqdn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_fqdn.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgsn_number.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgsn_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.mme_id.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ciot_optim_supp_ind.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.into_iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j);
                    }
                }
                InformationElement::Guti(j) => {
                    if let (0, true) = (j.ins, self.guti.is_none()) {
                        self.guti = Some(j);
                    }
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.rai.is_none()) {
                        self.rai = Some(j);
                    }
                }
                InformationElement::Ptmsi(j) => {
                    if let (0, true) = (j.ins, self.ptmsi.is_none()) {
                        self.ptmsi = Some(j);
                    }
                }
                InformationElement::PtmsiSignature(j) => {
                    if let (0, true) = (j.ins, self.ptmsi_signature.is_none()) {
                        self.ptmsi_signature = Some(j);
                    }
                }
                InformationElement::CompleteRequestMessage(j) => {
                    if let (0, true) = (j.ins, self.complete_tau_req_msg.is_none()) {
                        self.complete_tau_req_msg = Some(j);
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j);
                    }
                }
                InformationElement::PortNumber(j) => {
                    if let (0, true) = (j.ins, self.udp_src_port.is_none()) {
                        self.udp_src_port = Some(j);
                    }
                }
                InformationElement::RatType(j) => {
                    if let (0, true) = (j.ins, self.rat_type.is_none()) {
                        self.rat_type = Some(j);
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j);
                    }
                }
                InformationElement::HopCounter(j) => {
                    if let (0, true) = (j.ins, self.hop_counter.is_none()) {
                        self.hop_counter = Some(j);
                    }
                }
                InformationElement::ServingNetwork(j) => {
                    if let (0, true) = (j.ins, self.target_plmnid.is_none()) {
                        self.target_plmnid = Some(j);
                    }
                }
                InformationElement::Ldn(j) => {
                    if let (0, true) = (j.ins, self.mme_sgsn_ldn.is_none()) {
                        self.mme_sgsn_ldn = Some(j);
                    }
                }
                InformationElement::Fqdn(j) => {
                    match (j.ins, self.sgsn_fqdn.is_none(), self.mme_fqdn.is_none()) {
                        (0, true, _) => self.sgsn_fqdn = Some(j),
                        (1, _, true) => self.mme_fqdn = Some(j),
                        _ => (),
                    }
                }
                InformationElement::NodeNumber(j) => {
                    if let (0, true) = (j.ins, self.sgsn_number.is_none()) {
                        self.sgsn_number = Some(j);
                    }
                }
                InformationElement::NodeIdentifier(j) => {
                    match (j.ins, self.sgsn_id.is_none(), self.mme_id.is_none()) {
                        (0, true, _) => self.sgsn_id = Some(j),
                        (1, _, true) => self.mme_id = Some(j),
                        _ => (),
                    }
                }
                InformationElement::CIoTOptimizationSupportIndication(j) => {
                    if let (0, true) = (j.ins, self.ciot_optim_supp_ind.is_none()) {
                        self.ciot_optim_supp_ind = Some(j);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_ctx_req_unmarshal() {
    let encoded: [u8; 78] = [
        0x48, 0x82, 0x00, 0x4a, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x75, 0x00, 0x0a, 0x00, 0x99, 0xf9,
        0x10, 0x01, 0x2c, 0x0a, 0xff, 0xff, 0xff, 0xff, 0x6f, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff,
        0xff, 0x74, 0x00, 0x06, 0x00, 0x01, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0x52, 0x00, 0x01, 0x00,
        0x06, 0x53, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01,
        0x62, 0x9c, 0xc4,
    ];
    let decoded = ContextRequest {
        header: Gtpv2Header {
            msgtype: CTX_REQ,
            piggyback: false,
            message_prio: None,
            length: 74,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Some(Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        }),
        guti: Some(Guti {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            mmegi: 300,
            mmec: 10,
            mtmsi: 0xffffffff,
            ..Guti::default()
        }),
        ptmsi: Some(Ptmsi {
            ptmsi: 0xffffffff,
            ..Ptmsi::default()
        }),
        complete_tau_req_msg: Some(CompleteRequestMessage {
            length: 6,
            message: RequestMessage::TauRequest(vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
            ..CompleteRequestMessage::default()
        }),
        rat_type: Some(RatType {
            rat_type: Rat::Eutran,
            ..RatType::default()
        }),
        target_plmnid: Some(ServingNetwork {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            ..ServingNetwork::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
        ..ContextRequest::default()
    };
    let message = ContextRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_ctx_req_marshal() {
    let encoded: [u8; 78] = [
        0x48, 0x82, 0x00, 0x4a, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x75, 0x00, 0x0a, 0x00, 0x99, 0xf9,
        0x10, 0x01, 0x2c, 0x0a, 0xff, 0xff, 0xff, 0xff, 0x6f, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff,
        0xff, 0x74, 0x00, 0x06, 0x00, 0x01, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0x52, 0x00, 0x01, 0x00,
        0x06, 0x53, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01,
        0x62, 0x9c, 0xc4,
    ];
    let decoded = ContextRequest {
        header: Gtpv2Header {
            msgtype: CTX_REQ,
            piggyback: false,
            message_prio: None,
            length: 74,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Some(Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        }),
        guti: Some(Guti {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            mmegi: 300,
            mmec: 10,
            mtmsi: 0xffffffff,
            ..Guti::default()
        }),
        ptmsi: Some(Ptmsi {
            ptmsi: 0xffffffff,
            ..Ptmsi::default()
        }),
        complete_tau_req_msg: Some(CompleteRequestMessage {
            length: 6,
            message: RequestMessage::TauRequest(vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
            ..CompleteRequestMessage::default()
        }),
        rat_type: Some(RatType {
            rat_type: Rat::Eutran,
            ..RatType::default()
        }),
        target_plmnid: Some(ServingNetwork {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            ..ServingNetwork::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
        ..ContextRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
