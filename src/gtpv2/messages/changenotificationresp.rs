use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CHNG_NOTIF_RESP: u8 = 39;

// Definition of GTPv2-C Change Notification Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeNotificationResponse {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub mei: Option<Mei>,
    pub cause: Cause,
    pub cra: Option<ChangeReportingAction>,
    pub csg_ira: Option<CSGInformationReportingAction>,
    pub praa: Option<PresenceReportingAreaAction>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ChangeNotificationResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: CHNG_NOTIF_RESP,
            teid: Some(0),
            ..Default::default()
        };
        ChangeNotificationResponse {
            header: hdr,
            imsi: None,
            mei: None,
            cause: Cause::default(),
            cra: None,
            csg_ira: None,
            praa: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ChangeNotificationResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ChangeNotificationResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CHNG_NOTIF_RESP {
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
            elements.push(i.into());
        }
        if let Some(i) = self.mei.clone() {
            elements.push(i.into());
        }

        elements.push(self.cause.clone().into());

        if let Some(i) = self.cra.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.csg_ira.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.praa.clone() {
            elements.push(i.into());
        }

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
                        self.imsi = Some(j.clone());
                    }
                }
                InformationElement::Mei(j) => {
                    if let (0, true) = (j.ins, self.mei.is_none()) {
                        self.mei = Some(j.clone());
                    }
                }
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.cause, mandatory) = (j.clone(), true);
                    }
                }
                InformationElement::ChangeReportingAction(j) => {
                    if let (0, true) = (j.ins, self.cra.is_none()) {
                        self.cra = Some(j.clone());
                    }
                }
                InformationElement::CSGInformationReportingAction(j) => {
                    if let (0, true) = (j.ins, self.csg_ira.is_none()) {
                        self.csg_ira = Some(j.clone());
                    }
                }
                InformationElement::PresenceReportingAreaAction(j) => {
                    if let (0, true) = (j.ins, self.praa.is_none()) {
                        self.praa = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
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
fn test_change_notification_resp_unmarshal() {
    let encoded: [u8; 118] = [
        0x48, 0x27, 0x00, 0x72, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x49,
        0x29, 0x50, 0x01, 0x50, 0x94, 0x70, 0x02, 0x00, 0x02, 0x00, 0x10, 0x00, 0x83, 0x00, 0x01,
        0x00, 0x01, 0x92, 0x00, 0x01, 0x00, 0x07, 0xb1, 0x00, 0x3e, 0x00, 0x01, 0xff, 0xff, 0xff,
        0x11, 0x01, 0x01, 0x01, 0x01, 0x01, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0xff,
        0xff, 0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xff, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02,
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, 0x62,
        0xf2, 0x10, 0xff, 0xff, 0xaa, 0xaa, 0x01, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff,
    ];
    let decoded = ChangeNotificationResponse {
        header: Gtpv2Header {
            msgtype: CHNG_NOTIF_RESP,
            piggyback: false,
            message_prio: None,
            length: 114,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Some(Imsi {
            t: IMSI,
            length: 8,
            ins: 0,
            imsi: "901405101073874".to_string(),
        }),
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8694920510054907".to_string(),
        }),
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 16,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        cra: Some(ChangeReportingAction {
            t: CHANGE_RPRT,
            length: CHANGE_RPRT_LENGTH as u16,
            ins: 0,
            action: 1,
        }),
        csg_ira: Some(CSGInformationReportingAction {
            t: CSG_INFO_REPORT,
            length: 1,
            ins: 0,
            action: 7,
        }),
        praa: Some(PresenceReportingAreaAction {
            t: PRAA,
            length: 62,
            ins: 0,
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
                macro_id: 0xffffff,
            }],
            home_enb: vec![MacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0xffffff,
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
        }),
        ..ChangeNotificationResponse::default()
    };
    let message = ChangeNotificationResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_change_notification_resp_marshal() {
    let encoded: [u8; 118] = [
        0x48, 0x27, 0x00, 0x72, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x49,
        0x29, 0x50, 0x01, 0x50, 0x94, 0x70, 0x02, 0x00, 0x02, 0x00, 0x10, 0x00, 0x83, 0x00, 0x01,
        0x00, 0x01, 0x92, 0x00, 0x01, 0x00, 0x07, 0xb1, 0x00, 0x3e, 0x00, 0x01, 0xff, 0xff, 0xff,
        0x11, 0x01, 0x01, 0x01, 0x01, 0x01, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 0xf2, 0x10, 0xff,
        0xff, 0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xff, 0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02,
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, 0x62,
        0xf2, 0x10, 0xff, 0xff, 0xaa, 0xaa, 0x01, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff,
    ];
    let decoded = ChangeNotificationResponse {
        header: Gtpv2Header {
            msgtype: CHNG_NOTIF_RESP,
            piggyback: false,
            message_prio: None,
            length: 114,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Some(Imsi {
            t: IMSI,
            length: 8,
            ins: 0,
            imsi: "901405101073874".to_string(),
        }),
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8694920510054907".to_string(),
        }),
        cause: Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 16,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        },
        cra: Some(ChangeReportingAction {
            t: CHANGE_RPRT,
            length: CHANGE_RPRT_LENGTH as u16,
            ins: 0,
            action: 1,
        }),
        csg_ira: Some(CSGInformationReportingAction {
            t: CSG_INFO_REPORT,
            length: 1,
            ins: 0,
            action: 7,
        }),
        praa: Some(PresenceReportingAreaAction {
            t: PRAA,
            length: 62,
            ins: 0,
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
                macro_id: 0xffffff,
            }],
            home_enb: vec![MacroEnbId {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                macro_id: 0xffffff,
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
        }),
        ..ChangeNotificationResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
