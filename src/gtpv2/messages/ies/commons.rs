// Commons for GTP-C IEs

use crate::gtpv2::errors::GTPV2Error;

pub const MIN_IE_SIZE:usize = 4;

pub trait IEs {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized;
    fn len (&self) -> usize; // Total IE length = Type+Length+Instance+Value for TLIV messages
}
