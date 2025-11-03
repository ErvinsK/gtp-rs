use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const RAN_INFO_RELAY: u8 = 152;

// Definition of GTPv2-C RAN Information Relay Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RanInformationRelay {
    pub header: Gtpv2Header,
    pub bss_container: Fcause,
    pub rim_routing_addr: Option<TargetIdentification>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for RanInformationRelay {
    fn default() -> Self {
        RanInformationRelay {
            header: Gtpv2Header {
                msgtype: RAN_INFO_RELAY,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            bss_container: Fcause::default(),
            rim_routing_addr: None,
            private_ext: vec![],
        }
    }
}

impl Messages for RanInformationRelay {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = RanInformationRelay::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != RAN_INFO_RELAY {
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

        elements.push(self.bss_container.clone().into());

        if let Some(i) = self.rim_routing_addr.clone() {
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
                InformationElement::Fcause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        mandatory = true;
                        self.bss_container = j.clone();
                    };
                }
                InformationElement::TargetIdentification(j) => {
                    if let (0, true) = (j.ins, self.rim_routing_addr.is_none()) {
                        self.rim_routing_addr = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(FCAUSE))
        }
    }
}

#[test]
fn test_ran_info_relay_unmarshal() {
    let encoded: [u8; 44] = [
        0x48, 0x98, 0x00, 0x28, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x77, 0x00, 0x03,
        0x00, 0x02, 0xff, 0xaa, 0x79, 0x00, 0x0b, 0x00, 0x00, 0x62, 0xf3, 0x10, 0xff, 0xff, 0xaa,
        0xff, 0xaa, 0x10, 0x02, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = RanInformationRelay {
        header: Gtpv2Header {
            msgtype: RAN_INFO_RELAY,
            piggyback: false,
            message_prio: None,
            length: 40,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        bss_container: Fcause {
            length: 3,
            cause_type: CauseType::Nas,
            cause_field: vec![0xff, 0xaa],
            ..Fcause::default()
        },
        rim_routing_addr: Some(TargetIdentification {
            length: 11,
            ins: 0,
            target_type: TargetType::RncId(RncIdentifier {
                rai: Rai {
                    mcc: 263,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    lac: 0xffff,
                    rac: 0xaa,
                },
                rnc_id: 0xffaa,
                ext_rnc_id: Some(4098),
            }),
            ..TargetIdentification::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = RanInformationRelay::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_ran_info_relay_marshal() {
    let encoded: [u8; 44] = [
        0x48, 0x98, 0x00, 0x28, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x77, 0x00, 0x03,
        0x00, 0x02, 0xff, 0xaa, 0x79, 0x00, 0x0b, 0x00, 0x00, 0x62, 0xf3, 0x10, 0xff, 0xff, 0xaa,
        0xff, 0xaa, 0x10, 0x02, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = RanInformationRelay {
        header: Gtpv2Header {
            msgtype: RAN_INFO_RELAY,
            piggyback: false,
            message_prio: None,
            length: 40,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        bss_container: Fcause {
            length: 3,
            cause_type: CauseType::Nas,
            cause_field: vec![0xff, 0xaa],
            ..Fcause::default()
        },
        rim_routing_addr: Some(TargetIdentification {
            length: 11,
            ins: 0,
            target_type: TargetType::RncId(RncIdentifier {
                rai: Rai {
                    mcc: 263,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    lac: 0xffff,
                    rac: 0xaa,
                },
                rnc_id: 0xffaa,
                ext_rnc_id: Some(4098),
            }),
            ..TargetIdentification::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
