// Node Features IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Node Features IE TL

pub const NODEFEATURES:u8 = 152;
pub const NODEFEATURES_LENGTH:usize = 1;

// Node Features IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFeatures {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub prn:bool,                      // PGW Restart Notification
    pub mabr:bool,                     // Modify Access Bearers Request
    pub ntsr:bool,                     // Network Triggered Service Restoration
    pub ciot:bool,                     // Cellular IoT
    pub s1un:bool,                     // S1-U path notification feature
}

impl Default for NodeFeatures {
    fn default() -> Self {
        NodeFeatures { 
                    t: NODEFEATURES,
                    length:NODEFEATURES_LENGTH as u16,
                    ins: 0,
                    prn:false,                       
                    mabr:false,                       
                    ntsr:false,                        
                    ciot:false,                       
                    s1un:false,                             
                }
    }
}

impl From<NodeFeatures> for InformationElement {
    fn from(i: NodeFeatures) -> Self {
        InformationElement::NodeFeatures(i)
    }
}

impl IEs for NodeFeatures {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags = self.clone().intoarray().iter().map( |x| if *x {1} else {0}).enumerate().map( |(i,x)| x<<i).collect::<Vec<_>>().iter().sum::<u8>();
        buffer_ie.push(flags);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>=(NODEFEATURES_LENGTH+MIN_IE_SIZE) {
            let mut data=NodeFeatures{
                length:u16::from_be_bytes([buffer[1], buffer[2]]),
                ..Default::default()
            };
            data.ins = buffer[3];
            let flags = [buffer[4];5].iter().enumerate().map(|(i,x)| ((*x >> i) & 0x01) as u8 == 1).collect::<Vec<bool>>();
            data.fromarray(&flags[..]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(NODEFEATURES))
        }
    }

    fn len (&self) -> usize {
       NODEFEATURES_LENGTH+MIN_IE_SIZE 
    }

}

impl NodeFeatures {
    fn intoarray(self) -> [bool;5] {
        [
            self.prn,                       
            self.mabr,                       
            self.ntsr,                       
            self.ciot,                      
            self.s1un,                      
        ]
    }
    fn fromarray(&mut self, i:&[bool]) {
            self.prn = i[0];                      
            self.mabr = i[1];                      
            self.ntsr = i[2];                      
            self.ciot = i[3];                       
            self.s1un = i[4];                        
    }
}

#[test]
fn node_features_ie_marshal_test () {
    let encoded:[u8;5]=[0x98, 0x00, 0x01, 0x00, 0x1f];
    let decoded = NodeFeatures { t: NODEFEATURES, length: NODEFEATURES_LENGTH as u16, ins:0, prn:true, mabr:true, ntsr:true, ciot:true, s1un:true};
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn node_features_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x98, 0x00, 0x01, 0x00, 0x1e];
    let decoded = NodeFeatures { t: NODEFEATURES, length: NODEFEATURES_LENGTH as u16, ins:0, prn:false, mabr:true, ntsr:true, ciot:true, s1un:true};
    assert_eq!(NodeFeatures::unmarshal(&encoded).unwrap(), decoded);
}