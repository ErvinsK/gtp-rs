use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const ECHO_RESPONSE: u8 = 2;

// Definition of GTPv2-C Echo Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoResponse {
    pub header: Gtpv2Header,
    pub recovery: Recovery,
    pub sending_node_features: Option<NodeFeatures>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for EchoResponse {
    fn default() -> EchoResponse {
        let hdr = Gtpv2Header {
            msgtype: ECHO_RESPONSE,
            ..Default::default()
        };
        EchoResponse {
            header: hdr,
            recovery: Recovery::default(),
            sending_node_features: None,
            private_ext: vec![],
        }
    }
}

impl Messages for EchoResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = EchoResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ECHO_RESPONSE {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        let offset = message.header.length as usize + MANDATORY_HDR_LENGTH;

        if buffer.len() >= offset {
            match InformationElement::decoder(&buffer[MIN_HEADER_LENGTH..offset]) {
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

        elements.push(self.recovery.clone().into());

        if let Some(i) = self.sending_node_features.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));
        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: bool = false;
        for e in elements.iter() {
            match e {
                InformationElement::Recovery(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        mandatory = true;
                        self.recovery = j.clone();
                    }
                }
                InformationElement::NodeFeatures(j) => {
                    if let (0, true) = (j.ins, self.sending_node_features.is_none()) {
                        self.sending_node_features = Some(j.clone())
                    };
                }

                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY))
        }
    }
}

#[test]
fn test_echo_resp_unmarshal() {
    let encoded: [u8; 20] = [
        0x40, 0x02, 0x00, 0x10, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x21, 0xff, 0x00,
        0x03, 0x00, 0x00, 0x0a, 0xff,
    ];
    let decoded = EchoResponse {
        header: Gtpv2Header {
            msgtype: ECHO_RESPONSE,
            piggyback: false,
            message_prio: None,
            length: 16,
            teid: None,
            sqn: 0x2dcc38,
        },
        recovery: Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 33,
        },
        sending_node_features: None,
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 3,
            ins: 0,
            enterprise_id: 0x0a,
            value: vec![0xff],
        }],
    };
    assert_eq!(EchoResponse::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_echo_resp_no_mandatory_ie_unmarshal() {
    let encoded: [u8; 15] = [
        0x40, 0x02, 0x00, 0x0b, 0x2d, 0xcc, 0x38, 0x00, 0xff, 0x00, 0x03, 0x00, 0x00, 0x0a, 0xff,
    ];
    assert_eq!(
        EchoResponse::unmarshal(&encoded),
        Err(GTPV2Error::MessageMandatoryIEMissing(RECOVERY))
    );
}

#[test]
fn test_echo_resp_marshal() {
    let encoded: [u8; 20] = [
        0x40, 0x02, 0x00, 0x10, 0x2d, 0xcc, 0x38, 0x00, 0x03, 0x00, 0x01, 0x00, 0x21, 0xff, 0x00,
        0x03, 0x00, 0x00, 0x0a, 0xff,
    ];
    let decoded = EchoResponse {
        header: Gtpv2Header {
            msgtype: ECHO_RESPONSE,
            piggyback: false,
            message_prio: None,
            length: 16,
            teid: None,
            sqn: 0x2dcc38,
        },
        recovery: Recovery {
            t: RECOVERY,
            length: 1,
            ins: 0,
            recovery: 33,
        },
        sending_node_features: None,
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 3,
            ins: 0,
            enterprise_id: 0x0a,
            value: vec![0xff],
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    buffer.iter().for_each(|x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
