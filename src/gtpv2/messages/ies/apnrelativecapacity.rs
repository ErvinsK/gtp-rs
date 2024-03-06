// APN and Relative Capacity IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// APN and Relative Capacity IE Type

pub const APN_REL_CAP: u8 = 184;

// APN and Relative Capacity IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApnRelativeCapacity {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub relative_cap: u8,
    pub name: String,
}

impl Default for ApnRelativeCapacity {
    fn default() -> Self {
        ApnRelativeCapacity {
            t: APN_REL_CAP,
            length: 0,
            ins: 0,
            relative_cap: 0,
            name: "".to_string(),
        }
    }
}

impl From<ApnRelativeCapacity> for InformationElement {
    fn from(i: ApnRelativeCapacity) -> Self {
        InformationElement::ApnRelativeCapacity(i)
    }
}

impl IEs for ApnRelativeCapacity {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(APN_REL_CAP);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.relative_cap);
        let n: Vec<_> = self.name.split('.').collect();
        let mut z: Vec<u8> = vec![];
        for i in n.iter() {
            z.push(i.len() as u8);
            z.extend_from_slice(i.as_bytes());
        }
        buffer_ie.append(&mut z);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = ApnRelativeCapacity {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..ApnRelativeCapacity::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match buffer[4] {
                    i if i <= 100 => data.relative_cap = buffer[4],
                    _ => data.relative_cap = 0,
                }
                let mut donor: Vec<u8> = buffer[5..MIN_IE_SIZE + data.length as usize].to_vec();
                let mut k: Vec<Vec<char>> = vec![];
                loop {
                    if !donor.is_empty() {
                        let index: Vec<_> = donor.drain(..1).collect();
                        let mut part: Vec<_> = donor
                            .drain(..index[0] as usize)
                            .map(|x| x as char)
                            .collect();
                        part.push('.');
                        k.push(part);
                    } else {
                        break;
                    }
                }
                let mut p: Vec<char> = k.into_iter().flatten().collect();
                let _ = p.pop();
                data.name = p.into_iter().collect();
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(APN_REL_CAP))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(APN_REL_CAP))
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
fn apn_rel_cap_ie_marshal_test() {
    let encoded: [u8; 18] = [
        0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03,
        0x63, 0x6f, 0x6d,
    ];
    let decoded = ApnRelativeCapacity {
        t: APN_REL_CAP,
        length: 14,
        ins: 0,
        relative_cap: 100,
        name: "test.net.com".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn apn_rel_cap_ie_unmarshal_test() {
    let encoded: [u8; 18] = [
        0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03,
        0x63, 0x6f, 0x6d,
    ];
    let decoded = ApnRelativeCapacity {
        t: APN_REL_CAP,
        length: 14,
        ins: 0,
        relative_cap: 100,
        name: "test.net.com".to_string(),
    };
    assert_eq!(ApnRelativeCapacity::unmarshal(&encoded).unwrap(), decoded);
}
