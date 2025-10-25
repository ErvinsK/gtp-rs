// Common Flags IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Common Flags IE Type

pub const COMMONFLAGS: u8 = 148;
pub const COMMONFLAGS_LENGTH: u16 = 1;

// Common Flags IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonFlags {
    pub t: u8,
    pub length: u16,
    pub dual_addr_bearer: bool,
    pub upgrade_qos_support: bool,
    pub nrsn: bool,
    pub no_qos_negotiation: bool,
    pub mbms_counting_info: bool,
    pub ran_procedures_ready: bool,
    pub mbms_service_type: bool,
    pub prohibit_payload_compr: bool,
}

impl Default for CommonFlags {
    fn default() -> Self {
        CommonFlags {
            t: COMMONFLAGS,
            length: COMMONFLAGS_LENGTH,
            dual_addr_bearer: false,
            upgrade_qos_support: false,
            nrsn: false,
            no_qos_negotiation: false,
            mbms_counting_info: false,
            ran_procedures_ready: false,
            mbms_service_type: false,
            prohibit_payload_compr: false,
        }
    }
}

impl IEs for CommonFlags {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        let flags = self
            .clone()
            .into_array()
            .iter()
            .map(|x| if *x { 1 } else { 0 })
            .enumerate()
            .map(|(i, x)| x << (7 - i))
            .collect::<Vec<_>>()
            .iter()
            .sum::<u8>();
        buffer_ie.push(flags);
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= 4 {
            let mut data = CommonFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            let flags = [buffer[3]; 8]
                .iter()
                .enumerate()
                .map(|(i, x)| (*x & (0b10000000 >> i)) >> (7 - i))
                .collect::<Vec<u8>>();
            let to_bool = |i: u8| -> bool { i == 1 };
            data.dual_addr_bearer = to_bool(flags[0]);
            data.upgrade_qos_support = to_bool(flags[1]);
            data.nrsn = to_bool(flags[2]);
            data.no_qos_negotiation = to_bool(flags[3]);
            data.mbms_counting_info = to_bool(flags[4]);
            data.ran_procedures_ready = to_bool(flags[5]);
            data.mbms_service_type = to_bool(flags[6]);
            data.prohibit_payload_compr = to_bool(flags[7]);
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

impl CommonFlags {
    fn into_array(self) -> [bool; 8] {
        [
            self.dual_addr_bearer,
            self.upgrade_qos_support,
            self.nrsn,
            self.no_qos_negotiation,
            self.mbms_counting_info,
            self.ran_procedures_ready,
            self.mbms_service_type,
            self.prohibit_payload_compr,
        ]
    }
}

#[test]
fn commonflags_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0x94, 0x00, 0x01, 0x60];
    let ie_to_marshal = CommonFlags {
        t: COMMONFLAGS,
        length: COMMONFLAGS_LENGTH,
        dual_addr_bearer: false,
        upgrade_qos_support: true,
        nrsn: true,
        no_qos_negotiation: false,
        mbms_counting_info: false,
        ran_procedures_ready: false,
        mbms_service_type: false,
        prohibit_payload_compr: false,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn commonflags_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0x94, 0x00, 0x01, 0x60];
    let ie_unmarshalled = CommonFlags {
        t: COMMONFLAGS,
        length: COMMONFLAGS_LENGTH,
        dual_addr_bearer: false,
        upgrade_qos_support: true,
        nrsn: true,
        no_qos_negotiation: false,
        mbms_counting_info: false,
        ran_procedures_ready: false,
        mbms_service_type: false,
        prohibit_payload_compr: false,
    };
    assert_eq!(
        CommonFlags::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
