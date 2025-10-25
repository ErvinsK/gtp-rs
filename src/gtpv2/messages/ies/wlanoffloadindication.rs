// WLAN Offloadability Indication IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// WLAN Offloadability Indication IE TV

pub const WLAN_OFFLOAD_IND: u8 = 185;
pub const WLAN_OFFLOAD_IND_LENGTH: usize = 1;

// WLAN Offloadability Indication IE implementation

// EUTRAN Indication when set to '1' (true), this indicates that the UE has been authorized to perform WLAN offload from E-UTRAN.
// When set to '0' (false), this indicates that the UE has not been authorized to perform WLAN offload from E-UTRAN.
// UTRAN Indication when set to '1', this indicates that the UE has been authorized to perform WLAN offload from UTRAN.
// When set to '0', this indicates that the UE has not been authorized to perform WLAN offload from UTRAN.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WlanOffloadIndication {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub eutran_ind: bool,
    pub utran_ind: bool,
}

impl Default for WlanOffloadIndication {
    fn default() -> Self {
        WlanOffloadIndication {
            t: WLAN_OFFLOAD_IND,
            length: WLAN_OFFLOAD_IND_LENGTH as u16,
            ins: 0,
            eutran_ind: false,
            utran_ind: false,
        }
    }
}

impl From<WlanOffloadIndication> for InformationElement {
    fn from(i: WlanOffloadIndication) -> Self {
        InformationElement::WlanOffloadIndication(i)
    }
}

impl IEs for WlanOffloadIndication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(WLAN_OFFLOAD_IND);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match (self.eutran_ind, self.utran_ind) {
            (false, false) => buffer_ie.push(0x00),
            (false, true) => buffer_ie.push(0x01),
            (true, false) => buffer_ie.push(0x02),
            (true, true) => buffer_ie.push(0x03),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= WLAN_OFFLOAD_IND_LENGTH + MIN_IE_SIZE {
            let mut data = WlanOffloadIndication {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..WlanOffloadIndication::default()
            };
            match buffer[4] & 0x03 {
                0 => (data.eutran_ind, data.utran_ind) = (false, false),
                1 => (data.eutran_ind, data.utran_ind) = (false, true),
                2 => (data.eutran_ind, data.utran_ind) = (true, false),
                3 => (data.eutran_ind, data.utran_ind) = (true, true),
                _ => return Err(GTPV2Error::IEIncorrect(WLAN_OFFLOAD_IND)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(WLAN_OFFLOAD_IND))
        }
    }

    fn len(&self) -> usize {
        WLAN_OFFLOAD_IND_LENGTH + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn wlan_offload_ind_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0xb9, 0x00, 0x01, 0x00, 0x02];
    let decoded = WlanOffloadIndication {
        t: WLAN_OFFLOAD_IND,
        length: WLAN_OFFLOAD_IND_LENGTH as u16,
        ins: 0,
        eutran_ind: true,
        utran_ind: false,
    };
    let i = WlanOffloadIndication::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn wlan_offload_ind_ie_marshal_test() {
    let encoded: [u8; 5] = [0xb9, 0x00, 0x01, 0x00, 0x02];
    let decoded = WlanOffloadIndication {
        t: WLAN_OFFLOAD_IND,
        length: WLAN_OFFLOAD_IND_LENGTH as u16,
        ins: 0,
        eutran_ind: true,
        utran_ind: false,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
