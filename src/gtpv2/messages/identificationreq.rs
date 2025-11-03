use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const IDENTIFICATION_REQ: u8 = 128;

// Definition of GTPv2-C Identification Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentificationRequest {
    pub header: Gtpv2Header,
    pub guti: Option<Guti>,
    pub rai: Option<Uli>,
    pub ptmsi: Option<Ptmsi>,
    pub ptmsi_sig: Option<PtmsiSignature>,
    pub carm: Option<CompleteRequestMessage>, // Complete Attach Request Message
    pub ip_cplane: Option<IpAddress>,
    pub udp_src_port: Option<PortNumber>,
    pub hop_counter: Option<HopCounter>,
    pub target_plmnid: Option<ServingNetwork>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for IdentificationRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: IDENTIFICATION_REQ,
            teid: Some(0),
            ..Default::default()
        };
        IdentificationRequest {
            header: hdr,
            guti: None,
            rai: None,
            ptmsi: None,
            ptmsi_sig: None,
            carm: None,
            ip_cplane: None,
            udp_src_port: None,
            hop_counter: None,
            target_plmnid: None,
            private_ext: vec![],
        }
    }
}

impl Messages for IdentificationRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = IdentificationRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != IDENTIFICATION_REQ {
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

        if let Some(i) = self.guti.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.rai.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.ptmsi.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.ptmsi_sig.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.carm.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.ip_cplane.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.udp_src_port.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.hop_counter.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.target_plmnid.clone() {
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
                InformationElement::Guti(j) => {
                    if let (0, true) = (j.ins, self.guti.is_none()) {
                        self.guti = Some(j.clone());
                    }
                }
                InformationElement::Uli(j) => {
                    if let (0, true) = (j.ins, self.rai.is_none()) {
                        self.rai = Some(j.clone());
                    }
                }
                InformationElement::Ptmsi(j) => {
                    if let (0, true) = (j.ins, self.ptmsi.is_none()) {
                        self.ptmsi = Some(j.clone());
                    }
                }
                InformationElement::PtmsiSignature(j) => {
                    if let (0, true) = (j.ins, self.ptmsi_sig.is_none()) {
                        self.ptmsi_sig = Some(j.clone());
                    }
                }
                InformationElement::CompleteRequestMessage(j) => {
                    if let (0, true) = (j.ins, self.carm.is_none()) {
                        self.carm = Some(j.clone());
                    }
                }
                InformationElement::IpAddress(j) => {
                    if let (0, true) = (j.ins, self.ip_cplane.is_none()) {
                        self.ip_cplane = Some(j.clone());
                    }
                }
                InformationElement::PortNumber(j) => {
                    if let (0, true) = (j.ins, self.udp_src_port.is_none()) {
                        self.udp_src_port = Some(j.clone());
                    }
                }
                InformationElement::HopCounter(j) => {
                    if let (0, true) = (j.ins, self.hop_counter.is_none()) {
                        self.hop_counter = Some(j.clone());
                    }
                }
                InformationElement::ServingNetwork(j) => {
                    if let (0, true) = (j.ins, self.target_plmnid.is_none()) {
                        self.target_plmnid = Some(j.clone());
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        Ok(true)
    }
}

#[test]
fn test_identification_req_unmarshal() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 90] = [
        0x48, 0x80, 0x00, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x75, 0x00, 0x0a,
        0x00, 0x99, 0xf9, 0x10, 0x01, 0x2c, 0x0a, 0xff, 0xff, 0xff, 0xff, 0x56, 0x00, 0x08, 0x00,
        0x04, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x01, 0xff, 0x6f, 0x00, 0x04, 0x00, 0x27, 0xff, 0xaa,
        0x11, 0x70, 0x00, 0x04, 0x00, 0x27, 0xff, 0xaa, 0x11, 0x74, 0x00, 0x06, 0x00, 0x00, 0xaa,
        0xbb, 0xcc, 0xdd, 0xee, 0x4a, 0x00, 0x04, 0x00, 0x0a, 0x0a, 0x0a, 0x0a, 0x7e, 0x00, 0x02,
        0x00, 0x12, 0x34, 0x71, 0x00, 0x01, 0x00, 0x03, 0x53, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10,
    ];
    let decoded = IdentificationRequest {
        header: Gtpv2Header {
            msgtype: IDENTIFICATION_REQ,
            piggyback: false,
            message_prio: None,
            length: 86,
            teid: Some(0),
            sqn: 0x68,
        },
        guti: Some(Guti {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            mmegi: 300,
            mmec: 10,
            mtmsi: 0xffffffff,
            ..Guti::default()
        }),
        rai: Some(Uli {
            length: 8,
            loc: vec![Location::Rai(Rai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0x0bd9,
                rac: 0x01,
            })],
            ..Uli::default()
        }),
        ptmsi: Some(Ptmsi {
            ptmsi: 0x27ffaa11,
            ..Ptmsi::default()
        }),
        ptmsi_sig: Some(PtmsiSignature {
            ptmsi_sig: 0x27ffaa11,
            ..PtmsiSignature::default()
        }),
        carm: Some(CompleteRequestMessage {
            length: 6,
            message: RequestMessage::AttachRequest(vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
            ..CompleteRequestMessage::default()
        }),
        ip_cplane: Some(IpAddress {
            t: IP_ADDRESS,
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(10, 10, 10, 10)),
        }),
        udp_src_port: Some(PortNumber {
            port: 0x1234,
            ..PortNumber::default()
        }),
        hop_counter: Some(HopCounter {
            hop_counter: 0x03,
            ..HopCounter::default()
        }),
        target_plmnid: Some(ServingNetwork {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            ..ServingNetwork::default()
        }),
        ..IdentificationRequest::default()
    };
    let message = IdentificationRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_identification_req_marshal() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 90] = [
        0x48, 0x80, 0x00, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x75, 0x00, 0x0a,
        0x00, 0x99, 0xf9, 0x10, 0x01, 0x2c, 0x0a, 0xff, 0xff, 0xff, 0xff, 0x56, 0x00, 0x08, 0x00,
        0x04, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x01, 0xff, 0x6f, 0x00, 0x04, 0x00, 0x27, 0xff, 0xaa,
        0x11, 0x70, 0x00, 0x04, 0x00, 0x27, 0xff, 0xaa, 0x11, 0x74, 0x00, 0x06, 0x00, 0x00, 0xaa,
        0xbb, 0xcc, 0xdd, 0xee, 0x4a, 0x00, 0x04, 0x00, 0x0a, 0x0a, 0x0a, 0x0a, 0x7e, 0x00, 0x02,
        0x00, 0x12, 0x34, 0x71, 0x00, 0x01, 0x00, 0x03, 0x53, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10,
    ];
    let decoded = IdentificationRequest {
        header: Gtpv2Header {
            msgtype: IDENTIFICATION_REQ,
            piggyback: false,
            message_prio: None,
            length: 86,
            teid: Some(0),
            sqn: 0x68,
        },
        guti: Some(Guti {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            mmegi: 300,
            mmec: 10,
            mtmsi: 0xffffffff,
            ..Guti::default()
        }),
        rai: Some(Uli {
            length: 25,
            loc: vec![Location::Rai(Rai {
                mcc: 262,
                mnc: 1,
                mnc_is_three_digits: false,
                lac: 0x0bd9,
                rac: 0x01,
            })],
            ..Uli::default()
        }),
        ptmsi: Some(Ptmsi {
            ptmsi: 0x27ffaa11,
            ..Ptmsi::default()
        }),
        ptmsi_sig: Some(PtmsiSignature {
            ptmsi_sig: 0x27ffaa11,
            ..PtmsiSignature::default()
        }),
        carm: Some(CompleteRequestMessage {
            length: 6,
            message: RequestMessage::AttachRequest(vec![0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
            ..CompleteRequestMessage::default()
        }),
        ip_cplane: Some(IpAddress {
            t: IP_ADDRESS,
            length: 4,
            ins: 0,
            ip: IpAddr::V4(Ipv4Addr::new(10, 10, 10, 10)),
        }),
        udp_src_port: Some(PortNumber {
            port: 0x1234,
            ..PortNumber::default()
        }),
        hop_counter: Some(HopCounter {
            hop_counter: 0x03,
            ..HopCounter::default()
        }),
        target_plmnid: Some(ServingNetwork {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            ..ServingNetwork::default()
        }),
        ..IdentificationRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
