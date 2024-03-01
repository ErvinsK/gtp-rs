use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DEL_PDN_CONN_SET_REQ: u8 = 101;

// Definition of GTPv2-C Delete PDN Connection Set Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletePdnConnectionSetRequest {
    pub header: Gtpv2Header,
    pub mme_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub pgw_fqcsid: Option<Fqcsid>,
    pub epdg_fqcsid: Option<Fqcsid>,
    pub twan_fqcsid: Option<Fqcsid>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DeletePdnConnectionSetRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: DEL_PDN_CONN_SET_REQ,
            teid: Some(0),
            ..Default::default()
        };
        DeletePdnConnectionSetRequest {
            header: hdr,
            mme_fqcsid: None,
            sgw_fqcsid: None,
            pgw_fqcsid: None,
            epdg_fqcsid: None,
            twan_fqcsid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DeletePdnConnectionSetRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeletePdnConnectionSetRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DEL_PDN_CONN_SET_REQ {
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

        if let Some(i) = self.mme_fqcsid.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.sgw_fqcsid.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.pgw_fqcsid.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.epdg_fqcsid.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.twan_fqcsid.clone() {
            elements.push(i.into());
        }

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.iter() {
            match e {
                InformationElement::Fqcsid(j) => match j.ins {
                    0 => {
                        if self.mme_fqcsid.is_none() {
                            self.mme_fqcsid = Some(j.clone());
                        }
                    }
                    1 => {
                        if self.sgw_fqcsid.is_none() {
                            self.sgw_fqcsid = Some(j.clone());
                        }
                    }
                    2 => {
                        if self.pgw_fqcsid.is_none() {
                            self.pgw_fqcsid = Some(j.clone());
                        }
                    }
                    3 => {
                        if self.epdg_fqcsid.is_none() {
                            self.epdg_fqcsid = Some(j.clone());
                        }
                    }
                    4 => {
                        if self.twan_fqcsid.is_none() {
                            self.twan_fqcsid = Some(j.clone());
                        }
                    }
                    _ => (),
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_delete_pdn_connection_set_req_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 33] = [
        0x48, 0x65, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x26, 0x00, 0x2e, 0x00, 0x84, 0x00, 0x07,
        0x01, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07,
        0x00, 0x01, 0x00,
    ];
    let decoded = DeletePdnConnectionSetRequest {
        header: Gtpv2Header {
            msgtype: DEL_PDN_CONN_SET_REQ,
            piggyback: false,
            message_prio: None,
            length: 29,
            teid: Some(0),
            sqn: 0x26002e,
        },
        sgw_fqcsid: Some(Fqcsid {
            length: 7,
            ins: 1,
            nodeid: NodeId::V4(Ipv4Addr::new(139, 7, 133, 184)),
            csid: vec![0xffff],
            ..Fqcsid::default()
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..DeletePdnConnectionSetRequest::default()
    };
    let message = DeletePdnConnectionSetRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_delete_pdn_connection_set_req_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 33] = [
        0x48, 0x65, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x26, 0x00, 0x2e, 0x00, 0x84, 0x00, 0x07,
        0x01, 0x01, 0x8b, 0x07, 0x85, 0xb8, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07,
        0x00, 0x01, 0x00,
    ];
    let decoded = DeletePdnConnectionSetRequest {
        header: Gtpv2Header {
            msgtype: DEL_PDN_CONN_SET_REQ,
            piggyback: false,
            message_prio: None,
            length: 29,
            teid: Some(0),
            sqn: 0x26002e,
        },
        sgw_fqcsid: Some(Fqcsid {
            length: 7,
            ins: 1,
            nodeid: NodeId::V4(Ipv4Addr::new(139, 7, 133, 184)),
            csid: vec![0xffff],
            ..Fqcsid::default()
        }),
        private_ext: vec![PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec![0x07, 0x00, 0x01, 0x00],
        }],
        ..DeletePdnConnectionSetRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
