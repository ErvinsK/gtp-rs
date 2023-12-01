use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)

pub const INITIATE_PDP_CTX_ACTIVATION_REQUEST: u8 = 22;

// Definition of GTPv1-C Initiate PDP Context Activation Request

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiatePDPContextActivationRequest {
    pub header: Gtpv1Header,
    pub linked_nsapi: Nsapi,
    pub pco: Option<Pco>,
    pub qos: Qos,
    pub tft: Option<Tft>,
    pub correlation_id: CorrelationId,
    pub evolved_alloc: Option<EvolvedAllocationRetentionI>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for InitiatePDPContextActivationRequest {
    fn default() -> InitiatePDPContextActivationRequest {
        let hdr = Gtpv1Header {
            msgtype: INITIATE_PDP_CTX_ACTIVATION_REQUEST,
            ..Default::default()
        };
        InitiatePDPContextActivationRequest {
            header: hdr,
            linked_nsapi: Nsapi::default(),
            pco: None,
            qos: Qos::default(),
            tft: None,
            correlation_id: CorrelationId::default(),
            evolved_alloc: None,
            private_extension: None,
        }
    }
}

impl Messages for InitiatePDPContextActivationRequest {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header

        self.header.marshal(buffer);

        // Marshal Linked NSAPI IE

        self.linked_nsapi.marshal(buffer);

        // Marshal PCO IE

        if let Some(i) = self.pco {
            i.marshal(buffer)
        };

        // Marshal QoS IE

        self.qos.marshal(buffer);

        // Marshal TFT IE

        if let Some(i) = self.tft {
            i.marshal(buffer)
        };

        // Marshal Correlation ID IE

        self.correlation_id.marshal(buffer);

        // Marshal Evolved Allocation/Retention Priority I IE

        if let Some(i) = self.evolved_alloc {
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

        let mut message = InitiatePDPContextActivationRequest::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != INITIATE_PDP_CTX_ACTIVATION_REQUEST {
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
                        NSAPI => match Nsapi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.linked_nsapi = i;
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
                        QOS => match Qos::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.qos = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        TFT => match Tft::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.tft = Some(i);
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                        },
                        CORRELATIONID => match CorrelationId::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.correlation_id = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        EVOLVEDALLOCRETENTIONI => {
                            match EvolvedAllocationRetentionI::unmarshal(&buffer[cursor..]) {
                                Ok(i) => {
                                    if !msg_hash.contains_key(&buffer[cursor]) {
                                        increment = buffer[cursor];
                                        msg_hash.insert(buffer[cursor], 1);
                                        cursor += i.len();
                                        message.evolved_alloc = Some(i);
                                    } else {
                                        let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                        msg_hash.insert(buffer[cursor], n);
                                        increment = buffer[cursor];
                                        cursor += i.len();
                                    }
                                }
                                Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                            }
                        }
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
            match (
                msg_hash.get(&NSAPI),
                msg_hash.get(&QOS),
                msg_hash.get(&CORRELATIONID),
            ) {
                (Some(_), Some(_), Some(_)) => Ok(message),
                _ => Err(GTPV1Error::MessageMandatoryIEMissing),
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn init_pdp_ctx_activation_req_unmarshal_test() {
    let encoded: [u8; 36] = [
        0x32, 0x16, 0x00, 0x1c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x14, 0x05, 0x87,
        0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, 0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00,
        0x00, 0x00, 0xb7, 0x00, 0x01, 0xff,
    ];
    let decoded = InitiatePDPContextActivationRequest {
        header: Gtpv1Header {
            msgtype: INITIATE_PDP_CTX_ACTIVATION_REQUEST,
            length: 28,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        linked_nsapi: Nsapi { t: NSAPI, value: 5 },
        pco: None,
        qos: Qos {
            t: 135,
            length: 15,
            arp: 3,
            qos: vec![27, 99, 31, 115, 150, 115, 115, 116, 255, 255, 255, 0, 0, 0],
        },
        tft: None,
        correlation_id: CorrelationId {
            t: CORRELATIONID,
            length: CORRELATIONID_LENGTH,
            correlation_id: 0xff,
        },
        evolved_alloc: None,
        private_extension: None,
    };
    assert_eq!(
        InitiatePDPContextActivationRequest::unmarshal(&encoded).unwrap(),
        decoded
    );
}

#[test]
fn init_pdp_ctx_activation_req_marshal_test() {
    let encoded: [u8; 36] = [
        0x32, 0x16, 0x00, 0x1c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x14, 0x05, 0x87,
        0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, 0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00,
        0x00, 0x00, 0xb7, 0x00, 0x01, 0xff,
    ];
    let decoded = InitiatePDPContextActivationRequest {
        header: Gtpv1Header {
            msgtype: INITIATE_PDP_CTX_ACTIVATION_REQUEST,
            length: 28,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        linked_nsapi: Nsapi { t: NSAPI, value: 5 },
        pco: None,
        qos: Qos {
            t: 135,
            length: 15,
            arp: 3,
            qos: vec![27, 99, 31, 115, 150, 115, 115, 116, 255, 255, 255, 0, 0, 0],
        },
        tft: None,
        correlation_id: CorrelationId {
            t: CORRELATIONID,
            length: CORRELATIONID_LENGTH,
            correlation_id: 0xff,
        },
        evolved_alloc: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn init_pdp_ctx_activation_req_wrong_ie_order_unmarshal_test() {
    let encoded: [u8; 36] = [
        0x32, 0x16, 0x00, 0x1c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0xb7, 0x00, 0x01,
        0xff, 0x14, 0x05, 0x87, 0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, 0x96, 0x73, 0x73, 0x74,
        0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
    ];
    assert_eq!(
        InitiatePDPContextActivationRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn init_pdp_ctx_activation_req_missing_mandatory_ie_unmarshal_test() {
    let encoded: [u8; 32] = [
        0x32, 0x16, 0x00, 0x18, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x00, 0x00, 0x14, 0x05, 0x87,
        0x00, 0x0f, 0x03, 0x1b, 0x63, 0x1f, 0x73, 0x96, 0x73, 0x73, 0x74, 0xff, 0xff, 0xff, 0x00,
        0x00, 0x00,
    ];
    assert_eq!(
        InitiatePDPContextActivationRequest::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
