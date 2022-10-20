use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const CHNG_NOTIF_REQ:u8 = 38;

// Definition of GTPv2-C Change Notification Request Message

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeNotificationRequest {
    pub header:Gtpv2Header,
    pub imsi:Option<Imsi>,
    pub mei:Option<Mei>,
    pub indication:Option<Indication>,
    pub rattype:RatType,
    pub uli:Option<Uli>,
    pub uci:Option<Uci>,
    pub pgw_addr_control:Option<IpAddress>,
    pub linked_ebi:Option<Ebi>,
    pub prai: Option<PresenceReportingAreaInformation>,
    pub mo_exception_data_counter: Option<Counter>,
    pub secondary_rat_usage_report: Vec<SecondaryRatUsageDataReport>,    
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for ChangeNotificationRequest {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = CHNG_NOTIF_REQ;
        hdr.teid = Some(0);
        ChangeNotificationRequest {
            header:hdr,
            imsi:None,
            mei:None,
            indication:None,
            rattype: RatType::default(),
            uli:None,
            uci:None,
            pgw_addr_control:None,
            linked_ebi:None,
            prai: None,
            mo_exception_data_counter: None,
            secondary_rat_usage_report: vec!(),            
            private_ext: vec!(),
        }
    }
}

impl Messages for ChangeNotificationRequest {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ChangeNotificationRequest::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != CHNG_NOTIF_REQ {
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
        match self.indication.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        
        elements.push(self.rattype.clone().into());

