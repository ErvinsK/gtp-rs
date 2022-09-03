// Serving PLMN Rate Control IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// Serving PLMN Rate Control IE TL

pub const SERV_PLMN_RATE_CTRL:u8 = 198;
pub const SERV_PLMN_RATE_CTRL_LENGTH:usize = 4;

// Serving PLMN Rate Control IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct ServingPlmnRateControl {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub rate_ctrl_ul:u16,
    pub rate_ctrl_dl:u16,
}

impl Default for ServingPlmnRateControl {
    fn default() -> Self {
        ServingPlmnRateControl { t: SERV_PLMN_RATE_CTRL, length:SERV_PLMN_RATE_CTRL_LENGTH as u16, ins:0, rate_ctrl_ul:0, rate_ctrl_dl:0}
    }
}

impl From<ServingPlmnRateControl> for InformationElement {
    fn from(i: ServingPlmnRateControl) -> Self {
        InformationElement::ServingPlmnRateControl(i)
    }
}

impl IEs for ServingPlmnRateControl {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.rate_ctrl_ul.to_be_bytes());
        buffer_ie.extend_from_slice(&self.rate_ctrl_dl.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=SERV_PLMN_RATE_CTRL_LENGTH + MIN_IE_SIZE {
            let mut data=ServingPlmnRateControl::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3] & 0x0f;
            data.rate_ctrl_ul = u16::from_be_bytes([buffer[4],buffer[5]]); 
            data.rate_ctrl_dl = u16::from_be_bytes([buffer[6],buffer[7]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SERV_PLMN_RATE_CTRL))
        }
    }

    fn len (&self) -> usize {
       (self.length as usize) + 4 
    }

}

#[test]
fn serving_plmn_rate_ctrl_ie_marshal_test () {
    let encoded:[u8;8]=[0xc6, 0x00, 0x04, 0x00, 0x00, 0x64, 0x01, 0xf4];
    let decoded = ServingPlmnRateControl { t:SERV_PLMN_RATE_CTRL, length: SERV_PLMN_RATE_CTRL_LENGTH as u16, ins:0, rate_ctrl_ul:100, rate_ctrl_dl:500 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn serving_plmn_rate_ctrl_ie_unmarshal_test () {
    let encoded:[u8;8]=[0xc6, 0x00, 0x04, 0x00, 0x00, 0x64, 0x01, 0xf4];
    let decoded = ServingPlmnRateControl { t:SERV_PLMN_RATE_CTRL, length: SERV_PLMN_RATE_CTRL_LENGTH as u16, ins:0, rate_ctrl_ul:100, rate_ctrl_dl:500 };
    assert_eq!(ServingPlmnRateControl::unmarshal(&encoded).unwrap(), decoded);
}