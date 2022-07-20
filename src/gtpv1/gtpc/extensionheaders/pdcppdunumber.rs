use crate::gtpv1::gtpc::extensionheaders::commons::*;

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