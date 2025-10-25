use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const UE_ACTIVITY_NOTIF: u8 = 155;

// Definition of GTPv2-C UE Activity Notification Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeActivityNotification {
    pub header: Gtpv2Header,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for UeActivityNotification {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: UE_ACTIVITY_NOTIF,
            teid: Some(0),
            ..Default::default()
        };
        UeActivityNotification {
            header: hdr,
            private_ext: vec![],
        }
    }
}

impl Messages for UeActivityNotification {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = UeActivityNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UE_ACTIVITY_NOTIF {
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

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.into_iter() {
            if let InformationElement::PrivateExtension(j) = e {
                self.private_ext.push(j);
            }
        }
        Ok(true)
    }
}

#[test]
fn test_ue_activity_notification_unmarshal() {
    let encoded: [u8; 22] = [
        0x48, 0x9b, 0x00, 0x12, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0xff, 0x00, 0x06,
        0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = UeActivityNotification {
        header: Gtpv2Header {
            msgtype: UE_ACTIVITY_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 18,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let message = UeActivityNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_ue_activity_notification_marshal() {
    let encoded: [u8; 22] = [
        0x48, 0x9b, 0x00, 0x12, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0xff, 0x00, 0x06,
        0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = UeActivityNotification {
        header: Gtpv2Header {
            msgtype: UE_ACTIVITY_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 18,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
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
