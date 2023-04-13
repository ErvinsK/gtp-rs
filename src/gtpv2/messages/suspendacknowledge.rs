use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const SUSPEND_ACK: u8 = 163;

// Definition of GTPv2-C Suspend Acknowledge Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspendAcknowledge {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for SuspendAcknowledge {
    fn default() -> Self {
        let hdr = Gtpv2Header {
            msgtype: SUSPEND_ACK,
            teid: Some(0),
            ..Default::default()
        };
        SuspendAcknowledge {
            header: hdr,
            cause: Cause::default(),
            private_ext: vec![],
        }
    }
}

impl Messages for SuspendAcknowledge {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = SuspendAcknowledge::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != SUSPEND_ACK {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize) + 4 <= buffer.len() {
            match InformationElement::decoder(&buffer[12..]) {
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

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    if let (0, false) = (j.ins, mandatory) {
                        (self.cause, mandatory) = (j, true);
                    }
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
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
fn test_suspend_ack_unmarshal() {
    let encoded: [u8; 28] = [
        0x48, 0xa3, 0x00, 0x18, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let mut decoded = SuspendAcknowledge::default();
    decoded.header = Gtpv2Header {
        msgtype: SUSPEND_ACK,
        piggyback: false,
        message_prio: None,
        length: 24,
        teid: Some(0xa4789580),
        sqn: 0x4b291e,
    };
    decoded.cause = Cause {
        t: CAUSE,
        length: 2,
        ins: 0,
        value: 16,
        pce: false,
        bce: false,
        cs: false,
        offend_ie_type: None,
    };
    decoded.private_ext = vec![PrivateExtension {
        t: PRIVATE_EXT,
        length: 6,
        ins: 0,
        enterprise_id: 2011,
        value: vec![0x07, 0x00, 0x01, 0x00],
    }];
    let message = SuspendAcknowledge::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_suspend_ack_marshal() {
    let encoded: [u8; 28] = [
        0x48, 0xa3, 0x00, 0x18, 0xa4, 0x78, 0x95, 0x80, 0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 0x07, 0x00, 0x01, 0x00,
    ];
    let mut decoded = SuspendAcknowledge::default();
    decoded.header = Gtpv2Header {
        msgtype: SUSPEND_ACK,
        piggyback: false,
        message_prio: None,
        length: 24,
        teid: Some(0xa4789580),
        sqn: 0x4b291e,
    };
    decoded.cause = Cause {
        t: CAUSE,
        length: 2,
        ins: 0,
        value: 16,
        pce: false,
        bce: false,
        cs: false,
        offend_ie_type: None,
    };
    decoded.private_ext = vec![PrivateExtension {
        t: PRIVATE_EXT,
        length: 6,
        ins: 0,
        enterprise_id: 2011,
        value: vec![0x07, 0x00, 0x01, 0x00],
    }];
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
