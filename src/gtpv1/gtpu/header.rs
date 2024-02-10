use crate::gtpv1::errors::*;
use crate::gtpv1::gtpu::extensionheaders::*;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)

// Enum to hold all possible Extension headers for GTPv1-U

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtensionHeader {
    NoMoreExtensionHeaders,
    PDCPPDUNumber(PDCPPDUNumber),
    UDPPort(UDPPort),
    LongPDCPPDUNumber(LongPDCPPDUNumber),
    Sci(Sci),
    RanContainer(RanContainer),
    XwRanContainer(XwRanContainer),
    NrRanContainer(NrRanContainer),
    PduSessionContainer(PduSessionContainer),
    Unknown(Unknown),
}

impl ExtensionHeader {
    pub fn unmarshal(buffer: &[u8]) -> Result<ExtensionHeader, GTPV1Error> {
        match buffer[0] {
            NO_MORE_EXTENSION_HEADERS => Ok(ExtensionHeader::NoMoreExtensionHeaders),
            PDCP_PDU_NUMBER => match PDCPPDUNumber::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::PDCPPDUNumber(i)),
                Err(j) => Err(j),
            },
            UDP_PORT => match UDPPort::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::UDPPort(i)),
                Err(j) => Err(j),
            },
            LONG_PDCP_PDU_NUMBER_I => match LongPDCPPDUNumber::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::LongPDCPPDUNumber(i)),
                Err(j) => Err(j),
            },
            LONG_PDCP_PDU_NUMBER_II => match LongPDCPPDUNumber::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::LongPDCPPDUNumber(i)),
                Err(j) => Err(j),
            },
            SCI => match Sci::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::Sci(i)),
                Err(j) => Err(j),
            },
            RAN_CONTAINER => match RanContainer::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::RanContainer(i)),
                Err(j) => Err(j),
            },
            XW_RAN_CONTAINER => match XwRanContainer::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::XwRanContainer(i)),
                Err(j) => Err(j),
            },
            NR_RAN_CONTAINER => match NrRanContainer::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::NrRanContainer(i)),
                Err(j) => Err(j),
            },
            PDU_SESSION_CONTAINER => match PduSessionContainer::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::PduSessionContainer(i)),
                Err(j) => Err(j),
            },
            _ => match Unknown::unmarshal(buffer) {
                Ok(i) => Ok(ExtensionHeader::Unknown(i)),
                Err(j) => Err(j),
            },
        }
    }

    pub fn marshal(self, buffer: &mut Vec<u8>) {
        match self {
            ExtensionHeader::NoMoreExtensionHeaders => (),
            ExtensionHeader::PDCPPDUNumber(i) => i.marshal(buffer),
            ExtensionHeader::LongPDCPPDUNumber(i) => i.marshal(buffer),
            ExtensionHeader::Sci(i) => i.marshal(buffer),
            ExtensionHeader::UDPPort(i) => i.marshal(buffer),
            ExtensionHeader::RanContainer(i) => i.marshal(buffer),
            ExtensionHeader::XwRanContainer(i) => i.marshal(buffer),
            ExtensionHeader::NrRanContainer(i) => i.marshal(buffer),
            ExtensionHeader::PduSessionContainer(i) => i.marshal(buffer),
            ExtensionHeader::Unknown(i) => i.marshal(buffer),
        }
    }

    pub fn len(self) -> usize {
        match self {
            ExtensionHeader::NoMoreExtensionHeaders => 1,
            ExtensionHeader::PDCPPDUNumber(i) => i.len(),
            ExtensionHeader::LongPDCPPDUNumber(i) => i.len(),
            ExtensionHeader::Sci(i) => i.len(),
            ExtensionHeader::UDPPort(i) => i.len(),
            ExtensionHeader::RanContainer(i) => i.len(),
            ExtensionHeader::XwRanContainer(i) => i.len(),
            ExtensionHeader::NrRanContainer(i) => i.len(),
            ExtensionHeader::PduSessionContainer(i) => i.len(),
            ExtensionHeader::Unknown(i) => i.len(),
        }
    }

    pub fn is_empty(self) -> bool {
        match self {
            ExtensionHeader::NoMoreExtensionHeaders => true,
            ExtensionHeader::PDCPPDUNumber(i) => i.is_empty(),
            ExtensionHeader::LongPDCPPDUNumber(i) => i.is_empty(),
            ExtensionHeader::Sci(i) => i.is_empty(),
            ExtensionHeader::UDPPort(i) => i.is_empty(),
            ExtensionHeader::RanContainer(i) => i.is_empty(),
            ExtensionHeader::XwRanContainer(i) => i.is_empty(),
            ExtensionHeader::NrRanContainer(i) => i.is_empty(),
            ExtensionHeader::PduSessionContainer(i) => i.is_empty(),
            ExtensionHeader::Unknown(i) => i.is_empty(),
        }
    }
}

