use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const TRACE_SESSION_ACT: u8 = 71;

// Definition of GTPv2-C Trace Session Activation Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceSessionActivation {
    pub header: Gtpv2Header,
    pub imsi: Option<Imsi>,
    pub trace_info: TraceInformation,
    pub mei: Option<Mei>,
}

impl Default for TraceSessionActivation {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: TRACE_SESSION_ACT,
            teid: Some(0),
            ..Default::default()
        };
        TraceSessionActivation {
            header: hdr,
            imsi: None,
            trace_info: TraceInformation::default(),
            mei: None,
        }
    }
}

impl Messages for TraceSessionActivation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = TraceSessionActivation::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != TRACE_SESSION_ACT {
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

        if let Some(i) = self.imsi.clone() {
            elements.push(i.into());
        }

        elements.push(self.trace_info.clone().into());

        if let Some(i) = self.mei.clone() {
            elements.push(i.into());
        }

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    if let (0, true) = (j.ins, self.imsi.is_none()) {
                        self.imsi = Some(j.clone())
                    };
                }
                InformationElement::TraceInformation(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.trace_info, mandatory) = (j.clone(), true)
                    };
                }
                InformationElement::Mei(j) => {
                    if let (0, true) = (j.ins, self.mei.is_none()) {
                        self.mei = Some(j.clone())
                    };
                }
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(TRACEINFO))
        }
    }
}

#[test]
fn test_trace_session_activation_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 74] = [
        0x48, 0x47, 0x00, 0x46, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x60, 0x00, 0x22, 0x00, 0x99, 0xf9,
        0x10, 0xff, 0xff, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x07,
        0x85, 0xb8, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 0x01, 0x50, 0x94, 0x70,
    ];
    let decoded = TraceSessionActivation {
        header: Gtpv2Header {
            msgtype: TRACE_SESSION_ACT,
            piggyback: false,
            message_prio: None,
            length: 70,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        imsi: Some(Imsi {
            t: 0x01,
            length: 0x08,
            ins: 0x00,
            imsi: "901405101961873".to_string(),
        }),
        trace_info: TraceInformation {
            t: TRACEINFO,
            length: TRACEINFO_LENGTH as u16,
            ins: 0,
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            trace_id: 0xfffffa,
            trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            list_ne_types: 0xaaaa,
            trace_depth: 0x09,
            list_interfaces: vec![0; 12],
            trace_collection_ip: Ipv4Addr::new(139, 7, 133, 184),
        },
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8694920510054907".to_string(),
        }),
    };
    let message = TraceSessionActivation::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_trace_session_activation_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 74] = [
        0x48, 0x47, 0x00, 0x46, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 0x60, 0x00, 0x22, 0x00, 0x99, 0xf9,
        0x10, 0xff, 0xff, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x07,
        0x85, 0xb8, 0x4b, 0x00, 0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 0x01, 0x50, 0x94, 0x70,
    ];
    let decoded = TraceSessionActivation {
        header: Gtpv2Header {
            msgtype: TRACE_SESSION_ACT,
            piggyback: false,
            message_prio: None,
            length: 70,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
        imsi: Some(Imsi {
            t: 0x01,
            length: 0x08,
            ins: 0x00,
            imsi: "901405101961873".to_string(),
        }),
        trace_info: TraceInformation {
            t: TRACEINFO,
            length: TRACEINFO_LENGTH as u16,
            ins: 0,
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            trace_id: 0xfffffa,
            trigger_events: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            list_ne_types: 0xaaaa,
            trace_depth: 0x09,
            list_interfaces: vec![0; 12],
            trace_collection_ip: Ipv4Addr::new(139, 7, 133, 184),
        },
        mei: Some(Mei {
            t: MEI,
            length: 8,
            ins: 0,
            mei: "8694920510054907".to_string(),
        }),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
