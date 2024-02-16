use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const DELETE_IND_DATA_FW_TUN_RESP: u8 = 169;

// Definition of GTPv2-C Delete Indirect Data Forwarding Tunnel Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteIndirectDataForwardingTunnelResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub recovery: Option<Recovery>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DeleteIndirectDataForwardingTunnelResponse {
    fn default() -> DeleteIndirectDataForwardingTunnelResponse {
        DeleteIndirectDataForwardingTunnelResponse {
            header: Gtpv2Header {
                msgtype: DELETE_IND_DATA_FW_TUN_RESP,
                teid: Some(0),
                ..Default::default()
            },
            cause: Cause::default(),
            recovery: None,
            private_ext: vec![],
        }
    }
}

impl Messages for DeleteIndirectDataForwardingTunnelResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteIndirectDataForwardingTunnelResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_IND_DATA_FW_TUN_RESP {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        let offset = message.header.length as usize + MANDATORY_HDR_LENGTH;

        if buffer.len() >= offset{
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
fn test_create_indirect_data_fw_tunnel_resp_unmarshal() {
    let encoded: [u8; 33] = [
    0x48,    0xa9,    0x00,    0x1d,    0x09,    0x09,    0xa4,    0x56,
    0x00,    0x00,    0x2f,    0x00,    0x02,    0x00,    0x02,    0x00,
    0x10,    0x00,    0x03,    0x00,    0x01,    0x00,    0x64,    0xff,
    0x00,    0x06,    0x00,    0x00,    0x00,    0x01,    0x62,    0x9c,
    0xc4,
    ];
    let decoded = DeleteIndirectDataForwardingTunnelResponse {
        header: Gtpv2Header {
            msgtype: DELETE_IND_DATA_FW_TUN_RESP,
            piggyback: false,
            message_prio: None,
            length: 29,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Default::default()
        },
        recovery: Some(Recovery {
            recovery: 100,
            ..Default::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = DeleteIndirectDataForwardingTunnelResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_create_indirect_data_fw_tunnel_resp_marshal() {
    let encoded: [u8; 33] = [
    0x48,    0xa9,    0x00,    0x1d,    0x09,    0x09,    0xa4,    0x56,
    0x00,    0x00,    0x2f,    0x00,    0x02,    0x00,    0x02,    0x00,
    0x10,    0x00,    0x03,    0x00,    0x01,    0x00,    0x64,    0xff,
    0x00,    0x06,    0x00,    0x00,    0x00,    0x01,    0x62,    0x9c,
    0xc4,
    ];
    let decoded = DeleteIndirectDataForwardingTunnelResponse {
        header: Gtpv2Header {
            msgtype: DELETE_IND_DATA_FW_TUN_RESP,
            piggyback: false,
            message_prio: None,
            length: 29,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 0x10,
            ..Default::default()
        },
        recovery: Some(Recovery {
            recovery: 100,
            ..Default::default()
        }),
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
