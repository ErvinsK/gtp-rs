// GTP-U Extension Headers

    // Extension Field Values

     // Extension Header Types

     pub const NO_MORE_EXTENSION_HEADERS:u8 =    0b00000000;
     pub const LONG_PDCP_PDU_NUMBER_R16:u8 =     0b00000011;
     pub const SERVICE_CLASS_INDICATOR:u8 =      0b00100000;
     pub const UDP_PORT:u8 =                     0b01000000;
     pub const RAN_CONTAINER:u8 =                0b10000001;
     pub const LONG_PDCP_PDU_NUMBER_R15:u8 =     0b10000010;
     pub const XW_RAN_CONTAINER:u8 =             0b10000011;
     pub const NR_RAN_CONTAINER:u8 =             0b10000100;
     pub const PDU_SESSION_CONTAINER:u8 =        0b10000101;
     pub const PDCP_PDU_NUMBER:u8 =              0b11000000;

    // Common traits
    
    pub trait ExtensionHeaders {
        fn marshal (&self, buffer: &mut Vec<u8>);
        fn unmarshal (buffer:&[u8]) -> Self;
        fn len (&self) -> usize;
    }
    
    // Struct for UDP Port Extension Header 
    
    #[derive(Clone, Debug)]
    pub struct UDPPort {
        pub extension_header_type:u8,
        pub length:u8,
        pub port:u16,
    }
    
    impl Default for UDPPort {
        fn default() -> UDPPort {
            UDPPort {
                extension_header_type:UDP_PORT,
                length:0x01,
                port:0,
            }
        }
    }
    
    impl ExtensionHeaders for UDPPort {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.port>>8) as u8);
            buffer.push(((self.port<<8) >> 8) as u8); 
        }
    
        fn unmarshal(buffer: &[u8]) -> UDPPort {
            let mut data = UDPPort::default();
            data.port = ((buffer[1] as u16) << 8) | (buffer [2] as u16);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
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
    
    // Struct for Long PDCP PDU Number Extension header
    
    #[derive(Clone, Debug)]
    pub struct LongPDCPPDUNumber {
        pub extension_header_type:u8,
        pub length:u8,
        pub long_pdcp_pdu_number:u32,
    }
    
    impl Default for LongPDCPPDUNumber {
        fn default() -> LongPDCPPDUNumber {
            LongPDCPPDUNumber{
                extension_header_type: LONG_PDCP_PDU_NUMBER_R16,    // Default value according to Rel.16 
                length:0x02,
                long_pdcp_pdu_number:0,
            }
        } 
    }
    
    impl ExtensionHeaders for LongPDCPPDUNumber {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push((self.long_pdcp_pdu_number>>16) as u8);
            buffer.push(((self.long_pdcp_pdu_number<<16) >> 24) as u8);
            buffer.push(((self.long_pdcp_pdu_number<<24) >> 24) as u8);
            buffer.push(0x00);
            buffer.push(0x00);
            buffer.push(0x00);
        }
    
        fn unmarshal(buffer: &[u8]) -> LongPDCPPDUNumber {
            let mut data = LongPDCPPDUNumber::default();
            data.extension_header_type = buffer[0];
            data.long_pdcp_pdu_number = ((buffer[1] as u32) << 16) | ((buffer [2] as u32) << 8) | (buffer[3] as u32);
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for Service Class Indication Extension header
    
    #[derive(Clone, Debug)]
    pub struct ServiceClassIndicator {
        pub extension_header_type: u8,
        pub length: u8,
        pub service_class_indicator: u8,
    }
    
    impl Default for ServiceClassIndicator {
        fn default() -> ServiceClassIndicator {
            ServiceClassIndicator {
                extension_header_type: SERVICE_CLASS_INDICATOR,
                length: 0x01,
                service_class_indicator: 0,
            }
        }
    }
    
    impl ExtensionHeaders for ServiceClassIndicator {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.push(self.service_class_indicator);
            buffer.push(0x00);
        }
    
        fn unmarshal(buffer: &[u8]) -> ServiceClassIndicator {
            let mut data = ServiceClassIndicator::default();
            data.service_class_indicator = buffer[1];
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for RAN Container Extension header
    
    #[derive(Clone, Debug)]
    pub struct RANContainer {
        pub extension_header_type: u8,
        pub length: u8,
        pub ran_container: Vec<u8>,
    }
    
    impl Default for RANContainer {
        fn default() -> RANContainer {
            RANContainer {
                extension_header_type: RAN_CONTAINER,
                length:0x00,
                ran_container:vec!(),
            }
        }
    }
    
    impl ExtensionHeaders for RANContainer {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.extend(&self.ran_container);
        }
    
        fn unmarshal(buffer: &[u8]) -> RANContainer {
            let mut data = RANContainer::default();
            data.length = buffer[0];
            for i in 1..(data.length*4-2) as usize {
                data.ran_container.push(buffer[i]);
            }
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for Xw RAN Container 
    
    #[derive(Clone, Debug)]
    pub struct XwRANContainer {
        pub extension_header_type: u8,
        pub length: u8,
        pub xw_ran_container: Vec<u8>,
    }
    
    impl Default for XwRANContainer {
        fn default() -> XwRANContainer {
            XwRANContainer {
                extension_header_type: XW_RAN_CONTAINER,
                length:0x00,
                xw_ran_container:vec!(),
            }
        }
    }
    
    impl ExtensionHeaders for XwRANContainer {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.extend(&self.xw_ran_container);
        }
    
        fn unmarshal(buffer: &[u8]) -> XwRANContainer {
            let mut data = XwRANContainer::default();
            data.length = buffer[0];
            for i in 1..(data.length*4-2) as usize {
                data.xw_ran_container.push(buffer[i]);
            }
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for NR RAN Container
    
    #[derive(Clone, Debug)]
    pub struct NRRANContainer {
        pub extension_header_type: u8,
        pub length: u8,
        pub nr_ran_container: Vec<u8>,
    }
    
    impl Default for NRRANContainer {
        fn default() -> NRRANContainer {
            NRRANContainer {
                extension_header_type: NR_RAN_CONTAINER,
                length:0x00,
                nr_ran_container:vec!(),
            }
        }
    }
    
    impl ExtensionHeaders for NRRANContainer {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.extend(&self.nr_ran_container);
        }
    
        fn unmarshal(buffer: &[u8]) -> NRRANContainer {
            let mut data = NRRANContainer::default();
            data.length = buffer[0];
            for i in 1..(data.length*4-2) as usize {
                data.nr_ran_container.push(buffer[i]);
            }
            data
        }
    
        fn len (&self) -> usize {
            *&self.length as usize
        }
    }
    
    // Struct for PDU Session Container
    
    #[derive(Clone, Debug)]
    pub struct PDUSessionContainer {
        pub extension_header_type: u8,
        pub length: u8,
        pub pdu_session_container: Vec<u8>,
    }
    
    impl Default for PDUSessionContainer {
        fn default() -> PDUSessionContainer {
            PDUSessionContainer {
                extension_header_type: PDU_SESSION_CONTAINER,
                length:0x00,
                pdu_session_container:vec!(),
            }
        }
    }
    
    impl ExtensionHeaders for PDUSessionContainer {
        fn marshal(&self, buffer: &mut Vec<u8>) {
            buffer.push(self.extension_header_type);
            buffer.push(self.length);
            buffer.extend(&self.pdu_session_container);
        }
    
        fn unmarshal(buffer: &[u8]) -> PDUSessionContainer {
            let mut data = PDUSessionContainer::default();
            data.length = buffer[0];
            for i in 1..(data.length*4-2) as usize {
                data.pdu_session_container.push(buffer[i]);
            }
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
            LongPDCPPDUNumber(LongPDCPPDUNumber),
            ServiceClassIndicator(ServiceClassIndicator),
            UDPPort(UDPPort),
            RANContainer(RANContainer),
            XwRANContainer(XwRANContainer),
            NRRANContainer(NRRANContainer),
            PDUSessionContainer(PDUSessionContainer),
            PDCPPDUNumber(PDCPPDUNumber),
        }
    
        impl NextExtensionHeaderField {
            
            pub fn parse (v:&[u8]) -> NextExtensionHeaderField {
                match v[0] {
                    NO_MORE_EXTENSION_HEADERS => NextExtensionHeaderField::NoMoreExtensionHeaders,
                    LONG_PDCP_PDU_NUMBER_R16 => NextExtensionHeaderField::LongPDCPPDUNumber(LongPDCPPDUNumber::unmarshal(&v[1..])),
                    SERVICE_CLASS_INDICATOR => NextExtensionHeaderField::ServiceClassIndicator(ServiceClassIndicator::unmarshal(&v[1..])),
                    UDP_PORT => NextExtensionHeaderField::UDPPort(UDPPort::unmarshal(&v[1..])),                   
                    RAN_CONTAINER => NextExtensionHeaderField::RANContainer(RANContainer::unmarshal(&v[1..])),
                    LONG_PDCP_PDU_NUMBER_R15 => NextExtensionHeaderField::LongPDCPPDUNumber(LongPDCPPDUNumber::unmarshal(&v[1..])),
                    XW_RAN_CONTAINER => NextExtensionHeaderField::XwRANContainer(XwRANContainer::unmarshal(&v[1..])),
                    NR_RAN_CONTAINER => NextExtensionHeaderField::NRRANContainer(NRRANContainer::unmarshal(&v[1..])),
                    PDU_SESSION_CONTAINER => NextExtensionHeaderField::PDUSessionContainer(PDUSessionContainer::unmarshal(&v[1..])),
                    PDCP_PDU_NUMBER => NextExtensionHeaderField::PDCPPDUNumber(PDCPPDUNumber::unmarshal(&v[1..])),
                    _ => return NextExtensionHeaderField::Reserved,
    
                }
            }

            pub fn marshal (self, buffer: &mut Vec<u8>) {
                match self {
                    NextExtensionHeaderField::NoMoreExtensionHeaders => buffer.push (NO_MORE_EXTENSION_HEADERS),
                    NextExtensionHeaderField::Reserved => (),
                    _ => self.marshal(buffer),
                }
            }
    
            // return length in bytes, as encoded is the length is 4*bytes
    
            pub fn len (&self) -> usize {
                match *self {
                    NextExtensionHeaderField::NoMoreExtensionHeaders => 1,
                    NextExtensionHeaderField::Reserved => 1,
                    NextExtensionHeaderField::LongPDCPPDUNumber(LongPDCPPDUNumber{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::ServiceClassIndicator(ServiceClassIndicator{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::UDPPort(UDPPort{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::RANContainer(RANContainer{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::XwRANContainer(XwRANContainer{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::NRRANContainer(NRRANContainer{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::PDUSessionContainer(PDUSessionContainer{length, ..}) => (length as usize)*4,
                    NextExtensionHeaderField::PDCPPDUNumber(PDCPPDUNumber{length, ..}) => (length as usize)*4,
                }
            }
        }
    