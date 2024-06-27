use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MBMS_SESSION_UPD_REQ: u8 = 233;

// Definition of GTPv2-C MBMS Session Update Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsSessionUpdateRequest {
    pub header: Gtpv2Header,
    pub mbms_svc_area: Option<MbmsServiceArea>,
    pub tmgi: Tmgi,
    pub fteid_control: Option<Fteid>,
    pub mbms_sess_dur: MbmsSessionDuration,
    pub qos_profile: BearerQos,
    pub mbms_sess_id: Option<MbmsSessionId>,
    pub mbms_flow_id: Option<MbmsFlowId>,
    pub mbms_time_to_data_transfer: Option<MbmsTimeToDataTransfer>,
    pub mbms_data_transfer_sus: Option<AbsoluteTimeMbmsDataTransfer>, // MBMS Data Transfer Start/Update/Stop
    pub mbms_cell_list: Option<EcgiList>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for MbmsSessionUpdateRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MBMS_SESSION_UPD_REQ,
            teid: Some(0),
            ..Default::default()
        };
        MbmsSessionUpdateRequest {
            header: hdr,
            mbms_svc_area: None,
            tmgi: Tmgi::default(),
            fteid_control: None,
            mbms_sess_dur: MbmsSessionDuration::default(),
            qos_profile: BearerQos::default(),
            mbms_sess_id: None,
            mbms_flow_id: None,
            mbms_time_to_data_transfer: None,
            mbms_data_transfer_sus: None,
            mbms_cell_list: None,
            private_ext: vec![],
        }
    }
}

impl Messages for MbmsSessionUpdateRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = MbmsSessionUpdateRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MBMS_SESSION_UPD_REQ {
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
        let mut elements = Vec::new();

        if let Some(i) = self.mbms_svc_area.clone() {
            elements.push(i.into());
        }

        elements.push(self.tmgi.clone().into());

        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into());
        }

        elements.push(self.mbms_sess_dur.clone().into());

        elements.push(self.qos_profile.clone().into());

        if let Some(i) = self.mbms_sess_id.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mbms_flow_id.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mbms_time_to_data_transfer.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mbms_data_transfer_sus.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mbms_cell_list.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 3] = [false; 3];
        for e in elements.iter() {
            match e {
                InformationElement::MbmsSa(j) => {
                    if let (0, true) = (j.ins, self.mbms_svc_area.is_none()) {
                        self.mbms_svc_area = Some(j.clone());
                    }
                }
                InformationElement::Tmgi(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        self.tmgi = j.clone();
                        mandatory[0] = true;
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone());
                    }
                }
                InformationElement::MbmsSd(j) => {
                    if let (0, false) = (j.ins, mandatory[1]) {
                        self.mbms_sess_dur = j.clone();
                        mandatory[1] = true;
                    }
                }
                InformationElement::BearerQos(j) => {
                    if let (0, false) = (j.ins, mandatory[2]) {
                        self.qos_profile = j.clone();
                        mandatory[2] = true;
                    }
                }
                InformationElement::MbmsSessionId(j) => {
                    if let (0, true) = (j.ins, self.mbms_sess_id.is_none()) {
                        self.mbms_sess_id = Some(j.clone());
                    }
                }
                InformationElement::MbmsFlowId(j) => {
                    if let (0, true) = (j.ins, self.mbms_flow_id.is_none()) {
                        self.mbms_flow_id = Some(j.clone());
                    }
                }
                InformationElement::MbmsTimeToDataTransfer(j) => {
                    if let (0, true) = (j.ins, self.mbms_time_to_data_transfer.is_none()) {
                        self.mbms_time_to_data_transfer = Some(j.clone());
                    }
                }
                InformationElement::AbsoluteTimeMbmsDataTransfer(j) => {
                    if let (0, true) = (j.ins, self.mbms_data_transfer_sus.is_none()) {
                        self.mbms_data_transfer_sus = Some(j.clone());
                    }
                }
                InformationElement::EcgiList(j) => {
                    if let (0, true) = (j.ins, self.mbms_cell_list.is_none()) {
                        self.mbms_cell_list = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if let Some(j) = mandatory.iter().position(|&x| !x) {
            match j {
                0 => Err(GTPV2Error::MessageMandatoryIEMissing(TMGI)),
                1 => Err(GTPV2Error::MessageMandatoryIEMissing(MBMSSD)),
                2 => Err(GTPV2Error::MessageMandatoryIEMissing(BEARERQOS)),
                _ => Err(GTPV2Error::MessageMandatoryIEMissing(0)),
            }
        } else {
            Ok(true)
        }
    }
}

