// Header Compression Configuration IE - according to 3GPP TS 29.274 V17.10.0 (2023-12) and 3GPP TS 36.323 V15.0.0 (2018-08)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Header Compression Configuration IE TL

pub const HDRCOMPRCONFIG: u8 = 196;
pub const HDRCOMPRCONFIG_LENGTH: usize = 4;

// Header Compression Configuration IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderCompressionConfiguration {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub rohc_profiles: Vec<u16>, // ROHC Profiles (3GPP TS 36.323)
    pub max_cid: u16,
}

impl Default for HeaderCompressionConfiguration {
    fn default() -> Self {
        HeaderCompressionConfiguration {
            t: HDRCOMPRCONFIG,
            length: HDRCOMPRCONFIG_LENGTH as u16,
            ins: 0,
            rohc_profiles: vec![0],
            max_cid: 0,
        }
    }
}

impl From<HeaderCompressionConfiguration> for InformationElement {
    fn from(i: HeaderCompressionConfiguration) -> Self {
        InformationElement::HeaderCompressionConfiguration(i)
    }
}

impl IEs for HeaderCompressionConfiguration {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(HDRCOMPRCONFIG);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let mut i: u8 = 0;
        for profile in &self.rohc_profiles {
            match profile {
                0x0002 => {
                    if (i & 0b00000001) == 0b0 {
                        i += 0b00000001;
                    }
                }
                0x0003 => {
                    if (i & 0b00000010) == 0b0 {
                        i += 0b00000010;
                    }
                }
                0x0004 => {
                    if (i & 0b00000100) == 0b0 {
                        i += 0b00000100;
                    }
                }
                0x0006 => {
                    if (i & 0b00001000) == 0b0 {
                        i += 0b00001000;
                    }
                }
                0x0102 => {
                    if (i & 0b00010000) == 0b0 {
                        i += 0b00010000;
                    }
                }
                0x0103 => {
                    if (i & 0b00100000) == 0b0 {
                        i += 0b00100000;
                    }
                }
                0x0104 => {
                    if (i & 0b01000000) == 0b0 {
                        i += 0b01000000;
                    }
                }
                _ => (),
            }
        }
        buffer_ie.push(i);
        buffer_ie.push(0x00);
        buffer_ie.extend_from_slice(&self.max_cid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= (HDRCOMPRCONFIG_LENGTH + MIN_IE_SIZE) {
            let mut data = HeaderCompressionConfiguration {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..HeaderCompressionConfiguration::default()
            };
            (0u8..=7)
                .map(|i| buffer[4] & (1 << i))
                .for_each(|bit| match bit {
                    0b01 => data.rohc_profiles.push(0x0002),
                    0b10 => data.rohc_profiles.push(0x0003),
                    0b100 => data.rohc_profiles.push(0x0004),
                    0b1000 => data.rohc_profiles.push(0x0006),
                    0b10000 => data.rohc_profiles.push(0x0102),
                    0b100000 => data.rohc_profiles.push(0x0103),
                    0b1000000 => data.rohc_profiles.push(0x0104),
                    _ => (),
                });
            data.max_cid = u16::from_be_bytes([buffer[6], buffer[7]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(HDRCOMPRCONFIG))
        }
    }

    fn len(&self) -> usize {
        HDRCOMPRCONFIG_LENGTH + MIN_IE_SIZE
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
fn hdr_compr_config_ie_marshal_test() {
    let encoded: [u8; 8] = [0xc4, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00];
    let decoded = HeaderCompressionConfiguration {
        t: HDRCOMPRCONFIG,
        length: HDRCOMPRCONFIG_LENGTH as u16,
        ins: 0,
        rohc_profiles: vec![0x0002],
        max_cid: 0,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn hdr_compr_config_ie_unmarshal_test() {
    let encoded: [u8; 8] = [0xc4, 0x00, 0x04, 0x00, 0x7f, 0x00, 0x00, 0xff];
    let decoded = HeaderCompressionConfiguration {
        t: HDRCOMPRCONFIG,
        length: HDRCOMPRCONFIG_LENGTH as u16,
        ins: 0,
        rohc_profiles: vec![
            0x0000, 0x0002, 0x0003, 0x0004, 0x0006, 0x0102, 0x0103, 0x0104,
        ],
        max_cid: 0xff,
    };
    assert_eq!(
        HeaderCompressionConfiguration::unmarshal(&encoded).unwrap(),
        decoded
    );
}
