// GTPv1-C Information Elements
// According to 3GPP TS 29.060 V15.5.0 (2019-06)

use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}};
use crate::gtpv1::utils::*;

// Definitions of Information Elements

pub const CAUSE:u8 = 1;
pub const IMSI:u8 = 2;
pub const RAI:u8 = 3;
pub const RECOVERY:u8 = 14;
pub const SELECTION_MODE:u8 = 15;
pub const TEID_DATA:u8 = 16;
pub const TEID_CONTROL:u8 = 17;
pub const NSAPI:u8 = 20;
pub const CHARGING_CHARACTERISTICS:u8 = 26;
pub const TRACE_REFERENCE:u8 = 27;
pub const TRACE_TYPE:u8 = 28;
pub const END_USER_ADDRESS:u8 = 128;
pub const GTPU_PEER_ADDRESS:u8 = 133;
pub const EXTENSION_HEADER_TYPE_LIST:u8 = 141;
pub const PRIVATE_EXTENSION:u8 = 255;

// Length of IEs 

pub const CAUSE_LENGTH:usize = 1;
pub const IMSI_LENGTH:usize = 8;
pub const RAI_LENGTH:usize = 6;
pub const RECOVERY_LENGTH:usize = 1;
pub const SELECTION_MODE_LENGTH:usize = 1;
pub const TEID_LENGTH:usize = 4;
pub const NSAPI_LENGTH:usize = 1;
pub const CHARGING_CHARACTERISTICS_LENGTH:usize = 2;
pub const TRACE_REFERENCE_LENGTH:usize = 2;
pub const TRACE_TYPE_LENGTH:usize = 2;


pub trait IEs {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized;
    fn len (&self) -> usize; // Total IE length including Type+Value for TV messages, Type+Length+Value for TLV messages
}

// Recovery IE implementation 

#[derive(Debug, Clone)]
pub struct Recovery {
    pub t:u8,
    pub restart_counter:u8,
}

impl Default for Recovery {
    fn default() -> Recovery {
        Recovery {
            t:RECOVERY,
            restart_counter:0,
        }
    }
}

impl IEs for Recovery {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(RECOVERY);
        buffer.push(self.restart_counter);
    }

    fn unmarshal(buffer: &[u8]) -> Option<Recovery> {
        if buffer.len()>=RECOVERY_LENGTH {
            Some(Recovery { t:RECOVERY, restart_counter: buffer[1] })
        } else { 
            None
        }
        
    }

    fn len(&self) -> usize {
        RECOVERY_LENGTH+1
    }
}

// IMSI IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Imsi {
    pub t:u8,
    pub imsi:String,
}

impl Default for Imsi {
    fn default() -> Imsi {
        Imsi { t: IMSI, imsi: "0".to_string(), }        
    }
}

impl IEs for Imsi {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend(tbcd_encode(&self.imsi));
    }

    fn unmarshal (buffer:&[u8]) -> Option<Imsi> where Self:Sized {
        if buffer.len()>=IMSI_LENGTH+1 {
            let mut data = Imsi::default();
            match buffer[1..=8].try_into() {
               Ok(i) => data.imsi = tbcd_decode(i),
               Err(_) => return None, 
            }
            Some(data)
        } else {
            None
        }
    }
    
    fn len (&self) -> usize {
       IMSI_LENGTH+1 
    }
}

#[test]
fn imsi_ie_marshal_test () {
    let encoded_ie:[u8;9]=[0x02, 0x09, 0x41, 0x50, 0x01, 0x31, 0x72, 0x94, 0xf6];
    let test_struct = Imsi { t:0x02, imsi:"901405101327496".to_string(), };
    let i = Imsi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn imsi_ie_unmarshal_test () {
    let encoded_ie:[u8;9]=[0x02, 0x09, 0x41, 0x50, 0x01, 0x31, 0x72, 0x94, 0xf6];
    let test_struct = Imsi { t:0x02, imsi:"901405101327496".to_string(), };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}

// Routeing Area Identity (RAI) IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Rai {
    pub t: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub rac: u8,

}

impl Default for Rai {
    fn default() -> Self {
        Rai { t: 3, mcc: 0, mnc: 0, lac: 0, rac: 0 }
    }
}

