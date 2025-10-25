use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MBMS_SESSION_STOP_RESP: u8 = 236;

// Definition of GTPv2-C MBMS Session Stop Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsSessionStopResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for MbmsSessionStopResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MBMS_SESSION_STOP_RESP,
            teid: Some(0),
            ..Default::default()
        };
        MbmsSessionStopResponse {
            header: hdr,
            cause: Cause::default(),
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for MbmsSessionStopResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = MbmsSessionStopResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MBMS_SESSION_STOP_RESP {
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
        let mut elements: Vec<InformationElement> = vec![self.cause.clone().into()];

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: bool = false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        self.cause = j.clone();
                        mandatory = true;
                    }
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
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
fn test_mbms_session_stop_resp_unmarshal() {
    let encoded: [u8; 23] = [
        0x48, 0xec, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0xaa,
    ];
    let decoded = MbmsSessionStopResponse {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_STOP_RESP,
            piggyback: false,
            message_prio: None,
            length: 19,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            value: 16,
            ..Cause::default()
        },
        recovery: Some(Recovery {
            recovery: 0xaa,
            ..Recovery::default()
        }),
        ..MbmsSessionStopResponse::default()
    };
    let message = MbmsSessionStopResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_mbms_session_stop_resp_marshal() {
    let encoded: [u8; 23] = [
        0x48, 0xec, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x03, 0x00, 0x01, 0x00, 0xaa,
    ];
    let decoded = MbmsSessionStopResponse {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_STOP_RESP,
            piggyback: false,
            message_prio: None,
            length: 19,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            value: 16,
            ..Cause::default()
        },
        recovery: Some(Recovery {
            recovery: 0xaa,
            ..Recovery::default()
        }),
        ..MbmsSessionStopResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
