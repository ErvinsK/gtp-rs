// GUTI IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) and 3GPP TS 25.413

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// GUTI IE TL

pub const GUTI:u8 = 117;
pub const GUTI_LENGTH:usize = 10;

// GUTI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guti {
    pub t: u8,
    pub length:u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub mmegi: u16,
    pub mmec: u8,
    pub mtmsi: u32,
}

impl Default for Guti {
    fn default() -> Self {
        Guti { t: GUTI, length: GUTI_LENGTH as u16, ins:0, mcc: 0, mnc: 0, mmegi:0, mmec:0, mtmsi:0 }
    }
}

impl From<Guti> for InformationElement {
    fn from(i: Guti) -> Self {
        InformationElement::Guti(i)
    }
}

impl IEs for Guti {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer_ie.extend_from_slice(&self.mmegi.to_be_bytes());
        buffer_ie.push(self.mmec);
        buffer_ie.extend_from_slice(&self.mtmsi.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+GUTI_LENGTH {
            let mut data=Guti{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..7]);
            data.mmegi = u16::from_be_bytes([buffer[7],buffer[8]]);
            data.mmec = buffer[9];
            data.mtmsi = u32::from_be_bytes([buffer[10],buffer[11],buffer[12],buffer[13]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEInvalidLength(GUTI))
        }
    }

    fn len (&self) -> usize {
        GUTI_LENGTH+MIN_IE_SIZE
    }

    fn is_empty (&self) -> bool {
        self.length == 0
    }
}

#[test]
fn guti_ie_marshal_test() {
    let decoded = Guti { t:GUTI, length: GUTI_LENGTH as u16, ins:0,  mcc:999, mnc:1, mmegi:300, mmec: 10, mtmsi: 0xffffffff};
    let encoded:[u8;14] = [0x75, 0x00, 0x0a, 0x00, 0x99, 0xf9, 0x10, 0x01, 0x2c, 0x0a, 0xff, 0xff, 0xff, 0xff];
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn global_cn_id_ie_unmarshal_test() {
    let decoded = Guti { t:GUTI, length: GUTI_LENGTH as u16, ins:0,  mcc:999, mnc:1, mmegi:300, mmec: 10, mtmsi: 0xffffffff};
    let encoded:[u8;14] = [0x75, 0x00, 0x0a, 0x00, 0x99, 0xf9, 0x10, 0x01, 0x2c, 0x0a, 0xff, 0xff, 0xff, 0xff];
    assert_eq!(Guti::unmarshal(&encoded).unwrap(), decoded);
}