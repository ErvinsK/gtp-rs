use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DL_DATA_NOTIF_ACK: u8 = 177;

// Definition of GTPv2-C Downlink Data Notification Acknowledge Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownlinkDataNotificationAcknowledge {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub data_notification_delay: Option<DelayValue>,
    pub recovery: Option<Recovery>,
    pub dl_low_prio_traffic_throttling: Option<Throttling>,
    pub imsi: Option<Imsi>,
    pub dl_buff_dur: Option<EpcTimer>,
    pub dl_buff_sugg_packet_count: Option<IntegerNumber>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DownlinkDataNotificationAcknowledge {
    fn default() -> DownlinkDataNotificationAcknowledge {
        DownlinkDataNotificationAcknowledge {
            header: Gtpv2Header {
                msgtype: DL_DATA_NOTIF_ACK,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            cause: Cause::default(),
            data_notification_delay: None,
            recovery: None,
            dl_low_prio_traffic_throttling: None,
            imsi: None,
            dl_buff_dur: None,
            dl_buff_sugg_packet_count: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DownlinkDataNotificationAcknowledge {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DownlinkDataNotificationAcknowledge::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DL_DATA_NOTIF_ACK {
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

        if let Some(i) = self.data_notification_delay.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.dl_low_prio_traffic_throttling.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.dl_buff_dur.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.dl_buff_sugg_packet_count.clone() {
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
                InformationElement::Cause(j) => {
                    if j.ins == 0 {
                        mandatory = true;
                        self.cause = j.clone();
                    };
                }
                InformationElement::DelayValue(j) => {
                    if let (0, true) = (j.ins, self.data_notification_delay.is_none()) {
                        self.data_notification_delay = Some(j.clone())
                    };
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
                }
                InformationElement::Throttling(j) => {
                    if let (0, true) = (j.ins, self.dl_low_prio_traffic_throttling.is_none()) {
                        self.dl_low_prio_traffic_throttling = Some(j.clone())
                    };
                }
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone())
                    };
                }
                InformationElement::EpcTimer(j) => {
                    if let (0, true) = (j.ins, self.dl_buff_dur.is_none()) {
                        self.dl_buff_dur = Some(j.clone())
                    };
                }
                InformationElement::IntegerNumber(j) => {
                    if let (0, true) = (j.ins, self.dl_buff_sugg_packet_count.is_none()) {
                        self.dl_buff_sugg_packet_count = Some(j.clone())
                    };
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
fn test_dl_data_notif_ack_unmarshal() {
    let encoded: [u8; 64] = [
        0x48, 0xb1, 0x00, 0x3c, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x5c, 0x00, 0x01, 0x00, 0x01, 0x03, 0x00, 0x01, 0x00, 0x64, 0x9a, 0x00,
        0x02, 0x00, 0x22, 0x03, 0x01, 0x00, 0x06, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x9c,
        0x00, 0x01, 0x00, 0x64, 0xbb, 0x00, 0x01, 0x00, 0xff, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = DownlinkDataNotificationAcknowledge {
        header: Gtpv2Header {
            msgtype: DL_DATA_NOTIF_ACK,
            piggyback: false,
            message_prio: None,
            length: 60,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        data_notification_delay: Some(DelayValue {
            value: 0x01,
            ..DelayValue::default()
        }),
        recovery: Some(Recovery {
            recovery: 100,
            ..Recovery::default()
        }),
        dl_low_prio_traffic_throttling: Some(Throttling {
            delay_unit: 0x01,
            delay_value: 0x02,
            factor: 0x03,
            ..Throttling::default()
        }),
        imsi: Some(Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        }),
        dl_buff_dur: Some(EpcTimer {
            timer_unit: 0x03,
            timer_value: 0x04,
            ..EpcTimer::default()
        }),
        dl_buff_sugg_packet_count: Some(IntegerNumber {
            length: 1,
            number: vec![0xff],
            ..IntegerNumber::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..PrivateExtension::default()
        }],
    };
    let message = DownlinkDataNotificationAcknowledge::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_dl_data_notif_ack_marshal() {
    let encoded: [u8; 64] = [
        0x48, 0xb1, 0x00, 0x3c, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x5c, 0x00, 0x01, 0x00, 0x01, 0x03, 0x00, 0x01, 0x00, 0x64, 0x9a, 0x00,
        0x02, 0x00, 0x22, 0x03, 0x01, 0x00, 0x06, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x9c,
        0x00, 0x01, 0x00, 0x64, 0xbb, 0x00, 0x01, 0x00, 0xff, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = DownlinkDataNotificationAcknowledge {
        header: Gtpv2Header {
            msgtype: DL_DATA_NOTIF_ACK,
            piggyback: false,
            message_prio: None,
            length: 60,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        data_notification_delay: Some(DelayValue {
            value: 0x01,
            ..DelayValue::default()
        }),
        recovery: Some(Recovery {
            recovery: 100,
            ..Recovery::default()
        }),
        dl_low_prio_traffic_throttling: Some(Throttling {
            delay_unit: 0x01,
            delay_value: 0x02,
            factor: 0x03,
            ..Throttling::default()
        }),
        imsi: Some(Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        }),
        dl_buff_dur: Some(EpcTimer {
            timer_unit: 0x01,
            timer_value: 0x64,
            ..EpcTimer::default()
        }),
        dl_buff_sugg_packet_count: Some(IntegerNumber {
            length: 1,
            number: vec![0xff],
            ..IntegerNumber::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..PrivateExtension::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