        match self.uli.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.uci.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.pgw_addr_control.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }        
        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.prai.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.mo_exception_data_counter.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.secondary_rat_usage_report.iter().for_each(|x| elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone())));

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
        
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.iter() {
            match e {
                InformationElement::Imsi(j) => {
                    match (j.ins, self.imsi.is_none()) {
                        (0, true) => self.imsi = Some(j.clone()),
                        (_,_) => (),
                    }
                },
                InformationElement::Mei(j) => {
                    match (j.ins, self.mei.is_none()) {
                        (0, true) => self.mei = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Indication(j) => {
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::RatType(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.rattype, mandatory) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::Uli(j) => { // Two instances
                    match (j.ins, self.uli.is_none()) {
                        (0, true) => self.uli = Some(j.clone()),
                        _ => (),
                    }
                },                
                InformationElement::Uci(j) => {
                    match (j.ins, self.uci.is_none()) {
                        (0, true) => self.uci = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::IpAddress(j) => {   
                    match (j.ins, self.pgw_addr_control.is_none()) {
                        (0, true) => self.pgw_addr_control = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Ebi(j) => {  
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PresenceReportingAreaInformation(j) => {  
                    match (j.ins, self.prai.is_none()) {
                        (0, true) => self.prai = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Counter(j) => {  
                    match (j.ins, self.mo_exception_data_counter.is_none()) {
                        (0, true) => self.mo_exception_data_counter = Some(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::SecondaryRatUsageDataReport(j) => {
                    if j.ins == 0 {
                        self.secondary_rat_usage_report.push(j.clone());
                    }
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(RATTYPE))
        }
       
    }
}

#[test]
fn test_change_notification_req_unmarshal () {
    let encoded:[u8;125] = [
        0x48, 0x26, 0x00, 0x79, 0xe6, 0x4d, 0xa4, 0xef, 
        0x26, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x08, 0x00, 
        0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 
        0x4b, 0x00, 0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 
        0x01, 0x50, 0x94, 0x70, 0x52, 0x00, 0x01, 0x00, 
        0x06, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x32, 0xf4, 
        0x02, 0x0d, 0x59, 0x32, 0xf4, 0x02, 0x00, 0xc5, 
        0x58, 0x02, 0x49, 0x00, 0x01, 0x00, 0x05, 0xb2, 
        0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 
        0x00, 0xff, 0x02, 0xc7, 0x00, 0x05, 0x00, 0xee, 
        0x6b, 0x28, 0x00, 0x09, 0xc9, 0x00, 0x1b, 0x00, 
        0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 
        0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 
        0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07, 
        0xdb, 0x07, 0x00, 0x01, 0x00
    ];
    let mut decoded = ChangeNotificationRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:CHNG_NOTIF_REQ,
            piggyback:false,
            message_prio:None, 
            length:121, 
            teid:Some(0xe64da4ef), 
            sqn:0x26002e };
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
    decoded.rattype = 
            RatType {
                t:RATTYPE,
                length:1,
                ins:0,
                rat_type:Rat::Eutran,
            };   
    decoded.uli = Some (
        Uli {
            t:ULI,
            length:13,
            ins:0,
            loc: vec!(Location::Tai(Tai { mcc: 234, mnc:20, tac:0x0d59}),Location::Ecgi(Ecgi{ mcc: 234, mnc:20, eci:12933122})),
        });
    decoded.linked_ebi = Some( Ebi{
        t:EBI,
        length:EBI_LENGTH as u16,
        ins:0,
        value:5,
    });
    decoded.prai = Some(
        PresenceReportingAreaInformation { 
            t:PRAI, 
            length: 8, 
            ins:0, 
            prai: PresenceReportingArea::Ipra(0x00), 
            add_prai:Some(vec!(PresenceReportingArea::Opra(0xff))) 
    });
    decoded.mo_exception_data_counter = Some (
        Counter { t:COUNTER, length: COUNTER_LENGTH as u16, ins:0, timestamp: 4000000000, counter: 9 }
    );
    decoded.secondary_rat_usage_report = vec!(
        SecondaryRatUsageDataReport { 
            t:SCND_RAT_UDR, 
            length: SCND_RAT_UDR_LENGTH as u16, 
            ins:0, 
            irsgw: true, 
            irpgw:true, 
            rat_type:0, 
            ebi:5, 
            start_timestamp: 0xff, 
            end_timestamp:0xffff, 
            usg_data_dl:0xffffff00, 
            usg_data_ul:0xffff });
    decoded.private_ext = vec!(
        PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec!(0x07, 0x00, 0x01, 0x00),
            }
        );
    let message = ChangeNotificationRequest::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_change_notification_req_marshal () {
    let encoded:[u8;125] = [
        0x48, 0x26, 0x00, 0x79, 0xe6, 0x4d, 0xa4, 0xef, 
        0x26, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x08, 0x00, 
        0x09, 0x41, 0x50, 0x01, 0x01, 0x37, 0x78, 0xf4, 
        0x4b, 0x00, 0x08, 0x00, 0x68, 0x49, 0x29, 0x50, 
        0x01, 0x50, 0x94, 0x70, 0x52, 0x00, 0x01, 0x00, 
        0x06, 0x56, 0x00, 0x0d, 0x00, 0x18, 0x32, 0xf4, 
        0x02, 0x0d, 0x59, 0x32, 0xf4, 0x02, 0x00, 0xc5, 
        0x58, 0x02, 0x49, 0x00, 0x01, 0x00, 0x05, 0xb2, 
        0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 
        0x00, 0xff, 0x02, 0xc7, 0x00, 0x05, 0x00, 0xee, 
        0x6b, 0x28, 0x00, 0x09, 0xc9, 0x00, 0x1b, 0x00, 
        0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 
        0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 
        0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0xff, 0xff, 0xff, 0x00, 0x06, 0x00, 0x07, 
        0xdb, 0x07, 0x00, 0x01, 0x00
    ];
    let mut decoded = ChangeNotificationRequest::default();
    decoded.header = Gtpv2Header {
            msgtype:CHNG_NOTIF_REQ,
            piggyback:false,
            message_prio:None, 
            length:121, 
            teid:Some(0xe64da4ef), 
            sqn:0x26002e };
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
    decoded.rattype = 
            RatType {
                t:RATTYPE,
                length:1,
                ins:0,
                rat_type:Rat::Eutran,
            };   
    decoded.uli = Some (
        Uli {
            t:ULI,
            length:13,
            ins:0,
            loc: vec!(Location::Tai(Tai { mcc: 234, mnc:20, tac:0x0d59}),Location::Ecgi(Ecgi{ mcc: 234, mnc:20, eci:12933122})),
        });
    decoded.linked_ebi = Some( Ebi{
        t:EBI,
        length:EBI_LENGTH as u16,
        ins:0,
        value:5,
    });
    decoded.prai = Some(
        PresenceReportingAreaInformation { 
            t:PRAI, 
            length: 8, 
            ins:0, 
            prai: PresenceReportingArea::Ipra(0x00), 
            add_prai:Some(vec!(PresenceReportingArea::Opra(0xff))) 
    });
    decoded.mo_exception_data_counter = Some (
        Counter { t:COUNTER, length: COUNTER_LENGTH as u16, ins:0, timestamp: 4000000000, counter: 9 }
    );
    decoded.secondary_rat_usage_report = vec!(
        SecondaryRatUsageDataReport { 
            t:SCND_RAT_UDR, 
            length: SCND_RAT_UDR_LENGTH as u16, 
            ins:0, 
            irsgw: true, 
            irpgw:true, 
            rat_type:0, 
            ebi:5, 
            start_timestamp: 0xff, 
            end_timestamp:0xffff, 
            usg_data_dl:0xffffff00, 
            usg_data_ul:0xffff });
    decoded.private_ext = vec!(
        PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec!(0x07, 0x00, 0x01, 0x00),
            }
        );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
