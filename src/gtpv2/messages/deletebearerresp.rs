use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const DELETE_BEARER_RESP:u8 = 100;

// Definition of GTPv2-C Delete Bearer Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteBearerResponse {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub linked_ebi:Option<Ebi>,
    pub bearer_ctxs:Vec<BearerContext>,
    pub recovery:Option<Recovery>,
    pub mme_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub epdg_fqcsid: Option<Fqcsid>,
    pub twan_fqcsid: Option<Fqcsid>,
    pub pco:Option<Pco>,
    pub uetimezone: Option<UeTimeZone>,
    pub uli: Option<Uli>,
    pub uli_timestamp:Option<UliTimestamp>,
    pub twan_id: Option<TwanId>,
    pub twan_id_timestamp:Option<TwanIdTimeStamp>,
    pub overload_info:Vec<OverloadControlInfo>,
    pub ip: Option<IpAddress>,  // Either MME Id IE (S11/S8/S5) or UE Local IP IE (S2b)
    pub wlan_loc: Option<TwanId>,
    pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    pub ue_udpport: Option<PortNumber>,
    pub nbifom:Option<Fcontainer>,
    pub ue_tcpport: Option<PortNumber>,
    pub secondary_rat_usage_report:Vec<SecondaryRatUsageDataReport>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for DeleteBearerResponse {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = DELETE_BEARER_RESP;
        hdr.teid = Some(0);
        DeleteBearerResponse {
            header:hdr,
            cause:Cause::default(),
            linked_ebi:None,
            bearer_ctxs:vec!(),
            recovery:None,
            mme_fqcsid:None,
            sgw_fqcsid:None,
            epdg_fqcsid:None,
            twan_fqcsid:None,
            pco:None,
            uetimezone:None,
            uli:None,
            uli_timestamp:None,
            twan_id:None,
            twan_id_timestamp:None,
            overload_info:vec!(),
            ip:None,
            wlan_loc:None,
            wlan_loc_timestamp:None,
            ue_udpport:None,
            nbifom:None,
            ue_tcpport:None,
            secondary_rat_usage_report:vec!(),
            private_ext:vec!(),
        }
    }
}

impl Messages for DeleteBearerResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteBearerResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_BEARER_RESP {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            match InformationElement::decoder(&buffer[12..]) {
                Ok(i) => {
                    match message.from_vec(i) {
                        Ok(_) => Ok(message),
                        Err(j) => Err(j),
                    }
                },
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }

    fn to_vec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();
        
        elements.push(self.cause.clone().into());

        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        match self.recovery.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.mme_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.sgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.epdg_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.twan_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }    
        match self.uetimezone.clone() {
            Some(i) => elements.push(InformationElement::UeTimeZone(i)),
            None => (),
        }
        match self.uli.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.uli_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.twan_id.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.twan_id_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        match self.ip.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }        
        match self.wlan_loc.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.wlan_loc_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ue_udpport.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.nbifom.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ue_tcpport.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        } 

        self.secondary_rat_usage_report.iter().for_each(|x| elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone())));  

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory=false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory) = (j, true),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::BearerContext(j) => {
                    match j.ins {
                        0 => {
                            self.bearer_ctxs.push(j);
                        },
                        _ => (),
                    }
                }
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Fqcsid(j) => {  // 4 instances
                    match (j.ins, self.mme_fqcsid.is_none(), self.sgw_fqcsid.is_none(), self.epdg_fqcsid.is_none(), self.twan_fqcsid.is_none()) {
                        (0, true, _, _, _) => self.mme_fqcsid = Some(j),
                        (1, _, true, _, _) => self.sgw_fqcsid = Some(j),
                        (2, _, _, true,_) => self.epdg_fqcsid = Some(j),
                        (3, _, _, _, true) => self.twan_fqcsid = Some(j),
                        _ => (),
                    }
                }, 
                InformationElement::Pco(j) => {
                    match (j.ins, self.pco.is_none()) {
                        (0, true) => self.pco = Some(j),
                        _ => (),
                    }
                },
                InformationElement::UeTimeZone(j) => {
                    match (j.ins, self.uetimezone.is_none()) {
                        (0, true) => self.uetimezone = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Uli(j) => {
                    match (j.ins, self.uli.is_none()) {
                        (0, true) => self.uli = Some(j),
                        _ => (),
                    }
                },
                InformationElement::UliTimestamp(j) => {
                    match (j.ins, self.uli_timestamp.is_none()) {
                        (0, true) => self.uli_timestamp = Some(j),
                        _ => (),
                    }
                },
                InformationElement::TwanId(j) => { // 2 instances
                    match (j.ins, self.twan_id.is_none(), self.wlan_loc.is_none()) {
                        (0, true, _) => self.twan_id = Some(j),
                        (1, _, true) => self.wlan_loc = Some(j),
                        _ => (),
                    }
                },
                InformationElement::TwanIdTimeStamp(j) => { // 2 instances
                    match (j.ins, self.twan_id_timestamp.is_none(), self.wlan_loc_timestamp.is_none()) {
                        (0, true, _) => self.twan_id_timestamp = Some(j),
                        (1, _, true) => self.wlan_loc_timestamp = Some(j),
                        _ => (),
                    }
                },
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<3 => self.overload_info.push(j),
                        _ => (),
                    }
                }, 
                InformationElement::IpAddress(j) => {
                    match (j.ins, self.ip.is_none()) {
                        (0, true) => self.ip = Some(j),
                        _ => (),
                    }
                },
                InformationElement::PortNumber(j) => { // 2 instances
                    match (j.ins, self.ue_udpport.is_none(), self.ue_tcpport.is_none()) {
                        (0, true, _) => self.ue_udpport = Some(j),
                        (1, _, true) => self.ue_tcpport = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Fcontainer(j) => {  
                    match (j.ins, self.nbifom.is_none()) {
                        (0, true) => self.nbifom = Some(j),
                        _ => (),
                    }
                },

                InformationElement::SecondaryRatUsageDataReport(j) => self.secondary_rat_usage_report.push(j.clone()),

                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE))
        }
    }
}

