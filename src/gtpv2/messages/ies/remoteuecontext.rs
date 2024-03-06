//  Remote UE Context IE (Grouped IE) - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Remote UE Context IE Type

pub const REMOTE_UE_CTX: u8 = 191;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteUeContext {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub user_id: RemoteUserId,
    pub ue_ip: Option<RemoteUeIpInformation>,
}

impl Default for RemoteUeContext {
    fn default() -> Self {
        RemoteUeContext {
            t: REMOTE_UE_CTX,
            length: 5,
            ins: 0,
            user_id: RemoteUserId::default(),
            ue_ip: None,
        }
    }
}

impl From<RemoteUeContext> for GroupedIe {
    fn from(i: RemoteUeContext) -> Self {
        GroupedIe {
            t: REMOTE_UE_CTX,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for RemoteUeContext {
    fn from(i: GroupedIe) -> Self {
        let mut data = RemoteUeContext::default();
        (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            match j {
                InformationElement::RemoteUserId(k) => data.user_id = k,
                InformationElement::RemoteUeIpInformation(k) => data.ue_ip = Some(k),
                _ => (),
            }
        }
        data
    }
}

impl IEs for RemoteUeContext {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie = GroupedIe::from(self.clone());
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: RemoteUeContext = match GroupedIe::unmarshal(buffer) {
            Ok(i) => RemoteUeContext::from(i),
            Err(j) => return Err(j),
        };
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

impl RemoteUeContext {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];
        v.push(self.user_id.clone().into());
        if let Some(i) = self.ue_ip.clone() {
            v.push(i.into())
        };
        v
    }
}

#[test]
fn remote_ue_ctx_ie_unmarshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 45] = [
        0xbf, 0x00, 0x29, 0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91,
        0x16, 0x78, 0xf3, 0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34,
    ];
    let decoded = RemoteUeContext {
        t: REMOTE_UE_CTX,
        length: 41,
        ins: 0,
        user_id: RemoteUserId {
            t: REMOTE_USR_ID,
            length: 28,
            ins: 0,
            imsi: "901405101961873".to_string(),
            msisdn: Some("882285100125112".to_string()),
            imei: Some("8676480401323003".to_string()),
        },
        ue_ip: Some(RemoteUeIpInformation {
            t: REMOTE_UE_IP,
            length: 5,
            ins: 0,
            ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
        }),
    };
    let i = RemoteUeContext::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn remote_ue_ctx_ie_marshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 45] = [
        0xbf, 0x00, 0x29, 0x00, 0xc0, 0x00, 0x1c, 0x00, 0x03, 0x08, 0x09, 0x41, 0x50, 0x01, 0x91,
        0x16, 0x78, 0xf3, 0x08, 0x88, 0x22, 0x58, 0x01, 0x10, 0x52, 0x11, 0xf2, 0x08, 0x68, 0x67,
        0x84, 0x40, 0x10, 0x23, 0x03, 0x30, 0xc1, 0x00, 0x05, 0x00, 0x01, 0x0a, 0xc2, 0xba, 0x34,
    ];
    let decoded = RemoteUeContext {
        t: REMOTE_UE_CTX,
        length: 41,
        ins: 0,
        user_id: RemoteUserId {
            t: REMOTE_USR_ID,
            length: 28,
            ins: 0,
            imsi: "901405101961873".to_string(),
            msisdn: Some("882285100125112".to_string()),
            imei: Some("8676480401323003".to_string()),
        },
        ue_ip: Some(RemoteUeIpInformation {
            t: REMOTE_UE_IP,
            length: 5,
            ins: 0,
            ip: RemoteIpAddress::V4(Ipv4Addr::new(10, 194, 186, 52)),
        }),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
