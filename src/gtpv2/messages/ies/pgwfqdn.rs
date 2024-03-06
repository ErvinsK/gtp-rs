// PGW Fully Qualified Domain Name (FQDN) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// PGW FQDN IE Type

pub const PGW_FQDN: u8 = 215;

// PGW FQDN IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgwFqdn {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub name: String,
}

impl Default for PgwFqdn {
    fn default() -> Self {
        PgwFqdn {
            t: PGW_FQDN,
            length: 0,
            ins: 0,
            name: "".to_string(),
        }
    }
}

impl From<PgwFqdn> for InformationElement {
    fn from(i: PgwFqdn) -> Self {
        InformationElement::PgwFqdn(i)
    }
}

impl IEs for PgwFqdn {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PGW_FQDN);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
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
            let mut data = PgwFqdn {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..PgwFqdn::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                let mut donor: Vec<u8> =
                    buffer[MIN_IE_SIZE..(MIN_IE_SIZE + data.length as usize)].to_vec();
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
                Err(GTPV2Error::IEInvalidLength(PGW_FQDN))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(PGW_FQDN))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn pgw_fqdn_ie_marshal_test() {
    let encoded: [u8; 57] = [
        0xd7, 0x00, 0x35, 0x00, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65,
        0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e,
        0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70,
        0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00,
    ];
    let decoded = PgwFqdn {
        length: 53,
        name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
        ..PgwFqdn::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn pgw_fqdn_ie_unmarshal_test() {
    let encoded: [u8; 57] = [
        0xd7, 0x00, 0x35, 0x00, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65,
        0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e,
        0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70,
        0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00,
    ];
    let decoded = PgwFqdn {
        length: 53,
        name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
        ..PgwFqdn::default()
    };
    assert_eq!(PgwFqdn::unmarshal(&encoded).unwrap(), decoded);
}