impl IEs for Rai {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mcc_digits:Vec<u8> = to_digits(self.mcc);
        let mnc_digits:Vec<u8> = to_digits(self.mnc);
        buffer.push(self.t);
        buffer.push(mcc_digits[1]<<4 | mcc_digits[0]);
        if mnc_digits.len()==2 {
            buffer.push(0b1111<<4 | mcc_digits[2]);
        } else {
            buffer.push(mnc_digits[2]<<4 | mcc_digits[2]);
        }
        buffer.push(mnc_digits[1]<<4 | mnc_digits[0]);
        buffer.extend_from_slice(&self.lac.to_be_bytes());
        buffer.push(self.rac);
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=RAI_LENGTH+1 {
            let mut data:Rai=Rai::default();
            let mut mcc_digits:Vec<u8>=vec!();
            let mut mnc_digits:Vec<u8>=vec!();
            mcc_digits.push(buffer[1] & 0b1111);
            mcc_digits.push(buffer[1] >> 4);
            mcc_digits.push(buffer[2] & 0b00001111);
            mnc_digits.push(buffer[3] & 0b1111);
            mnc_digits.push(buffer[3] >> 4);
            if buffer[2]>>4 != 0b1111 {
                mnc_digits.push(buffer[2]>>4);
            }
            if let Ok(i) = mcc_digits.iter().flat_map( |c| char::from_digit(*c as u32, 10)).collect::<String>().parse::<u16>() {
                data.mcc=i;
            }
            if let Ok(i) = mnc_digits.iter().flat_map( |c| char::from_digit(*c as u32, 10)).collect::<String>().parse::<u16>() {
                data.mnc=i;
            }
            data.lac=u16::from_be_bytes([buffer[4],buffer[5]]);
            data.rac=buffer[6];
            Some (data)
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        RAI_LENGTH+1
    }

}

#[test]
fn rai_ie_marshal_test() {
    let rai_to_marshal = Rai { t:3, mcc:999, mnc:111, lac:999, rac: 67};
    let rai_marshalled:[u8;7] = [0x03, 0x99, 0x19, 0x11, 0x03, 0xe7, 0x43];
    let mut buffer:Vec<u8>=vec!();
    rai_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer,rai_marshalled);
}

#[test]
fn rai_ie_unmarshal_test() {
    let rai_unmarshalled = Rai { t:3, mcc:999, mnc:111, lac:999, rac: 67};
    let rai_to_unmarshal:[u8;7] = [0x03, 0x99, 0x19, 0x11, 0x03, 0xe7, 0x43];
    assert_eq!(Rai::unmarshal(&rai_to_unmarshal).unwrap(), rai_unmarshalled);
}

// Selection Mode IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionMode {
    pub t:u8,
    pub value:u8,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode { t: SELECTION_MODE, value: 0 }
    }
}

impl IEs for SelectionMode {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(0b11111100 | self.value);
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=SELECTION_MODE_LENGTH+1 {
            let mut data=SelectionMode::default();
            data.value = buffer[1] & 0b11;
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        SELECTION_MODE_LENGTH+1
    }
}

#[test]
fn selectionmode_ie_marshal_test() {
    let ie_to_marshal=SelectionMode{ t: SELECTION_MODE, value:2};
    let ie_marshalled:[u8;2]=[0x0f, 0xfe];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn selectionmode_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;2]=[0x0f, 0xfc];
    let ie_unmarshalled = SelectionMode { t: SELECTION_MODE, value:0};
    assert_eq!(SelectionMode::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}

// TEID IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct Teid {
    pub t:u8,
    pub teid:u32,
}

impl Default for Teid {
    fn default() -> Teid {
        Teid {
            t:TEID_DATA,
            teid:0,
        }
    }
}

impl IEs for Teid {

    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.teid.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Option<Teid> {
        if buffer.len()>=TEID_LENGTH+1 {
            let mut data = Teid::default();
            data.t=buffer[0];
            data.teid=u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]);
            Some(data)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        TEID_LENGTH+1
    }
}

#[test]
fn teid_ie_marshal_test () {
    let ie_marshalled:[u8;5] = [0x10, 0x63, 0x41, 0xaf, 0xd7];
    let ie_to_marshal = Teid { t: TEID_DATA, teid: 0x6341afd7};
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn teid_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;5] = [0x11, 0x63, 0x41, 0xaf, 0xd7];
    let ie_unmarshalled = Teid { t: TEID_CONTROL, teid: 0x6341afd7};
    assert_eq!(Teid::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}

// NSAPI IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct Nsapi {
    pub t:u8,
    pub value:u8,
}

impl Default for Nsapi {
    fn default() -> Self {
        Nsapi { t: NSAPI, value: 0 }
    }
}

impl IEs for Nsapi {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.value);
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=NSAPI_LENGTH+1 {
            let mut data=Nsapi::default();
            data.value = buffer[1] & 0b1111;
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        NSAPI_LENGTH+1
    }
}

