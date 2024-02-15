use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const CREATE_IND_DATA_FW_TUN_REQ: u8 = 166;

// Definition of GTPv2-C Create Indirect Data Forwarding Tunnel Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateIndirectDataForwardingTunnelRequest {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub mei: Option<Mei>,
    pub indication: Option<Indication>,
    pub fteid_control: Option<Fteid>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateIndirectDataForwardingTunnelRequest {
    fn default() -> CreateIndirectDataForwardingTunnelRequest {
        CreateIndirectDataForwardingTunnelRequest {
            header: Gtpv2Header {
                msgtype: CREATE_IND_DATA_FW_TUN_REQ,
                teid: Some(0),
                ..Default::default()
            },
            imsi: None,
            mei: None,
            indication: None,
            fteid_control: None,
            bearer_ctxs: vec![BearerContext::default(),],
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CreateIndirectDataForwardingTunnelRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateIndirectDataForwardingTunnelRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_IND_DATA_FW_TUN_REQ {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        let offset = message.header.length as usize + MANDATORY_HDR_LENGTH;

        if buffer.len() >= offset{
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

        if let Some(i) = self.mei.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone())
                    };
                }
                InformationElement::Mei(j) => {
                    if let (0, true) = (j.ins, self.mei.is_none()) {
                        self.mei = Some(j.clone())
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
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        mandatory = true;
                        self.bearer_ctxs.push(j.clone());
                    }
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX))
        }
    }
}

#[test]
fn test_create_indirect_data_fw_tunnel_req_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 97] = [
        0x48, 0x5f, 0x00, 0x5d, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81,
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 0x5d, 0x00, 0x34, 0x00,
        0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e,
        0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e,
        0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateBearerRequest {
        header: Gtpv2Header {
            msgtype: CREATE_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 93,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        linked_ebi: Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 5,
        },
        pco: Some(Pco {
            t: PCO,
            length: 20,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
                0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61,
            ],
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
        }],
        ..CreateBearerRequest::default()
    };

    let message = CreateBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_indirect_data_fw_tunnel_req_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 97] = [
        0x48, 0x5f, 0x00, 0x5d, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x4e, 0x00, 0x14, 0x00, 0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81,
        0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61, 0x5d, 0x00, 0x34, 0x00,
        0x49, 0x00, 0x01, 0x00, 0x00, 0x57, 0x00, 0x09, 0x02, 0x85, 0x3b, 0x95, 0x98, 0x5a, 0x3e,
        0x99, 0x89, 0x55, 0x50, 0x00, 0x16, 0x00, 0x2c, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e,
        0x00, 0x04, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateBearerRequest {
        header: Gtpv2Header {
            msgtype: CREATE_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 93,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        linked_ebi: Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 5,
        },
        pco: Some(Pco {
            t: PCO,
            length: 20,
            ins: 0,
            pco: vec![
                0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08,
                0x83, 0x06, 0x0a, 0x40, 0xd0, 0x61,
            ],
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
        }],
        ..CreateBearerRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
