// APN Restriction IE - according to 3GPP TS 29.060 V15.5.0 (2019-06) 

use crate::gtpv1::{gtpc::ies::commons::*, utils::*, errors::GTPV1Error};

// APN Restriction IE Type

pub const APNRESTRICTION:u8 = 149;
pub const APNRESTRICTION_LENGTH:u16 = 1;

// APN Restriction Values as per 3GPP 23.060 V16.0.0 (2019-03)

pub const NO_APN_RESTRICTION:u8 = 0; // No existing contexts or restrictions. All APN restriction values are allowed to be established.
pub const PUBLIC1:u8 = 1;           // WAP or MMS only. APN restriction values allowed - 1, 2, 3.
pub const PUBLIC2:u8 = 2;           // Internet or PSPDN. APN restriction values allowed - 1, 2.
pub const PRIVATE1:u8 = 3;          // Corporate access (who use MMS). APN restriction values allowed - 1
pub const PRIVATE2:u8 = 4;          // Corporate access (who do not use MMS). APN restriction values allowed - None.

// APN Restriction IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct ApnRestriction {
    pub t:u8,
    pub length:u16,
    pub restriction_type:u8,
}

impl Default for ApnRestriction {
    fn default() -> Self {
        ApnRestriction { t: APNRESTRICTION, length:APNRESTRICTION_LENGTH, restriction_type:0}
    }
}

impl IEs for ApnRestriction {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.push (self.restriction_type);
        set_tlv_ie_length(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
        if buffer.len()>=4 {
            let mut data=ApnRestriction::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            match buffer[3] {
                i if i<=4 => data.restriction_type=i,
                _ => return Err(GTPV1Error::IncorrectIE),
            }
            Ok(data)
        } else {
            Err(GTPV1Error::InvalidIELength)
        }
    }

    fn len (&self) -> usize {
       APNRESTRICTION_LENGTH as usize 
    }

}

#[test]
fn apnrestriction_ie_marshal_test () {
    let ie_marshalled:[u8;4]=[0x95, 0x00, 0x01, 0x00];
    let ie_to_marshal = ApnRestriction { t:APNRESTRICTION, length: APNRESTRICTION_LENGTH, restriction_type:0 };
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,ie_marshalled);
}

#[test]
fn apnrestriciton_ie_unmarshal_test () {
    let ie_to_unmarshal:[u8;4]=[0x95, 0x00, 0x01, 0x00];
    let ie_unmarshalled = ApnRestriction { t:APNRESTRICTION, length: APNRESTRICTION_LENGTH, restriction_type:0 };
    assert_eq!(ApnRestriction::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}