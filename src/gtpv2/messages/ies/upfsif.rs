// UP Function Selection Indication Flags (UPFSIF) IE - according to 3GPP TS 29.274 V15.5.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// UPFSIF IE TV

pub const UPFSIF:u8 = 202;
pub const UPFSIF_LENGTH:usize = 1;

// UPFSIF IE implementation 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpFunctionSelectionIndicationFlags {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub dcnr:bool,          // DCNR - Dual Connectivity with NR
}

impl Default for UpFunctionSelectionIndicationFlags {
    fn default() -> UpFunctionSelectionIndicationFlags {
        UpFunctionSelectionIndicationFlags { t: UPFSIF, length:UPFSIF_LENGTH as u16, ins:0, dcnr:false }        
    }
}

impl From<UpFunctionSelectionIndicationFlags> for InformationElement {
    fn from(i: UpFunctionSelectionIndicationFlags) -> Self {
        InformationElement::UpFunctionSelectionIndicationFlags(i)
    }
}

impl IEs for UpFunctionSelectionIndicationFlags {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.dcnr {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= UPFSIF_LENGTH + MIN_IE_SIZE {
            let mut data = UpFunctionSelectionIndicationFlags{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            match buffer[4] {
                0 => data.dcnr = false,
                1 => data.dcnr = true,
                _ => return Err(GTPV2Error::IEIncorrect(UPFSIF)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(UPFSIF))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }

    fn is_empty (&self) -> bool {
        self.length == 0
    }
}

#[test]
fn upfsif_ie_unmarshal_test () {
    let encoded:[u8;5]=[0xca, 0x00, 0x01, 0x00, 0x01];
    let decoded = UpFunctionSelectionIndicationFlags { t:UPFSIF, length: UPFSIF_LENGTH as u16, ins:0, dcnr: true };
    let i = UpFunctionSelectionIndicationFlags::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn upfsif_ie_marshal_test () {
    let encoded:[u8;5]=[0xca, 0x00, 0x01, 0x00, 0x01];
    let decoded = UpFunctionSelectionIndicationFlags { t:UPFSIF, length: UPFSIF_LENGTH as u16, ins:0, dcnr: true };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
