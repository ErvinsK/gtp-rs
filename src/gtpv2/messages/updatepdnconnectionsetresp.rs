use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const UPD_PDN_CONN_SET_RESP: u8 = 201;

// Definition of GTPv2-C Update PDN Connection Set Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePndConnectionSetResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub pgw_fqcsid: Option<Fqcsid>,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for UpdatePndConnectionSetResponse {
    fn default() -> UpdatePndConnectionSetResponse {
        UpdatePndConnectionSetResponse {
            header: Gtpv2Header {
                msgtype: UPD_PDN_CONN_SET_RESP,
                teid: Some(0),
                ..Default::default()
            },
            cause: Cause::default(),
            pgw_fqcsid: None,
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for UpdatePndConnectionSetResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = UpdatePndConnectionSetResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UPD_PDN_CONN_SET_RESP {
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

        elements.push(self.cause.clone().into());

        if let Some(i) = self.pgw_fqcsid.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.recovery.clone() {
            elements.push(i.into())
        };

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if j.ins == 0 {
                        mandatory = true;
                        self.cause = j.clone();
                    };
                }
                InformationElement::Fqcsid(j) => {
                    if let (0, true) = (j.ins, self.pgw_fqcsid.is_none()) {
                        self.pgw_fqcsid = Some(j.clone());
                    }
                }
                InformationElement::Recovery(j) => {
                    if let (0, true) = (j.ins, self.recovery.is_none()) {
                        self.recovery = Some(j.clone())
                    };
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
fn test_update_pdn_connection_set_resp_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 44] = [
        0x48, 0xc9, 0x00, 0x28, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x84, 0x00, 0x07, 0x00, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff, 0x03,
        0x00, 0x01, 0x00, 0x64, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = UpdatePndConnectionSetResponse {
        header: Gtpv2Header {
            msgtype: UPD_PDN_CONN_SET_RESP,
            piggyback: false,
            message_prio: None,
            length: 40,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        pgw_fqcsid: Some(Fqcsid {
            length: 7,
            ins: 0,
            nodeid: NodeId::V4(Ipv4Addr::new(139, 7, 133, 184)),
            csid: vec![0xffff],
            ..Fqcsid::default()
        }),
        recovery: Some(Recovery {
            recovery: 100,
            ..Recovery::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..PrivateExtension::default()
        }],
    };
    let message = UpdatePndConnectionSetResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_update_pdn_connection_set_resp_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 44] = [
        0x48, 0xc9, 0x00, 0x28, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x84, 0x00, 0x07, 0x00, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff, 0x03,
        0x00, 0x01, 0x00, 0x64, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = UpdatePndConnectionSetResponse {
        header: Gtpv2Header {
            msgtype: UPD_PDN_CONN_SET_RESP,
            piggyback: false,
            message_prio: None,
            length: 40,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        pgw_fqcsid: Some(Fqcsid {
            length: 7,
            ins: 0,
            nodeid: NodeId::V4(Ipv4Addr::new(139, 7, 133, 184)),
            csid: vec![0xffff],
            ..Fqcsid::default()
        }),
        recovery: Some(Recovery {
            recovery: 100,
            ..Recovery::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..PrivateExtension::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
