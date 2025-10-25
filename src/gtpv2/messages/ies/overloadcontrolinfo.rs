// Overload Control IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Overload Control IE Type

pub const OVERLOAD_CNTRL: u8 = 180;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverloadControlInfo {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub sqn: Sqn,
    pub metric: Metric,
    pub validity: EpcTimer,
    pub list: Option<Vec<Apn>>,
}

impl Default for OverloadControlInfo {
    fn default() -> Self {
        OverloadControlInfo {
            t: OVERLOAD_CNTRL,
            length: 17,
            ins: 0,
            sqn: Sqn::default(),
            metric: Metric::default(),
            validity: EpcTimer::default(),
            list: None,
        }
    }
}

impl From<OverloadControlInfo> for GroupedIe {
    fn from(i: OverloadControlInfo) -> Self {
        GroupedIe {
            t: OVERLOAD_CNTRL,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for OverloadControlInfo {
    fn from(i: GroupedIe) -> Self {
        let mut data = OverloadControlInfo::default();
        (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            let mut apns: Vec<Apn> = vec![];
            match j {
                InformationElement::Sqn(k) => data.sqn = k,
                InformationElement::Metric(k) => data.metric = k,
                InformationElement::EpcTimer(k) => data.validity = k,
                InformationElement::Apn(k) => apns.push(k),
                _ => (),
            }
            if !apns.is_empty() {
                data.list = Some(apns);
            }
        }
        data
    }
}

impl IEs for OverloadControlInfo {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie: GroupedIe = self.clone().into();
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: OverloadControlInfo = match GroupedIe::unmarshal(buffer) {
            Ok(i) => OverloadControlInfo::from(i),
            Err(j) => return Err(j),
        };
        if let Some(i) = data.list.clone() {
            if i.len() > 10 {
                return Err(GTPV2Error::IEIncorrect(OVERLOAD_CNTRL));
            }
        }
        Ok(data)
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

impl OverloadControlInfo {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];
        v.push(self.sqn.clone().into());
        v.push(self.metric.clone().into());
        v.push(self.validity.clone().into());
        if let Some(i) = self.list.clone() {
            for j in i.into_iter() {
                v.push(j.into())
            }
        }
        v
    }
}

#[test]
fn overload_control_ie_unmarshal_test() {
    let encoded: [u8; 39] = [
        0xb4, 0x00, 0x23, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01,
        0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73,
        0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d,
    ];
    let decoded = OverloadControlInfo {
        t: OVERLOAD_CNTRL,
        length: 35,
        ins: 1,
        sqn: Sqn {
            t: SQN,
            length: SQN_LENGTH as u16,
            ins: 0,
            sqn: 0xffaaee11,
        },
        metric: Metric {
            t: METRIC,
            length: METRIC_LENGTH as u16,
            ins: 0,
            metric: 0x60,
        },
        validity: EpcTimer {
            t: EPC_TIMER,
            length: EPC_TIMER_LENGTH as u16,
            ins: 0,
            timer_unit: 3,
            timer_value: 31,
        },
        list: Some(vec![Apn {
            t: APN,
            length: 13,
            ins: 0,
            name: "test.net.com".to_string(),
        }]),
    };
    let i = OverloadControlInfo::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn overload_control_ie_marshal_test() {
    let encoded: [u8; 39] = [
        0xb4, 0x00, 0x23, 0x01, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01,
        0x00, 0x60, 0x9c, 0x00, 0x01, 0x00, 0x7f, 0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73,
        0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d,
    ];
    let decoded = OverloadControlInfo {
        t: OVERLOAD_CNTRL,
        length: 35,
        ins: 1,
        sqn: Sqn {
            t: SQN,
            length: SQN_LENGTH as u16,
            ins: 0,
            sqn: 0xffaaee11,
        },
        metric: Metric {
            t: METRIC,
            length: METRIC_LENGTH as u16,
            ins: 0,
            metric: 0x60,
        },
        validity: EpcTimer {
            t: EPC_TIMER,
            length: EPC_TIMER_LENGTH as u16,
            ins: 0,
            timer_unit: 3,
            timer_value: 31,
        },
        list: Some(vec![Apn {
            t: APN,
            length: 13,
            ins: 0,
            name: "test.net.com".to_string(),
        }]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
