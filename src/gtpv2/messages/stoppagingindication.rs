use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const STOP_PAGING_IND: u8 = 164;

// Definition of GTPv2-C Stop Paging Indication Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StopPagingIndication {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for StopPagingIndication {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: STOP_PAGING_IND,
            teid: Some(0),
            ..Default::default()
        };
        StopPagingIndication {
            header: hdr,
            imsi: None,
            private_ext: vec![],
        }
    }
}

impl Messages for StopPagingIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = StopPagingIndication::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != STOP_PAGING_IND {
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

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.into_iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if j.ins == 0 {
                        self.imsi = Some(j);
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
fn test_stop_paging_indication_unmarshal() {
    let encoded: [u8; 34] = [
        0x48, 0xa4, 0x00, 0x1e, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb,
        0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = StopPagingIndication {
        header: Gtpv2Header {
            msgtype: STOP_PAGING_IND,
            piggyback: false,
            message_prio: None,
            length: 30,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Some(Imsi {
            t: 0x01,
            length: 0x08,
            ins: 0x00,
            imsi: "901405101961873".to_string(),
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
    };
    let message = StopPagingIndication::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_stop_paging_indication_marshal() {
    let encoded: [u8; 34] = [
        0x48, 0xa4, 0x00, 0x1e, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb,
        0x07, 0x00, 0x01, 0x00,
    ];
    let decoded = StopPagingIndication {
        header: Gtpv2Header {
            msgtype: STOP_PAGING_IND,
            piggyback: false,
            message_prio: None,
            length: 30,
            teid: Some(0xa4789580),
            sqn: 0x4b291e,
        },
        imsi: Some(Imsi {
            t: 0x01,
            length: 0x08,
            ins: 0x00,
            imsi: "901405101961873".to_string(),
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
