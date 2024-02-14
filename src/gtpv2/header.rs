use crate::gtpv2::errors::*;

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

// Definition of GTPv2 Header

pub const MANDATORY_HDR_LENGTH: usize = 4;
pub const MIN_HEADER_LENGTH: usize = 8;
pub const MAX_HEADER_LENGTH: usize = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gtpv2Header {
    pub msgtype: u8,
    pub piggyback: bool,
    pub message_prio: Option<u8>,
    pub length: u16,
    pub teid: Option<u32>,
    pub sqn: u32,
}

// Implementation of GTPv2 Header

impl Default for Gtpv2Header {
    fn default() -> Gtpv2Header {
        Gtpv2Header {
            msgtype: 0,
            piggyback: false,
            message_prio: None,
            length: MIN_HEADER_LENGTH as u16,
            teid: None,
            sqn: 0,
        }
    }
}

impl Gtpv2Header {
    pub fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.construct_flags());
        buffer.push(self.msgtype);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        if let Some(i) = self.teid {
            buffer.extend_from_slice(&i.to_be_bytes())
        };
        buffer.extend_from_slice(&self.sqn.to_be_bytes()[1..]);
        match self.message_prio {
            Some(i) => buffer.push(i << 4),
            None => buffer.push(0x00),
        }
    }

    pub fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_HEADER_LENGTH {
            let mut data = Gtpv2Header::default();
            if (buffer[0] >> 5) != 2 {
                return Err(GTPV2Error::HeaderVersionNotSupported);
            }
            match (buffer[0] >> 3) & 0x01 {
                0 => {
                    match (buffer[0] >> 4) & 0x01 {
                        0 => data.piggyback = false,
                        _ => data.piggyback = true,
                    }
                    data.msgtype = buffer[1];
                    data.length = match u16::from_be_bytes([buffer[2], buffer[3]]) {
                        0 => return Err(GTPV2Error::HeaderInvalidLength),
                        _ => u16::from_be_bytes([buffer[2], buffer[3]]),
                    };
                    if data.length < (MIN_HEADER_LENGTH - MANDATORY_HDR_LENGTH) as u16 {
                        return Err(GTPV2Error::MessageInvalidLength(0));
                    }
                    data.sqn = u32::from_be_bytes([0x00, buffer[4], buffer[5], buffer[6]]);
                }
                1 => {
                    if buffer.len() >= MAX_HEADER_LENGTH {
                        match (buffer[0] >> 4) & 0x01 {
                            0 => data.piggyback = false,
                            _ => data.piggyback = true,
                        }
                        data.msgtype = buffer[1];
                        data.length = u16::from_be_bytes([buffer[2], buffer[3]]);
                        if data.length < (MAX_HEADER_LENGTH - 4) as u16 {
                            return Err(GTPV2Error::MessageInvalidLength(0));
                        }
                        data.teid = Some(u32::from_be_bytes([
                            buffer[4], buffer[5], buffer[6], buffer[7],
                        ]));
                        data.sqn = u32::from_be_bytes([0x00, buffer[8], buffer[9], buffer[10]]);
                        match (buffer[0] >> 2) & 0x01 {
                            0 => data.message_prio = None,
                            _ => data.message_prio = Some(buffer[11] >> 4),
                        }
                    } else {
                        return Err(GTPV2Error::MessageInvalidLength(0));
                    }
                }
                _ => return Err(GTPV2Error::MessageInvalidMessageFormat),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::MessageInvalidLength(0))
        }
    }

    // Struct helper functions

    fn construct_flags(&self) -> u8 {
        let mut flags: u8 = 0;
        if self.message_prio.is_some() {
            flags = 0x04;
        }
        if self.teid.is_some() {
            flags |= 0x08;
        }
        if self.piggyback {
            flags |= 0x10;
        }
        flags | 0x40
    }
}

