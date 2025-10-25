use crate::gtpv1::errors::*;
use crate::gtpv1::gtpu::*;
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

pub const ERROR_INDICATION: u8 = 26;

// Definition of GTPv1-U Echo Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorIndication {
    pub header: Gtpv1Header,
    pub teid_data: Teid,
    pub peer_addr: GsnAddress,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for ErrorIndication {
    fn default() -> ErrorIndication {
        let hdr = Gtpv1Header {
            msgtype: ERROR_INDICATION,
            ..Default::default()
        };
        ErrorIndication {
            header: hdr,
            teid_data: Teid::default(),
            peer_addr: GsnAddress::default(),
            private_extension: None,
        }
    }
}

impl Messages for ErrorIndication {
    fn marshal(self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        self.teid_data.marshal(buffer);
        self.peer_addr.marshal(buffer);
        if let Some(i) = self.private_extension {
            let mut buffer_ie: Vec<u8> = vec![];
            i.marshal(&mut buffer_ie);
            buffer.append(&mut buffer_ie);
        }
        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = ErrorIndication::default();
        match Gtpv1Header::unmarshal(buffer) {
            Ok(i) => message.header = i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ERROR_INDICATION {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length + 8) as usize <= buffer.len() {
            let mut cursor = message.header.get_header_size();
            let mut increment: u8 = 0;
            loop {
                if cursor >= buffer.len() {
                    break;
                }
                if buffer[cursor] >= increment {
                    match buffer[cursor] {
                        TEID_DATA => match Teid::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.teid_data = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                        },
                        GSN_ADDRESS => match GsnAddress::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                if !msg_hash.contains_key(&buffer[cursor]) {
                                    increment = buffer[cursor];
                                    msg_hash.insert(buffer[cursor], 1);
                                    cursor += i.len();
                                    message.peer_addr = i;
                                } else {
                                    let n = *msg_hash.get(&buffer[cursor]).unwrap() + 1;
                                    msg_hash.insert(buffer[cursor], n);
                                    increment = buffer[cursor];
                                    cursor += i.len();
                                }
                            }
                            Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
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
            match (msg_hash.get(&TEID_DATA), msg_hash.get(&GSN_ADDRESS)) {
                (Some(_), Some(_)) => Ok(message),
                _ => Err(GTPV1Error::MessageMandatoryIEMissing),
            }
        } else {
            Err(GTPV1Error::MessageLengthError)
        }
    }
}

#[test]
fn test_error_indication_unmarshal() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 28] = [
        0x36, 0x1a, 0x0, 0x14, 0x0, 0x0, 0x0, 0x0, 0x49, 0xca, 0x0, 0x40, 0x1, 0x10, 0x0, 0x0,
        0x10, 0x0, 0x0, 0xff, 0xff, 0x85, 0x0, 0x4, 0x64, 0x75, 0x82, 0x35,
    ];
    let decoded = ErrorIndication {
        header: Gtpv1Header {
            msgtype: ERROR_INDICATION,
            length: 20,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: Some(vec![ExtensionHeader::UDPPort(UDPPort {
                extension_header_type: UDP_PORT,
                length: UDP_PORT_LENGTH,
                udp_port: 4096,
            })]),
        },
        teid_data: Teid {
            t: TEID_DATA,
            teid: 0xffff,
        },
        peer_addr: GsnAddress {
            t: GSN_ADDRESS,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(100, 117, 130, 53)),
        },
        private_extension: None,
    };
    assert_eq!(ErrorIndication::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_error_indication_marshal() {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded: [u8; 28] = [
        0x36, 0x1a, 0x0, 0x14, 0x0, 0x0, 0x0, 0x0, 0x49, 0xca, 0x0, 0x40, 0x1, 0x10, 0x0, 0x0,
        0x10, 0x0, 0x0, 0xff, 0xff, 0x85, 0x0, 0x4, 0x64, 0x75, 0x82, 0x35,
    ];
    let decoded = ErrorIndication {
        header: Gtpv1Header {
            msgtype: ERROR_INDICATION,
            length: 20,
            teid: 0,
            sequence_number: Some(18890),
            npdu_number: None,
            extension_headers: Some(vec![ExtensionHeader::UDPPort(UDPPort {
                extension_header_type: UDP_PORT,
                length: UDP_PORT_LENGTH,
                udp_port: 4096,
            })]),
        },
        teid_data: Teid {
            t: TEID_DATA,
            teid: 0xffff,
        },
        peer_addr: GsnAddress {
            t: GSN_ADDRESS,
            length: 4,
            ip: IpAddr::V4(Ipv4Addr::new(100, 117, 130, 53)),
        },
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_error_indication_without_mandatory_ie_unmarshal() {
    let encoded: [u8; 23] = [
        0x36, 0x1a, 0x0, 0x0f, 0x0, 0x0, 0x0, 0x0, 0x49, 0xca, 0x0, 0x40, 0x1, 0x10, 0x0, 0x0,
        0x85, 0x0, 0x4, 0x64, 0x75, 0x82, 0x35,
    ];
    assert_eq!(
        ErrorIndication::unmarshal(&encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}

#[test]
fn overflow_test() {
    // Panic (addition overflow) when trying to unmarshalling ErrorIndication GTPU message #1
    let mut buffer: Vec<u8> = vec![];
    let message = ErrorIndication {
        teid_data: Teid {
            t: 255,
            teid: 4294967295,
        },
        ..ErrorIndication::default()
    };

    message.marshal(&mut buffer);
    let _result = ErrorIndication::unmarshal(&buffer);
}
