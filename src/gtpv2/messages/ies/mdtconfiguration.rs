// MDT Configuration IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MDT Configuration IE IE Type

pub const MDTCONFIG: u8 = 162;

// MDT Job Type Enum
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum MdtJobType {
    #[default]
    ImmediateMdtonly,
    LoggedMdtonly,
    TraceOnly,
    ImmediateMdtandTrace,
    RlfReportsonly,
    RcefReportsonly,
    LoggedMbsfnMdt,
}

impl From<&MdtJobType> for u8 {
    fn from(i: &MdtJobType) -> u8 {
        match i {
            MdtJobType::ImmediateMdtonly => 0,
            MdtJobType::LoggedMdtonly => 1,
            MdtJobType::TraceOnly => 2,
            MdtJobType::ImmediateMdtandTrace => 3,
            MdtJobType::RlfReportsonly => 4,
            MdtJobType::RcefReportsonly => 5,
            MdtJobType::LoggedMbsfnMdt => 6,
        }
    }
}

impl From<u8> for MdtJobType {
    fn from(i: u8) -> MdtJobType {
        match i {
            0 => MdtJobType::ImmediateMdtonly,
            1 => MdtJobType::LoggedMdtonly,
            2 => MdtJobType::TraceOnly,
            3 => MdtJobType::ImmediateMdtandTrace,
            4 => MdtJobType::RlfReportsonly,
            5 => MdtJobType::RcefReportsonly,
            6 => MdtJobType::LoggedMbsfnMdt,
            _ => MdtJobType::default(),
        }
    }
}

// MDT PLMN implementation

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MdtPlmn {
    pub mcc: u16,
    pub mnc: u16,
    pub mnc_is_three_digits: bool,
}

// MDT Configuration IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdtConfiguration {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub mdt_job_type: MdtJobType,
    pub list_measurements: [u8; 4],
    pub reporting_trigger: u8,
    pub report_interval: u8,
    pub report_amount: u8,
    pub rsrp_event_threshold: u8,
    pub rsrq_event_threshold: u8,
    pub area_scope: Option<Vec<u8>>,
    pub collection_period_rrm: Option<u8>, // Collection period for RRM measurements LTE
    pub measurement_period_lte: Option<u8>, // Measurement period for LTE measurements
    pub positioning_method: Option<u8>,    // Positioning method
    pub mdt_plmns_list: Option<Vec<MdtPlmn>>,
}

impl Default for MdtConfiguration {
    fn default() -> Self {
        MdtConfiguration {
            t: MDTCONFIG,
            length: 0,
            ins: 0,
            mdt_job_type: MdtJobType::default(),
            list_measurements: [0; 4],
            reporting_trigger: 0,
            report_interval: 0,
            report_amount: 0,
            rsrp_event_threshold: 0,
            rsrq_event_threshold: 0,
            area_scope: None,
            collection_period_rrm: None,
            measurement_period_lte: None,
            positioning_method: None,
            mdt_plmns_list: None,
        }
    }
}

impl From<MdtConfiguration> for InformationElement {
    fn from(i: MdtConfiguration) -> Self {
        InformationElement::MdtConfiguration(i)
    }
}

