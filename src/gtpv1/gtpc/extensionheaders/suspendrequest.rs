use crate::gtpv1::gtpc::extensionheaders::commons::*;

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