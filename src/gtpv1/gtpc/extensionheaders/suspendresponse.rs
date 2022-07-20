use crate::gtpv1::gtpc::extensionheaders::commons::*;

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