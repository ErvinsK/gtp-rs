// APN Rate Control Status IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*, utils::*};

// APN Rate Control Status IE TL

pub const APN_RATE_CNTRL: u8 = 204;
pub const APN_RATE_CNTR_LENGTH: usize = 20;

// APN Rate Control Status IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApnRateControlStatus {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ul_packets_allowed: u32,
    pub nmbr_add_exception_reports: u32,
    pub dl_packets_allowed: u32,
    pub validity_time: u64,
}

impl Default for ApnRateControlStatus {
    fn default() -> Self {
        ApnRateControlStatus {
            t: APN_RATE_CNTRL,
            length: APN_RATE_CNTR_LENGTH as u16,
            ins: 0,
            ul_packets_allowed: 0,
            nmbr_add_exception_reports: 0,
            dl_packets_allowed: 0,
            validity_time: 0,
        }
    }
}

impl From<ApnRateControlStatus> for InformationElement {
    fn from(i: ApnRateControlStatus) -> Self {
        InformationElement::ApnRateControlStatus(i)
    }
}

impl IEs for ApnRateControlStatus {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(APN_RATE_CNTRL);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.ul_packets_allowed.to_be_bytes());
        buffer_ie.extend_from_slice(&self.nmbr_add_exception_reports.to_be_bytes());
        buffer_ie.extend_from_slice(&self.dl_packets_allowed.to_be_bytes());
        buffer_ie.extend_from_slice(&self.validity_time.to_be_bytes());
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= APN_RATE_CNTR_LENGTH + MIN_IE_SIZE {
            let data = ApnRateControlStatus {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ul_packets_allowed: u32::from_be_bytes([
                    buffer[4], buffer[5], buffer[6], buffer[7],
                ]),
                nmbr_add_exception_reports: u32::from_be_bytes([
                    buffer[8], buffer[9], buffer[10], buffer[11],
                ]),
                dl_packets_allowed: u32::from_be_bytes([
                    buffer[12], buffer[13], buffer[14], buffer[15],
                ]),
                validity_time: u64::from_slice(&buffer[16..24]),
                ..ApnRateControlStatus::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(APN_RATE_CNTRL))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
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
fn apn_rate_control_status_ie_unmarshal_test() {
    let encoded: [u8; 24] = [
        0xcc, 0x00, 0x14, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xaa, 0xaa, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x4e, 0x4e,
    ];
    let decoded = ApnRateControlStatus {
        t: APN_RATE_CNTRL,
        length: APN_RATE_CNTR_LENGTH as u16,
        ins: 0,
        ul_packets_allowed: 0xffff,
        nmbr_add_exception_reports: 0xaaaa,
        dl_packets_allowed: 0xffff,
        validity_time: 0x4e4e4e,
    };
    let i = ApnRateControlStatus::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn apn_rate_control_status_ie_marshal_test() {
    let encoded: [u8; 24] = [
        0xcc, 0x00, 0x14, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xaa, 0xaa, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x4e, 0x4e,
    ];
    let decoded = ApnRateControlStatus {
        t: APN_RATE_CNTRL,
        length: APN_RATE_CNTR_LENGTH as u16,
        ins: 0,
        ul_packets_allowed: 0xffff,
        nmbr_add_exception_reports: 0xaaaa,
        dl_packets_allowed: 0xffff,
        validity_time: 0x4e4e4e,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
