use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DL_DATA_NOTIF: u8 = 179;

// Definition of GTPv2-C Downlink Data Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownlinkDataNotification {
    pub header: Gtpv2Header,
    pub cause: Option<Cause>,
    pub ebi: Option<Ebi>,
    pub arp: Option<Arp>,
    pub imsi: Option<Imsi>,
    pub fteid_control: Option<Fteid>,
    pub indication: Option<Indication>,
    pub load_control: Vec<LoadControl>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub psi: Option<PagingServiceInfo>,
    pub dl_data_pckts_size: Option<IntegerNumber>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DownlinkDataNotification {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: DL_DATA_NOTIF,
            teid: Some(0),
            ..Gtpv2Header::default()
        };
        DownlinkDataNotification {
            header: hdr,
            cause: None,
            ebi: None,
            arp: None,
            imsi: None,
            fteid_control: None,
            indication: None,
            load_control: vec![],
            overload_info: vec![],
            psi: None,
            dl_data_pckts_size: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DownlinkDataNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DownlinkDataNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DL_DATA_NOTIF {
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

        if let Some(i) = self.cause.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.ebi.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.arp.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.imsi.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.fteid_control.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.indication.clone() {
            elements.push(i.into());
        }

        self.load_control
            .iter()
            .for_each(|x| elements.push(InformationElement::LoadControlInfo(x.clone())));

        self.overload_info
            .iter()
            .for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        if let Some(i) = self.psi.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.dl_data_pckts_size.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, true) = (j.ins, self.cause.is_none()) {
                        self.cause = Some(j.clone());
                    }
                }
                InformationElement::Ebi(j) => {
                    if let (0, true) = (j.ins, self.ebi.is_none()) {
                        self.ebi = Some(j.clone());
                    }
                }
                InformationElement::Arp(j) => {
                    if let (0, true) = (j.ins, self.arp.is_none()) {
                        self.arp = Some(j.clone());
                    }
                }
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone());
                    }
                }
                InformationElement::Fteid(j) => {
                    if let (0, true) = (j.ins, self.fteid_control.is_none()) {
                        self.fteid_control = Some(j.clone());
                    }
                }
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j.clone());
                    }
                }
                InformationElement::LoadControlInfo(j) => {
                    if j.ins == 0 {
                        self.load_control.push(j.clone());
                    }
                }
                InformationElement::OverloadControlInfo(j) => {
                    if j.ins == 0 {
                        self.overload_info.push(j.clone());
                    }
                }
                InformationElement::PagingServiceInfo(j) => {
                    if let (0, true) = (j.ins, self.psi.is_none()) {
                        self.psi = Some(j.clone());
                    }
                }
                InformationElement::IntegerNumber(j) => {
                    if let (0, true) = (j.ins, self.dl_data_pckts_size.is_none()) {
                        self.dl_data_pckts_size = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_dl_data_notification_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 129] = [
        0x48, 0xb3, 0x00, 0x7d, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x06, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x9b, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00,
        0x06, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x57, 0x00, 0x09, 0x00, 0x85, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x4d, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa,
        0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65,
        0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x12, 0x00, 0xb7,
        0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01,
        0x00, 0x7f, 0xba, 0x00, 0x03, 0x00, 0x05, 0x01, 0x03,
    ];
    let decoded = DownlinkDataNotification {
        header: Gtpv2Header {
            msgtype: DL_DATA_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 125,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Some(Cause {
            value: 0x06,
            ..Cause::default()
        }),
        ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        arp: Some(Arp {
            pci: false,
            pl: 0,
            pvi: false,
            ..Arp::default()
        }),
        imsi: Some(Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        }),
        fteid_control: Some(Fteid {
            t: 87,
            length: 9,
            ins: 0,
            interface: 5,
            teid: 0x3b95985a,
            ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
            ipv6: None,
        }),
        indication: Some(Indication {
            aosi: true,
            ..Indication::default()
        }),
        load_control: vec![LoadControl {
            t: LOAD_CNTRL,
            length: 31,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            load_metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            list: Some(vec![ApnRelativeCapacity {
                t: APN_REL_CAP,
                length: 14,
                ins: 0,
                relative_cap: 100,
                name: "test.net.com".to_string(),
            }]),
        }],
        overload_info: vec![OverloadControlInfo {
            t: OVERLOAD_CNTRL,
            length: 18,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            validity: EpcTimer {
                t: EPC_TIMER,
                length: EPC_TIMER_LENGTH as u16,
                ins: 0,
                timer_unit: 3,
                timer_value: 31,
            },
            list: None,
        }],
        psi: Some(PagingServiceInfo {
            length: 3,
            ebi: 5,
            paging_policy: Some(0x03),
            ..PagingServiceInfo::default()
        }),
        ..DownlinkDataNotification::default()
    };
    let message = DownlinkDataNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_dl_data_notification_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 129] = [
        0x48, 0xb3, 0x00, 0x7d, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x06, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x9b, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00,
        0x06, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x57, 0x00, 0x09, 0x00, 0x85, 0x3b, 0x95,
        0x98, 0x5a, 0x3e, 0x99, 0x89, 0x55, 0x4d, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa,
        0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65,
        0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0xb4, 0x00, 0x12, 0x00, 0xb7,
        0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01, 0x00, 0x60, 0x9c, 0x00, 0x01,
        0x00, 0x7f, 0xba, 0x00, 0x03, 0x00, 0x05, 0x01, 0x03,
    ];
    let decoded = DownlinkDataNotification {
        header: Gtpv2Header {
            msgtype: DL_DATA_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 125,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Some(Cause {
            value: 0x06,
            ..Cause::default()
        }),
        ebi: Some(Ebi {
            t: EBI,
            length: 1,
            ins: 0,
            value: 5,
        }),
        arp: Some(Arp {
            pci: false,
            pl: 0,
            pvi: false,
            ..Arp::default()
        }),
        imsi: Some(Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        }),
        fteid_control: Some(Fteid {
            t: 87,
            length: 9,
            ins: 0,
            interface: 5,
            teid: 0x3b95985a,
            ipv4: Some(Ipv4Addr::new(62, 153, 137, 85)),
            ipv6: None,
        }),
        indication: Some(Indication {
            aosi: true,
            ..Indication::default()
        }),
        load_control: vec![LoadControl {
            t: LOAD_CNTRL,
            length: 31,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            load_metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            list: Some(vec![ApnRelativeCapacity {
                t: APN_REL_CAP,
                length: 14,
                ins: 0,
                relative_cap: 100,
                name: "test.net.com".to_string(),
            }]),
        }],
        overload_info: vec![OverloadControlInfo {
            t: OVERLOAD_CNTRL,
            length: 18,
            ins: 0,
            sqn: Sqn {
                t: SQN,
                length: SQN_LENGTH as u16,
                ins: 0,
                sqn: 0xffaaee11,
            },
            metric: Metric {
                t: METRIC,
                length: METRIC_LENGTH as u16,
                ins: 0,
                metric: 0x60,
            },
            validity: EpcTimer {
                t: EPC_TIMER,
                length: EPC_TIMER_LENGTH as u16,
                ins: 0,
                timer_unit: 3,
                timer_value: 31,
            },
            list: None,
        }],
        psi: Some(PagingServiceInfo {
            ebi: 5,
            paging_policy: Some(0x03),
            ..PagingServiceInfo::default()
        }),
        ..DownlinkDataNotification::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
