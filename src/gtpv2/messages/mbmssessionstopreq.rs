use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const MBMS_SESSION_STOP_REQ: u8 = 235;

// Definition of GTPv2-C MBMS Session Stop Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbmsSessionStopRequest {
    pub header: Gtpv2Header,
    pub mbms_flow_id: Option<MbmsFlowId>,
    pub mbms_data_transfer_stop: Option<AbsoluteTimeMbmsDataTransfer>, // MBMS Data Transfer Stop
    pub mbms_flags: Option<MbmsFlags>,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for MbmsSessionStopRequest {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: MBMS_SESSION_STOP_REQ,
            teid: Some(0),
            ..Default::default()
        };
        MbmsSessionStopRequest {
            header: hdr,
            mbms_flow_id: None,
            mbms_data_transfer_stop: None,
            mbms_flags: None,
            private_ext: vec![],
        }
    }
}

impl Messages for MbmsSessionStopRequest {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = MbmsSessionStopRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != MBMS_SESSION_STOP_REQ {
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
        let mut elements = Vec::new();

        if let Some(i) = self.mbms_flow_id.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mbms_data_transfer_stop.clone() {
            elements.push(i.into());
        }

        if let Some(i) = self.mbms_flags.clone() {
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
                InformationElement::MbmsFlowId(j) => {
                    if let (0, true) = (j.ins, self.mbms_flow_id.is_none()) {
                        self.mbms_flow_id = Some(j.clone());
                    }
                }
                InformationElement::AbsoluteTimeMbmsDataTransfer(j) => {
                    if let (0, true) = (j.ins, self.mbms_data_transfer_stop.is_none()) {
                        self.mbms_data_transfer_stop = Some(j.clone());
                    }
                }
                InformationElement::MbmsFlags(j) => {
                    if let (0, true) = (j.ins, self.mbms_flags.is_none()) {
                        self.mbms_flags = Some(j.clone());
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
fn test_mbms_session_stop_req_unmarshal() {
    let encoded: [u8; 35] = [
        0x48, 0xeb, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x8d, 0x00, 0x02,
        0x00, 0x0a, 0x0f, 0xa4, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xab, 0x00, 0x01, 0x00, 0x01,
    ];
    let decoded = MbmsSessionStopRequest {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_STOP_REQ,
            piggyback: false,
            message_prio: None,
            length: 31,
            teid: Some(0),
            sqn: 0x68,
        },
        mbms_flow_id: Some(MbmsFlowId {
            mbms_flowid: 0x0a0f,
            ..MbmsFlowId::default()
        }),
        mbms_data_transfer_stop: Some(AbsoluteTimeMbmsDataTransfer {
            seconds: 0xffff,
            ..AbsoluteTimeMbmsDataTransfer::default()
        }),
        mbms_flags: Some(MbmsFlags {
            msri: true,
            lmri: false,
            ..MbmsFlags::default()
        }),
        ..MbmsSessionStopRequest::default()
    };
    let message = MbmsSessionStopRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_mmbms_session_stop_req_marshal() {
    let encoded: [u8; 35] = [
        0x48, 0xeb, 0x00, 0x1f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x00, 0x8d, 0x00, 0x02,
        0x00, 0x0a, 0x0f, 0xa4, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xab, 0x00, 0x01, 0x00, 0x01,
    ];
    let decoded = MbmsSessionStopRequest {
        header: Gtpv2Header {
            msgtype: MBMS_SESSION_STOP_REQ,
            piggyback: false,
            message_prio: None,
            length: 31,
            teid: Some(0),
            sqn: 0x68,
        },
        mbms_flow_id: Some(MbmsFlowId {
            mbms_flowid: 0x0a0f,
            ..MbmsFlowId::default()
        }),
        mbms_data_transfer_stop: Some(AbsoluteTimeMbmsDataTransfer {
            seconds: 0xffff,
            ..AbsoluteTimeMbmsDataTransfer::default()
        }),
        mbms_flags: Some(MbmsFlags {
            msri: true,
            lmri: false,
            ..MbmsFlags::default()
        }),
        ..MbmsSessionStopRequest::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