impl IEs for MdtConfiguration {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MDTCONFIG);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.mdt_job_type));
        buffer_ie.extend_from_slice(&self.list_measurements);
        buffer_ie.push(self.reporting_trigger);
        buffer_ie.push(self.report_interval);
        buffer_ie.push(self.report_amount);
        buffer_ie.push(self.rsrp_event_threshold);
        buffer_ie.push(self.rsrq_event_threshold);
        if let Some(i) = self.area_scope.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        {
            let mut i = if self.collection_period_rrm.is_some() {
                0b00000001
            } else {
                0
            };
            if self.measurement_period_lte.is_some() {
                i |= 0b00000010;
            }
            if self.positioning_method.is_some() {
                i |= 0b00000100;
            }
            if self.mdt_plmns_list.is_some() {
                i |= 0b00001000;
            }
            buffer_ie.push(i);
        }
        if let Some(i) = self.collection_period_rrm {
            buffer_ie.push(i);
        }
        if let Some(i) = self.measurement_period_lte {
            buffer_ie.push(i);
        }
        if let Some(i) = self.positioning_method {
            buffer_ie.push(i);
        }
        if let Some(i) = &self.mdt_plmns_list {
            buffer_ie.push(i.len() as u8);
            for plmn in i {
                buffer_ie.append(&mut mcc_mnc_encode(
                    plmn.mcc,
                    plmn.mnc,
                    plmn.mnc_is_three_digits,
                ));
            }
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 11 + MIN_IE_SIZE {
            let mut data = MdtConfiguration {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                mdt_job_type: MdtJobType::from(buffer[4]),
                list_measurements: [buffer[5], buffer[6], buffer[7], buffer[8]],
                reporting_trigger: buffer[9],
                report_interval: buffer[10],
                report_amount: buffer[11],
                rsrp_event_threshold: buffer[12],
                rsrq_event_threshold: buffer[13],
                ..MdtConfiguration::default()
            };
            let mut cursor: usize = 14;
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                match len {
                    0 => (),
                    _ => {
                        data.area_scope = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    }
                }
            }
            {
                let pli = matches!(buffer[cursor] >> 3, 0x01);
                let pmi = matches!(buffer[cursor] >> 2 & 0x01, 0x01);
                let mpi = matches!(buffer[cursor] >> 1 & 0x01, 0x01);
                let crrmi = matches!(buffer[cursor] & 0x01, 0x01);
                cursor += 1;
                if crrmi && buffer.len() > cursor {
                    data.collection_period_rrm = Some(buffer[cursor]);
                    cursor += 1;
                }
                if mpi && buffer.len() > cursor {
                    data.measurement_period_lte = Some(buffer[cursor]);
                    cursor += 1;
                }
                if pmi && buffer.len() > cursor {
                    data.positioning_method = Some(buffer[cursor]);
                    cursor += 1;
                }
                if pli {
                    let number_plmn = if buffer[cursor] < 16 {
                        buffer[cursor]
                    } else {
                        return Err(GTPV2Error::IEIncorrect(MDTCONFIG));
                    };
                    cursor += 1;
                    if buffer.len() >= cursor + number_plmn as usize * 3 {
                        let mut plmns = Vec::new();
                        while cursor < buffer.len() {
                            if buffer.len() >= cursor + 3 {
                                let (mcc, mnc, mnc_is_three_digits) =
                                    mcc_mnc_decode(&buffer[cursor..cursor + 3]);
                                plmns.push(MdtPlmn {
                                    mcc,
                                    mnc,
                                    mnc_is_three_digits,
                                });
                                cursor += 3;
                            } else {
                                return Err(GTPV2Error::IEInvalidLength(MDTCONFIG));
                            }
                        }
                        data.mdt_plmns_list = Some(plmns);
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MDTCONFIG));
                    }
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MDTCONFIG))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn mdtconfig_ie_marshal_test() {
    let encoded: [u8; 36] = [
        0xa2, 0x00, 0x20, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x04,
        0x01, 0x02, 0x03, 0x04, 0x0f, 0x01, 0x02, 0x03, 0x04, 0x32, 0xf4, 0x10, 0x22, 0xf0, 0x50,
        0x62, 0xf5, 0x30, 0x31, 0xf4, 0x60,
    ];
    let decoded = MdtConfiguration {
        length: 32,
        mdt_job_type: MdtJobType::ImmediateMdtonly,
        list_measurements: [0x01, 0x02, 0x03, 0x04],
        reporting_trigger: 0x05,
        report_interval: 0x06,
        report_amount: 0x07,
        rsrp_event_threshold: 0x08,
        rsrq_event_threshold: 0x09,
        area_scope: Some(vec![0x01, 0x02, 0x03, 0x04]),
        collection_period_rrm: Some(0x01),
        measurement_period_lte: Some(0x02),
        positioning_method: Some(0x03),
        mdt_plmns_list: Some(vec![
            MdtPlmn {
                mcc: 234,
                mnc: 1,
                mnc_is_three_digits: false,
            },
            MdtPlmn {
                mcc: 220,
                mnc: 5,
                mnc_is_three_digits: false,
            },
            MdtPlmn {
                mcc: 265,
                mnc: 3,
                mnc_is_three_digits: false,
            },
            MdtPlmn {
                mcc: 134,
                mnc: 6,
                mnc_is_three_digits: false,
            },
        ]),
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn mtdconfig_ie_unmarshal_test() {
    let encoded: [u8; 36] = [
        0xa2, 0x00, 0x20, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x04,
        0x01, 0x02, 0x03, 0x04, 0x0f, 0x01, 0x02, 0x03, 0x04, 0x32, 0xf4, 0x10, 0x22, 0xf0, 0x50,
        0x62, 0xf5, 0x30, 0x31, 0xf4, 0x60,
    ];
    let decoded = MdtConfiguration {
        length: 32,
        mdt_job_type: MdtJobType::ImmediateMdtonly,
        list_measurements: [0x01, 0x02, 0x03, 0x04],
        reporting_trigger: 0x05,
        report_interval: 0x06,
        report_amount: 0x07,
        rsrp_event_threshold: 0x08,
        rsrq_event_threshold: 0x09,
        area_scope: Some(vec![0x01, 0x02, 0x03, 0x04]),
        collection_period_rrm: Some(0x01),
        measurement_period_lte: Some(0x02),
        positioning_method: Some(0x03),
        mdt_plmns_list: Some(vec![
            MdtPlmn {
                mcc: 234,
                mnc: 1,
                mnc_is_three_digits: false,
            },
            MdtPlmn {
                mcc: 220,
                mnc: 5,
                mnc_is_three_digits: false,
            },
            MdtPlmn {
                mcc: 265,
                mnc: 3,
                mnc_is_three_digits: false,
            },
            MdtPlmn {
                mcc: 134,
                mnc: 6,
                mnc_is_three_digits: false,
            },
        ]),
        ..Default::default()
    };
    assert_eq!(MdtConfiguration::unmarshal(&encoded).unwrap(), decoded);
}
