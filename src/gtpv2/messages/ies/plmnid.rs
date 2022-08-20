// PLMN ID IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::commons::*};

// PLMN ID IE TL

pub const PLMNID:u8 = 120;
pub const PLMNID_LENGTH:usize = 3;

// PLMN ID IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct PlmnId {
    pub t: u8,
    pub length:u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
}

impl Default for PlmnId {
    fn default() -> Self {
        PlmnId { t: PLMNID, length: PLMNID_LENGTH as u16, ins:0, mcc: 0, mnc: 0 }
    }
}

impl IEs for PlmnId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+PLMNID_LENGTH {
            let mut data=PlmnId::default();
            data.length = u16::from_be_bytes([buffer[1],buffer[2]]);
            data.ins = buffer[3];
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..7]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEInvalidLength)
        }
    }

    fn len (&self) -> usize {
        PLMNID_LENGTH+MIN_IE_SIZE
    }

}

#[test]
fn plmnid_ie_marshal_test() {
    let decoded = PlmnId { t:PLMNID, length: PLMNID_LENGTH as u16, ins:0,  mcc:999, mnc:1};
    let encoded:[u8;7] = [0x78, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10];
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn serving_nw_ie_unmarshal_test() {
    let decoded = PlmnId { t:PLMNID, length: PLMNID_LENGTH as u16, ins:0,  mcc:999, mnc:1};
    let encoded:[u8;7] = [0x78, 0x00, 0x03, 0x00, 0x99, 0xf9, 0x10];
    assert_eq!(PlmnId::unmarshal(&encoded).unwrap(), decoded);
}