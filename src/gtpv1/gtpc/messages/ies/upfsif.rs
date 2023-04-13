// UP Function Selection Indication Flags (UPFSIF) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// UPFSIF IE TV

pub const UPFSIF: u8 = 224;
pub const UPFSIF_LENGTH: u16 = 1;

// UPFSIF IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpFunctionSelectionIndicationFlags {
    pub t: u8,
    pub length: u16,
    pub dcnr: bool, // DCNR - Dual Connectivity with NR
}

impl Default for UpFunctionSelectionIndicationFlags {
    fn default() -> UpFunctionSelectionIndicationFlags {
        UpFunctionSelectionIndicationFlags {
            t: UPFSIF,
            length: UPFSIF_LENGTH,
            dcnr: false,
        }
    }
}

impl IEs for UpFunctionSelectionIndicationFlags {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        match self.dcnr {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= UPFSIF_LENGTH as usize + 3 {
            let data = UpFunctionSelectionIndicationFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                dcnr: match buffer[3] {
                    0 => false,
                    1 => true,
                    _ => return Err(GTPV1Error::IEIncorrect),
                },
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        UPFSIF_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn upfsif_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xe0, 0x00, 0x01, 0x01];
    let test_struct = UpFunctionSelectionIndicationFlags {
        t: UPFSIF,
        length: UPFSIF_LENGTH,
        dcnr: true,
    };
    let i = UpFunctionSelectionIndicationFlags::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn upfsif_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xe0, 0x00, 0x01, 0x01];
    let test_struct = UpFunctionSelectionIndicationFlags {
        t: UPFSIF,
        length: UPFSIF_LENGTH,
        dcnr: true,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
