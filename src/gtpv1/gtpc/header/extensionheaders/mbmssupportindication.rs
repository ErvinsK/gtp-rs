use crate::gtpv1::gtpc::header::extensionheaders::commons::*;

pub const MBMS_SUPPORT_INDICATION:u8 = 1;
pub const MBMS_SUPPORT_INDICATION_LENGTH:u8 = 1;

// Struct for MBMS Support Indication 

#[derive(Clone, Debug, PartialEq)]
pub struct MBMSSupportIndication {
    pub extension_header_type:u8,
    pub length:u8,
    pub value:u16,
}

impl Default for MBMSSupportIndication {
    fn default() -> MBMSSupportIndication {
        MBMSSupportIndication {
            extension_header_type:MBMS_SUPPORT_INDICATION,
            length:MBMS_SUPPORT_INDICATION_LENGTH,
            value:DEFAULT,
        }
    }
}

impl ExtensionHeaders for MBMSSupportIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.extend_from_slice(&self.value.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> MBMSSupportIndication {
        let mut data = MBMSSupportIndication::default();
        data.value = u16::from_be_bytes([buffer[2],buffer [3]]);
        data
    }

    fn len (&self) -> usize {
        (self.length*4) as usize
    }
}

#[test]
fn mbmssupport_ind_exthdr_unmarshal_test () {
    let encoded_ie:[u8;4]=[0x01, 0x01, 0xff, 0xff];
    let test_struct = MBMSSupportIndication { extension_header_type:MBMS_SUPPORT_INDICATION, length: MBMS_SUPPORT_INDICATION_LENGTH, value: DEFAULT };
    let i = MBMSSupportIndication::unmarshal(&encoded_ie);
    assert_eq!(i, test_struct);
}

#[test]
fn mbmssupport_ind_exthdr_marshal_test () {
    let encoded_ie:[u8;4]=[0x01, 0x01, 0xff, 0xff];
    let test_struct = MBMSSupportIndication { extension_header_type:MBMS_SUPPORT_INDICATION, length: MBMS_SUPPORT_INDICATION_LENGTH, value: DEFAULT };
    let mut buffer:Vec<u8>=vec!();
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}