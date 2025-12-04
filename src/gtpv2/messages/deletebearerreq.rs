use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DELETE_BEARER_REQ: u8 = 99;

// Definition of GTPv2-C Delete Bearer Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteBearerRequest {
    pub header: Gtpv2Header,
    pub linked_ebi: Option<Ebi>,
    pub ebi: Option<Ebi>,
    pub bearer_ctxs: Vec<BearerContext>,
    pub pti: Option<Pti>,
    pub pco: Option<Pco>,
    pub pgw_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub cause: Option<Cause>,
    pub indication: Option<Indication>,
    pub load_control: Vec<LoadControl>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub nbifom: Option<Fcontainer>,
    pub apn_rate_control_status: Option<ApnRateControlStatus>,
    pub epco: Option<Epco>,
    pub pgw_change_info: Vec<PgwChangeInfo>,
    pub fteid_control: Option<Fteid>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DeleteBearerRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: DELETE_BEARER_REQ,
            teid: Some(0),
            ..Default::default()
        };
        DeleteBearerRequest {
            header: hdr,
            linked_ebi: None,
            ebi: None,
            bearer_ctxs: vec![],
            pti: None,
            pco: None,
            pgw_fqcsid: None,
            sgw_fqcsid: None,
            cause: None,
            indication: None,
            load_control: vec![],
            overload_info: vec![],
            nbifom: None,
            apn_rate_control_status: None,
            epco: None,
            pgw_change_info: vec![],
            fteid_control: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DeleteBearerRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteBearerRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_BEARER_REQ {
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

        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ebi.clone() {
            elements.push(i.into())
        };

        self.bearer_ctxs
            .iter()
            .for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        if let Some(i) = self.pti.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pco.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.pgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.cause.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        self.load_control
            .iter()
            .for_each(|x| elements.push(InformationElement::LoadControlInfo(x.clone())));

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.nbifom.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.apn_rate_control_status.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.epco.clone() {
            elements.push(i.into())
        };

        self.pgw_change_info
            .iter()
            .for_each(|x| elements.push(InformationElement::PgwChangeInfo(x.clone())));

        if let Some(i) = self.fteid_control.clone() {
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
                InformationElement::Ebi(j) => {
                    // 2 instances
                    match (j.ins, self.linked_ebi.is_none(), self.ebi.is_none()) {
                        (0, true, _) => self.linked_ebi = Some(j),
                        (1, _, true) => self.ebi = Some(j),
                        _ => (),
                    }
                }
                InformationElement::BearerContext(j) => {
                    if j.ins == 0 {
                        self.bearer_ctxs.push(j)
                    };
                }
                InformationElement::Pti(j) => {
                    if let (0, true) = (j.ins, self.pti.is_none()) {
                        self.pti = Some(j)
                    };
                }
                InformationElement::Pco(j) => {
                    if let (0, true) = (j.ins, self.pco.is_none()) {
                        self.pco = Some(j)
                    };
                }
                InformationElement::Fqcsid(j) => {
                    // 2 instances
                    match (j.ins, self.pgw_fqcsid.is_none(), self.sgw_fqcsid.is_none()) {
                        (0, true, _) => self.pgw_fqcsid = Some(j),
                        (1, _, true) => self.sgw_fqcsid = Some(j),
                        _ => (),
                    }
                }
                InformationElement::Cause(j) => {
                    if let (0, true) = (j.ins, self.cause.is_none()) {
                        self.cause = Some(j)
                    };
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j)
                    };
                }
                InformationElement::LoadControlInfo(j) => {
                    if j.ins < 3 {
                        self.load_control.push(j)
                    };
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins < 2 {
                        self.overload_info.push(j)
                    };
                }
                InformationElement::Fcontainer(j) => {
                    if let (0, true) = (j.ins, self.nbifom.is_none()) {
                        self.nbifom = Some(j)
                    };
                }
                InformationElement::ApnRateControlStatus(j) => {
                    if let (0, true) = (j.ins, self.apn_rate_control_status.is_none()) {
                        self.apn_rate_control_status = Some(j)
                    };
                }
                InformationElement::Epco(j) => {
                    if let (0, true) = (j.ins, self.epco.is_none()) {
                        self.epco = Some(j)
                    };
                }
                InformationElement::PgwChangeInfo(j) => {
                    if j.ins == 2 {
                        self.pgw_change_info.push(j)
                    };
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j)
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_delete_bearer_req_unmarshal() {
    let encoded: [u8; 43] = [
        0x48, 0x63, 0x00, 0x27, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x5d, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x02, 0x00, 0x08, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x64, 0x00, 0x01, 0x00, 0xfa, 0x02, 0x00, 0x02, 0x00, 0x08, 0x00,
    ];
    let decoded = DeleteBearerRequest {
        header: Gtpv2Header {
            msgtype: DELETE_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 39,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Some(Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 8,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        }),
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        pti: Some(Pti {
            t: PTI,
            length: PTI_LENGTH as u16,
            ins: 0,
            pti: 0xfa,
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 11,
            ins: 0,
            cause: Some(Cause {
                t: CAUSE,
                length: 2,
                ins: 0,
                value: 8,
                pce: false,
                bce: false,
                cs: false,
                offend_ie_type: None,
            }),
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
            fteids: vec![],
            bearer_qos: None,
            ..BearerContext::default()
        }],
        ..DeleteBearerRequest::default()
    };
    let message = DeleteBearerRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_delete_bearer_req_marshal() {
    let encoded: [u8; 43] = [
        0x48, 0x63, 0x00, 0x27, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x5d, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x02, 0x00, 0x08, 0x00, 0x49, 0x00, 0x01,
        0x00, 0x05, 0x64, 0x00, 0x01, 0x00, 0xfa, 0x02, 0x00, 0x02, 0x00, 0x08, 0x00,
    ];
    let decoded = DeleteBearerRequest {
        header: Gtpv2Header {
            msgtype: DELETE_BEARER_REQ,
            piggyback: false,
            message_prio: None,
            length: 39,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        cause: Some(Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 8,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        }),
        linked_ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        pti: Some(Pti {
            t: PTI,
            length: PTI_LENGTH as u16,
            ins: 0,
            pti: 0xfa,
        }),
        bearer_ctxs: vec![BearerContext {
            t: BEARER_CTX,
            length: 11,
            ins: 0,
            cause: Some(Cause {
                t: CAUSE,
                length: 2,
                ins: 0,
                value: 8,
                pce: false,
                bce: false,
                cs: false,
                offend_ie_type: None,
            }),
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
            fteids: vec![],
            bearer_qos: None,
            ..BearerContext::default()
        }],
        ..DeleteBearerRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
