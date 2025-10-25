use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::commons::*;
use crate::gtpv1::gtpc::messages::ies::*;
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const DELETE_PDP_CONTEXT_REQUEST: u8 = 20;

// Definition of GTPv1-C Delete PDP Context Request

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeletePDPContextRequest {
    pub header: Gtpv1Header,
    pub cause: Option<Cause>,
    pub teardown: Option<TeardownInd>,
    pub nsapi: Nsapi,
    pub pco: Option<Pco>,
    pub uli: Option<Uli>,
    pub ms_timezone: Option<MsTimeZone>,
    pub ext_common_flags: Option<ExtendedCommonFlags>,
    pub uli_timestamp: Option<UliTimestamp>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for DeletePDPContextRequest {
    fn default() -> DeletePDPContextRequest {
        let hdr = Gtpv1Header {
            msgtype: DELETE_PDP_CONTEXT_REQUEST,
            ..Default::default()
        };
        DeletePDPContextRequest {
            header: hdr,
            cause: None,
            teardown: None,
            nsapi: Nsapi::default(),
            pco: None,
            uli: None,
            ms_timezone: None,
            ext_common_flags: None,
            uli_timestamp: None,
            private_extension: None,
        }
    }
}

impl Messages for DeletePDPContextRequest {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Cause IE

        if let Some(i) = self.cause {
            i.marshal(buffer)
        };

        // Marshal Teardown Ind IE

        if let Some(i) = self.teardown {
            i.marshal(buffer)
        };

        // Marshal NSAPI IE

        self.nsapi.marshal(buffer);

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

        // Marshal Extended Common Flags IE

        if let Some(i) = self.ext_common_flags {
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

        let mut message = DeletePDPContextRequest::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_PDP_CONTEXT_REQUEST {
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
                                    message.cause = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        TEARDOWN_IND => match TeardownInd::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.teardown = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        NSAPI => match Nsapi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.nsapi = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
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
                        EXTCOMMONFLAGS => match ExtendedCommonFlags::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.ext_common_flags = Some(i);
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
            if msg_hash.contains_key(&NSAPI) {
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
fn delete_pdp_ctx_req_unmarshal_test() {
    let encoded: [u8; 16] = [
        0x32, 0x14, 0x00, 0x08, 0x85, 0x92, 0xc6, 0x4a, 0x07, 0xe7, 0x00, 0x00, 0x13, 0xff, 0x14,
        0x06,
    ];
    let decoded = DeletePDPContextRequest {
        header: Gtpv1Header {
            msgtype: DELETE_PDP_CONTEXT_REQUEST,
            length: 8,
            teid: 2240988746,
            sequence_number: Some(2023),
            npdu_number: None,
            extension_headers: None,
        },
        cause: None,
        teardown: Some(TeardownInd {
            t: TEARDOWN_IND,
            teardown: true,
        }),
        nsapi: Nsapi { t: 20, value: 6 },
        pco: None,
        uli: None,
        ms_timezone: None,
        ext_common_flags: None,
        uli_timestamp: None,
        private_extension: None,
    };
    assert_eq!(
        DeletePDPContextRequest::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn delete_pdp_ctx_req_marshal_test() {
    let encoded: [u8; 16] = [
        0x32, 0x14, 0x00, 0x08, 0x85, 0x92, 0xc6, 0x4a, 0x07, 0xe7, 0x00, 0x00, 0x13, 0xff, 0x14,
        0x06,
    ];
    let decoded = DeletePDPContextRequest {
        header: Gtpv1Header {
            msgtype: DELETE_PDP_CONTEXT_REQUEST,
            length: 8,
            teid: 2240988746,
            sequence_number: Some(2023),
            npdu_number: None,
            extension_headers: None,
        },
        cause: None,
        teardown: Some(TeardownInd {
            t: TEARDOWN_IND,
            teardown: true,
        }),
        nsapi: Nsapi { t: 20, value: 6 },
        pco: None,
        uli: None,
        ms_timezone: None,
        ext_common_flags: None,
        uli_timestamp: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn delete_pdp_ctx_req_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 16] = [
        0x32, 0x14, 0x00, 0x08, 0x85, 0x92, 0xc6, 0x4a, 0x07, 0xe7, 0x00, 0x00, 0x14, 0x06, 0x13,
        0xff,
    ];
    assert_eq!(
        DeletePDPContextRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn delete_pdp_ctx_req_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 14] = [
        0x32, 0x14, 0x00, 0x06, 0x85, 0x92, 0xc6, 0x4a, 0x07, 0xe7, 0x00, 0x00, 0x13, 0xff,
    ];
    assert_eq!(
        DeletePDPContextRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
