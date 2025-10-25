// Bearer Flags IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Bearer Flags IE Type

pub const BEARERFLAGS: u8 = 97;
pub const BEARERFLAGS_LENGTH: usize = 1;

// Bearer Flags IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerFlags {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub asi: bool, // ASI (Activity Status Indicator): When set to 1, this flag indicates that the bearer context is preserved in the CN without corresponding Radio Access Bearer established.
    pub vind: bool, // Vind (vSRVCC indicator): This flag is used to indicate that this bearer is an IMS video bearer and is candidate for PS-to-CS vSRVCC handover.
    pub vb: bool, // VB (Voice Bearer): This flag is used to indicate a voice bearer when doing PS-to-CS (v)SRVCC handover.
    pub ppc: bool, // PPC (Prohibit Payload Compression): This flag is used to determine whether an SGSN should attempt to compress the payload of user data when the users asks for it to be compressed (PPC = 0), or not (PPC = 1).
}

impl Default for BearerFlags {
    fn default() -> Self {
        BearerFlags {
            t: BEARERFLAGS,
            length: BEARERFLAGS_LENGTH as u16,
            ins: 0,
            asi: false,
            vind: false,
            vb: false,
            ppc: false,
        }
    }
}

impl From<BearerFlags> for InformationElement {
    fn from(i: BearerFlags) -> Self {
        InformationElement::BearerFlags(i)
    }
}

impl IEs for BearerFlags {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(BEARERFLAGS);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let mut flags: u8 = 0x0;
        if self.asi {
            flags = 0x08;
        }
        if self.vind {
            flags |= 0x04;
        }
        if self.vb {
            flags |= 0x02;
        }
        if self.ppc {
            flags |= 0x01;
        }
        buffer_ie.push(flags);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + BEARERFLAGS_LENGTH {
            let mut data = BearerFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..BearerFlags::default()
            };
            let flags = buffer[4];
            if flags & 0x08 == 0x08 {
                data.asi = true;
            }
            if flags & 0x04 == 0x04 {
                data.vind = true;
            }
            if flags & 0x02 == 0x02 {
                data.vb = true;
            }
            if flags & 0x01 == 0x01 {
                data.ppc = true;
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(BEARERFLAGS))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn bearer_flags_ie_marshal_test() {
    let encoded: [u8; 5] = [0x61, 0x00, 0x01, 0x00, 0x0d];
    let decoded = BearerFlags {
        t: BEARERFLAGS,
        length: BEARERFLAGS_LENGTH as u16,
        ins: 0,
        asi: true,
        vind: true,
        vb: false,
        ppc: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn bearer_flags_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x61, 0x00, 0x01, 0x00, 0x0d];
    let decoded = BearerFlags {
        t: BEARERFLAGS,
        length: BEARERFLAGS_LENGTH as u16,
        ins: 0,
        asi: true,
        vind: true,
        vb: false,
        ppc: true,
    };
    assert_eq!(BearerFlags::unmarshal(&encoded).unwrap(), decoded);
}
