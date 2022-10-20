use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const SUSPEND_NOTIF:u8 = 162;

// Definition of GTPv2-C Suspend Notification Message

#[derive(Debug, Clone, PartialEq)]
pub struct SuspendNotification {
    pub header:Gtpv2Header,
    pub imsi:Option<Imsi>,
    pub rai:Option<Uli>,
    pub linked_ebi:Option<Ebi>,
    pub ptmsi:Option<Ptmsi>,
    pub orig_node:Option<NodeType>,
    pub ip_control:Option<IpAddress>,
    pub udp_port:Option<PortNumber>,
    pub hop_counter:Option<HopCounter>,
    pub fteid_control:Option<Fteid>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for SuspendNotification {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = SUSPEND_NOTIF;
        hdr.teid = Some(0);
        SuspendNotification {
            header:hdr,
            imsi:None,
            rai:None,
            linked_ebi:None,
            ptmsi:None,
            orig_node:None,
            ip_control:None,
            udp_port:None,
            hop_counter:None,
            fteid_control:None,
            private_ext:vec!(),
        }
    }
}

impl Messages for SuspendNotification {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = SuspendNotification::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != SUSPEND_NOTIF {
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
                Err(j) => return Err(j),
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
        match self.rai.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ptmsi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.orig_node.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ip_control.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.udp_port.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.hop_counter.clone() {
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
        for e in elements.into_iter() {
            match e {
                InformationElement::Imsi(j) => {
                    match (j.ins, self.imsi.is_none()) {
                        (0, true) => self.imsi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Uli(j) => {
                    match (j.ins, self.rai.is_none()) {
                        (0, true) => self.rai = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {  
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Ptmsi(j) => {
                    match (j.ins, self.ptmsi.is_none()) {
                        (0, true) => self.ptmsi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::NodeType(j) => {
                    match (j.ins, self.orig_node.is_none()) {
                        (0, true) => self.orig_node = Some(j),
                        _ => (),
                    }
                },
                InformationElement::IpAddress(j) => {
                    match (j.ins, self.ip_control.is_none()) {
                        (0, true) => self.ip_control = Some(j),
                        _ => (),
                    }
                },
                InformationElement::PortNumber(j) => {
                    match (j.ins, self.udp_port.is_none()) {
                        (0, true) => self.udp_port = Some(j),
                        _ => (),
                    }
                },
                InformationElement::HopCounter(j) => {
                    match (j.ins, self.hop_counter.is_none()) {
                        (0, true) => self.hop_counter = Some(j),
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
        Ok(true)
    }
}

#[test]
fn test_suspend_notification_unmarshal () {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded:[u8;49] = [
        0x48, 0xa2, 0x00, 0x2d, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x49, 0x00, 0x01, 0x00, 
        0x05, 0x4a, 0x00, 0x04, 0x00, 0x3e, 0x6b, 0x58, 
        0x5e, 0x7e, 0x00, 0x02, 0x00, 0x0d, 0x10, 0x71, 
        0x00, 0x01, 0x00, 0x08, 0x57, 0x00, 0x09, 0x00, 
        0x86, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, 
        0x2d
    ];
    let mut decoded = SuspendNotification::default();
    decoded.header = Gtpv2Header {
            msgtype:SUSPEND_NOTIF,
            piggyback:false,
            message_prio:None, 
            length:45, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.ip_control = Some (
    IpAddress { t:IP_ADDRESS, length:4, ins:0, ip:IpAddr::V4(Ipv4Addr::new(62,107,88,94)) }
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
    decoded.udp_port = Some (
        PortNumber { t: PORT_NBR, length: PORT_NBR_LENGTH as u16, ins: 0, port: 3344 }
    );
    decoded.hop_counter = Some (
        HopCounter { t: HOP_CNTR, length: HOP_CNTR_LENGTH as u16, ins: 0, hop_counter: 8 }
    );
    let message = SuspendNotification::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_suspend_notification_marshal () {
    use std::net::{IpAddr, Ipv4Addr};
    let encoded:[u8;49] = [
        0x48, 0xa2, 0x00, 0x2d, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x49, 0x00, 0x01, 0x00, 
        0x05, 0x4a, 0x00, 0x04, 0x00, 0x3e, 0x6b, 0x58, 
        0x5e, 0x7e, 0x00, 0x02, 0x00, 0x0d, 0x10, 0x71, 
        0x00, 0x01, 0x00, 0x08, 0x57, 0x00, 0x09, 0x00, 
        0x86, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, 
        0x2d
    ];
    let mut decoded = SuspendNotification::default();
    decoded.header = Gtpv2Header {
            msgtype:SUSPEND_NOTIF,
            piggyback:false,
            message_prio:None, 
            length:45, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
    decoded.linked_ebi = Some (
        Ebi {
            t:EBI,
            length:1,
            ins:0,
            value:5,
        }
    );
    decoded.ip_control = Some (
    IpAddress { t:IP_ADDRESS, length:4, ins:0, ip:IpAddr::V4(Ipv4Addr::new(62,107,88,94)) }
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
    decoded.udp_port = Some (
        PortNumber { t: PORT_NBR, length: PORT_NBR_LENGTH as u16, ins: 0, port: 3344 }
    );
    decoded.hop_counter = Some (
        HopCounter { t: HOP_CNTR, length: HOP_CNTR_LENGTH as u16, ins: 0, hop_counter: 8 }
    );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer,encoded);
}