// Definition of GTPv1-U Header

pub const MIN_HEADER_LENGTH: usize = 8;
pub const SQN_LENGTH: usize = 2;
pub const NPDU_NUMBER_LENGTH: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gtpv1Header {
    pub msgtype: u8,
    pub length: u16,
    pub teid: u32,
    pub sequence_number: Option<u16>,
    pub npdu_number: Option<u8>,
    pub extension_headers: Option<Vec<ExtensionHeader>>,
}

impl Default for Gtpv1Header {
    fn default() -> Self {
        Gtpv1Header {
            msgtype: 0,
            length: MIN_HEADER_LENGTH as u16,
            teid: 0,
            sequence_number: None,
            npdu_number: None,
            extension_headers: None,
        }
    }
}

impl Gtpv1Header {
    pub fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.construct_flags());
        buffer.push(self.msgtype);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.extend_from_slice(&self.teid.to_be_bytes());
        match (
            self.sequence_number.is_some(),
            self.npdu_number.is_some(),
            self.extension_headers.is_some(),
        ) {
            (true, true, true) => {
                buffer.extend_from_slice(&self.sequence_number.unwrap().to_be_bytes());
                buffer.push(self.npdu_number.unwrap());
                self.marshal_ext_hdr(buffer);
            }
            (true, true, false) => {
                buffer.extend_from_slice(&self.sequence_number.unwrap().to_be_bytes());
                buffer.push(self.npdu_number.unwrap());
                buffer.push(NO_MORE_EXTENSION_HEADERS);
            }
            (true, false, false) => {
                buffer.extend_from_slice(&self.sequence_number.unwrap().to_be_bytes());
                buffer.push(0x00);
                buffer.push(NO_MORE_EXTENSION_HEADERS);
            }
            (true, false, true) => {
                buffer.extend_from_slice(&self.sequence_number.unwrap().to_be_bytes());
                buffer.push(0x00);
                self.marshal_ext_hdr(buffer);
            }
            (false, false, false) => (),
            (false, false, true) => {
                buffer.extend_from_slice(&[0; 3]);
                self.marshal_ext_hdr(buffer);
            }
            (false, true, true) => {
                buffer.extend_from_slice(&[0; 2]);
                buffer.push(self.npdu_number.unwrap());
                self.marshal_ext_hdr(buffer);
            }
            (false, true, false) => {
                buffer.extend_from_slice(&[0; 2]);
                buffer.push(self.npdu_number.unwrap());
                buffer.push(NO_MORE_EXTENSION_HEADERS);
            }
        }
    }

    pub fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= MIN_HEADER_LENGTH {
            let mut data = Gtpv1Header::default();
            let gtp_version = buffer[0] >> 5;
            let gtp_type = (buffer[0] & 0x10) >> 4;
            match (gtp_version, gtp_type) {
                (1, 1) => (),
                (_, _) => return Err(GTPV1Error::HeaderVersionNotSupported),
            }
            data.msgtype = buffer[1];
            data.length = match u16::from_be_bytes([buffer[2], buffer[3]]) {
                0 => return Err(GTPV1Error::HeaderInvalidLength),
                _ => u16::from_be_bytes([buffer[2], buffer[3]]),
            };
            data.teid = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
            match (
                (buffer[0] & 0x02) >> 1,
                buffer[0] & 0x01,
                (buffer[0] & 0x04) >> 2,
            ) {
                (1, 1, 1) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = Some(buffer[10]);
                        match Gtpv1Header::unmarshal_ext_hdr(&buffer[11..]) {
                            Ok(i) => data.extension_headers = Some(i),
                            Err(j) => return Err(j),
                        }
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                (1, 1, 0) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = Some(buffer[10]);
                        data.extension_headers = None;
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                (1, 0, 0) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = None;
                        data.extension_headers = None;
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                (1, 0, 1) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = None;
                        match Gtpv1Header::unmarshal_ext_hdr(&buffer[11..]) {
                            Ok(i) => data.extension_headers = Some(i),
                            Err(j) => return Err(j),
                        }
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                (0, 0, 0) => {
                    data.sequence_number = None;
                    data.npdu_number = None;
                    data.extension_headers = None;
                }
                (0, 0, 1) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = Some(buffer[10]);
                        match Gtpv1Header::unmarshal_ext_hdr(&buffer[11..]) {
                            Ok(i) => data.extension_headers = Some(i),
                            Err(j) => return Err(j),
                        }
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                (0, 1, 1) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = Some(buffer[10]);
                        match Gtpv1Header::unmarshal_ext_hdr(&buffer[11..]) {
                            Ok(i) => data.extension_headers = Some(i),
                            Err(j) => return Err(j),
                        }
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                (0, 1, 0) => {
                    if buffer[8..].len() >= 4 {
                        data.sequence_number = Some(u16::from_be_bytes([buffer[8], buffer[9]]));
                        data.npdu_number = Some(buffer[10]);
                        match Gtpv1Header::unmarshal_ext_hdr(&buffer[11..]) {
                            Ok(i) => data.extension_headers = Some(i),
                            Err(j) => return Err(j),
                        }
                    } else {
                        return Err(GTPV1Error::HeaderInvalidLength);
                    }
                }
                _ => (),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::HeaderInvalidLength)
        }
    }

    // Struct helper functions

    fn construct_flags(&self) -> u8 {
        let mut flags: u8 = 0;
        if self.extension_headers.is_some() {
            flags = 0x04;
        }
        if self.sequence_number.is_some() {
            flags |= 0x02;
        }
        if self.npdu_number.is_some() {
            flags |= 0x01;
        }
        flags | 0x30
    }

    fn marshal_ext_hdr(&self, buffer: &mut Vec<u8>) {
        if let Some(i) = &self.extension_headers {
            if !i.is_empty() {
                for k in i.iter() {
                    k.clone().marshal(buffer);
                }
                buffer.push(NO_MORE_EXTENSION_HEADERS);
            } else {
                buffer.push(NO_MORE_EXTENSION_HEADERS);
            }
        } else {
            buffer.push(NO_MORE_EXTENSION_HEADERS);
        }
    }

    fn unmarshal_ext_hdr(buffer: &[u8]) -> Result<Vec<ExtensionHeader>, GTPV1Error> {
        let mut data: Vec<ExtensionHeader> = vec![];
        let mut cursor = 0;
        loop {
            match ExtensionHeader::unmarshal(&buffer[cursor..]) {
                Ok(i) => {
                    if i == ExtensionHeader::NoMoreExtensionHeaders {
                        break;
                    }
                    cursor += i.clone().len();
                    data.push(i);
                }
                Err(j) => return Err(j),
            }
        }
        Ok(data)
    }

    pub fn get_header_size(&self) -> usize {
        match (
            self.sequence_number.is_some(),
            self.npdu_number.is_some(),
            self.extension_headers.is_some(),
        ) {
            (true, true, true) => {
                MIN_HEADER_LENGTH
                    + SQN_LENGTH
                    + NPDU_NUMBER_LENGTH
                    + self
                        .extension_headers
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|x| x.len())
                        .sum::<usize>()
                    + 1
            }
            (true, true, false) => MIN_HEADER_LENGTH + SQN_LENGTH + NPDU_NUMBER_LENGTH + 1,
            (true, false, false) => MIN_HEADER_LENGTH + SQN_LENGTH + NPDU_NUMBER_LENGTH + 1,
            (true, false, true) => {
                MIN_HEADER_LENGTH
                    + SQN_LENGTH
                    + NPDU_NUMBER_LENGTH
                    + self
                        .extension_headers
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|x| x.len())
                        .sum::<usize>()
                    + 1
            }
            (false, false, false) => MIN_HEADER_LENGTH,
            (false, false, true) => {
                MIN_HEADER_LENGTH
                    + SQN_LENGTH
                    + NPDU_NUMBER_LENGTH
                    + self
                        .extension_headers
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|x| x.len())
                        .sum::<usize>()
                    + 1
            }
            (false, true, true) => {
                MIN_HEADER_LENGTH
                    + SQN_LENGTH
                    + NPDU_NUMBER_LENGTH
                    + self
                        .extension_headers
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|x| x.len())
                        .sum::<usize>()
                    + 1
            }
            (false, true, false) => MIN_HEADER_LENGTH + SQN_LENGTH + NPDU_NUMBER_LENGTH + 1,
        }
    }
}

