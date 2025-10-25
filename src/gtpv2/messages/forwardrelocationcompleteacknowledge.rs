use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const FWD_RELOC_COMPLETE_ACK: u8 = 136;

// Definition of GTPv2-C Forward Relocation Complete Acknowledge Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardRelocationCompleteAcknowledge {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub scnd_rat_udrs: Vec<SecondaryRatUsageDataReport>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ForwardRelocationCompleteAcknowledge {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: FWD_RELOC_COMPLETE_ACK,
            teid: Some(0),
            ..Default::default()
        };
        ForwardRelocationCompleteAcknowledge {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            scnd_rat_udrs: vec![],
            private_ext: vec![],
        }
    }
}

impl Messages for ForwardRelocationCompleteAcknowledge {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ForwardRelocationCompleteAcknowledge::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != FWD_RELOC_COMPLETE_ACK {
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

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        self.scnd_rat_udrs.iter().for_each(|x| {
            elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone()))
        });

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
                        (self.cause, mandatory) = (j, true);
                    }
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j);
                    }
                }
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins < 2 {
                        self.scnd_rat_udrs.push(j)
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
fn test_fwd_reloc_complete_ack_unmarshal() {
    let encoded: [u8; 95] = [
        0x48, 0x88, 0x00, 0x5b, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x08, 0xc9, 0x00, 0x1b, 0x00, 0x01, 0x02, 0x05,
        0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xc9, 0x00, 0x1b, 0x01, 0x01, 0x07,
        0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07,
        0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ForwardRelocationCompleteAcknowledge {
        header: Gtpv2Header {
            msgtype: FWD_RELOC_COMPLETE_ACK,
            piggyback: false,
            message_prio: None,
            length: 91,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
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
        recovery: Some(Recovery {
            recovery: 8,
            ..Recovery::default()
        }),
        scnd_rat_udrs: vec![
            SecondaryRatUsageDataReport {
                ins: 0,
                irsgw: false,
                irpgw: true,
                rat_type: 2,
                ebi: 5,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
            SecondaryRatUsageDataReport {
                ins: 1,
                irsgw: false,
                irpgw: true,
                rat_type: 7,
                ebi: 5,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
        ],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let message = ForwardRelocationCompleteAcknowledge::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_fwd_reloc_complete_ack_marshal() {
    let encoded: [u8; 95] = [
        0x48, 0x88, 0x00, 0x5b, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0x08, 0xc9, 0x00, 0x1b, 0x00, 0x01, 0x02, 0x05,
        0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xc9, 0x00, 0x1b, 0x01, 0x01, 0x07,
        0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07,
        0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ForwardRelocationCompleteAcknowledge {
        header: Gtpv2Header {
            msgtype: FWD_RELOC_COMPLETE_ACK,
            piggyback: false,
            message_prio: None,
            length: 91,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
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
        recovery: Some(Recovery {
            recovery: 8,
            ..Recovery::default()
        }),
        scnd_rat_udrs: vec![
            SecondaryRatUsageDataReport {
                ins: 0,
                irsgw: false,
                irpgw: true,
                rat_type: 2,
                ebi: 5,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
            SecondaryRatUsageDataReport {
                ins: 1,
                irsgw: false,
                irpgw: true,
                rat_type: 7,
                ebi: 5,
                start_timestamp: 0xff,
                end_timestamp: 0xffff,
                usg_data_dl: 0xffffff00,
                usg_data_ul: 0xffff,
                ..SecondaryRatUsageDataReport::default()
            },
        ],
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
