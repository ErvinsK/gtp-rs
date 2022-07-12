use crate::gtpv1::errors::*;
use crate::gtpv1::gtpu::extension_headers::*;

// According to 3GPP TS 29.281 V16.0.0 (2019-12)
   
    // Definition of GTPv1 Header
    
        pub const MIN_HEADER_LENGTH:usize = 8;
        pub const SQN_LENGTH:usize = 2;
        pub const NPDU_NUMBER_LENGTH:usize = 1;

        #[derive(Debug, Clone)]
        pub struct Gtpv1Header {
            pub version:u8,
            pub protocol_type:u8,
            pub extension_header_flag:bool,
            pub sequence_number_flag:bool,
            pub npdu_number_flag:bool,
            pub msgtype:u8,
            pub length:u16,
            pub teid:u32,
            pub sequence_number:Option<u16>,
            pub npdu_number:Option<u8>,
            pub extension_headers:Vec<NextExtensionHeaderField>
        }
        
    // Implementation of GTPv1 Header
    
        impl Gtpv1Header {
    
    // Construct new empty GTPv1 Header
    
            pub fn new () -> Gtpv1Header {
                Gtpv1Header {
                    version:1,
                    protocol_type:1,
                    extension_header_flag:false,
                    sequence_number_flag:false,
                    npdu_number_flag:false,
                    msgtype:0,
                    length:0,
                    teid:0,
                    sequence_number:None,
                    npdu_number:None,
                    extension_headers:vec!(),
                } 
            }

    // Marshal GTPv1 header into a mutable byte vector
    
            pub fn marshal (self, buffer: &mut Vec<u8>) {
                
                // Marshal first octet

                match (self.extension_header_flag, self.sequence_number_flag, self.npdu_number_flag) {
                    (false, false, false) => buffer.push(self.version << 5 | self.protocol_type << 4 ),
                    (false, false, true) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b001),
                    (false, true, true) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b011),
                    (true, true, true) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b111),
                    (true, false, false) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b100),
                    (true, true, false) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b110),
                    (true, false, true) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b101),
                    (false, true, false) => buffer.push(self.version << 5 | self.protocol_type << 4 | 0b010),
                }

                buffer.push(self.msgtype);
                buffer.extend_from_slice(&self.length.to_be_bytes());
                buffer.extend_from_slice(&self.teid.to_be_bytes());

                if self.sequence_number_flag || self.npdu_number_flag || self.extension_header_flag {
                  
                    if let Some(i) = self.sequence_number {
                        buffer.extend_from_slice(&i.to_be_bytes());
                    } else {
                        buffer.push(0x0);
                        buffer.push(0x0);
                    }
                    
                    if let Some(i) = self.npdu_number {
                        buffer.extend_from_slice(&i.to_be_bytes());
                    } else {
                        buffer.push(0x0);
                    }

                    // Marshal Extension Headers

                    if !self.extension_headers.is_empty() { 
                        for i in self.extension_headers.clone().into_iter() {
                            if !matches!(i, NextExtensionHeaderField::NoMoreExtensionHeaders) {
                                i.marshal(buffer);
                            } else {
                                break;
                            }
                        }              
                        buffer.push(NO_MORE_EXTENSION_HEADERS);        
                    } else {
                        buffer.push(NO_MORE_EXTENSION_HEADERS);
                    }
                }

            }

    // Parse GTPv1 header from byte slice
    
            pub fn unmarshal (packet:&[u8]) -> Result<Gtpv1Header, GTPV1Error > {
                if packet.len()<8 {
                    Err(GTPV1Error::HeaderSizeTooSmall)
                } else {
                    let mut header = Gtpv1Header::new();                  
                    header.version = packet [0] >> 5;
                    header.protocol_type = (packet [0] & 0b10000) >> 4;
                    header.msgtype = packet [1];
                    header.length = ((packet[2] as u16) << 8)| packet[3] as u16;
                    header.teid = ((packet [4] as u32) << 24) | ((packet[5] as u32) << 16) | ((packet[6] as u32) << 8) | (packet[7] as u32);       
                    match packet [0] & 0b111 {
                        0b000 => {
                                header.extension_header_flag = false;                          
                                header.sequence_number_flag = false;
                                header.sequence_number = None;
                                header.npdu_number_flag = false;
                                header.npdu_number = None;
                            }, 

                        0b111 => {
                                header.sequence_number_flag = true;
                                match read_sequence_number(&packet[8..]) {
                                    Ok(i) => header.sequence_number=Some(i),
                                    Err(y) => return Err(y),
                                }                         
                                header.npdu_number_flag = true;
                                match read_npdu_number(&packet[10..]) {
                                    Ok(i) => header.npdu_number = Some(i),
                                    Err(y) => return Err(y),
                                } 
                                header.extension_header_flag = true;
                                match read_next_extension_headers (&packet[11..]) {
                                    Ok(i) => header.extension_headers = i,
                                    Err(y) => return Err(y),
                                }
                            },

                        0b011 => {
                                header.extension_header_flag = false;
                                header.extension_headers.push(NextExtensionHeaderField::NoMoreExtensionHeaders);                              
                                header.sequence_number_flag = true;
                                match read_sequence_number(&packet[8..]) {
                                    Ok(i) => header.sequence_number=Some(i),
                                    Err(y) => return Err(y),
                                }
                                header.npdu_number_flag = true;
                                match read_npdu_number(&packet[10..]) {
                                    Ok(i) => header.npdu_number = Some(i),
                                    Err(y) => return Err(y),
                                } 
                        },

                        0b001 => {
                                header.extension_header_flag = false;
                                header.extension_headers.push(NextExtensionHeaderField::NoMoreExtensionHeaders);
                                header.sequence_number_flag = false;
                                header.sequence_number = None;
                                header.npdu_number_flag = true;
                                match read_npdu_number(&packet[10..]) {
                                    Ok(i) => header.npdu_number = Some(i),
                                    Err(y) => return Err(y),
                                } 
                        },
                        
                        0b101 => {
                                header.extension_header_flag = true;
                                match read_next_extension_headers (&packet[11..]) {
                                    Ok(i) => header.extension_headers = i,
                                    Err(y) => return Err(y),
                                }
                                header.sequence_number_flag = false;
                                header.sequence_number = None;
                                header.npdu_number_flag = true;
                                match read_npdu_number(&packet[10..]) {
                                    Ok(i) => header.npdu_number = Some(i),
                                    Err(y) => return Err(y),
                                } 
                        },

                        0b110 => {
                                header.extension_header_flag = true;
                                match read_next_extension_headers (&packet[11..]) {
                                    Ok(i) => header.extension_headers = i,
                                    Err(y) => return Err(y),
                                }
                                header.sequence_number_flag = true;
                                match read_sequence_number(&packet[8..]) {
                                    Ok(i) => header.sequence_number= Some(i),
                                    Err(y) => return Err(y),
                                }
                                header.npdu_number_flag = false;
                                header.npdu_number = None;
                        },

                        0b010 => {
                                header.extension_header_flag = false;
                                header.extension_headers.push(NextExtensionHeaderField::NoMoreExtensionHeaders);                                
                                header.sequence_number_flag = true;
                                match read_sequence_number(&packet[8..]) {
                                    Ok(i) => header.sequence_number=Some(i),
                                    Err(y) => return Err(y),
                                }
                                header.npdu_number_flag = false;
                                header.npdu_number = None;
                        },
                        _ => return Err(GTPV1Error::HeaderFlagError),
                    }
                    Ok(header)
                }
            }
        
        pub fn extension_headers_length (&self) -> usize {
                let mut i:usize = 0;
                for x in &self.extension_headers {
                    i += x.len();
                }
               i
        }
        
        pub fn header_offset (&self) -> usize {

            match (self.sequence_number_flag, self.npdu_number_flag, self.extension_header_flag) {
                (false, false,  false) => MIN_HEADER_LENGTH,      
                (true,  false,  false) => MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+1,        
                (true,  true,   false) => MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+1,
                (true,  true,   true) =>  MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+self.extension_headers_length(),
                (true,  false,  true) =>  MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+self.extension_headers_length(),
                (false, true,   true) =>  MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+self.extension_headers_length(),  
                (false, false,  true) =>  MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+self.extension_headers_length(),
                (false, true,   false) => MIN_HEADER_LENGTH+SQN_LENGTH+NPDU_NUMBER_LENGTH+1,  
                }
            }
        }  
        
