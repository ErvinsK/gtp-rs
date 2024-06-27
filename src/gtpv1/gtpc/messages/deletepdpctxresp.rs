use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const DELETE_PDP_CONTEXT_RESPONSE: u8 = 21;

// Definition of GTPv1-C Delete PDP Context Response

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletePDPContextResponse {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub pco: Option<Pco>,
    pub uli: Option<Uli>,
    pub ms_timezone: Option<MsTimeZone>,
    pub uli_timestamp: Option<UliTimestamp>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for DeletePDPContextResponse {
    fn default() -> DeletePDPContextResponse {
        let hdr = Gtpv1Header {
            msgtype: DELETE_PDP_CONTEXT_RESPONSE,
            ..Default::default()
        };
        DeletePDPContextResponse {
            header: hdr,
            cause: Cause::default(),
            pco: None,
            uli: None,
            ms_timezone: None,
            uli_timestamp: None,
            private_extension: None,
        }
    }
}

impl Messages for DeletePDPContextResponse {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Cause IE

        self.cause.marshal(buffer);

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal ULI IE

        if let Some(i) = self.uli {
            i.marshal(buffer)
        };

        // Marshal MS Time Zone IE

        if let Some(i) = self.ms_timezone {
            i.marshal(buffer)
        };

        // Marshal ULI Timestamp IE

        if let Some(i) = self.uli_timestamp {
            i.marshal(buffer)
        };

        // Marshal Private Extension IE

        if let Some(i) = self.private_extension {
            i.marshal(buffer)
        };

        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = DeletePDPContextResponse::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_PDP_CONTEXT_RESPONSE {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length + 8) as usize <= buffer.len() {
            let mut cursor = message.header.len();
            let mut increment: u8 = 0;
            loop {
                if cursor >= buffer.len() {
                    break;
                }
                if buffer[cursor] >= increment {
                    match buffer[cursor] {
                        CAUSE => match Cause::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.cause = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        PCO => match Pco::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.pco = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        ULI => match Uli::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.uli = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        MSTIMEZONETYPE => match MsTimeZone::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.ms_timezone = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        ULI_TIMESTAMP => match UliTimestamp::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.uli_timestamp = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        PRIVATE_EXTENSION => match PrivateExtension::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.private_extension = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        _ => return Err(GTPV1Error::MessageInvalidMessageFormat),
                    }
                } else {
                    return Err(GTPV1Error::MessageInvalidMessageFormat);
                }
            }
            if msg_hash.contains_key(&CAUSE) {
                Ok(message)
            } else {
                Err(GTPV1Error::MessageMandatoryIEMissing)
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn delete_pdp_ctx_resp_unmarshal_test() {
    let encoded: [u8; 14] = [
        0x32, 0x15, 0x00, 0x06, 0x0c, 0xbf, 0x0a, 0x50, 0x07, 0xe7, 0x00, 0x00, 0x01, 0x80,
    ];
    let decoded = DeletePDPContextResponse {
        header: Gtpv1Header {
            msgtype: DELETE_PDP_CONTEXT_RESPONSE,
            length: 6,
            teid: 213846608,
            sequence_number: Some(2023),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause {
            t: CAUSE,
            value: 128,
        },
        pco: None,
        uli: None,
        ms_timezone: None,
        uli_timestamp: None,
        private_extension: None,
    };
    assert_eq!(
        DeletePDPContextResponse::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn delete_pdp_ctx_resp_marshal_test() {
    let encoded: [u8; 14] = [
        0x32, 0x15, 0x00, 0x06, 0x0c, 0xbf, 0x0a, 0x50, 0x07, 0xe7, 0x00, 0x00, 0x01, 0x80,
    ];
    let decoded = DeletePDPContextResponse {
        header: Gtpv1Header {
            msgtype: DELETE_PDP_CONTEXT_RESPONSE,
            length: 6,
            teid: 213846608,
            sequence_number: Some(2023),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause {
            t: CAUSE,
            value: 128,
        },
        pco: None,
        uli: None,
        ms_timezone: None,
        uli_timestamp: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn delete_pdp_ctx_resp_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 21] = [
        0x32, 0x15, 0x00, 0x0d, 0x0c, 0xbf, 0x0a, 0x50, 0x07, 0xe7, 0x00, 0x00, 0xd6, 0x00, 0x04,
        0xee, 0x6b, 0x28, 0x00, 0x01, 0x80,
    ];
    assert_eq!(
        DeletePDPContextResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn delete_pdp_ctx_resp_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 19] = [
        0x32, 0x15, 0x00, 0x0b, 0x0c, 0xbf, 0x0a, 0x50, 0x07, 0xe7, 0x00, 0x00, 0xd6, 0x00, 0x04,
        0xee, 0x6b, 0x28, 0x00,
    ];
    assert_eq!(
        DeletePDPContextResponse::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