#[test]
fn test_gtpv1_hdr_bare_unmarshal() {
    let encoded: [u8; 8] = [0x30, 0xff, 0x00, 0x34, 0x16, 0x62, 0x67, 0x19];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0xff,
        length: 52,
        teid: 0x16626719,
        sequence_number: None,
        npdu_number: None,
        extension_headers: None,
    };
    assert_eq!(Gtpv1Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv1_hdr_bare_marshal() {
    let encoded: [u8; 8] = [0x30, 0xff, 0x00, 0x34, 0x16, 0x62, 0x67, 0x19];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0xff,
        length: 52,
        teid: 0x16626719,
        sequence_number: None,
        npdu_number: None,
        extension_headers: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_unmarshal() {
    let encoded: [u8; 12] = [
        0x32, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 6,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: None,
    };
    assert_eq!(Gtpv1Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_marshal() {
    let encoded: [u8; 12] = [
        0x32, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 6,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_and_one_ext_header_unmarshal() {
    let encoded: [u8; 16] = [
        0x36, 0x02, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x40, 0x01, 0x10, 0x00,
        0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 8,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: Some(vec![ExtensionHeader::UDPPort(UDPPort {
            extension_header_type: UDP_PORT,
            length: UDP_PORT_LENGTH,
            udp_port: 4096,
        })]),
    };
    assert_eq!(Gtpv1Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_and_one_ext_header_marshal() {
    let encoded: [u8; 16] = [
        0x36, 0x02, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x40, 0x01, 0x10, 0x00,
        0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 8,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: Some(vec![ExtensionHeader::UDPPort(UDPPort {
            extension_header_type: UDP_PORT,
            length: UDP_PORT_LENGTH,
            udp_port: 4096,
        })]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_and_two_ext_header_unmarshal() {
    let encoded: [u8; 20] = [
        0x36, 0x02, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x40, 0x01, 0x10, 0x00,
        0xc0, 0x01, 0x10, 0x00, 0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 12,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: Some(vec![
            ExtensionHeader::UDPPort(UDPPort {
                extension_header_type: UDP_PORT,
                length: UDP_PORT_LENGTH,
                udp_port: 4096,
            }),
            ExtensionHeader::PDCPPDUNumber(PDCPPDUNumber {
                extension_header_type: PDCP_PDU_NUMBER,
                length: PDCP_PDU_NUMBER_LENGTH,
                pdcp_pdu_number: 4096,
            }),
        ]),
    };
    assert_eq!(Gtpv1Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_and_two_ext_header_marshal() {
    let encoded: [u8; 20] = [
        0x36, 0x02, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x40, 0x01, 0x10, 0x00,
        0xc0, 0x01, 0x10, 0x00, 0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 12,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: Some(vec![
            ExtensionHeader::UDPPort(UDPPort {
                extension_header_type: UDP_PORT,
                length: UDP_PORT_LENGTH,
                udp_port: 4096,
            }),
            ExtensionHeader::PDCPPDUNumber(PDCPPDUNumber {
                extension_header_type: PDCP_PDU_NUMBER,
                length: PDCP_PDU_NUMBER_LENGTH,
                pdcp_pdu_number: 4096,
            }),
        ]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_npdu_and_two_ext_header_unmarshal() {
    let encoded: [u8; 20] = [
        0x36, 0x02, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x40, 0x01, 0x10, 0x00,
        0xc0, 0x01, 0x10, 0x00, 0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 8,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: Some(vec![
            ExtensionHeader::UDPPort(UDPPort {
                extension_header_type: UDP_PORT,
                length: UDP_PORT_LENGTH,
                udp_port: 4096,
            }),
            ExtensionHeader::PDCPPDUNumber(PDCPPDUNumber {
                extension_header_type: PDCP_PDU_NUMBER,
                length: PDCP_PDU_NUMBER_LENGTH,
                pdcp_pdu_number: 4096,
            }),
        ]),
    };
    assert_eq!(Gtpv1Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv1_hdr_with_sqn_npdu_and_two_ext_header_marshal() {
    let encoded: [u8; 20] = [
        0x36, 0x02, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x4e, 0x00, 0x40, 0x01, 0x10, 0x00,
        0xc0, 0x01, 0x10, 0x00, 0x00,
    ];
    let decoded: Gtpv1Header = Gtpv1Header {
        msgtype: 0x02,
        length: 8,
        teid: 0x0,
        sequence_number: Some(0xf64e),
        npdu_number: None,
        extension_headers: Some(vec![
            ExtensionHeader::UDPPort(UDPPort {
                extension_header_type: UDP_PORT,
                length: UDP_PORT_LENGTH,
                udp_port: 4096,
            }),
            ExtensionHeader::PDCPPDUNumber(PDCPPDUNumber {
                extension_header_type: PDCP_PDU_NUMBER,
                length: PDCP_PDU_NUMBER_LENGTH,
                pdcp_pdu_number: 4096,
            }),
        ]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn infinite_loop_issue_2() {
    // This is a packet that caused an infinite loop in the unmarshal function
    // we add it to the header
    let header = Gtpv1Header {
        extension_headers: Some(vec![ExtensionHeader::UDPPort(UDPPort {
            length: 0,
            ..UDPPort::default()
        })]),
        ..Gtpv1Header::default()
    };
    let mut array: Vec<u8> = vec![];
    header.marshal(&mut array);
    assert_eq!(
        Gtpv1Header::unmarshal(&array),
        Err(GTPV1Error::ExtHeaderInvalidLength)
    );
}