fn read_sequence_number (packet:&[u8]) -> Result<u16, GTPV1Error> {
    if packet.len()<2 {
        Err(GTPV1Error::HeaderSizeTooSmall)
    } else {
        Ok(((packet[0] as u16) << 8) | packet[1] as u16)
    }
}
        
fn read_npdu_number (packet:&[u8]) -> Result<u8, GTPV1Error> {
    if packet.len()<1 {
        Err(GTPV1Error::HeaderSizeTooSmall)
    } else {
        Ok(packet[0])
    }
}
        
fn read_next_extension_headers (packet:&[u8]) -> Result<Vec<NextExtensionHeaderField>,GTPV1Error> {
    if packet.len()<1 {
        Err(GTPV1Error::HeaderSizeTooSmall)
    } else {
        let mut result=vec!();
        let mut i:usize=0;
        loop {
            let t = NextExtensionHeaderField::parse(&packet[i..]);
            if matches!(t, NextExtensionHeaderField::NoMoreExtensionHeaders) || matches!(t, NextExtensionHeaderField::Reserved) {
                result.push(t);
                break;
            } else {
                i += t.len();
                result.push(t);
            }
        }
        Ok(result)
    }      
}

#[test]
fn test_read_npdu_number () {
    let npdu_number:[u8;1] = [0xff];
    assert_eq!(read_npdu_number(&npdu_number).unwrap(),0xff);
}

#[test]
fn test_sequence_number () {
    let sqn:[u8;2] = [0xff,0xff];
    assert_eq!(read_sequence_number(&sqn).unwrap(), 0xffff);
}

#[test]
fn test_sequence_number_too_small () {
    let sqn:[u8;1] = [0xff];
    assert_eq!(read_sequence_number(&sqn).unwrap_err(), GTPV1Error::HeaderSizeTooSmall);
}