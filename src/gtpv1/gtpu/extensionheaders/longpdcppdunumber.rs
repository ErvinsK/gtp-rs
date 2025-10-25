use crate::gtpv1::{errors::GTPV1Error, gtpu::extensionheaders::commons::*};

pub const LONG_PDCP_PDU_NUMBER_I: u8 = 0x03;
pub const LONG_PDCP_PDU_NUMBER_II: u8 = 0x82;
pub const LONG_PDCP_PDU_NUMBER_LENGTH: u8 = 2;

// Struct for Long PDCP PDU Number Extension Header

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongPDCPPDUNumber {
    pub extension_header_type: u8,
    pub length: u8,
    pub long_pdcp_pdu_number: u32,
}

impl Default for LongPDCPPDUNumber {
    fn default() -> LongPDCPPDUNumber {
        LongPDCPPDUNumber {
            extension_header_type: LONG_PDCP_PDU_NUMBER_I,
            length: LONG_PDCP_PDU_NUMBER_LENGTH,
            long_pdcp_pdu_number: 0,
        }
    }
}

impl ExtensionHeaders for LongPDCPPDUNumber {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.extend_from_slice(&(self.long_pdcp_pdu_number << 8).to_be_bytes());
        buffer.push(0x00);
        buffer.push(0x00);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = LongPDCPPDUNumber {
            extension_header_type: match buffer[0] {
                LONG_PDCP_PDU_NUMBER_I => buffer[0],
                LONG_PDCP_PDU_NUMBER_II => buffer[0],
                _ => return Err(GTPV1Error::ExtHeaderUnknown),
            },
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.long_pdcp_pdu_number = u32::from_be_bytes([0x0, buffer[2], buffer[3], buffer[4]]);
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
fn long_pdcp_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 8] = [0x03, 0x02, 0x03, 0xff, 0xff, 0x00, 0x00, 0x00];
    let test_struct = LongPDCPPDUNumber {
        extension_header_type: LONG_PDCP_PDU_NUMBER_I,
        length: LONG_PDCP_PDU_NUMBER_LENGTH,
        long_pdcp_pdu_number: 0x3ffff,
    };
    let i = LongPDCPPDUNumber::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn long_pdcp_exthdr_marshal_test() {
    let encoded_ie: [u8; 8] = [0x03, 0x02, 0x03, 0xff, 0xff, 0x00, 0x00, 0x00];
    let test_struct = LongPDCPPDUNumber {
        extension_header_type: LONG_PDCP_PDU_NUMBER_I,
        length: LONG_PDCP_PDU_NUMBER_LENGTH,
        long_pdcp_pdu_number: 0x3ffff,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
