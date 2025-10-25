// Trace Reference IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*};

// Trace Reference IE TL

pub const TRACE_REFERENCE: u8 = 27;
pub const TRACE_REFERENCE_LENGTH: usize = 2;

// Trace Reference IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct TraceReference {
    pub t: u8,
    pub value: u16,
}

impl Default for TraceReference {
    fn default() -> Self {
        TraceReference {
            t: TRACE_REFERENCE,
            value: 0,
        }
    }
}

impl IEs for TraceReference {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.t);
        buffer.extend_from_slice(&self.value.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() > TRACE_REFERENCE_LENGTH {
            let data = TraceReference {
                value: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        TRACE_REFERENCE_LENGTH + 1
    }
    fn is_empty(&self) -> bool {
        false
    }
}

#[test]
fn trace_reference_ie_marshal_test() {
    let ie_to_marshal = TraceReference {
        t: TRACE_REFERENCE,
        value: 1010,
    };
    let ie_marshalled: [u8; 3] = [0x1b, 0x03, 0xf2];
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn trace_reference_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 3] = [0x1b, 0x03, 0xf2];
    let ie_unmarshalled = TraceReference {
        t: TRACE_REFERENCE,
        value: 1010,
    };
    assert_eq!(
        TraceReference::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
