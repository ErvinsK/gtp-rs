use crate::gtpv2::{
    errors::*,
    header::*,
    messages::{commons::*, ies::*},
    utils::*,
};

// According to 3GPP TS 29.274 V17.10.0 (2023-12)

pub const UE_REG_QUERY_RESP: u8 = 159;

// Definition of GTPv2-C UE Registration Query Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UeRegistrationQueryResponse {
    pub header: Gtpv2Header,
    pub cause: Cause,
    pub imsi: Imsi,
    pub selected_cnoid: PlmnId,
    pub private_ext: Vec<PrivateExtension>,
}

impl Default for UeRegistrationQueryResponse {
    fn default() -> UeRegistrationQueryResponse {
        UeRegistrationQueryResponse {
            header: Gtpv2Header {
                msgtype: UE_REG_QUERY_RESP,
                teid: Some(0),
                ..Gtpv2Header::default()
            },
            cause: Cause::default(),
            imsi: Imsi::default(),
            selected_cnoid: PlmnId::default(),
            private_ext: vec![],
        }
    }
}

impl Messages for UeRegistrationQueryResponse {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.tovec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = UeRegistrationQueryResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != UE_REG_QUERY_RESP {
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

        elements.push(self.cause.clone().into());

        elements.push(self.imsi.clone().into());

        elements.push(self.selected_cnoid.clone().into());

        self.private_ext
            .iter()
            .for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));

        elements
    }

    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory: [bool; 3] = [false; 3];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    if j.ins == 0 && !mandatory[0] {
                        mandatory[0] = true;
                        self.cause = j.clone();
                    };
                }
                InformationElement::Imsi(j) => {
                    if j.ins == 0 && !mandatory[1] {
                        mandatory[1] = true;
                        self.imsi = j.clone();
                    };
                }
                InformationElement::PlmnId(j) => {
                    if j.ins == 0 && !mandatory[2] {
                        mandatory[2] = true;
                        self.selected_cnoid = j.clone();
                    };
                }
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1], mandatory[2]) {
            (true, true, true) => Ok(true),
            (false, _, _) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (_, false, _) => Err(GTPV2Error::MessageMandatoryIEMissing(IMSI)),
            (_, _, false) => Err(GTPV2Error::MessageMandatoryIEMissing(PLMNID)),
        }
    }
}

#[test]
fn test_ue_reg_query_resp_unmarshal() {
    let encoded: [u8; 45] = [
        0x48, 0x9f, 0x00, 0x29, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x01, 0x00, 0x06, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0x00,
        0x03, 0x00, 0x99, 0xf9, 0x10, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = UeRegistrationQueryResponse {
        header: Gtpv2Header {
            msgtype: UE_REG_QUERY_RESP,
            piggyback: false,
            message_prio: None,
            length: 41,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 16,
            ..Cause::default()
        },
        imsi: Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        },
        selected_cnoid: PlmnId {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            ..PlmnId::default()
        },
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let message = UeRegistrationQueryResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message, decoded);
}

#[test]
fn test_ue_reg_query_resp_marshal() {
    let encoded: [u8; 45] = [
        0x48, 0x9f, 0x00, 0x29, 0x09, 0x09, 0xa4, 0x56, 0x00, 0x00, 0x2f, 0x00, 0x02, 0x00, 0x02,
        0x00, 0x10, 0x00, 0x01, 0x00, 0x06, 0x00, 0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0x00,
        0x03, 0x00, 0x99, 0xf9, 0x10, 0xff, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x62, 0x9c, 0xc4,
    ];
    let decoded = UeRegistrationQueryResponse {
        header: Gtpv2Header {
            msgtype: UE_REG_QUERY_RESP,
            piggyback: false,
            message_prio: None,
            length: 41,
            teid: Some(0x0909a456),
            sqn: 0x2f,
        },
        cause: Cause {
            value: 16,
            ..Cause::default()
        },
        imsi: Imsi {
            length: 6,
            imsi: "901405101073".to_string(),
            ..Imsi::default()
        },
        selected_cnoid: PlmnId {
            mcc: 999,
            mnc: 1,
            mnc_is_three_digits: false,
            ..PlmnId::default()
        },
        private_ext: vec![PrivateExtension {
            length: 6,
            value: vec![0x01, 0x62, 0x9c, 0xc4],
            ..Default::default()
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!("{:#04x}, ", x));
    assert_eq!(buffer, encoded);
}
