// RAB Context IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// RAB Context IE Type

pub const RABCTX:u8 = 124;
pub const RABCTX_LENGTH:usize = 9;

// RAB Context IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RabContext {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub nsapi:u8,
    pub dl_gtpu_sqn:u16,
    pub ul_gtpu_sqn:u16,
    pub dl_pdcp_sqn:u16,
    pub ul_pdcp_sqn:u16,    
}

impl Default for RabContext {
    fn default() -> Self {
        RabContext { t: RABCTX, length:RABCTX_LENGTH as u16, ins:0, nsapi:0, dl_gtpu_sqn:0, ul_gtpu_sqn:0, dl_pdcp_sqn:0, ul_pdcp_sqn:0}
    }
}

impl From<RabContext> for InformationElement {
    fn from(i: RabContext) -> Self {
        InformationElement::RabContext(i)
    }
}

impl IEs for RabContext {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.nsapi);
        buffer_ie.extend_from_slice(&self.dl_gtpu_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ul_gtpu_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.dl_pdcp_sqn.to_be_bytes());
        buffer_ie.extend_from_slice(&self.ul_pdcp_sqn.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=MIN_IE_SIZE+RABCTX_LENGTH {
            let mut data=RabContext::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.nsapi = buffer[4] & 0x0f;
            data.dl_gtpu_sqn = u16::from_be_bytes([buffer[5],buffer[6]]);
            data.ul_gtpu_sqn = u16::from_be_bytes([buffer[7],buffer[8]]);
            data.dl_pdcp_sqn = u16::from_be_bytes([buffer[9],buffer[10]]);
            data.ul_pdcp_sqn = u16::from_be_bytes([buffer[11],buffer[12]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(RABCTX))
        }
    }

    fn len (&self) -> usize {
       RABCTX_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn rab_context_ie_marshal_test () {
    let encoded:[u8;13]=[0x7c, 0x00, 0x09, 0x00, 0x05, 0xff, 0x00, 0x00, 0xff, 0xaa, 0x00, 0x00, 0xaa];
    let decoded = RabContext { t:RABCTX, length: RABCTX_LENGTH as u16, ins:0, nsapi:5, dl_gtpu_sqn:0xff00, ul_gtpu_sqn:0x00ff, dl_pdcp_sqn:0xaa00, ul_pdcp_sqn:0x00aa };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn rab_context_ie_unmarshal_test () {
    let encoded:[u8;13]=[0x7c, 0x00, 0x09, 0x00, 0x05, 0xff, 0x00, 0x00, 0xff, 0xaa, 0x00, 0x00, 0xaa];
    let decoded = RabContext { t:RABCTX, length: RABCTX_LENGTH as u16, ins:0, nsapi:5, dl_gtpu_sqn:0xff00, ul_gtpu_sqn:0x00ff, dl_pdcp_sqn:0xaa00, ul_pdcp_sqn:0x00aa };
    assert_eq!(RabContext::unmarshal(&encoded).unwrap(), decoded);
}
