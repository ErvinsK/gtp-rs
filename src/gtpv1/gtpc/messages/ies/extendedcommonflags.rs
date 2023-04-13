// Extended Common Flags IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// Extended Common Flags IE Type

pub const EXTCOMMONFLAGS: u8 = 193;
pub const EXTCOMMONFLAGS_LENGTH: u16 = 1;

// Extended Common Flags IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedCommonFlags {
    pub t: u8,
    pub length: u16,
    pub unauthenticated_imsi: bool,
    pub ccrsi: bool,  // CSG Change Reporting Support Indication
    pub cpsr: bool,   // CS to PS SRVCC
    pub retloc: bool, // Retrieve Location
    pub vb: bool,     // Voice Bearer
    pub pcri: bool,   // P-CSCF Restoration Indication
    pub bdwi: bool,   // Buffered DL Data Waiting Indication
    pub uasi: bool,   // UE available for Signalling Indication
}

impl Default for ExtendedCommonFlags {
    fn default() -> Self {
        ExtendedCommonFlags {
            t: EXTCOMMONFLAGS,
            length: EXTCOMMONFLAGS_LENGTH,
            unauthenticated_imsi: false,
            ccrsi: false,
            cpsr: false,
            retloc: false,
            vb: false,
            pcri: false,
            bdwi: false,
            uasi: false,
        }
    }
}

impl IEs for ExtendedCommonFlags {
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
            let mut data = ExtendedCommonFlags {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            let flags = [buffer[3]; 8]
                .iter()
                .enumerate()
                .map(|(i, x)| (*x & (0b10000000 >> i)) >> (7 - i))
                .collect::<Vec<u8>>();
            let to_bool = |i: u8| -> bool { i == 1 };
            data.uasi = to_bool(flags[0]);
            data.bdwi = to_bool(flags[1]);
            data.pcri = to_bool(flags[2]);
            data.vb = to_bool(flags[3]);
            data.retloc = to_bool(flags[4]);
            data.cpsr = to_bool(flags[5]);
            data.ccrsi = to_bool(flags[6]);
            data.unauthenticated_imsi = to_bool(flags[7]);
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

impl ExtendedCommonFlags {
    fn into_array(self) -> [bool; 8] {
        [
            self.uasi,
            self.bdwi,
            self.pcri,
            self.vb,
            self.retloc,
            self.cpsr,
            self.ccrsi,
            self.unauthenticated_imsi,
        ]
    }
}

#[test]
fn extcommonflags_ie_marshal_test() {
    let ie_marshalled: [u8; 4] = [0xc1, 0x00, 0x01, 0x61];
    let ie_to_marshal = ExtendedCommonFlags {
        t: EXTCOMMONFLAGS,
        length: EXTCOMMONFLAGS_LENGTH,
        uasi: false,
        bdwi: true,
        pcri: true,
        vb: false,
        retloc: false,
        cpsr: false,
        ccrsi: false,
        unauthenticated_imsi: true,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn extcommonflags_ie_unmarshal_test() {
    let ie_to_unmarshal: [u8; 4] = [0xc1, 0x00, 0x01, 0x61];
    let ie_unmarshalled = ExtendedCommonFlags {
        t: EXTCOMMONFLAGS,
        length: EXTCOMMONFLAGS_LENGTH,
        uasi: false,
        bdwi: true,
        pcri: true,
        vb: false,
        retloc: false,
        cpsr: false,
        ccrsi: false,
        unauthenticated_imsi: true,
    };
    assert_eq!(
        ExtendedCommonFlags::unmarshal(&ie_to_unmarshal).unwrap(),
        ie_unmarshalled
    );
}
