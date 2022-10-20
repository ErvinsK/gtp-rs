// Global CN-Id IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) and 3GPP TS 25.413

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// Global CN-Id IE TL

pub const GLOBAL_CN_ID:u8 = 89;
pub const GLOBAL_CN_ID_LENGTH:usize = 5;

// Global CN-Id IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalCnId {
    pub t: u8,
    pub length:u16,
    pub ins: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub cnid: u16,
}

impl Default for GlobalCnId {
    fn default() -> Self {
        GlobalCnId { t: GLOBAL_CN_ID, length: GLOBAL_CN_ID_LENGTH as u16, ins:0, mcc: 0, mnc: 0, cnid:0 }
    }
}

impl From<GlobalCnId> for InformationElement {
    fn from(i: GlobalCnId) -> Self {
        InformationElement::GlobalCnId(i)
    }
}

impl IEs for GlobalCnId {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.append(&mut mcc_mnc_encode(self.mcc, self.mnc));
        buffer_ie.extend_from_slice(&self.cnid.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+GLOBAL_CN_ID_LENGTH {
            let mut data=GlobalCnId::default();
            data.length = u16::from_be_bytes([buffer[1],buffer[2]]);
            data.ins = buffer[3];
            (data.mcc, data.mnc) = mcc_mnc_decode(&buffer[4..7]);
            data.cnid = u16::from_be_bytes([buffer[7],buffer[8]]);
            Ok (data)
        } else {
            Err(GTPV2Error::IEInvalidLength(GLOBAL_CN_ID))
        }
    }

    fn len (&self) -> usize {
        GLOBAL_CN_ID_LENGTH+MIN_IE_SIZE
    }

}

#[test]
fn global_cn_id_ie_marshal_test() {
    let decoded = GlobalCnId { t:GLOBAL_CN_ID, length: GLOBAL_CN_ID_LENGTH as u16, ins:0,  mcc:999, mnc:1, cnid:4000};
    let encoded:[u8;9] = [0x59, 0x00, 0x05, 0x00, 0x99, 0xf9, 0x10, 0x0f, 0xa0];
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn global_cn_id_ie_unmarshal_test() {
    let decoded = GlobalCnId { t:GLOBAL_CN_ID, length: GLOBAL_CN_ID_LENGTH as u16, ins:0,  mcc:999, mnc:1, cnid:4000};
    let encoded:[u8;9] = [0x59, 0x00, 0x05, 0x00, 0x99, 0xf9, 0x10, 0x0f, 0xa0];
    assert_eq!(GlobalCnId::unmarshal(&encoded).unwrap(), decoded);
}