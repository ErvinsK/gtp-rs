// GTP-C Extension Headers

    // Extension Field Values

    pub const DEFAULT:u16 = 0xffff;

    // Extension Header Types

     pub const NO_MORE_EXTENSION_HEADERS:u8 =                        0b00000000;
     pub const MBMS_SUPPORT_INDICATION:u8 =                          0b00000001;
     pub const MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION:u8 =      0b00000010;
     pub const PDCP_PDU_NUMBER:u8 =                                  0b11000000;
     pub const SUSPEND_REQUEST:u8 =                                  0b11000001;
     pub const SUSPEND_RESPONSE:u8 =                                 0b11000010;

    
    // Common traits
    
    pub trait ExtensionHeaders {
        fn marshal (&self, buffer: &mut Vec<u8>);
        fn unmarshal (buffer:&[u8]) -> Self;
        fn len (&self) -> usize;
    }
    
    // Struct for PDCP PDU Number Extension Header
    
    #[derive(Clone, Debug)]
    pub struct PDCPPDUNumber {
        pub extension_header_type:u8,
        pub length:u8,
        pub pdcp_pdu_number:u16,
    }
    
    impl Default for PDCPPDUNumber {
        fn default() -> PDCPPDUNumber {
            PDCPPDUNumber {
                extension_header_type:PDCP_PDU_NUMBER,
                length:0x01,
                pdcp_pdu_number:0,
            }
        }
    }
    
    impl ExtensionHeaders for PDCPPDUNumber {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.pdcp_pdu_number>>8) as u8);
            buffer.push(((self.pdcp_pdu_number<<8) >> 8) as u8); 
        }
    
        fn unmarshal(buffer: &[u8]) -> PDCPPDUNumber {
            let mut data = PDCPPDUNumber::default();
            data.pdcp_pdu_number = ((buffer[1] as u16) << 8) | (buffer [2] as u16);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }

    // Struct for Suspend Request 
    
    #[derive(Clone, Debug)]
    pub struct SuspendRequest {
        pub extension_header_type:u8,
        pub length:u8,
        pub value:u16,
    }
    
    impl Default for SuspendRequest {
        fn default() -> SuspendRequest {
            SuspendRequest {
                extension_header_type:SUSPEND_REQUEST,
                length:0x01,
                value:DEFAULT,
            }
        }
    }
    
    impl ExtensionHeaders for SuspendRequest {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.value>>8) as u8);
            buffer.push(((self.value<<8) >> 8) as u8); 
        }
    
        fn unmarshal(buffer: &[u8]) -> SuspendRequest {
            let mut data = SuspendRequest::default();
            data.value = ((buffer[1] as u16) << 8) | (buffer [2] as u16);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for Suspend Response 
    
    #[derive(Clone, Debug)]
    pub struct SuspendResponse {
        pub extension_header_type:u8,
        pub length:u8,
        pub value:u16,
    }
    
    impl Default for SuspendResponse {
        fn default() -> SuspendResponse {
            SuspendResponse {
                extension_header_type:SUSPEND_RESPONSE,
                length:0x01,
                value:DEFAULT,
            }
        }
    }
    
    impl ExtensionHeaders for SuspendResponse {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.value>>8) as u8);
            buffer.push(((self.value<<8) >> 8) as u8); 
        }
    
        fn unmarshal(buffer: &[u8]) -> SuspendResponse {
            let mut data = SuspendResponse::default();
            data.value = ((buffer[1] as u16) << 8) | (buffer [2] as u16);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for MBMS Support Indication 
    
    #[derive(Clone, Debug)]
    pub struct MBMSSupportIndication {
        pub extension_header_type:u8,
        pub length:u8,
        pub value:u16,
    }
    
    impl Default for MBMSSupportIndication {
        fn default() -> MBMSSupportIndication {
            MBMSSupportIndication {
                extension_header_type:MBMS_SUPPORT_INDICATION,
                length:0x01,
                value:DEFAULT,
            }
        }
    }
    
    impl ExtensionHeaders for MBMSSupportIndication {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.value>>8) as u8);
            buffer.push(((self.value<<8) >> 8) as u8); 
        }
    
        fn unmarshal(buffer: &[u8]) -> MBMSSupportIndication {
            let mut data = MBMSSupportIndication::default();
            data.value = ((buffer[1] as u16) << 8) | (buffer [2] as u16);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }

    // Struct for MS Info Change Reporting Support Indication 
    
    #[derive(Clone, Debug)]
    pub struct MSInfoChangeReportingSupportIndication {
        pub extension_header_type:u8,
        pub length:u8,
        pub value:u16,
    }
    
    impl Default for MSInfoChangeReportingSupportIndication {
        fn default() -> MSInfoChangeReportingSupportIndication {
            MSInfoChangeReportingSupportIndication {
                extension_header_type:MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION,
                length:0x01,
                value:DEFAULT,
            }
        }
    }
    
    impl ExtensionHeaders for MSInfoChangeReportingSupportIndication {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.value>>8) as u8);
            buffer.push(((self.value<<8) >> 8) as u8); 
        }
    
        fn unmarshal(buffer: &[u8]) -> MSInfoChangeReportingSupportIndication {
            let mut data = MSInfoChangeReportingSupportIndication::default();
            data.value = ((buffer[1] as u16) << 8) | (buffer [2] as u16);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Enum to hold all possible Extension headers
    
        #[derive(Clone, Debug)]
        pub enum NextExtensionHeaderField {
            NoMoreExtensionHeaders,
            Reserved,
            PDCPPDUNumber(PDCPPDUNumber),
            SuspendRequest(SuspendRequest),
            SuspendResponse(SuspendResponse),
            MBMSSupportIndication(MBMSSupportIndication),
            MSInfoChangeReportingSupportIndication(MSInfoChangeReportingSupportIndication),
        }
    
        impl NextExtensionHeaderField {
            
            pub fn parse (v:&[u8]) -> NextExtensionHeaderField {
                match v[0] {
                    NO_MORE_EXTENSION_HEADERS => NextExtensionHeaderField::NoMoreExtensionHeaders,
                    PDCP_PDU_NUMBER => NextExtensionHeaderField::PDCPPDUNumber(PDCPPDUNumber::unmarshal(&v[1..])),
                    SUSPEND_REQUEST => NextExtensionHeaderField::SuspendRequest(SuspendRequest::unmarshal(&v[1..])),
                    SUSPEND_RESPONSE => NextExtensionHeaderField::SuspendResponse(SuspendResponse::unmarshal(&v[1..])),
                    MBMS_SUPPORT_INDICATION => NextExtensionHeaderField::MBMSSupportIndication(MBMSSupportIndication::unmarshal(&v[1..])),
                    MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION => NextExtensionHeaderField::MSInfoChangeReportingSupportIndication(MSInfoChangeReportingSupportIndication::unmarshal(&v[1..])),
                    _ => return NextExtensionHeaderField::Reserved,
    
                }
            }

            pub fn marshal (self, buffer: &mut Vec<u8>) {
                match self {
                    NextExtensionHeaderField::NoMoreExtensionHeaders => buffer.push (NO_MORE_EXTENSION_HEADERS),
                    NextExtensionHeaderField::Reserved => (),
                    NextExtensionHeaderField::PDCPPDUNumber(i) => i.marshal(buffer),
                    NextExtensionHeaderField::SuspendRequest(i) => i.marshal(buffer),
                    NextExtensionHeaderField::SuspendResponse(i) => i.marshal(buffer),
                    NextExtensionHeaderField::MBMSSupportIndication(i) => i.marshal(buffer),
                    NextExtensionHeaderField::MSInfoChangeReportingSupportIndication(i) => i.marshal(buffer),
                }
            }
    
            // return length in bytes, as encoded is the length is 4*bytes
    
            pub fn len (&self) -> usize {
                let size = |i:u8| -> usize { (i as usize)*4 };
                match *self {
                    NextExtensionHeaderField::NoMoreExtensionHeaders => 1,
                    NextExtensionHeaderField::Reserved => 1,
                    NextExtensionHeaderField::PDCPPDUNumber(PDCPPDUNumber{length, ..}) => size(length),
                    NextExtensionHeaderField::SuspendRequest(SuspendRequest{length, ..}) => size(length),
                    NextExtensionHeaderField::SuspendResponse(SuspendResponse{length, ..}) => size(length),
                    NextExtensionHeaderField::MBMSSupportIndication(MBMSSupportIndication{length, ..}) => size(length),
                    NextExtensionHeaderField::MSInfoChangeReportingSupportIndication(MSInfoChangeReportingSupportIndication{length, ..}) => size(length),
                }
            }
        }
    