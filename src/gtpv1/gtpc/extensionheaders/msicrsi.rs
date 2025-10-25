use crate::gtpv1::{errors::GTPV1Error, gtpc::extensionheaders::commons::*};

pub const MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION: u8 = 2;
pub const MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION_LENGTH: u8 = 1;

// Struct for MS Info Change Reporting Support Indication

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSInfoChangeReportingSupportIndication {
    pub extension_header_type: u8,
    pub length: u8,
    pub value: u16,
}

impl Default for MSInfoChangeReportingSupportIndication {
    fn default() -> MSInfoChangeReportingSupportIndication {
        MSInfoChangeReportingSupportIndication {
            extension_header_type: MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION,
            length: MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION_LENGTH,
            value: DEFAULT,
        }
    }
}

impl ExtensionHeaders for MSInfoChangeReportingSupportIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.extend_from_slice(&self.value.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = MSInfoChangeReportingSupportIndication {
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.value = u16::from_be_bytes([buffer[2], buffer[3]]);
            Ok(data)
        } else {
            Err(GTPV1Error::ExtHeaderInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length * 4) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn msicrsi_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0x02, 0x01, 0xff, 0xff];
    let test_struct = MSInfoChangeReportingSupportIndication {
        extension_header_type: MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION,
        length: MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION_LENGTH,
        value: DEFAULT,
    };
    let i = MSInfoChangeReportingSupportIndication::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn msicrsi_exthdr_marshal_test() {
    let encoded_ie: [u8; 4] = [0x02, 0x01, 0xff, 0xff];
    let test_struct = MSInfoChangeReportingSupportIndication {
        extension_header_type: MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION,
        length: MS_INFO_CHANGE_REPORTING_SUPPORT_INDICATION_LENGTH,
        value: DEFAULT,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