#[test]
fn nsapi_ie_marshal_test() {
    let ie_to_marshal=Nsapi{ t: NSAPI, value:5};
    let ie_marshalled:[u8;2]=[0x14, 0x05];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn nsapi_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;2]=[0x14, 0x05];
    let ie_unmarshalled = Nsapi { t: NSAPI, value:5};
    assert_eq!(Nsapi::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}

// Charging Characteristics IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct ChargingCharacteristics {
    pub t:u8,
    pub value:u8, // Normal charging = 0b1000, Prepaid charging = 0b0100, Flat rate charging = 0b0010, Hot billing charging = 0b0001
}

impl Default for ChargingCharacteristics {
    fn default() -> Self {
        ChargingCharacteristics { t: CHARGING_CHARACTERISTICS, value: 0b1000 }
    }
}

impl IEs for ChargingCharacteristics {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.value);
        buffer.push(0x00);
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=CHARGING_CHARACTERISTICS_LENGTH+1 {
            let mut data=ChargingCharacteristics::default();
            data.value = buffer[1] & 0b1111;
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        CHARGING_CHARACTERISTICS_LENGTH+1
    }
}

#[test]
fn charging_characteristics_ie_marshal_test() {
    let ie_to_marshal=ChargingCharacteristics{ t: CHARGING_CHARACTERISTICS, value:0b1000};
    let ie_marshalled:[u8;3]=[0x1a, 0x08, 0x00];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn charging_characteristics_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;3]=[0x1a, 0x08, 0x00];
    let ie_unmarshalled = ChargingCharacteristics { t: CHARGING_CHARACTERISTICS, value:0b1000};
    assert_eq!(ChargingCharacteristics::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}

// Trace Reference IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct TraceReference {
    pub t:u8,
    pub value:u16, 
}

impl Default for TraceReference {
    fn default() -> Self {
        TraceReference { t: TRACE_REFERENCE, value: 0 }
    }
}

impl IEs for TraceReference {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.value.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=TRACE_REFERENCE_LENGTH+1 {
            let mut data=TraceReference::default();
            data.value = u16::from_be_bytes([buffer[1],buffer[2]]);
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        TRACE_REFERENCE_LENGTH+1
    }
}

#[test]
fn trace_reference_ie_marshal_test() {
    let ie_to_marshal=TraceReference{ t: TRACE_REFERENCE, value:1010};
    let ie_marshalled:[u8;3]=[0x1b, 0x03, 0xf2];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn trace_reference_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;3]=[0x1b, 0x03, 0xf2];
    let ie_unmarshalled = TraceReference { t: TRACE_REFERENCE, value:1010};
    assert_eq!(TraceReference::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}

// Trace Type IE implementation

#[derive(Debug, Clone, PartialEq)]

pub struct TraceType {
    pub t:u8,
    pub value:u16, 
}

impl Default for TraceType {
    fn default() -> Self {
        TraceType { t: TRACE_TYPE, value: 0 }
    }
}

impl IEs for TraceType {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.value.to_be_bytes());
    }

    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self:Sized {
        if buffer.len()>=TRACE_TYPE_LENGTH+1 {
            let mut data=TraceType::default();
            data.value = u16::from_be_bytes([buffer[1],buffer[2]]);
            Some(data) 
        } else {
            None
        }
    }

    fn len (&self) -> usize {
        TRACE_TYPE_LENGTH+1
    }
}

#[test]
fn trace_type_ie_marshal_test() {
    let ie_to_marshal=TraceType{ t: TRACE_TYPE, value:2};
    let ie_marshalled:[u8;3]=[0x1c, 0x00, 0x02];
    let mut buffer:Vec<u8>=vec!();
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn trace_type_ie_unmarshal_test() {
    let ie_to_unmarshal:[u8;3]=[0x1c, 0x00, 0x02];
    let ie_unmarshalled = TraceType { t: TRACE_TYPE, value:2};
    assert_eq!(TraceType::unmarshal(&ie_to_unmarshal).unwrap(), ie_unmarshalled);
}

// End User Address IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct EndUserAddress {
    pub t: u8,
    pub length: u16,
    pub pdp_type_org: u8,
    pub pdp_type_nbr: u8,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl Default for EndUserAddress {
    fn default() -> Self {
        EndUserAddress { t: END_USER_ADDRESS, length: 2, pdp_type_org: 0b11110001, pdp_type_nbr: 0x21, ipv4: None, ipv6: None }
    }
}

impl IEs for EndUserAddress {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        
    }
}

