use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const UE_REG_QUERY_REQ: u8 = 158;

// Definition of GTPv2-C UE Registration Query Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeRegistrationQueryRequest {
    pub header: Gtpv2Header,
    pub imsi: Imsi,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for UeRegistrationQueryRequest {
    fn default() -> UeRegistrationQueryRequest {
        UeRegistrationQueryRequest {
            header: Gtpv2Header {
                msgtype: UE_REG_QUERY_REQ,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            imsi: Imsi::default(),
            private_ext: vec![],
        }
    }
}

impl Messages for UeRegistrationQueryRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = UeRegistrationQueryRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UE_REG_QUERY_REQ {
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

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if j.ins == 0 {
                        mandatory = true;
                        self.imsi = j.clone();
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(IMSI))
        }
    }
}

#[test]
fn test_ue_reg_query_req_unmarshal() {
    let encoded: [u8; 32] = [
        0x48, 0x9e, 0x00, 0x1c, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x06,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62,
        0x9c, 0xc4,
    ];
    let decoded = UeRegistrationQueryRequest {
        header: Gtpv2Header {
            msgtype: UE_REG_QUERY_REQ,
            piggyback: false,
            message_prio: None,
            length: 28,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        },
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..PrivateExtension::default()
        }],
    };
    let message = UeRegistrationQueryRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_ue_reg_query_req_marshal() {
    let encoded: [u8; 32] = [
        0x48, 0x9e, 0x00, 0x1c, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x01, 0x00, 0x06,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62,
        0x9c, 0xc4,
    ];
    let decoded = UeRegistrationQueryRequest {
        header: Gtpv2Header {
            msgtype: UE_REG_QUERY_REQ,
            piggyback: false,
            message_prio: None,
            length: 28,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        imsi: Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
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
