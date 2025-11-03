// Routeing Area Identity (RAI) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Routeing Area Identity (RAI) IE TL

pub const RAI: u8 = 3;
pub const RAI_LENGTH: usize = 6;

// Routeing Area Identity (RAI) IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rai {
    pub t: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
    pub lac: u16,
    pub rac: u8,
}

impl Default for Rai {
    fn default() -> Self {
        Rai {
            t: RAI,
            mcc: 0,
            mnc: 0,
            mnc_is_three_digits: false,
            lac: 0,
            rac: 0,
        }
    }
}

impl IEs for Rai {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.append(&mut mcc_mnc_encode(
            self.mcc,
            self.mnc,
            self.mnc_is_three_digits,
        ));
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() > RAI_LENGTH {
            let (mcc, mnc, mnc_is_three_digits) = mcc_mnc_decode(&buffer[1..=3]);
            let data: Rai = Rai {
                mcc,
                mnc,
                mnc_is_three_digits,
                lac: u16::from_be_bytes([buffer[4], buffer[5]]),
                rac: buffer[6],
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        RAI_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn rai_ie_marshal_test() {
    let rai_to_marshal = Rai {
        t: 3,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        lac: 999,
        rac: 67,
    };
    let rai_marshalled: [u8; 7] = [0x03, 0x99, 0xf9, 0x10, 0x03, 0xe7, 0x43];
    let mut buffer: Vec<u8> = vec![];
    rai_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, rai_marshalled);
}

#[test]
fn rai_ie_unmarshal_test() {
    let rai_unmarshalled = Rai {
        t: 3,
        mcc: 999,
        mnc: 1,
        mnc_is_three_digits: false,
        lac: 999,
        rac: 67,
    };
    let rai_to_unmarshal: [u8; 7] = [0x03, 0x99, 0xf9, 0x10, 0x03, 0xe7, 0x43];
    assert_eq!(Rai::unmarshal(&rai_to_unmarshal).unwrap(), rai_unmarshalled);
}
