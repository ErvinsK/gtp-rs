// Millisecond Timestamp IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Millisecond Timestamp IE Type

pub const MS_TIMESTAMP:u8 = 188;
pub const MS_TIMESTAMP_LENGTH:usize = 6;

// Millisecond Timestamp IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct MilliSecondTimestamp {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub timestamp:u64,          //  Timestamp represents a 48 bit unsigned integer in network order format and are encoded as the number of milliseconds since 00:00:00 January 1, 1900 00:00 UTC, i.e. as the rounded value of 1000 x the value of the 64-bit timestamp (Seconds  + (Fraction / (1<<32))) defined in section 6 of IETF RFC 5905 [53].
}

impl Default for MilliSecondTimestamp {
    fn default() -> MilliSecondTimestamp {
        MilliSecondTimestamp { t: MS_TIMESTAMP, length:MS_TIMESTAMP_LENGTH as u16, ins:0, timestamp:0 }        
    }
}

impl From<MilliSecondTimestamp> for InformationElement {
    fn from(i: MilliSecondTimestamp) -> Self {
        InformationElement::MilliSecondTimestamp(i)
    }
}

impl IEs for MilliSecondTimestamp {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&u64::to_be_bytes(self.timestamp));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= MS_TIMESTAMP_LENGTH+MIN_IE_SIZE {
            let mut data = MilliSecondTimestamp::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.timestamp = u64::from_be_bytes([0x00, 0x00, buffer[4],buffer[5],buffer[6],buffer[7],buffer[8],buffer[9]]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MS_TIMESTAMP))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn ms_timestamp_ie_unmarshal_test () {
    let encoded:[u8;10]=[0xbc, 0x00, 0x06, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let decoded = MilliSecondTimestamp { t:MS_TIMESTAMP, length: MS_TIMESTAMP_LENGTH as u16, ins:0, timestamp: 0xffffffffffff };
    let i = MilliSecondTimestamp::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn ms_timestamp_ie_marshal_test () {
    let encoded:[u8;10]=[0xbc, 0x00, 0x06, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let decoded = MilliSecondTimestamp { t:MS_TIMESTAMP, length: MS_TIMESTAMP_LENGTH as u16, ins:0, timestamp: 0xffffffffffff };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
