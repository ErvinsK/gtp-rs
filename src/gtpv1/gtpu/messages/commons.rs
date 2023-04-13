use crate::gtpv1::errors::*;

// Common traits of GTPv1-U Messages

pub trait Messages {
    fn marshal(self, buffer: &mut Vec<u8>);
    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized;
    // fn len (&self) -> usize;
}
