// GTP-C Extension Headers Commons

    // Extension Header Common Field Values

    pub const DEFAULT:u16 = 0xffff;

    // Extension Header Common Type

    pub const NO_MORE_EXTENSION_HEADERS:u8 = 0;
  
    // Common trait
    
    pub trait ExtensionHeaders {
        fn marshal (&self, buffer: &mut Vec<u8>);
        fn unmarshal (buffer:&[u8]) -> Self;
        fn len (&self) -> usize;
    }

    