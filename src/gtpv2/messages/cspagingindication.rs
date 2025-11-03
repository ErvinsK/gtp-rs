use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CS_PAGING_IND: u8 = 151;

// Definition of GTPv2-C CS Paging Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsPagingIndication {
    pub header: Gtpv2Header,
    pub imsi: Imsi,
    pub vlr: Fqdn,
    pub tmsi: Option<Tmsi>,
    pub lai: Option<Uli>,
    pub global_cnid: Option<GlobalCnId>,
    pub channel_needed: Option<ChannelNeeded>,
    pub emlpp_prio: Option<EmlppPriority>,
    pub service_indicator: Option<ServiceIndicator>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CsPagingIndication {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: CS_PAGING_IND,
            teid: Some(0),
            ..Default::default()
        };
        CsPagingIndication {
            header: hdr,
            imsi: Imsi::default(),
            vlr: Fqdn::default(),
            tmsi: None,
            lai: None,
            global_cnid: None,
            channel_needed: None,
            emlpp_prio: None,
            service_indicator: None,
            private_ext: vec![],
        }
    }
}

impl Messages for CsPagingIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CsPagingIndication::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CS_PAGING_IND {
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

        elements.push(self.imsi.clone().into());

        elements.push(self.vlr.clone().into());

        if let Some(i) = self.tmsi.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.lai.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.global_cnid.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.channel_needed.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.emlpp_prio.clone() {
            elements.push(i.into());
        }
        if let Some(i) = self.service_indicator.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 2] = [false; 2];
        for e in elements.into_iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        self.imsi = j;
                        mandatory[0] = true;
                    }
                }
                InformationElement::Fqdn(j) => {
                    if let (0, false) = (j.ins, mandatory[1]) {
                        self.vlr = j;
                        mandatory[1] = true;
                    }
                }
                InformationElement::Tmsi(j) => {
                    if let (0, true) = (j.ins, self.tmsi.is_none()) {
                        self.tmsi = Some(j);
                    }
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.lai.is_none()) {
                        self.lai = Some(j);
                    }
                }

                InformationElement::GlobalCnId(j) => {
                    if let (0, true) = (j.ins, self.global_cnid.is_none()) {
                        self.global_cnid = Some(j);
                    }
                }
                InformationElement::ChannelNeeded(j) => {
                    if let (0, true) = (j.ins, self.channel_needed.is_none()) {
                        self.channel_needed = Some(j);
                    }
                }
                InformationElement::EmlppPriority(j) => {
                    if let (0, true) = (j.ins, self.emlpp_prio.is_none()) {
                        self.emlpp_prio = Some(j);
                    }
                }
                InformationElement::ServiceIndicator(j) => {
                    if let (0, true) = (j.ins, self.service_indicator.is_none()) {
                        self.service_indicator = Some(j);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        match mandatory {
            [true, true] => Ok(true),
            [false, _] => Err(GTPV2Error::MessageMandatoryIEMissing(IMSI)),
            [_, false] => Err(GTPV2Error::MessageMandatoryIEMissing(FQDN)),
        }
    }
}

