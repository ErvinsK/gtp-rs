// Routeing Area Identity (RAI) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::gtpc::ies::commons::{*};
use crate::gtpv1::utils::{*};

// Routeing Area Identity (RAI) IE TL

pub const RAI:u8 = 3;
pub const RAI_LENGTH:usize = 6;

// Routeing Area Identity (RAI) IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Rai {
    pub t: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub rac: u8,

}

impl Default for Rai {
    fn default() -> Self {
        Rai { t: 3, mcc: 0, mnc: 0, lac: 0, rac: 0 }
    }
}

impl IEs for Rai {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mcc_digits:Vec<u8> = to_digits(self.mcc);
        let mnc_digits:Vec<u8> = to_digits(self.mnc);
        buffer.push(self.t);
        buffer.push(mcc_digits[1]<<4 | mcc_digits[0]);
        if mnc_digits.len()==2 {
            buffer.push(0b1111<<4 | mcc_digits[2]);
        } else {
            buffer.push(mnc_digits[2]<<4 | mcc_digits[2]);
        }
        buffer.push(mnc_digits[1]<<4 | mnc_digits[0]);
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=RAI_LENGTH+1 {
            let mut data:Rai=Rai::default();
            let mut mcc_digits:Vec<u8>=vec!();
            let mut mnc_digits:Vec<u8>=vec!();
            mcc_digits.push(buffer[1] & 0b1111);
            mcc_digits.push(buffer[1] >> 4);
            mcc_digits.push(buffer[2] & 0b00001111);
            mnc_digits.push(buffer[3] & 0b1111);
            mnc_digits.push(buffer[3] >> 4);
            if buffer[2]>>4 != 0b1111 {
                mnc_digits.push(buffer[2]>>4);
            }
            if let Ok(i) = mcc_digits.iter().flat_map( |c| char::from_digit(*c as u32, 10)).collect::<String>().parse::<u16>() {
                data.mcc=i;
            }
            if let Ok(i) = mnc_digits.iter().flat_map( |c| char::from_digit(*c as u32, 10)).collect::<String>().parse::<u16>() {
                data.mnc=i;
            }
            data.lac=u16::from_be_bytes([buffer[4],buffer[5]]);
            data.rac=buffer[6];
            Some (data)
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        RAI_LENGTH+1
    }

}

#[test]
fn rai_ie_marshal_test() {
    let rai_to_marshal = Rai { t:3, mcc:999, mnc:111, lac:999, rac: 67};
    let rai_marshalled:[u8;7] = [0x03, 0x99, 0x19, 0x11, 0x03, 0xe7, 0x43];
    let mut buffer:Vec<u8>=vec!();
    rai_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,rai_marshalled);
}

#[test]
fn rai_ie_unmarshal_test() {
    let rai_unmarshalled = Rai { t:3, mcc:999, mnc:111, lac:999, rac: 67};
    let rai_to_unmarshal:[u8;7] = [0x03, 0x99, 0x19, 0x11, 0x03, 0xe7, 0x43];
    assert_eq!(Rai::unmarshal(&rai_to_unmarshal).unwrap(), rai_unmarshalled);
}