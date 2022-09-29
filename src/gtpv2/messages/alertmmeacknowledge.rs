use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const ALERT_MME_ACK:u8 = 154;

// Definition of GTPv2-C Alert MME Acknowledge Message

#[derive(Debug, Clone, PartialEq)]
pub struct AlertMmeAcknowledge {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for AlertMmeAcknowledge {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = ALERT_MME_ACK;
        hdr.teid = Some(0);
        AlertMmeAcknowledge {
            header:hdr,
            cause:Cause::default(),
            private_ext:vec!(),
        }
    }
}

impl Messages for AlertMmeAcknowledge {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = AlertMmeAcknowledge::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != ALERT_MME_ACK {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            let ies:Vec<InformationElement>;
            match InformationElement::decoder(&buffer[12..]) {
                Ok(i) => ies = i,
                Err(j) => return Err(j),
            }
            match message.from_vec(ies) {
                Ok(_) => Ok(message),
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }

    fn to_vec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();

        elements.push(self.cause.clone().into());

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory = false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory) = (j, true),
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
            Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE))
        }
        
    }
}

#[test]
fn test_alert_mme_ack_unmarshal () {
    let encoded:[u8;28] = [
        0x48, 0x9a, 0x00, 0x18, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 
        0x07, 0x00, 0x01, 0x00,
    ];
    let mut decoded = AlertMmeAcknowledge::default();
    decoded.header = Gtpv2Header {
            msgtype:ALERT_MME_ACK,
            piggyback:false,
            message_prio:None, 
            length:24, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
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
    decoded.private_ext = vec!(
        PrivateExtension {
            t: PRIVATE_EXT,
            length: 6,
            ins: 0,
            enterprise_id: 2011,
            value: vec!(0x07, 0x00, 0x01, 0x00),
            }
    );
    let message = AlertMmeAcknowledge::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_alert_mme_ack_marshal () {
    let encoded:[u8;28] = [
        0x48, 0x9a, 0x00, 0x18, 0xa4, 0x78, 0x95, 0x80, 
        0x4b, 0x29, 0x1e, 0x00, 0x02, 0x00, 0x02, 0x00, 
        0x10, 0x00, 0xff, 0x00, 0x06, 0x00, 0x07, 0xdb, 
        0x07, 0x00, 0x01, 0x00,
    ];
    let mut decoded = AlertMmeAcknowledge::default();
    decoded.header = Gtpv2Header {
            msgtype:ALERT_MME_ACK,
            piggyback:false,
            message_prio:None, 
            length:24, 
            teid:Some(0xa4789580), 
            sqn:0x4b291e };
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