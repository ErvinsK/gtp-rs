use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const CHNG_NOTIF_RESP:u8 = 39;

// Definition of GTPv2-C Change Notification Response Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeNotificationResponse {
    pub header:Gtpv2Header,
    pub imsi:Option<Imsi>,
    pub mei:Option<Mei>,
    pub cause:Cause,
    pub cra:Option<ChangeReportingAction>,
    pub csg_ira:Option<CSGInformationReportingAction>,
    pub praa:Option<PresenceReportingAreaAction>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for ChangeNotificationResponse {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = CHNG_NOTIF_RESP;
        hdr.teid = Some(0);
        ChangeNotificationResponse {
            header:hdr,
            imsi:None,
            mei:None,
            cause:Cause::default(),
            cra:None,
            csg_ira:None,
            praa:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for ChangeNotificationResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ChangeNotificationResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CHNG_NOTIF_RESP {
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
        
        match self.imsi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.mei.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        elements.push(self.cause.clone().into());

        match self.cra.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.csg_ira.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.praa.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory=false;
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    match (j.ins, self.imsi.is_none()) {
                        (0, true) => self.imsi = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Mei(j) => {
                    match (j.ins, self.mei.is_none()) {
                        (0, true) => self.mei = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::ChangeReportingAction(j) => {
                    match (j.ins, self.cra.is_none()) {
                        (0, true) => self.cra = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::CSGInformationReportingAction(j) => {
                    match (j.ins, self.csg_ira.is_none()) {
                        (0, true) => self.csg_ira = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PresenceReportingAreaAction(j) => {  
                    match (j.ins, self.praa.is_none()) {
                        (0, true) => self.praa = Some(j.clone()),
                        _ => (),
                    }
                }, 
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
fn test_change_notification_resp_unmarshal () {
    let encoded:[u8;118] = [
        0x48, 0x27, 0x00, 0x72, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08, 0x00, 
        0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 
        0x4b, 0x00, 0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 
        0x01, 0x50, 0x94, 0x70, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x83, 0x00, 0x01, 0x00, 0x01, 0x92, 
        0x00, 0x01, 0x00, 0x07, 0xb1, 0x00, 0x3e, 0x00, 
        0x01, 0xff, 0xff, 0xff, 0x11, 0x01, 0x01, 0x01, 
        0x01, 0x01, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 
        0xf2, 0x10, 0xff, 0xff, 0xff, 0x62, 0xf2, 0x10, 
        0xff, 0xff, 0xff, 0x62, 0xf2, 0x10, 0x01, 0xba, 
        0x40, 0x02, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 
        0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, 
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xaa, 0x01, 
        0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff
    ];
    let mut decoded = ChangeNotificationResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:CHNG_NOTIF_RESP,
            piggyback:false,
            message_prio:None, 
            length:114, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.imsi = Some (
            Imsi {
                    t:IMSI,
                    length:8,
                    ins:0,
                    imsi:"901405101073874".to_string(),
            });
    decoded.mei = Some (
            Mei {
                t:MEI,
                length:8,
                ins:0,
                mei:"8694920510054907".to_string(),
            });
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
    decoded.cra = Some (
        ChangeReportingAction { t:CHANGE_RPRT, length: CHANGE_RPRT_LENGTH as u16, ins:0, action:1 }
    );
    decoded.csg_ira = Some (
        CSGInformationReportingAction { t:CSG_INFO_REPORT, length: 1, ins: 0, action: 7 }
    );
    decoded.praa = Some (
        PresenceReportingAreaAction { t:PRAA, length: 62, ins:0, 
            inapra:false,
            action:1,
            prai: 0xffffff, 
            tai: vec!(Tai { mcc: 262, mnc:1, tac:0x0bd9}),
            rai: vec!(Rai { mcc: 262, mnc: 1, lac:0xffff, rac:0xaa}),
            macro_enb: vec!(MacroEnbId { mcc: 262, mnc: 1, macro_id:0xffffff}),
            home_enb: vec!(MacroEnbId { mcc: 262, mnc: 1, macro_id:0xffffff}),
            ecgi: vec!(Ecgi{ mcc: 262, mnc:1, eci:28983298}),
            sai: vec!(Sai { mcc: 262, mnc: 1, lac:0xffff, sac:0xdddd}),
            cgi: vec!(Cgi { mcc: 262, mnc: 1, lac:0xffff, ci:0xaaaa}),
            ext_macro_enb: vec!(ExtMacroEnbId { mcc: 262, mnc: 1, smenb:false, ext_macro_id:0x0fffff})
         }
    );
    let message = ChangeNotificationResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_change_notification_resp_marshal () {
    let encoded:[u8;118] = [
        0x48, 0x27, 0x00, 0x72, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08, 0x00, 
        0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 
        0x4b, 0x00, 0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 
        0x01, 0x50, 0x94, 0x70, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0x83, 0x00, 0x01, 0x00, 0x01, 0x92, 
        0x00, 0x01, 0x00, 0x07, 0xb1, 0x00, 0x3e, 0x00, 
        0x01, 0xff, 0xff, 0xff, 0x11, 0x01, 0x01, 0x01, 
        0x01, 0x01, 0x62, 0xf2, 0x10, 0x0b, 0xd9, 0x62, 
        0xf2, 0x10, 0xff, 0xff, 0xff, 0x62, 0xf2, 0x10, 
        0xff, 0xff, 0xff, 0x62, 0xf2, 0x10, 0x01, 0xba, 
        0x40, 0x02, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 
        0xff, 0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, 
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xaa, 0x01, 
        0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff
    ];
    let mut decoded = ChangeNotificationResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:CHNG_NOTIF_RESP,
            piggyback:false,
            message_prio:None, 
            length:114, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.imsi = Some (
            Imsi {
                    t:IMSI,
                    length:8,
                    ins:0,
                    imsi:"901405101073874".to_string(),
            });
    decoded.mei = Some (
            Mei {
                t:MEI,
                length:8,
                ins:0,
                mei:"8694920510054907".to_string(),
            });
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
    decoded.cra = Some (
        ChangeReportingAction { t:CHANGE_RPRT, length: CHANGE_RPRT_LENGTH as u16, ins:0, action:1 }
    );
    decoded.csg_ira = Some (
        CSGInformationReportingAction { t:CSG_INFO_REPORT, length: 1, ins: 0, action: 7 }
    );
    decoded.praa = Some (
        PresenceReportingAreaAction { t:PRAA, length: 62, ins:0, 
            inapra:false,
            action:1,
            prai: 0xffffff, 
            tai: vec!(Tai { mcc: 262, mnc:1, tac:0x0bd9}),
            rai: vec!(Rai { mcc: 262, mnc: 1, lac:0xffff, rac:0xaa}),
            macro_enb: vec!(MacroEnbId { mcc: 262, mnc: 1, macro_id:0xffffff}),
            home_enb: vec!(MacroEnbId { mcc: 262, mnc: 1, macro_id:0xffffff}),
            ecgi: vec!(Ecgi{ mcc: 262, mnc:1, eci:28983298}),
            sai: vec!(Sai { mcc: 262, mnc: 1, lac:0xffff, sac:0xdddd}),
            cgi: vec!(Cgi { mcc: 262, mnc: 1, lac:0xffff, ci:0xaaaa}),
            ext_macro_enb: vec!(ExtMacroEnbId { mcc: 262, mnc: 1, smenb:false, ext_macro_id:0x0fffff})
         }
    );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