#[test]
fn test_cs_paging_indication_unmarshal() {
    let encoded: [u8; 130] = [
        0x48, 0x97, 0x00, 0x7e, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x88, 0x00, 0x35, 0x00, 0x05, 0x74,
        0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x03, 0x6d, 0x73, 0x63, 0x02,
        0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63,
        0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
        0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x58, 0x00, 0x04, 0x00, 0x3e, 0x6b, 0x58, 0x5e, 0x56,
        0x00, 0x0d, 0x00, 0x18, 0x42, 0xf7, 0x10, 0xab, 0xea, 0x42, 0xf7, 0x10, 0x00, 0x2a, 0x46,
        0x10, 0x59, 0x00, 0x05, 0x00, 0x99, 0xf9, 0x10, 0x0f, 0xa0, 0x85, 0x00, 0x01, 0x00, 0xff,
        0x86, 0x00, 0x01, 0x00, 0x00, 0x95, 0x00, 0x01, 0x00, 0x02,
    ];
    let decoded = CsPagingIndication {
        header: Gtpv2Header {
            msgtype: CS_PAGING_IND,
            piggyback: false,
            message_prio: None,
            length: 126,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        },
        vlr: Fqdn {
            length: 53,
            name: "topon.nodes.msc.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
            ..Fqdn::default()
        },
        tmsi: Some(Tmsi {
            tmsi: 0x3e6b585e,
            ..Tmsi::default()
        }),
        lai: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 247,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    tac: 0xabea,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 247,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    eci: 2770448,
                }),
            ],
        }),
        global_cnid: Some(GlobalCnId {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            cnid: 4000,
            ..GlobalCnId::default()
        }),
        channel_needed: Some(ChannelNeeded {
            chnl_needed: 0xff,
            ..ChannelNeeded::default()
        }),
        emlpp_prio: Some(EmlppPriority {
            priority: 0,
            ..EmlppPriority::default()
        }),
        service_indicator: Some(ServiceIndicator {
            indicator: ServiceIndication::SmsIndicator,
            ..ServiceIndicator::default()
        }),
        ..CsPagingIndication::default()
    };
    let message = CsPagingIndication::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_cs_paging_indication_marshal() {
    let encoded: [u8; 130] = [
        0x48, 0x97, 0x00, 0x7e, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x99, 0x41, 0x55, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x88, 0x00, 0x35, 0x00, 0x05, 0x74,
        0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x03, 0x6d, 0x73, 0x63, 0x02,
        0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63,
        0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
        0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x58, 0x00, 0x04, 0x00, 0x3e, 0x6b, 0x58, 0x5e, 0x56,
        0x00, 0x0d, 0x00, 0x18, 0x42, 0xf7, 0x10, 0xab, 0xea, 0x42, 0xf7, 0x10, 0x00, 0x2a, 0x46,
        0x10, 0x59, 0x00, 0x05, 0x00, 0x99, 0xf9, 0x10, 0x0f, 0xa0, 0x85, 0x00, 0x01, 0x00, 0xff,
        0x86, 0x00, 0x01, 0x00, 0x00, 0x95, 0x00, 0x01, 0x00, 0x02,
    ];
    let decoded = CsPagingIndication {
        header: Gtpv2Header {
            msgtype: CS_PAGING_IND,
            piggyback: false,
            message_prio: None,
            length: 126,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Imsi {
            length: 0x08,
            imsi: "991455101961873".to_string(),
            ..Imsi::default()
        },
        vlr: Fqdn {
            length: 53,
            name: "topon.nodes.msc.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
            ..Fqdn::default()
        },
        tmsi: Some(Tmsi {
            tmsi: 0x3e6b585e,
            ..Tmsi::default()
        }),
        lai: Some(Uli {
            t: ULI,
            length: 13,
            ins: 0,
            loc: vec![
                Location::Tai(Tai {
                    mcc: 247,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    tac: 0xabea,
                }),
                Location::Ecgi(Ecgi {
                    mcc: 247,
                    mnc: 1,
                    mnc_is_three_digits: false,
                    eci: 2770448,
                }),
            ],
        }),
        global_cnid: Some(GlobalCnId {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            cnid: 4000,
            ..GlobalCnId::default()
        }),
        channel_needed: Some(ChannelNeeded {
            chnl_needed: 0xff,
            ..ChannelNeeded::default()
        }),
        emlpp_prio: Some(EmlppPriority {
            priority: 0,
            ..EmlppPriority::default()
        }),
        service_indicator: Some(ServiceIndicator {
            indicator: ServiceIndication::SmsIndicator,
            ..ServiceIndicator::default()
        }),
        ..CsPagingIndication::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
