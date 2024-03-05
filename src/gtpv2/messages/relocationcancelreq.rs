use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const RELOC_CANCEL_REQ: u8 = 139;

// Definition of GTPv2-C Relocation Cancel Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelocationCancelRequest {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub mei: Option<Mei>,
    pub indication: Option<Indication>,
    pub ranap_cause: Option<Fcause>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for RelocationCancelRequest {
    fn default() -> Self {
        RelocationCancelRequest {
            header: Gtpv2Header {
                msgtype: RELOC_CANCEL_REQ,
                teid: Some(0),
                ..Default::default()
            },
            imsi: None,
            mei: None,
            indication: None,
            ranap_cause: None,
            private_ext: vec![],
        }
    }
}

impl Messages for RelocationCancelRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = RelocationCancelRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != RELOC_CANCEL_REQ {
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
            elements.push(i.into())
        };

        if let Some(i) = self.mei.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.indication.clone() {
            elements.push(i.into())
        };

        if let Some(i) = self.ranap_cause.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
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
                InformationElement::Fcause(j) => {
                    if let (0, true) = (j.ins, self.ranap_cause.is_none()) {
                        self.ranap_cause = Some(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_reloc_cancel_req_unmarshal() {
    let encoded: [u8; 67] = [
        0x48, 0x8b, 0x00, 0x3f, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0x4d, 0x00, 0x0a, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x00, 0x03, 0x00, 0x00, 0xff, 0xaa, 0xff, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = RelocationCancelRequest {
        header: Gtpv2Header {
            msgtype: RELOC_CANCEL_REQ,
            piggyback: false,
            message_prio: None,
            length: 63,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Some(Imsi {
            length: 8,
            imsi: "901405101961873".to_string(),
            ..Default::default()
        }),
        mei: Some(Mei {
            mei: "8676480401323003".to_string(),
            ..Default::default()
        }),
        indication: Some(Indication {
            uimsi: true,
            ..Default::default()
        }),
        ranap_cause: Some(Fcause {
            length: 3,
            cause_type: CauseType::RadioNetworkLayer,
            cause_field: vec![0xff, 0xaa],
            ..Fcause::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = RelocationCancelRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_reloc_cancel_req_marshal() {
    let encoded: [u8; 67] = [
        0x48, 0x8b, 0x00, 0x3f, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0x4d, 0x00, 0x0a, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x00, 0x03, 0x00, 0x00, 0xff, 0xaa, 0xff, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = RelocationCancelRequest {
        header: Gtpv2Header {
            msgtype: RELOC_CANCEL_REQ,
            piggyback: false,
            message_prio: None,
            length: 63,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Some(Imsi {
            length: 8,
            imsi: "901405101961873".to_string(),
            ..Default::default()
        }),
        mei: Some(Mei {
            mei: "8676480401323003".to_string(),
            ..Default::default()
        }),
        indication: Some(Indication {
            uimsi: true,
            ..Default::default()
        }),
        ranap_cause: Some(Fcause {
            length: 3,
            cause_type: CauseType::RadioNetworkLayer,
            cause_field: vec![0xff, 0xaa],
            ..Fcause::default()
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
