// Commons for GTP-C IEs

use crate::gtpv1::errors::GTPV1Error;

pub trait IEs {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized;
    fn len (&self) -> usize; // Total IE length including Type+Value for TV messages, Type+Length+Value for TLV messages
}

// Implementation of Empty IE for internal needs

// Empty IE Implementation

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyIE {}

impl Default for EmptyIE {
    fn default() -> EmptyIE {
        EmptyIE {}         
    }
}

impl IEs for EmptyIE {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        ()
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized {
            Ok(EmptyIE::default())       
    }
    
    fn len (&self) -> usize {
       0 
    }
}