// GTP-U Peer Address IE implementation

#[derive(Debug, Clone)]
pub struct GTPUPeerAddress {
    pub t:u8,
    pub length:u16,
    pub ip:IpAddr,
}

impl Default for GTPUPeerAddress {
    fn default() -> GTPUPeerAddress {
        GTPUPeerAddress {
            t:GTPU_PEER_ADDRESS,
            length:4,
            ip:IpAddr::V4(Ipv4Addr::new(0,0,0,0)),
        }
    }
}

impl IEs for GTPUPeerAddress {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        match self.ip {
            IpAddr::V4(i) => {
                buffer.push(0x00);
                buffer.push(0x04);
                buffer.append(&mut i.octets().to_vec());
            }, 
            IpAddr::V6(i) => {
                buffer.push(0x00);
                buffer.push(0x10);
                buffer.append(&mut i.octets().to_vec());
            },   
        }
    }

    fn unmarshal(buffer: &[u8]) -> Option<GTPUPeerAddress> {
        if buffer.len()>=7 {
            match (buffer[1] as u16)<<8 | (buffer[2] as u16) {
                0x04 => {
                    let mut data = GTPUPeerAddress::default();
                    data.ip = IpAddr::from([buffer[2], buffer[3], buffer[4], buffer[5]]);
                    return Some(data);
                } 
                0x10 => {
                    if buffer.len()>=0x13 {
                        let mut data = GTPUPeerAddress::default();
                        data.length = 0x10;
                        let mut dst = [0;16];
                        dst.clone_from_slice(&buffer[3..19]);
                        data.ip = IpAddr::from(dst);
                        return Some(data);
                    } else { 
                        return None;
                    }   
                }
            _ => None,
            }
        } else {
            None
        }
    }
    
    fn len(&self) -> usize {
        (self.length+3) as usize
    }
}
// Extension Header Type List IE implementation

#[derive(Debug, Clone)]
pub struct ExtensionHeaderTypeList {
    pub t:u8,
    pub length:u8,
    pub list:Vec<u8>,
}

impl Default for ExtensionHeaderTypeList {
    fn default() -> ExtensionHeaderTypeList {
        ExtensionHeaderTypeList {
            t:EXTENSION_HEADER_TYPE_LIST,
            length:0,
            list:vec!(),
        }
    }
}

impl IEs for ExtensionHeaderTypeList {

    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push(self.length);
        buffer.append(&mut self.list.clone());
    }

    fn unmarshal(buffer: &[u8]) -> Option<ExtensionHeaderTypeList> {
        if buffer.len()<=2 {
            None
        } else {
            match buffer[1] as usize {
                i if i <= buffer[2..].len() => {
                    let mut data = ExtensionHeaderTypeList::default();
                    data.length=buffer[1];
                    data.list=buffer[2..data.length as usize].to_vec(); 
                    return Some(data);
                    }
                _ => None,   
            }
        } 
    }
    

    fn len(&self) -> usize {
        (self.length+2) as usize
    }
}

// Private Extension IE implementation

#[derive(Debug, Clone)]
pub struct PrivateExtension {
    pub t:u8,
    pub length:u16,
    pub extension_id:u16,
    pub extension_value:Vec<u8>,
}


impl Default for PrivateExtension {
    fn default() -> PrivateExtension {
        PrivateExtension {
            t:PRIVATE_EXTENSION,
            length:0,
            extension_id:0,
            extension_value:vec!(),
        }
    }
}

impl IEs for PrivateExtension {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push((self.length >> 8) as u8);
        buffer.push(((self.length << 8) >> 8) as u8);
        buffer.push((self.extension_id>>8) as u8);
        buffer.push(((self.extension_id<<8) >> 8) as u8);
        buffer.append(&mut self.extension_value.clone());
    }

    fn unmarshal(buffer: &[u8]) -> Option<PrivateExtension> {
        if buffer.len()<6 {
            None
        } else {
            match ((buffer[1] as u16)<<8 | (buffer[2] as u16)) as usize {
                i if i<=buffer[3..].len() => {
                    let mut data = PrivateExtension::default();
                    data.length = (buffer[1] as u16)<<8 | (buffer[2] as u16);
                    data.extension_id = (buffer[3] as u16)<<8 | (buffer[4] as u16);
                    data.extension_value = buffer[2..(data.length as usize-2)].to_vec(); 
                    Some(data)
                }
                _ => None,
            }
        }    
    }

    fn len(&self) -> usize {
        (self.length+3) as usize
    }
}