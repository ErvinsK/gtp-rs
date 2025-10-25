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

impl Iterator for dyn Messages {
    type Item = InformationElement;
    fn next(&mut self) -> Option<Self::Item> {
        let mut elements = self.tovec();
        if elements.is_empty() {
            None
        } else {
            elements.pop()
        }
    }
}

pub fn vec_by_ins<'a, T: IEs>(ins: u8, vec: &'a [T]) -> Vec<&'a T> {
    vec.iter()
        .filter(|x| (*x).get_ins() == ins)
        .collect::<Vec<&'a T>>()
}
