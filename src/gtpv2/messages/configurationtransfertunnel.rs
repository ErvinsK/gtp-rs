use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const CONFIG_TRANSFER_TUNNEL: u8 = 141;

// Definition of GTPv2-C Configuration Transfer Tunnel Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationTransferTunnel {
    pub header: Gtpv2Header,
    pub container: Fcontainer,
    pub target_id: TargetIdentification,
    pub connected_target_id: Option<TargetIdentification>,
}

impl Default for ConfigurationTransferTunnel {
    fn default() -> Self {
        ConfigurationTransferTunnel {
            header: Gtpv2Header {
                msgtype: CONFIG_TRANSFER_TUNNEL,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            container: Fcontainer::default(),
            target_id: TargetIdentification::default(),
            connected_target_id: None,
        }
    }
}

impl Messages for ConfigurationTransferTunnel {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ConfigurationTransferTunnel::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CONFIG_TRANSFER_TUNNEL {
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

        elements.push(self.container.clone().into());

        elements.push(self.target_id.clone().into());

        if let Some(i) = self.connected_target_id.clone() {
            elements.push(i.into())
        };

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 2] = [false, false];
        for e in elements.iter() {
            match e {
                InformationElement::Fcontainer(j) => {
                    if let (0, false) = (j.ins, mandatory[0]) {
                        self.container=j.clone();
                        mandatory[0] = true;
                    };
                },
                InformationElement::TargetIdentification(j) => {
                    if let (0, false) = (j.ins, mandatory[1]) {
                        self.target_id = j.clone();
                        mandatory[1] = true;
                    };
                },
                InformationElement::PduNumbers(j) => {
                    if let (0, true) = (j.ins, self.pdu_numbers.is_none()) {
                        self.pdu_numbers = Some(j.clone())
                    };
                },
                InformationElement::Fcontainer(j) => {
                    if j.ins < 2 {
                        self.eutran_containers.push(j.clone())
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_fwd_access_ctx_notif_unmarshal() {
    let encoded: [u8; 116] = [
        0x48,0x89,0x00,0x70,0x09,0x09,0xa4,0x56,0x00,0x00,0x2f,0x00,0x7c,0x00,0x09,0x00,
        0x05,0xff,0x00,0x00,0xff,0xaa,0x00,0x00,0xaa,0x7c,0x00,0x09,0x00,0x06,0xff,0x00,
        0x00,0xff,0xaa,0x00,0x00,0xaa,0x7c,0x00,0x09,0x00,0x07,0xff,0x00,0x00,0xff,0xaa,
        0x00,0x00,0xaa,0x7d,0x00,0x14,0x00,0x80,0x80,0x21,0x10,0x01,0x01,0x00,0x10,0x81,
        0x06,0x00,0x00,0x00,0x00,0x83,0x06,0x00,0x00,0x00,0x00,0x6e,0x00,0x09,0x00,0x05,
        0xff,0x00,0x00,0xff,0xaa,0x00,0x00,0xaa,0x76,0x00,0x05,0x00,0x02,0x01,0x62,0x9c,
        0xc4,0x76,0x00,0x05,0x00,0x02,0x01,0x62,0x9c,0xc4,0xff,0x00,0x06,0x00,0x00,0x00,
        0x01,0x62,0x9c,0xc4,
    ];
    let decoded = ForwardAccessContextNotification {
        header: Gtpv2Header {
            msgtype: FWD_ACCESS_CTX_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 112,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        rab_ctxs: vec![
            RabContext {
                nsapi: 5,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                dl_pdcp_sqn: 0xaa00,
                ul_pdcp_sqn: 0x00aa,
                ..RabContext::default()
            },
            RabContext {
                nsapi: 6,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                dl_pdcp_sqn: 0xaa00,
                ul_pdcp_sqn: 0x00aa,
                ..RabContext::default()
            },
            RabContext {
                nsapi: 7,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                dl_pdcp_sqn: 0xaa00,
                ul_pdcp_sqn: 0x00aa,
                ..RabContext::default()
            },
        ],
        src_rnc_pdcp_ctx: Some( 
            SourceRncPdcpContextInfo {
                t: SRC_RNC_PDCP,
                length: 20,
                ins: 0,
                rrc_container: vec![
                    0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
                    0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
                ],
            }
        ),
        pdu_numbers: Some(
            PduNumbers {
                nsapi: 5,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                send_npdu: 0xaa00,
                receive_npdu: 0x00aa,
                ..PduNumbers::default()
            }
        ),
        eutran_containers: vec![
            Fcontainer {
                length: 5,
                container: Container::Bss(vec![0x01, 0x62, 0x9c, 0xc4]),
                ..Fcontainer::default()
            },
            Fcontainer {
                length: 5,
                container: Container::Bss(vec![0x01, 0x62, 0x9c, 0xc4]),
                ..Fcontainer::default()
            },
        ],
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = ForwardAccessContextNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_fwd_access_ctx_notif_marshal() {
    let encoded: [u8; 116] = [
        0x48,0x89,0x00,0x70,0x09,0x09,0xa4,0x56,0x00,0x00,0x2f,0x00,0x7c,0x00,0x09,0x00,
        0x05,0xff,0x00,0x00,0xff,0xaa,0x00,0x00,0xaa,0x7c,0x00,0x09,0x00,0x06,0xff,0x00,
        0x00,0xff,0xaa,0x00,0x00,0xaa,0x7c,0x00,0x09,0x00,0x07,0xff,0x00,0x00,0xff,0xaa,
        0x00,0x00,0xaa,0x7d,0x00,0x14,0x00,0x80,0x80,0x21,0x10,0x01,0x01,0x00,0x10,0x81,
        0x06,0x00,0x00,0x00,0x00,0x83,0x06,0x00,0x00,0x00,0x00,0x6e,0x00,0x09,0x00,0x05,
        0xff,0x00,0x00,0xff,0xaa,0x00,0x00,0xaa,0x76,0x00,0x05,0x00,0x02,0x01,0x62,0x9c,
        0xc4,0x76,0x00,0x05,0x00,0x02,0x01,0x62,0x9c,0xc4,0xff,0x00,0x06,0x00,0x00,0x00,
        0x01,0x62,0x9c,0xc4,
    ];
    let decoded = ForwardAccessContextNotification {
        header: Gtpv2Header {
            msgtype: FWD_ACCESS_CTX_NOTIF,
            piggyback: false,
            message_prio: None,
            length: 112,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        rab_ctxs: vec![
            RabContext {
                nsapi: 5,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                dl_pdcp_sqn: 0xaa00,
                ul_pdcp_sqn: 0x00aa,
                ..RabContext::default()
            },
            RabContext {
                nsapi: 6,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                dl_pdcp_sqn: 0xaa00,
                ul_pdcp_sqn: 0x00aa,
                ..RabContext::default()
            },
            RabContext {
                nsapi: 7,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                dl_pdcp_sqn: 0xaa00,
                ul_pdcp_sqn: 0x00aa,
                ..RabContext::default()
            },
        ],
        src_rnc_pdcp_ctx: Some( 
            SourceRncPdcpContextInfo {
                t: SRC_RNC_PDCP,
                length: 20,
                ins: 0,
                rrc_container: vec![
                    0x80, 0x80, 0x21, 0x10, 0x01, 0x01, 0x00, 0x10, 0x81, 0x06, 0x00, 0x00, 0x00, 0x00,
                    0x83, 0x06, 0x00, 0x00, 0x00, 0x00,
                ],
            }
        ),
        pdu_numbers: Some(
            PduNumbers {
                nsapi: 5,
                dl_gtpu_sqn: 0xff00,
                ul_gtpu_sqn: 0x00ff,
                send_npdu: 0xaa00,
                receive_npdu: 0x00aa,
                ..PduNumbers::default()
            }
        ),
        eutran_containers: vec![
            Fcontainer {
                length: 5,
                container: Container::Bss(vec![0x01, 0x62, 0x9c, 0xc4]),
                ..Fcontainer::default()
            },
            Fcontainer {
                length: 5,
                container: Container::Bss(vec![0x01, 0x62, 0x9c, 0xc4]),
                ..Fcontainer::default()
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
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
