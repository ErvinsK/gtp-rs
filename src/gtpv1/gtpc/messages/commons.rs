// According to 3GPP TS 29.060 V15.5.0 (2019-06) and 3GPP TS 29.281 V16.0.0 (2019-12)

// Definition of GTPv1 Messages

pub const ECHO_REQUEST:u8 = 1;
pub const ECHO_RESPONSE:u8 = 2;
pub const VERSION_NOT_SUPPORTED:u8 = 3;
pub const CREATE_PDP_CONTEXT_REQUEST:u8 = 16;
pub const CREATE_PDP_CONTEXT_RESPONSE:u8 = 17;
pub const UPDATE_PDP_CONTEXT_REQUEST:u8 = 18;
pub const UPDATE_PDP_CONTEXT_RESPONSE:u8 = 19;
pub const DELETE_PDP_CONTEXT_REQUEST:u8 = 20;
pub const DELETE_PDP_CONTEXT_RESPONSE:u8 = 21;
pub const ERROR_INDICATION:u8 = 26;
pub const SUPPORT_EXTENSION_HEADERS_NOTIFICATION:u8 = 31;
pub const END_MARKER:u8 = 254;
pub const G_PDU:u8 = 255;

// Common traits of GTPv1 Messages

pub trait Messages {
    fn marshal (self, buffer: &mut Vec<u8>);
    fn unmarshal (header:Gtpv1Header, buffer:&[u8]) -> Result<Self, GTPV1Error> where Self:Sized;
    //fn len (&self) -> usize;
}