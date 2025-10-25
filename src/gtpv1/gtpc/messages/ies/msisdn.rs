// MSISDN IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// MSISDN IE TLV

pub const MSISDN: u8 = 134;

// MSISDN IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msisdn {
    pub t: u8,
    pub length: u16,
    pub extension: u8,
    pub number_nature: u8,
    pub number_plan: u8,
    pub msisdn: String,
}

impl Default for Msisdn {
    fn default() -> Msisdn {
        Msisdn {
            t: MSISDN,
            length: 2,
            extension: 1,
            number_nature: 1,
            number_plan: E164,
            msisdn: "0".to_string(),
        }
    }
}

impl IEs for Msisdn {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push((self.extension << 7) | (self.number_nature << 4) | self.number_plan);
        buffer_ie.extend(tbcd_encode(&self.msisdn));
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Msisdn, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 3 {
            let mut data = Msisdn {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            if check_tlv_ie_buffer(data.length, buffer) {
                data.extension = (buffer[3] & 0x80) >> 7;
                data.number_nature = (buffer[3] & 0x70) >> 4;
                data.number_plan = buffer[3] & 0x0f;
                if data.number_plan == E164 || data.number_plan == E212 {
                    match buffer[4..=11].try_into() {
                        Ok(i) => data.msisdn = tbcd_decode(i),
                        Err(_) => return Err(GTPV1Error::IEIncorrect),
                    }
                    Ok(data)
                } else {
                    Err(GTPV1Error::IEIncorrect)
                }
            } else {
                Err(GTPV1Error::IEInvalidLength)
            }
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn msisdn_ie_marshal_test() {
    let encoded_ie: [u8; 12] = [
        0x86, 0x00, 0x09, 0x91, 0x99, 0x88, 0x58, 0x01, 0x51, 0x88, 0x16, 0xf5,
    ];
    let test_struct = Msisdn {
        t: MSISDN,
        length: 9,
        extension: 1,
        number_nature: 1,
        number_plan: E164,
        msisdn: "998885101588615".to_string(),
    };
    let i = Msisdn::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn msisdn_ie_unmarshal_test() {
    let encoded_ie: [u8; 12] = [
        0x86, 0x00, 0x09, 0x91, 0x99, 0x88, 0x58, 0x01, 0x51, 0x88, 0x16, 0xf5,
    ];
    let test_struct = Msisdn {
        t: MSISDN,
        length: 9,
        extension: 1,
        number_nature: 1,
        number_plan: E164,
        msisdn: "998885101588615".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
