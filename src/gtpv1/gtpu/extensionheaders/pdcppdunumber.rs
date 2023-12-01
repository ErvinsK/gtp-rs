use crate::gtpv1::{errors::GTPV1Error, gtpu::extensionheaders::commons::*};

pub const PDCP_PDU_NUMBER: u8 = 0xc0;
pub const PDCP_PDU_NUMBER_LENGTH: u8 = 1;

// Struct for PDCP PDU Number Extension Header

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PDCPPDUNumber {
    pub extension_header_type: u8,
    pub length: u8,
    pub pdcp_pdu_number: u16,
}

impl Default for PDCPPDUNumber {
    fn default() -> PDCPPDUNumber {
        PDCPPDUNumber {
            extension_header_type: PDCP_PDU_NUMBER,
            length: PDCP_PDU_NUMBER_LENGTH,
            pdcp_pdu_number: 0,
        }
    }
}

impl ExtensionHeaders for PDCPPDUNumber {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.extend_from_slice(&self.pdcp_pdu_number.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = PDCPPDUNumber {
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.pdcp_pdu_number = u16::from_be_bytes([buffer[2], buffer[3]]);
            Ok(data)
        } else {
            Err(GTPV1Error::ExtHeaderInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length * 4) as usize
    }

    fn is_empty(&self) -> bool {
        self.pdcp_pdu_number == 0
    }
}

#[test]
fn pdcp_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xc0, 0x01, 0x10, 0x00];
    let test_struct = PDCPPDUNumber {
        extension_header_type: PDCP_PDU_NUMBER,
        length: PDCP_PDU_NUMBER_LENGTH,
        pdcp_pdu_number: 4096,
    };
    let i = PDCPPDUNumber::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn pdcp_exthdr_marshal_test() {
    let encoded_ie: [u8; 4] = [0xc0, 0x01, 0x10, 0x00];
    let test_struct = PDCPPDUNumber {
        extension_header_type: PDCP_PDU_NUMBER,
        length: PDCP_PDU_NUMBER_LENGTH,
        pdcp_pdu_number: 4096,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
