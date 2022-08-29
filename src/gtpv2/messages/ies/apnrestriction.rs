// APN Restriction IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// APN Restriction IE Type

pub const APNRESTRICTION:u8 = 127;
pub const APNRESTRICTION_LENGTH:usize = 1;

// APN Restriction Enum and Values as per 3GPP 23.060 V16.0.0 (2019-03)

#[derive(Debug, Clone, PartialEq)]
pub enum Restriction {
    NoApnRestriction,
    Public1,
    Public2,
    Private1,
    Private2,
}

impl Restriction {
    fn enum_to_value (i:&Restriction) -> u8 {
        match i {
            Restriction::NoApnRestriction => 0,
            Restriction::Public1 => 1,
            Restriction::Public2 => 2,
            Restriction::Private1 => 3,
            Restriction::Private2 => 4,
        }
    }
    fn value_to_enum (i:u8) -> Result<Restriction, GTPV2Error> {
        match i {
            0 => Ok(Restriction::NoApnRestriction),
            1 => Ok(Restriction::Public1),
            2 => Ok(Restriction::Public2),
            3 => Ok(Restriction::Private1),
            4 => Ok(Restriction::Private2),
            _ => Err(GTPV2Error::IEIncorrect(APNRESTRICTION)),
        }
    }
}

// APN Restriction IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct ApnRestriction {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub restriction_type:Restriction,
}

impl Default for ApnRestriction {
    fn default() -> Self {
        ApnRestriction { t: APNRESTRICTION, length:APNRESTRICTION_LENGTH as u16, ins:0, restriction_type:Restriction::NoApnRestriction}
    }
}

impl From<ApnRestriction> for InformationElement {
    fn from(i: ApnRestriction) -> Self {
        InformationElement::ApnRestriction(i)
    }
}

impl IEs for ApnRestriction {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push (Restriction::enum_to_value(&self.restriction_type));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=APNRESTRICTION_LENGTH+MIN_IE_SIZE {
            let mut data=ApnRestriction::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            match Restriction::value_to_enum(buffer[4]) {
                Ok(i) => data.restriction_type=i,
                Err(j) => return Err(j),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(APNRESTRICTION))
        }
    }

    fn len (&self) -> usize {
       APNRESTRICTION_LENGTH+MIN_IE_SIZE 
    }

}

#[test]
fn apnrestriction_ie_marshal_test () {
    let encoded:[u8;5]=[0x7f, 0x00, 0x01, 0x00, 0x00];
    let decoded = ApnRestriction { t:APNRESTRICTION, length: APNRESTRICTION_LENGTH as u16, ins:0, restriction_type:Restriction::NoApnRestriction };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn apnrestriciton_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x7f, 0x00, 0x01, 0x00, 0x00];
    let decoded = ApnRestriction { t:APNRESTRICTION, length: APNRESTRICTION_LENGTH as u16, ins:0, restriction_type:Restriction::NoApnRestriction };
    assert_eq!(ApnRestriction::unmarshal(&encoded).unwrap(), decoded);
}