use crate::gtpv1::gtpc::extensionheaders::commons::*;

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