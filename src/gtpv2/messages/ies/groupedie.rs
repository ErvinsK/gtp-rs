// Grouped IE - Container

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*, utils::*};

// Grouped IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct GroupedIe {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub elements: Vec<InformationElement>,
}

impl Default for GroupedIe {
    fn default() -> Self {
        GroupedIe {
            t: BEARER_CTX,
            length: 0,
            ins: 0,
            elements: vec![],
        }
    }
}

impl IEs for GroupedIe {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        InformationElement::encoder(self.elements.clone(), &mut buffer_ie);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = GroupedIe {
                t: buffer[0],
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3],
                elements: vec![],
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                match InformationElement::decoder(&buffer[4..(data.length + 4) as usize]) {
                    Ok(i) => data.elements = i,
                    Err(j) => return Err(j),
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(buffer[0]))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(buffer[0]))
        }
    }

    fn len(&self) -> usize {
        (self.length + 4) as usize
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
fn grouped_ie_marshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 48] = [
        0x5d, 0x00, 0x2c, 0x01, 0x49, /* ...].,.I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x84, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, /* ....L... */
        0x2d, 0x50, 0x00, 0x16, 0x00, 0x6c, 0x09, 0x00, /* -P...l.. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00,
    ];
    let decoded = GroupedIe {
        t: 93,
        length: 44,
        ins: 1,
        elements: vec![
            InformationElement::Ebi(Ebi {
                t: 73,
                length: 1,
                ins: 0,
                value: 5,
            }),
            InformationElement::Fteid(Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 4,
                teid: 114393676,
                ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
                ipv6: None,
            }),
            InformationElement::BearerQos(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 1,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
        ],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn grouped_ie_unmarshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 48] = [
        0x5d, 0x00, 0x2c, 0x01, 0x49, /* ...].,.I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x84, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, /* ....L... */
        0x2d, 0x50, 0x00, 0x16, 0x00, 0x6c, 0x09, 0x00, /* -P...l.. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00,
    ];
    let decoded = GroupedIe {
        t: 93,
        length: 44,
        ins: 1,
        elements: vec![
            InformationElement::Ebi(Ebi {
                t: 73,
                length: 1,
                ins: 0,
                value: 5,
            }),
            InformationElement::Fteid(Fteid {
                t: 87,
                length: 9,
                ins: 2,
                interface: 4,
                teid: 114393676,
                ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
                ipv6: None,
            }),
            InformationElement::BearerQos(BearerQos {
                t: 80,
                length: 22,
                ins: 0,
                pre_emption_vulnerability: 0,
                priority_level: 11,
                pre_emption_capability: 1,
                qci: 9,
                maxbr_ul: 0,
                maxbr_dl: 0,
                gbr_ul: 0,
                gbr_dl: 0,
            }),
        ],
    };
    assert_eq!(GroupedIe::unmarshal(&encoded).unwrap(), decoded);
}
