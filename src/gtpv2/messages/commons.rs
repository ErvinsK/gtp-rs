use std::collections::HashMap;
use crate::gtpv2::errors::*;

// Common traits of GTPv2 Messages

pub trait Messages {
    fn marshal (self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized;
    //fn len (&self) -> usize;
}