#[test]
fn test_gtpv2_hdr_t0_unmarshal() {
    let encoded: [u8; 8] = [0x40, 0x01, 0x00, 0x08, 0x6d, 0x3d, 0x7c, 0x00];
    let decoded = Gtpv2Header {
        msgtype: 0x01,
        length: 8,
        teid: None,
        sqn: 0x6d3d7c,
        piggyback: false,
        message_prio: None,
    };
    assert_eq!(Gtpv2Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv2_hdr_t0_version_incorrect_unmarshal() {
    let encoded: [u8; 8] = [0x20, 0x01, 0x00, 0x06, 0x6d, 0x3d, 0x7c, 0x00];
    assert_eq!(
        Gtpv2Header::unmarshal(&encoded),
        Err(GTPV2Error::HeaderVersionNotSupported)
    );
}

#[test]
fn test_gtpv2_hdr_t0_invalid_length_unmarshal() {
    let encoded: [u8; 8] = [0x40, 0x01, 0x00, 0x02, 0x6d, 0x3d, 0x7c, 0x00];
    assert_eq!(
        Gtpv2Header::unmarshal(&encoded),
        Err(GTPV2Error::MessageInvalidLength(0))
    );
}

#[test]
fn test_gtpv2_hdr_t0_marshal() {
    let encoded: [u8; 8] = [0x40, 0x01, 0x00, 0x08, 0x6d, 0x3d, 0x7c, 0x00];
    let decoded = Gtpv2Header {
        msgtype: 0x01,
        length: 8,
        teid: None,
        sqn: 0x6d3d7c,
        piggyback: false,
        message_prio: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gtpv2_hdr_t1_unmarshal() {
    let encoded: [u8; 12] = [
        0x48, 0x34, 0x00, 0x0c, 0x41, 0x76, 0xf6, 0x1e, 0x3c, 0xea, 0x57, 0x00,
    ];
    let decoded = Gtpv2Header {
        msgtype: 0x34,
        length: 0x0c,
        teid: Some(0x4176f61e),
        sqn: 0x3cea57,
        piggyback: false,
        message_prio: None,
    };
    assert_eq!(Gtpv2Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv2_hdr_t1_invalid_length_unmarshal() {
    let encoded: [u8; 12] = [
        0x48, 0x34, 0x00, 0x06, 0x41, 0x76, 0xf6, 0x1e, 0x3c, 0xea, 0x57, 0x00,
    ];
    assert_eq!(
        Gtpv2Header::unmarshal(&encoded),
        Err(GTPV2Error::MessageInvalidLength(0))
    );
}

#[test]
fn test_gtpv2_hdr_t1_marshal() {
    let encoded: [u8; 12] = [
        0x48, 0x34, 0x00, 0x0c, 0x41, 0x76, 0xf6, 0x1e, 0x3c, 0xea, 0x57, 0x00,
    ];
    let decoded = Gtpv2Header {
        msgtype: 0x34,
        length: 0x0c,
        teid: Some(0x4176f61e),
        sqn: 0x3cea57,
        piggyback: false,
        message_prio: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn test_gtpv2_hdr_t1_with_msg_prio_unmarshal() {
    let encoded: [u8; 12] = [
        0x4c, 0x34, 0x00, 0x0c, 0x41, 0x76, 0xf6, 0x1e, 0x3c, 0xea, 0x57, 0xf0,
    ];
    let decoded = Gtpv2Header {
        msgtype: 0x34,
        length: 0x0c,
        teid: Some(0x4176f61e),
        sqn: 0x3cea57,
        piggyback: false,
        message_prio: Some(0x0f),
    };
    assert_eq!(Gtpv2Header::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn test_gtpv2_hdr_t1_with_msg_prio_marshal() {
    let encoded: [u8; 12] = [
        0x4c, 0x34, 0x00, 0x0c, 0x41, 0x76, 0xf6, 0x1e, 0x3c, 0xea, 0x57, 0xf0,
    ];
    let decoded = Gtpv2Header {
        msgtype: 0x34,
        length: 0x0c,
        teid: Some(0x4176f61e),
        sqn: 0x3cea57,
        piggyback: false,
        message_prio: Some(0x0f),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
