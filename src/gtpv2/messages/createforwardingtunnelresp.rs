use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_FWD_TUNNEL_RESP: u8 = 161;

// Definition of GTPv2-C Create Forwarding Tunnel Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateForwardingTunnelResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub s1udf: Vec<S1udf>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateForwardingTunnelResponse {
    fn default() -> CreateForwardingTunnelResponse {
        CreateForwardingTunnelResponse {
            header: Gtpv2Header {
                msgtype: CREATE_FWD_TUNNEL_RESP,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            cause: Cause::default(),
            s1udf: vec![],
            private_ext: vec![],
        }
    }
}

impl Messages for CreateForwardingTunnelResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateForwardingTunnelResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_FWD_TUNNEL_RESP {
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

        self.s1udf
            .iter()
            .for_each(|x| elements.push(InformationElement::S1udf(x.clone())));

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
                        self.cause = j.clone();
                        mandatory = true;
                    }
                }
                InformationElement::S1udf(j) => {
                    if j.ins == 0 {
                        self.s1udf.push(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(S103_PDF))
        }
    }
}

#[test]
fn test_create_fwd_tunnel_resp_unmarshal() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 68] = [
        0x48, 0xa1, 0x00, 0x40, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x5b, 0x00, 0x0a, 0x00, 0x05, 0x04, 0x0a, 0x0a, 0x0a, 0x0a, 0x00, 0x00,
        0xff, 0xaa, 0x5b, 0x00, 0x16, 0x00, 0x06, 0x10, 0x00, 0xfd, 0x00, 0xff, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xff, 0xff, 0x00,
        0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateForwardingTunnelResponse {
        header: Gtpv2Header {
            msgtype: CREATE_FWD_TUNNEL_RESP,
            piggyback: false,
            message_prio: None,
            length: 64,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        s1udf: vec![
            S1udf {
                t: S1UDF,
                length: 10,
                ins: 0,
                ebi: 5,
                sgw_ip: IpAddr::V4(Ipv4Addr::new(10, 10, 10, 10)),
                sgw_s1u_teid: 0xffaa,
            },
            S1udf {
                t: S1UDF,
                length: 22,
                ins: 0,
                ebi: 6,
                sgw_ip: IpAddr::V6(Ipv6Addr::new(0xfd, 0xff, 0, 0, 0, 0, 0, 0)),
                sgw_s1u_teid: 0xaaff,
            },
        ],
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = CreateForwardingTunnelResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_fwd_tunnel_resp_marshal() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 68] = [
        0x48, 0xa1, 0x00, 0x40, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x5b, 0x00, 0x0a, 0x00, 0x05, 0x04, 0x0a, 0x0a, 0x0a, 0x0a, 0x00, 0x00,
        0xff, 0xaa, 0x5b, 0x00, 0x16, 0x00, 0x06, 0x10, 0x00, 0xfd, 0x00, 0xff, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xff, 0xff, 0x00,
        0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = CreateForwardingTunnelResponse {
        header: Gtpv2Header {
            msgtype: CREATE_FWD_TUNNEL_RESP,
            piggyback: false,
            message_prio: None,
            length: 64,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Cause::default()
        },
        s1udf: vec![
            S1udf {
                t: S1UDF,
                length: 10,
                ins: 0,
                ebi: 5,
                sgw_ip: IpAddr::V4(Ipv4Addr::new(10, 10, 10, 10)),
                sgw_s1u_teid: 0xffaa,
            },
            S1udf {
                t: S1UDF,
                length: 22,
                ins: 0,
                ebi: 6,
                sgw_ip: IpAddr::V6(Ipv6Addr::new(0xfd, 0xff, 0, 0, 0, 0, 0, 0)),
                sgw_s1u_teid: 0xaaff,
            },
        ],
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
