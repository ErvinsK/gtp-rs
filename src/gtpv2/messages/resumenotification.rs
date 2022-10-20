use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const RESUME_NOTIF:u8 = 164;

// Definition of GTPv2-C Resume Notification Message

#[derive(Debug, Clone, PartialEq)]
pub struct ResumeNotification {
    pub header:Gtpv2Header,
    pub imsi:Imsi,
    pub linked_ebi:Option<Ebi>,
    pub orig_node:Option<NodeType>,
    pub fteid_control:Option<Fteid>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for ResumeNotification {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = RESUME_NOTIF;
        hdr.teid = Some(0);
        ResumeNotification {
            header:hdr,
            imsi:Imsi::default(),
            linked_ebi:None,
            orig_node:None,
            fteid_control:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for ResumeNotification {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = ResumeNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != RESUME_NOTIF {
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

        elements.push(self.imsi.clone().into());

        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.orig_node.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.fteid_control.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Imsi(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.imsi, mandatory) = (j, true),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {  
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::NodeType(j) => {
                    match (j.ins, self.orig_node.is_none()) {
                        (0, true) => self.orig_node = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Fteid(j) => {
                    match (j.ins, self.fteid_control.is_none()) {
                        (0, true) => self.fteid_control = Some(j),
                        _ => (),
                    }
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j),
                _ => (),
            }
        }
        if mandatory {
            Ok(true)
        } else {
            Err(GTPV2Error::MessageMandatoryIEMissing(IMSI))
        }
    }
}

#[test]
fn test_resume_notification_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;42] = [
        0x48, 0xa4, 0x00, 0x26, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08, 0x00, 
        0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 
        0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 
        0x00, 0x86, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 
        0x8b, 0x2d,
    ];
    let mut decoded = ResumeNotification::default();
    decoded.header = Gtpv2Header {
            msgtype:RESUME_NOTIF,
            piggyback:false,
            message_prio:None, 
            length:38, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.imsi = Imsi { t:0x01, length:0x08, ins:0x00, imsi:"901405101961873".to_string(), };
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.fteid_control = Some(
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193,254,139,45)),
            ipv6:None
        });
    let message = ResumeNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_resume_notification_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;42] = [
        0x48, 0xa4, 0x00, 0x26, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x01, 0x00, 0x08, 0x00, 
        0x09, 0x41, 0x50, 0x01, 0x91, 0x16, 0x78, 0xf3, 
        0x49, 0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 
        0x00, 0x86, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 
        0x8b, 0x2d,
    ];
    let mut decoded = ResumeNotification::default();
    decoded.header = Gtpv2Header {
            msgtype:RESUME_NOTIF,
            piggyback:false,
            message_prio:None, 
            length:38, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.imsi = Imsi { t:0x01, length:0x08, ins:0x00, imsi:"901405101961873".to_string(), };
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.fteid_control = Some(
        Fteid {
            t:FTEID,
            length:9,
            ins:0,
            interface: 6,
            teid: 0x06d1824c,
            ipv4: Some(Ipv4Addr::new(193,254,139,45)),
            ipv6:None
        });
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
