use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const ISR_STATUS_IND: u8 = 157;

// Definition of GTPv2-C ISR Status Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsrStatusIndication {
    pub header: Gtpv2Header,
    pub action_ind: ActionIndication,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for IsrStatusIndication {
    fn default() -> IsrStatusIndication {
        IsrStatusIndication {
            header: Gtpv2Header {
                msgtype: ISR_STATUS_IND,
                teid: Some(0),
                ..Default::default()
            },
            action_ind: ActionIndication::default(),
            private_ext: vec![],
        }
    }
}

impl Messages for IsrStatusIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = IsrStatusIndication::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ISR_STATUS_IND {
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

        elements.push(self.action_ind.clone().into());

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::ActionIndication(j) => {
                    if j.ins == 0 {
                        mandatory = true;
                        self.action_ind = j.clone();
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(ACTION_IND))
        }
    }
}

#[test]
fn test_isr_status_ind_unmarshal() {
    let encoded: [u8; 27] = [
        0x48, 0x9d, 0x00, 0x17, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0xa8, 0x00, 0x01,
        0x00, 0x03, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = IsrStatusIndication {
        header: Gtpv2Header {
            msgtype: ISR_STATUS_IND,
            piggyback: false,
            message_prio: None,
            length: 23,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        action_ind: ActionIndication {
            indication: IndicationValues::PagingStopIndication,
            ..ActionIndication::default()
        },
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = IsrStatusIndication::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_isr_status_ind_marshal() {
    let encoded: [u8; 27] = [
        0x48, 0x9d, 0x00, 0x17, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0xa8, 0x00, 0x01,
        0x00, 0x03, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = IsrStatusIndication {
        header: Gtpv2Header {
            msgtype: ISR_STATUS_IND,
            piggyback: false,
            message_prio: None,
            length: 23,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        action_ind: ActionIndication {
            indication: IndicationValues::PagingStopIndication,
            ..ActionIndication::default()
        },
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer, encoded);
}