#[test]
fn test_mbms_session_update_req_unmarshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 148] = [
        0x48, 0xe9, 0x00, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x8b, 0x00, 0x05,
        0x00, 0x01, 0x00, 0x00, 0xff, 0xff, 0x9e, 0x00, 0x06, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x57, 0x00, 0x19, 0x00, 0xca, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a,
        0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27,
        0x8a, 0x00, 0x03, 0x00, 0x00, 0xc8, 0x0a, 0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x8c, 0x00, 0x01, 0x00, 0x0a, 0x8d, 0x00, 0x02, 0x00, 0x0a, 0x0f, 0x99,
        0x00, 0x01, 0x00, 0xff, 0xa4, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xbe, 0x00, 0x17, 0x00, 0x00, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x62,
        0xf2, 0x10, 0x01, 0xba, 0x40, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x01,
    ];
    let decoded = MbmsSessionUpdateRequest {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_UPD_REQ,
            piggyback: false,
            message_prio: None,
            length: 144,
            teid: Some(0),
            sqn: 0x68,
        },
        mbms_svc_area: Some(MbmsServiceArea {
            length: 5,
            mbms_sa: vec![0, 0xffff],
            ..MbmsServiceArea::default()
        }),
        tmgi: Tmgi {
            tmgi: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
            ..Tmgi::default()
        },
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        mbms_sess_dur: MbmsSessionDuration {
            seconds: 400,
            days: 10,
            ..MbmsSessionDuration::default()
        },
        qos_profile: BearerQos {
            pre_emption_vulnerability: 0,
            priority_level: 9,
            pre_emption_capability: 1,
            qci: 9,
            maxbr_ul: 0,
            maxbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
            ..BearerQos::default()
        },
        mbms_sess_id: Some(MbmsSessionId {
            mbms_sessionid: 0x0a,
            ..MbmsSessionId::default()
        }),
        mbms_flow_id: Some(MbmsFlowId {
            mbms_flowid: 0x0a0f,
            ..MbmsFlowId::default()
        }),
        mbms_time_to_data_transfer: Some(MbmsTimeToDataTransfer {
            mbms_time_to_data: 0xff,
            ..MbmsTimeToDataTransfer::default()
        }),
        mbms_data_transfer_sus: Some(AbsoluteTimeMbmsDataTransfer {
            seconds: 0xffff,
            ..AbsoluteTimeMbmsDataTransfer::default()
        }),
        mbms_cell_list: Some(EcgiList {
            length: 23,
            ecgi_list: Some(vec![
                Ecgi {
                    mcc: 262,
                    mnc: 1,
                    eci: 28983298,
                },
                Ecgi {
                    mcc: 262,
                    mnc: 1,
                    eci: 28983299,
                },
                Ecgi {
                    mcc: 262,
                    mnc: 1,
                    eci: 28983297,
                },
            ]),
            ..EcgiList::default()
        }),
        ..MbmsSessionUpdateRequest::default()
    };
    let message = MbmsSessionUpdateRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_mmbms_session_update_req_marshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 148] = [
        0x48, 0xe9, 0x00, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x8b, 0x00, 0x05,
        0x00, 0x01, 0x00, 0x00, 0xff, 0xff, 0x9e, 0x00, 0x06, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x57, 0x00, 0x19, 0x00, 0xca, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a,
        0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27,
        0x8a, 0x00, 0x03, 0x00, 0x00, 0xc8, 0x0a, 0x50, 0x00, 0x16, 0x00, 0x64, 0x09, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x8c, 0x00, 0x01, 0x00, 0x0a, 0x8d, 0x00, 0x02, 0x00, 0x0a, 0x0f, 0x99,
        0x00, 0x01, 0x00, 0xff, 0xa4, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xbe, 0x00, 0x17, 0x00, 0x00, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, 0x62,
        0xf2, 0x10, 0x01, 0xba, 0x40, 0x03, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x01,
    ];
    let decoded = MbmsSessionUpdateRequest {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_UPD_REQ,
            piggyback: false,
            message_prio: None,
            length: 144,
            teid: Some(0),
            sqn: 0x68,
        },
        mbms_svc_area: Some(MbmsServiceArea {
            length: 5,
            mbms_sa: vec![0, 0xffff],
            ..MbmsServiceArea::default()
        }),
        tmgi: Tmgi {
            tmgi: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
            ..Tmgi::default()
        },
        fteid_control: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        mbms_sess_dur: MbmsSessionDuration {
            seconds: 400,
            days: 10,
            ..MbmsSessionDuration::default()
        },
        qos_profile: BearerQos {
            pre_emption_vulnerability: 0,
            priority_level: 9,
            pre_emption_capability: 1,
            qci: 9,
            maxbr_ul: 0,
            maxbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
            ..BearerQos::default()
        },
        mbms_sess_id: Some(MbmsSessionId {
            mbms_sessionid: 0x0a,
            ..MbmsSessionId::default()
        }),
        mbms_flow_id: Some(MbmsFlowId {
            mbms_flowid: 0x0a0f,
            ..MbmsFlowId::default()
        }),
        mbms_time_to_data_transfer: Some(MbmsTimeToDataTransfer {
            mbms_time_to_data: 0xff,
            ..MbmsTimeToDataTransfer::default()
        }),
        mbms_data_transfer_sus: Some(AbsoluteTimeMbmsDataTransfer {
            seconds: 0xffff,
            ..AbsoluteTimeMbmsDataTransfer::default()
        }),
        mbms_cell_list: Some(EcgiList {
            length: 23,
            ecgi_list: Some(vec![
                Ecgi {
                    mcc: 262,
                    mnc: 1,
                    eci: 28983298,
                },
                Ecgi {
                    mcc: 262,
                    mnc: 1,
                    eci: 28983299,
                },
                Ecgi {
                    mcc: 262,
                    mnc: 1,
                    eci: 28983297,
                },
            ]),
            ..EcgiList::default()
        }),
        ..MbmsSessionUpdateRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
