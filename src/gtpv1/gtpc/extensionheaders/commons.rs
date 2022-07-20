use crate::gtpv1::gtpc::extensionheaders::*;


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