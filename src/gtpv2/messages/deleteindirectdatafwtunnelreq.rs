use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const DELETE_IND_DATA_FW_TUN_REQ: u8 = 168;

// Definition of GTPv2-C Delete Indirect Data Forwarding Tunnel Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteIndirectDataForwardingTunnelRequest {
    pub header: Gtpv2Header,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for DeleteIndirectDataForwardingTunnelRequest {
    fn default() -> DeleteIndirectDataForwardingTunnelRequest {
        DeleteIndirectDataForwardingTunnelRequest {
            header: Gtpv2Header {
                msgtype: DELETE_IND_DATA_FW_TUN_REQ,
                teid: Some(0),
                ..Default::default()
            },
            private_ext: vec![],
        }
    }
}

impl Messages for DeleteIndirectDataForwardingTunnelRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteIndirectDataForwardingTunnelRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_IND_DATA_FW_TUN_REQ {
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

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        for e in elements.iter() {
            if let InformationElement::PrivateExtension(j) = e {
                self.private_ext.push(j.clone());
            }
        }
        Ok(true)
    }
}

#[test]
fn test_delete_indirect_data_fw_tunnel_req_unmarshal() {
    let encoded: [u8; 22] = [
        0x48, 0xa8, 0x00, 0x12, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0xff, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = DeleteIndirectDataForwardingTunnelRequest {
        header: Gtpv2Header {
            msgtype: DELETE_IND_DATA_FW_TUN_REQ,
            piggyback: false,
            message_prio: None,
            length: 18,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = DeleteIndirectDataForwardingTunnelRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_delete_indirect_data_fw_tunnel_req_marshal() {
    let encoded: [u8; 22] = [
        0x48, 0xa8, 0x00, 0x12, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0xff, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = DeleteIndirectDataForwardingTunnelRequest {
        header: Gtpv2Header {
            msgtype: DELETE_IND_DATA_FW_TUN_REQ,
            piggyback: false,
            message_prio: None,
            length: 18,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
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
