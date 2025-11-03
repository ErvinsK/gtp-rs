use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const TRACE_SESSION_DEACT: u8 = 72;

// Definition of GTPv2-C Trace Session Deactivation Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceSessionDeactivation {
    pub header: Gtpv2Header,
    pub trace_info: TraceInformation,
}

impl Default for TraceSessionDeactivation {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: TRACE_SESSION_DEACT,
            teid: Some(0),
            ..Default::default()
        };
        TraceSessionDeactivation {
            header: hdr,
            trace_info: TraceInformation::default(),
        }
    }
}

impl Messages for TraceSessionDeactivation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = TraceSessionDeactivation::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != TRACE_SESSION_DEACT {
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
        vec![self.trace_info.clone().into()]
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        elements.iter().for_each(|e| {
            if let InformationElement::TraceInformation(j) = e {
                if let (0, false) = (j.ins, mandatory) {
                    (self.trace_info, mandatory) = (j.clone(), true)
                }
            }
        });
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(TRACEINFO))
        }
    }
}

#[test]
fn test_trace_session_deactivation_unmarshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 50] = [
        0x48, 0x48, 0x00, 0x2e, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x60, 0x00, 0x22,
        0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xaa, 0xaa, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let decoded = TraceSessionDeactivation {
        header: Gtpv2Header {
            msgtype: TRACE_SESSION_DEACT,
            piggyback: false,
            message_prio: None,
            length: 46,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
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
    };
    let message = TraceSessionDeactivation::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_trace_session_deactivation_marshal() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 50] = [
        0x48, 0x48, 0x00, 0x2e, 0xe6, 0x4d, 0xa4, 0xef, 0x26, 0x00, 0x2e, 0x00, 0x60, 0x00, 0x22,
        0x00, 0x99, 0xf9, 0x10, 0xff, 0xff, 0xfa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xaa, 0xaa, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x8b, 0x07, 0x85, 0xb8,
    ];
    let decoded = TraceSessionDeactivation {
        header: Gtpv2Header {
            msgtype: TRACE_SESSION_DEACT,
            piggyback: false,
            message_prio: None,
            length: 46,
            teid: Some(0xe64da4ef),
            sqn: 0x26002e,
        },
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
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
