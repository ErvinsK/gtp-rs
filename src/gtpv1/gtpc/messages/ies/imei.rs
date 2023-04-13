// IMEI(SV) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// IMEI(SV) IE TV

pub const IMEI: u8 = 154;
pub const IMEI_LENGTH: u16 = 8;

// IMEI(SV) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imei {
    pub t: u8,
    pub length: u16,
    pub imei: String,
}

impl Default for Imei {
    fn default() -> Imei {
        Imei {
            t: IMEI,
            length: IMEI_LENGTH,
            imei: "0".to_string(),
        }
    }
}

impl IEs for Imei {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.extend(tbcd_encode(&self.imei));
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Imei, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= (IMEI_LENGTH + 3) as usize {
            let mut data = Imei::default();
            match buffer[3..=10].try_into() {
                Ok(i) => data.imei = tbcd_decode(i),
                Err(_) => return Err(GTPV1Error::IEIncorrect),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (IMEI_LENGTH + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn imei_ie_unmarshal_test() {
    let encoded_ie: [u8; 11] = [
        0x9a, 0x00, 0x08, 0x53, 0x77, 0x69, 0x01, 0x16, 0x73, 0x06, 0xf0,
    ];
    let test_struct = Imei {
        t: IMEI,
        length: IMEI_LENGTH,
        imei: "357796106137600".to_string(),
    };
    let i = Imei::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn imei_ie_marshal_test() {
    let encoded_ie: [u8; 11] = [
        0x9a, 0x00, 0x08, 0x53, 0x77, 0x69, 0x01, 0x16, 0x73, 0x06, 0xf0,
    ];
    let test_struct = Imei {
        t: IMEI,
        length: IMEI_LENGTH,
        imei: "357796106137600".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
