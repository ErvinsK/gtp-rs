// Overload Control IE (Grouped IE) - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Overload Control IE T

pub const OVERLOAD_CNTRL:u8 = 180;

#[derive(Debug, Clone, PartialEq)]
pub struct OverloadControl {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub sqn:Sqn,
    pub metric:Metric,
    pub validity:EpcTimer,
    pub list:Option<Vec<Apn>>,
}

impl Default for OverloadControl {
    fn default() -> Self {
        OverloadControl { t: OVERLOAD_CNTRL, 
                        length: 17,
                        ins:0,
                        sqn:Sqn::default(),
                        metric:Metric::default(),
                        validity:EpcTimer::default(),
                        list:None,   
                    }        
    }
}

impl From<OverloadControl> for GroupedIe {
    fn from(i: OverloadControl) -> Self {
        GroupedIe { t: OVERLOAD_CNTRL,
                    length: 0, 
                    ins: 0, 
                    elements: i.to_vec(), 
                }
    }
} 

impl From<GroupedIe> for OverloadControl {
    fn from(i: GroupedIe) -> Self {
       let mut data = OverloadControl::default();
       (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
       for j in i.elements.into_iter() {
            let mut apns:Vec<Apn>=vec!();
            match j {
                InformationElement::Sqn(k) => data.sqn=k,
                InformationElement::Metric(k) => data.metric=k,
                InformationElement::EpcTimer(k) => data.validity=k,
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

impl IEs for OverloadControl {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let g_ie = GroupedIe::from(self.clone());
        g_ie.marshal(buffer);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        let data:OverloadControl;
        match GroupedIe::unmarshal(buffer) {
            Ok(i) => data = OverloadControl::from(i),
            Err(j) => return Err(j),
        }
        match data.list.clone() {
            Some(i) => {
                if i.len()>10 {
                    return Err(GTPV2Error::IEIncorrect(OVERLOAD_CNTRL));
                }
            },
            None => (), 
        }
        Ok(data)
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

impl OverloadControl {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v:Vec<InformationElement> = vec!();        
        v.push(self.sqn.clone().into());
        v.push(self.metric.clone().into());
        v.push(self.validity.clone().into());
        match self.list.clone() {
            Some(i) => {
                for j in i.into_iter() {
                    v.push(j.into())
                }
            },
            None => (),
        }
        v
    }
}

#[test]
fn overload_control_ie_unmarshal_test () {
    let encoded:[u8;39]=[
        0xb4, 0x00, 0x23, 0x00, 
        0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11,
        0xb6, 0x00, 0x01, 0x00, 0x60,
        0x9c, 0x00, 0x01, 0x00, 0x7f,
        0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d,   
    ];
   let decoded = OverloadControl { 
    t: OVERLOAD_CNTRL, 
    length: 35, 
    ins: 0, 
    sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
    metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
    validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:31 },
    list: Some(vec!(
        Apn { t:APN, length: 13, ins:0, name: "test.net.com".to_string() }
    )),
    };
    let i = OverloadControl::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn overload_control_ie_marshal_test () {
    let encoded:[u8;39]=[
        0xb4, 0x00, 0x23, 0x00, 
        0xb7, 0x00, 0x04, 0x00, 0xff, 0xaa, 0xee, 0x11,
        0xb6, 0x00, 0x01, 0x00, 0x60,
        0x9c, 0x00, 0x01, 0x00, 0x7f,
        0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e, 0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d,   
    ];
   let decoded = OverloadControl { 
    t: LOAD_CNTRL, 
    length: 35, 
    ins: 0, 
    sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
    metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
    validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:31 },
    list: Some(vec!(
        Apn { t:APN, length: 13, ins:0, name: "test.net.com".to_string() }
    )),
    };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}