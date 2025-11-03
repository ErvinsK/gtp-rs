use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CHNG_NOTIF_REQ: u8 = 38;

// Definition of GTPv2-C Change Notification Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeNotificationRequest {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub mei: Option<Mei>,
    pub indication: Option<Indication>,
    pub rattype: RatType,
    pub uli: Option<Uli>,
    pub uci: Option<Uci>,
    pub pgw_addr_control: Option<IpAddress>,
    pub linked_ebi: Option<Ebi>,
    pub prai: Option<PresenceReportingAreaInformation>,
    pub mo_exception_data_counter: Option<Counter>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,
    pub pscellid: Option<PSCellId>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ChangeNotificationRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: CHNG_NOTIF_REQ,
            teid: Some(0),
            ..Default::default()
        };
        ChangeNotificationRequest {
            header: hdr,
            imsi: None,
            mei: None,
            indication: None,
            rattype: RatType::default(),
            uli: None,
            uci: None,
            pgw_addr_control: None,
            linked_ebi: None,
            prai: None,
            mo_exception_data_counter: None,
            secondary_rat_usage_report: vec![],
            pscellid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ChangeNotificationRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ChangeNotificationRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CHNG_NOTIF_REQ {
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
        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }

        elements.push(self.rattype.clone().into());

        if let Some(i) = self.uli.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.uci.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.pgw_addr_control.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.linked_ebi.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.prai.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.mo_exception_data_counter.clone() {
            elements.push(i.into());
        }

        self.secondary_rat_usage_report.iter().for_each(|x| {
            elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone()))
        });

        if let Some(i) = self.pscellid.clone() {
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
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
                    }
                }
                InformationElement::RatType(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.rattype, mandatory) = (j.clone(), true);
                    }
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.uli.is_none()) {
                        self.uli = Some(j.clone());
                    }
                }
                InformationElement::Uci(j) => {
                    if let (0, true) = (j.ins, self.uci.is_none()) {
                        self.uci = Some(j.clone());
                    }
                }
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.pgw_addr_control.is_none()) {
                        self.pgw_addr_control = Some(j.clone());
                    }
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.linked_ebi.is_none()) {
                        self.linked_ebi = Some(j.clone());
                    }
                }
                InformationElement::PresenceReportingAreaInformation(j) => {
                    if let (0, true) = (j.ins, self.prai.is_none()) {
                        self.prai = Some(j.clone());
                    }
                }
                InformationElement::Counter(j) => {
                    if let (0, true) = (j.ins, self.mo_exception_data_counter.is_none()) {
                        self.mo_exception_data_counter = Some(j.clone());
                    }
                }
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j.clone());
                    }
                }
                InformationElement::PSCellId(j) => {
                    if let (0, true) = (j.ins, self.pscellid.is_none()) {
                        self.pscellid = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(RATTYPE))
        }
    }
}

#[test]
fn test_change_notification_req_unmarshal() {
    let encoded: [u8; 125] = [
        0x48, 0x26, 0x00, 0x79, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x49,
        0x29, 0x50, 0x01, 0x50, 0x94, 0x70, 0x52, 0x00, 0x01, 0x00, 0x06, 0x56, 0x00, 0x0d, 0x00,
        0x18, 0x32, 0xf4, 0x02, 0x0d, 0x59, 0x32, 0xf4, 0x02, 0x00, 0xc5, 0x58, 0x02, 0x49, 0x00,
        0x01, 0x00, 0x05, 0xb2, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0xff, 0x02,
        0xc7, 0x00, 0x05, 0x00, 0xee, 0x6b, 0x28, 0x00, 0x09, 0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00,
        0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07,
        0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ChangeNotificationRequest {
        header: Gtpv2Header {
            msgtype: CHNG_NOTIF_REQ,
            piggyback: false,
            message_prio: None,
            length: 121,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
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
        rattype: RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        },
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
        linked_ebi: Some(Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 5,
        }),
        prai: Some(PresenceReportingAreaInformation {
            t: PRAI,
            length: 8,
            ins: 0,
            prai: PresenceReportingArea::Ipra(0x00),
            add_prai: Some(vec![PresenceReportingArea::Opra(0xff)]),
        }),
        mo_exception_data_counter: Some(Counter {
            t: COUNTER,
            length: COUNTER_LENGTH as u16,
            ins: 0,
            timestamp: 4000000000,
            counter: 9,
        }),
        secondary_rat_usage_report: vec![SecondaryRatUsageDataReport {
            t: SCND_RAT_UDR,
            length: SCND_RAT_UDR_LENGTH as u16,
            ins: 0,
            irsgw: true,
            irpgw: true,
            rat_type: 0,
            ebi: 5,
            start_timestamp: 0xff,
            end_timestamp: 0xffff,
            usg_data_dl: 0xffffff00,
            usg_data_ul: 0xffff,
            ..SecondaryRatUsageDataReport::default()
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ChangeNotificationRequest::default()
    };
    let message = ChangeNotificationRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_change_notification_req_marshal() {
    let encoded: [u8; 125] = [
        0x48, 0x26, 0x00, 0x79, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x49,
        0x29, 0x50, 0x01, 0x50, 0x94, 0x70, 0x52, 0x00, 0x01, 0x00, 0x06, 0x56, 0x00, 0x0d, 0x00,
        0x18, 0x32, 0xf4, 0x02, 0x0d, 0x59, 0x32, 0xf4, 0x02, 0x00, 0xc5, 0x58, 0x02, 0x49, 0x00,
        0x01, 0x00, 0x05, 0xb2, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0xff, 0x02,
        0xc7, 0x00, 0x05, 0x00, 0xee, 0x6b, 0x28, 0x00, 0x09, 0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00,
        0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07,
        0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ChangeNotificationRequest {
        header: Gtpv2Header {
            msgtype: CHNG_NOTIF_REQ,
            piggyback: false,
            message_prio: None,
            length: 121,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
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
        rattype: RatType {
            t: RATTYPE,
            length: 1,
            ins: 0,
            rat_type: Rat::Eutran,
        },
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
        linked_ebi: Some(Ebi {
            t: EBI,
            length: EBI_LENGTH as u16,
            ins: 0,
            value: 5,
        }),
        prai: Some(PresenceReportingAreaInformation {
            t: PRAI,
            length: 8,
            ins: 0,
            prai: PresenceReportingArea::Ipra(0x00),
            add_prai: Some(vec![PresenceReportingArea::Opra(0xff)]),
        }),
        mo_exception_data_counter: Some(Counter {
            t: COUNTER,
            length: COUNTER_LENGTH as u16,
            ins: 0,
            timestamp: 4000000000,
            counter: 9,
        }),
        secondary_rat_usage_report: vec![SecondaryRatUsageDataReport {
            t: SCND_RAT_UDR,
            length: SCND_RAT_UDR_LENGTH as u16,
            ins: 0,
            irsgw: true,
            irpgw: true,
            rat_type: 0,
            ebi: 5,
            start_timestamp: 0xff,
            end_timestamp: 0xffff,
            usg_data_dl: 0xffffff00,
            usg_data_ul: 0xffff,
            ..SecondaryRatUsageDataReport::default()
        }],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..ChangeNotificationRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
