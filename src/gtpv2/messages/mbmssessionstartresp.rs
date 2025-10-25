use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MBMS_SESSION_STRT_RESP: u8 = 232;

// Definition of GTPv2-C MBMS Session Start Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsSessionStartResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub fteid_control: Fteid,
    pub mbms_distr_ack: Option<MbmsDistributionAck>,
    pub sgsn_fteid: Option<Fteid>,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for MbmsSessionStartResponse {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MBMS_SESSION_STRT_RESP,
            teid: Some(0),
            ..Default::default()
        };
        MbmsSessionStartResponse {
            header: hdr,
            cause: Cause::default(),
            fteid_control: Fteid::default(),
            mbms_distr_ack: None,
            sgsn_fteid: None,
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for MbmsSessionStartResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = MbmsSessionStartResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MBMS_SESSION_STRT_RESP {
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
        let mut elements: Vec<InformationElement> =
            vec![self.cause.clone().into(), self.fteid_control.clone().into()];

        if let Some(i) = self.mbms_distr_ack.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.sgsn_fteid.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 2] = [false; 2];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        self.cause = j.clone();
                        mandatory[0] = true;
                    }
                }
                InformationElement::Fteid(j) => match j.ins {
                    0 => {
                        if !mandatory[1] {
                            self.fteid_control = j.clone();
                            mandatory[1] = true;
                        }
                    }
                    1 => {
                        if self.sgsn_fteid.is_none() {
                            self.sgsn_fteid = Some(j.clone());
                        }
                    }
                    _ => (),
                },

                InformationElement::MbmsDistributionAck(j) => {
                    if let (0, true) = (j.ins, self.mbms_distr_ack.is_none()) {
                        self.mbms_distr_ack = Some(j.clone());
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
        if let Some(j) = mandatory.iter().position(|&x| !x) {
            match j {
                0 => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
                1 => Err(GTPV2Error::MessageMandatoryIEMissing(FTEID)),
                _ => Err(GTPV2Error::MessageMandatoryIEMissing(0)),
            }
        } else {
            Ok(true)
        }
    }
}

#[test]
fn test_mbms_session_start_resp_unmarshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 86] = [
        0x48, 0xe8, 0x00, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x19, 0x00, 0xca, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d,
        0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x27, 0x8f, 0x00, 0x01, 0x00, 0x02, 0x57, 0x00, 0x19, 0x01, 0xca, 0x23, 0xed, 0x38,
        0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x03, 0x00, 0x01, 0x00, 0xaa,
    ];
    let decoded = MbmsSessionStartResponse {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_STRT_RESP,
            piggyback: false,
            message_prio: None,
            length: 82,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            value: 16,
            ..Cause::default()
        },
        fteid_control: Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        },
        mbms_distr_ack: Some(MbmsDistributionAck {
            distr_id: 0x02,
            ..MbmsDistributionAck::default()
        }),
        sgsn_fteid: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 1,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        recovery: Some(Recovery {
            recovery: 0xaa,
            ..Recovery::default()
        }),
        ..MbmsSessionStartResponse::default()
    };
    let message = MbmsSessionStartResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_mbms_session_start_resp_marshal() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 86] = [
        0x48, 0xe8, 0x00, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x57, 0x00, 0x19, 0x00, 0xca, 0x23, 0xed, 0x38, 0x20, 0xd9, 0xab, 0x8d,
        0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x27, 0x8f, 0x00, 0x01, 0x00, 0x02, 0x57, 0x00, 0x19, 0x01, 0xca, 0x23, 0xed, 0x38,
        0x20, 0xd9, 0xab, 0x8d, 0xf2, 0x2a, 0x04, 0x4a, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x03, 0x00, 0x01, 0x00, 0xaa,
    ];
    let decoded = MbmsSessionStartResponse {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_STRT_RESP,
            piggyback: false,
            message_prio: None,
            length: 82,
            teid: Some(0),
            sqn: 0x68,
        },
        cause: Cause {
            t: CAUSE,
            value: 16,
            ..Cause::default()
        },
        fteid_control: Fteid {
            t: FTEID,
            length: 25,
            ins: 0,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        },
        mbms_distr_ack: Some(MbmsDistributionAck {
            distr_id: 0x02,
            ..MbmsDistributionAck::default()
        }),
        sgsn_fteid: Some(Fteid {
            t: FTEID,
            length: 25,
            ins: 1,
            interface: 10,
            teid: 0x23ed3820,
            ipv4: Some(Ipv4Addr::new(217, 171, 141, 242)),
            ipv6: Some(Ipv6Addr::new(0x2a04, 0x4a45, 0x4, 0x0, 0x0, 0x0, 0x0, 0x27)),
        }),
        recovery: Some(Recovery {
            recovery: 0xaa,
            ..Recovery::default()
        }),
        ..MbmsSessionStartResponse::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
