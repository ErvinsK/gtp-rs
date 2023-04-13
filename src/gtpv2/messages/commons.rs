use crate::gtpv2::{errors::*, messages::ies::*};

// Common traits of GTPv2 Messages

pub trait Messages {
    fn marshal(&self, buffer: &mut Vec<u8>);
    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error>
    where
        Self: Sized;
    fn tovec(&self) -> Vec<InformationElement>;
    fn fromvec(&mut self, elements: Vec<InformationElement>) -> Result<bool, GTPV2Error>;
    //fn len (&self) -> usize;
}
