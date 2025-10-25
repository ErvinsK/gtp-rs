// IMSI IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// IMSI IE TV

pub const IMSI: u8 = 2;
pub const IMSI_LENGTH: usize = 8;

// IMSI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imsi {
    pub t: u8,
    pub imsi: String,
}

impl Default for Imsi {
    fn default() -> Imsi {
        Imsi {
            t: IMSI,
            imsi: "0".to_string(),
        }
    }
}

impl IEs for Imsi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend(tbcd_encode(&self.imsi));
    }

    fn unmarshal(buffer: &[u8]) -> Result<Imsi, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() > IMSI_LENGTH {
            let mut data = Imsi::default();
            match buffer[1..=8].try_into() {
                Ok(i) => data.imsi = tbcd_decode(i),
                Err(_) => return Err(GTPV1Error::IEIncorrect),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        IMSI_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        self.imsi == *"0"
    }
}

#[test]
fn imsi_ie_marshal_test() {
    let encoded_ie: [u8; 9] = [0x02, 0x09, 0x41, 0x50, 0x01, 0x31, 0x72, 0x94, 0xf6];
    let test_struct = Imsi {
        t: 0x02,
        imsi: "901405101327496".to_string(),
    };
    let i = Imsi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn imsi_ie_unmarshal_test() {
    let encoded_ie: [u8; 9] = [0x02, 0x09, 0x41, 0x50, 0x01, 0x31, 0x72, 0x94, 0xf6];
    let test_struct = Imsi {
        t: 0x02,
        imsi: "901405101327496".to_string(),
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
