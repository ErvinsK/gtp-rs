// Extended Common Flags II IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Extended Common Flags II IE Type

pub const EXTCOMMONFLAGS_II: u8 = 218;
pub const EXTCOMMONFLAGS_II_LENGTH: u16 = 1;

// Extended Common Flags II IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedCommonFlagsII {
    pub t: u8,
    pub length: u16,
    pub pnsi: bool,   // Pending Network Initiated PDN Connection Signalling Indication
    pub dtci: bool,   // Delay Tolerant Connection Indication
    pub pmtsmi: bool, // Pending MT Short Message Indication
}

impl Default for ExtendedCommonFlagsII {
    fn default() -> Self {
        ExtendedCommonFlagsII {
            t: EXTCOMMONFLAGS_II,
            length: EXTCOMMONFLAGS_II_LENGTH,
            pnsi: false,
            dtci: false,
            pmtsmi: false,
        }
    }
}

impl IEs for ExtendedCommonFlagsII {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        let from_bool = |i: bool| -> u8 {
            if i {
                1
            } else {
                0
            }
        };
        let flags = from_bool(self.pmtsmi) << 2 | from_bool(self.dtci) << 1 | from_bool(self.pnsi);
        buffer_ie.push(flags);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 4 {
            let mut data = ExtendedCommonFlagsII {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            let to_bool = |i: u8| -> bool { i == 1 };
            data.pnsi = to_bool(buffer[3] & 1);
            data.dtci = to_bool((buffer[3] >> 1) & 1);
            data.pmtsmi = to_bool((buffer[3] >> 2) & 1);
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length + 3) as usize
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn extcommonflagsii_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0xda, 0x00, 0x01, 0x03];
    let ie_to_marshal = ExtendedCommonFlagsII {
        t: EXTCOMMONFLAGS_II,
        length: EXTCOMMONFLAGS_II_LENGTH,
        pnsi: true,
        dtci: true,
        pmtsmi: false,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn extcommonflagsii_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0xda, 0x00, 0x01, 0x03];
    let ie_unmarshalled = ExtendedCommonFlagsII {
        t: EXTCOMMONFLAGS_II,
        length: EXTCOMMONFLAGS_II_LENGTH,
        pnsi: true,
        dtci: true,
        pmtsmi: false,
    };
    assert_eq!(
        ExtendedCommonFlagsII::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
