// GTP-U Extension Headers Commons

// Extension Header Common Field Values

use crate::gtpv1::errors::GTPV1Error;

// Extension Header Common Type

pub const NO_MORE_EXTENSION_HEADERS: u8 = 0;

// Common trait

pub trait ExtensionHeaders {
    fn marshal(&self, buffer: &mut Vec<u8>);
    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
