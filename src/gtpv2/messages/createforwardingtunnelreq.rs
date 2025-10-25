use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CREATE_FWD_TUNNEL_REQ: u8 = 160;

// Definition of GTPv2-C Create Forwarding Tunnel Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateForwardingTunnelRequest {
    pub header: Gtpv2Header,
    pub s103_pdf: Vec<S103pdf>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for CreateForwardingTunnelRequest {
    fn default() -> CreateForwardingTunnelRequest {
        CreateForwardingTunnelRequest {
            header: Gtpv2Header {
                msgtype: CREATE_FWD_TUNNEL_REQ,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            s103_pdf: vec![],
            private_ext: vec![],
        }
    }
}

impl Messages for CreateForwardingTunnelRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = CreateForwardingTunnelRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CREATE_FWD_TUNNEL_REQ {
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

        self.s103_pdf
            .iter()
            .for_each(|x| elements.push(InformationElement::S103pdf(x.clone())));

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::S103pdf(j) => {
                    if j.ins == 0 {
                        self.s103_pdf.push(j.clone());
                        mandatory = true;
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
fn test_create_fwd_tunnel_req_unmarshal() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded: [u8; 76] = [
        0x48, 0xa0, 0x00, 0x48, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x5a, 0x00, 0x17,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x5a, 0x00, 0x17, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c,
        0xc4,
    ];
    let decoded = CreateForwardingTunnelRequest {
        header: Gtpv2Header {
            msgtype: CREATE_FWD_TUNNEL_REQ,
            piggyback: false,
            message_prio: None,
            length: 72,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        s103_pdf: vec![
            S103pdf {
                length: 23,
                hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                gre_key: 0,
                eps_bearer_ids: vec![5],
                ..S103pdf::default()
            },
            S103pdf {
                length: 23,
                hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                gre_key: 0,
                eps_bearer_ids: vec![6],
                ..S103pdf::default()
            },
        ],
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = CreateForwardingTunnelRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_fwd_tunnel_req_marshal() {
    use std::net::{IpAddr, Ipv6Addr};
    let encoded: [u8; 76] = [
        0x48, 0xa0, 0x00, 0x48, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x5a, 0x00, 0x17,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x5a, 0x00, 0x17, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c,
        0xc4,
    ];
    let decoded = CreateForwardingTunnelRequest {
        header: Gtpv2Header {
            msgtype: CREATE_FWD_TUNNEL_REQ,
            piggyback: false,
            message_prio: None,
            length: 72,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        s103_pdf: vec![
            S103pdf {
                t: S103_PDF,
                length: 23,
                ins: 0,
                hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                gre_key: 0,
                eps_bearer_ids: vec![5],
            },
            S103pdf {
                length: 23,
                hsgw_ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                gre_key: 0,
                eps_bearer_ids: vec![6],
                ..S103pdf::default()
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
