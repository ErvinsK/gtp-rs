// Direct Tunnel Flags IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Direct Tunnel Flags IE Type

pub const DTF: u8 = 182;
pub const DTF_LENGTH: u16 = 1;

// Direct Tunnel Flags IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectTunnelFlags {
    pub t: u8,
    pub length: u16,
    pub dti: bool,  // Direct Tunnel Indicator (DTI)
    pub gcsi: bool, // The GPRS-CSI (GCSI)
    pub ei: bool,   // Error Indication (EI)
}

impl Default for DirectTunnelFlags {
    fn default() -> Self {
        DirectTunnelFlags {
            t: DTF,
            length: DTF_LENGTH,
            dti: false,
            gcsi: false,
            ei: false,
        }
    }
}

impl IEs for DirectTunnelFlags {
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
        let flags = from_bool(self.ei) << 2 | from_bool(self.gcsi) << 1 | from_bool(self.dti);
        buffer_ie.push(flags);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        if buffer.len() >= 4 {
            let mut data = DirectTunnelFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            let to_bool = |i: u8| -> bool { i == 1 };
            data.dti = to_bool(buffer[3] & 1);
            data.gcsi = to_bool((buffer[3] >> 1) & 1);
            data.ei = to_bool((buffer[3] >> 2) & 1);
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
fn dtf_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0xb6, 0x00, 0x01, 0x03];
    let ie_to_marshal = DirectTunnelFlags {
        t: DTF,
        length: DTF_LENGTH,
        dti: true,
        gcsi: true,
        ei: false,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn dtf_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0xb6, 0x00, 0x01, 0x03];
    let ie_unmarshalled = DirectTunnelFlags {
        t: DTF,
        length: DTF_LENGTH,
        dti: true,
        gcsi: true,
        ei: false,
    };
    assert_eq!(
        DirectTunnelFlags::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
