use std::net::{IpAddr, Ipv4Addr};

// Definitions of Information Elements

pub const RECOVERY:u8 = 14;
pub const TEID:u8 = 16;
pub const GTPU_PEER_ADDRESS:u8 = 133;
pub const EXTENSION_HEADER_TYPE_LIST:u8 = 141;
pub const PRIVATE_EXTENSION:u8 = 255;
pub const RECOVERY_LENGTH:usize = 2;
pub const TEID_LENGTH:usize = 5;


pub trait IEs {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Option<Self> where Self: std::marker::Sized;
    fn len (&self) -> usize; // Total IE length including Type+Value for TV messages, Type+Length+Value for TLV messages
}

// Recovery IE implementation 

#[derive(Debug)]
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
        if buffer.len()>=RECOVERY_LENGTH && buffer[0] == RECOVERY {
            Some(Recovery { t:RECOVERY, restart_counter: buffer[1] })
        } else { 
            None
        }
        
    }

    fn len(&self) -> usize {
        RECOVERY_LENGTH
    }
}

// TEID IE implementation 

#[derive(Debug)]
pub struct Teid {
    pub t:u8,
    pub teid:u32,
}

impl Default for Teid {
    fn default() -> Teid {
        Teid {
            t:TEID,
            teid:0,
        }
    }
}

impl IEs for Teid {

    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.push((self.teid>>24) as u8);
        buffer.push(((self.teid<<8)>>24) as u8);
        buffer.push(((self.teid<<16)>>24) as u8);
        buffer.push(((self.teid<<24)>>24) as u8);
    }

    fn unmarshal(buffer: &[u8]) -> Option<Teid> {
        if buffer.len()>=TEID_LENGTH {
            let mut data = Teid::default();
            data.teid=(buffer[1] as u32)<<24 | (buffer[2] as u32) <<16 | (buffer[3] as u32) <<8 | (buffer[4] as u32);
            Some(data)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        TEID_LENGTH
    }
}

// GTP-U Peer Address IE implementation

#[derive(Debug)]
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
                buffer.push(0x04);
                buffer.append(&mut i.octets().to_vec());
            }, 
            IpAddr::V6(i) => {
                buffer.push(0x1);
                buffer.push(0x0);
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

#[derive(Debug)]
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

#[derive(Debug)]
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