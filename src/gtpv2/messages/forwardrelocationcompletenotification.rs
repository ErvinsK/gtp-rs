use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const FWD_RELOC_COMPLETE_NOTIF: u8 = 135;

// Definition of GTPv2-C Forward Relocation Complete Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardRelocationCompleteNotification {
    pub header: Gtpv2Header,
    pub indication: Option<Indication>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for ForwardRelocationCompleteNotification {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: FWD_RELOC_COMPLETE_NOTIF,
            teid: Some(0),
            ..Default::default()
        };
        ForwardRelocationCompleteNotification {
            header: hdr,
            indication: None,
            private_ext: vec![],
        }
    }
}

impl Messages for ForwardRelocationCompleteNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ForwardRelocationCompleteNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != FWD_RELOC_COMPLETE_NOTIF {
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

        if let Some(i) = self.indication.clone() {
            elements.push(InformationElement::Indication(i));
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.into_iter() {
            match e {
                InformationElement::Indication(j) => {
                    if let (0, true) = (j.ins, self.indication.is_none()) {
                        self.indication = Some(j);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_fwd_reloc_complete_notif_unmarshal() {
    let encoded: [u8; 36] = [
        0x48, 0x87, 0x00, 0x20, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x4d, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0xff, 0x00, 0x06, 0x00,
        0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ForwardRelocationCompleteNotification {
        header: Gtpv2Header {
            msgtype: FWD_RELOC_COMPLETE_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 32,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        indication: Some(Indication {
            nsenbi: true,
            ..Indication::default()
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let message = ForwardRelocationCompleteNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_fwd_reloc_complete_notif_marshal() {
    let encoded: [u8; 36] = [
        0x48, 0x87, 0x00, 0x20, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x4d, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0xff, 0x00, 0x06, 0x00,
        0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = ForwardRelocationCompleteNotification {
        header: Gtpv2Header {
            msgtype: FWD_RELOC_COMPLETE_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 32,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        indication: Some(Indication {
            nsenbi: true,
            ..Indication::default()
        }),
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
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