#[test]
fn test_delete_bearer_resp_unmarshal () {
    let encoded:[u8;54] = [
        0x48, 0x64, 0x00, 0x32, 0x78, 0x7d, 0xaf, 0x3c, 
        0x1b, 0x7a, 0xae, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x72, 
        0x00, 0x02, 0x00, 0x80, 0x00, 0x56, 0x00, 0x0d, 
        0x00, 0x18, 0x42, 0xf7, 0x10, 0xab, 0xea, 0x42, 
        0xf7, 0x10, 0x00, 0x2a, 0x46, 0x10, 0xaa, 0x00, 
        0x04, 0x00, 0xe5, 0xce, 0x77, 0xef,
    ];
    let mut decoded = DeleteBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:50, 
            teid:Some(0x787daf3c), 
            sqn:0x1b7aae };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:16,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }    
    );
    decoded.uli = Some (
        Uli {
            t:ULI,
            length:13,
            ins:0,
            loc: vec!(Location::Tai(Tai { mcc: 247, mnc:1, tac:0xabea}),Location::Ecgi(Ecgi{ mcc: 247, mnc:1, eci:2770448})),
        });
    decoded.uli_timestamp = Some (
        UliTimestamp { 
            t: ULI_TIMESTAMP, 
            length: 4,
            ins: 0,
            timestamp: 0xe5ce77ef }
    );
    decoded.uetimezone = Some (
        UeTimeZone { t: UETIMEZONE, length: 2, ins: 0, time_zone: 2, dst: 0 }
    );
   
    
    let message = DeleteBearerResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_delete_bearer_resp_marshal () {
    let encoded:[u8;54] = [
        0x48, 0x64, 0x00, 0x32, 0x78, 0x7d, 0xaf, 0x3c, 
        0x1b, 0x7a, 0xae, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x49, 0x00, 0x01, 0x00, 0x05, 0x72, 
        0x00, 0x02, 0x00, 0x80, 0x00, 0x56, 0x00, 0x0d, 
        0x00, 0x18, 0x42, 0xf7, 0x10, 0xab, 0xea, 0x42, 
        0xf7, 0x10, 0x00, 0x2a, 0x46, 0x10, 0xaa, 0x00, 
        0x04, 0x00, 0xe5, 0xce, 0x77, 0xef,
    ];
    let mut decoded = DeleteBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:DELETE_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:50, 
            teid:Some(0x787daf3c), 
            sqn:0x1b7aae };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:16,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }    
    );
    decoded.uli = Some (
        Uli {
            t:ULI,
            length:13,
            ins:0,
            loc: vec!(Location::Tai(Tai { mcc: 247, mnc:1, tac:0xabea}),Location::Ecgi(Ecgi{ mcc: 247, mnc:1, eci:2770448})),
        });
    decoded.uli_timestamp = Some (
        UliTimestamp { 
            t: ULI_TIMESTAMP, 
            length: 4,
            ins: 0,
            timestamp: 0xe5ce77ef }
    );
    decoded.uetimezone = Some (
        UeTimeZone { t: UETIMEZONE, length: 2, ins: 0, time_zone: 2, dst: 0 }
    );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
