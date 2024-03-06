// Load Control IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Load Control IE T

pub const LOAD_CNTRL: u8 = 181;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadControl {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub sqn: Sqn,
    pub load_metric: Metric,
    pub list: Option<Vec<ApnRelativeCapacity>>,
}

impl Default for LoadControl {
    fn default() -> Self {
        LoadControl {
            t: LOAD_CNTRL,
            length: 0,
            ins: 0,
            sqn: Sqn::default(),
            load_metric: Metric::default(),
            list: None,
        }
    }
}

impl From<LoadControl> for GroupedIe {
    fn from(i: LoadControl) -> Self {
        GroupedIe {
            t: LOAD_CNTRL,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for LoadControl {
    fn from(i: GroupedIe) -> Self {
        let mut data = LoadControl::default();
        (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            let mut apns: Vec<ApnRelativeCapacity> = vec![];
            match j {
                InformationElement::Sqn(k) => data.sqn = k,
                InformationElement::Metric(k) => data.load_metric = k,
                InformationElement::ApnRelativeCapacity(k) => apns.push(k),
                _ => (),
            }
            if !apns.is_empty() {
                data.list = Some(apns);
            }
        }
        data
    }
}

impl IEs for LoadControl {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie = GroupedIe::from(self.clone());
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: LoadControl = match GroupedIe::unmarshal(buffer) {
            Ok(i) => LoadControl::from(i),
            Err(j) => return Err(j),
        };
        if let Some(i) = data.list.clone() {
            if i.len() > 10 {
                return Err(GTPV2Error::IEIncorrect(LOAD_CNTRL));
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

impl LoadControl {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];
        v.push(self.sqn.clone().into());
        v.push(self.load_metric.clone().into());
        if let Some(i) = self.list.clone() {
            for j in i.into_iter() {
                v.push(j.into())
            }
        }
        v
    }
}

#[test]
fn load_control_ie_unmarshal_test() {
    let encoded: [u8; 35] = [
        0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01,
        0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65,
        0x74, 0x03, 0x63, 0x6f, 0x6d,
    ];
    let decoded = LoadControl {
        t: LOAD_CNTRL,
        length: 31,
        ins: 0,
        sqn: Sqn {
            t: SQN,
            length: SQN_LENGTH as u16,
            ins: 0,
            sqn: 0xffaaee11,
        },
        load_metric: Metric {
            t: METRIC,
            length: METRIC_LENGTH as u16,
            ins: 0,
            metric: 0x60,
        },
        list: Some(vec![ApnRelativeCapacity {
            t: APN_REL_CAP,
            length: 14,
            ins: 0,
            relative_cap: 100,
            name: "test.net.com".to_string(),
        }]),
    };
    let i = LoadControl::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn load_control_ie_marshal_test() {
    let encoded: [u8; 35] = [
        0xb5, 0x00, 0x1f, 0x00, 0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11, 0xb6, 0x00, 0x01,
        0x00, 0x60, 0xb8, 0x00, 0x0e, 0x00, 0x64, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65,
        0x74, 0x03, 0x63, 0x6f, 0x6d,
    ];
    let decoded = LoadControl {
        t: LOAD_CNTRL,
        length: 31,
        ins: 0,
        sqn: Sqn {
            t: SQN,
            length: SQN_LENGTH as u16,
            ins: 0,
            sqn: 0xffaaee11,
        },
        load_metric: Metric {
            t: METRIC,
            length: METRIC_LENGTH as u16,
            ins: 0,
            metric: 0x60,
        },
        list: Some(vec![ApnRelativeCapacity {
            t: APN_REL_CAP,
            length: 14,
            ins: 0,
            relative_cap: 100,
            name: "test.net.com".to_string(),
        }]),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
