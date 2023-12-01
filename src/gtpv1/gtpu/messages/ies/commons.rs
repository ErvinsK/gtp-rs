// Commons for GTPv1-U IEs

use crate::gtpv1::errors::GTPV1Error;

pub trait IEs {
    fn marshal(&self, buffer: &mut Vec<u8>);
    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized;
    fn len(&self) -> usize; // Total IE length including Type+Value for TV messages, Type+Length+Value for TLV messages
    fn is_empty(&self) -> bool;
}
