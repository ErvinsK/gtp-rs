// Bearer Context IE (Grouped IE) - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::*};

// Bearer Context IE T

pub const BEARER_CTX:u8 = 93;

#[derive(Debug, Clone, PartialEq)]
pub struct BearerContext {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub ebi:Ebi,
    pub cause:Option<Cause>,
    pub tft:Option<BearerTft>,
    pub enb_fteid:Option<Fteid>,
    pub sgsn_fteid:Option<Fteid>,
    pub sgw_fteid:Option<Fteid>,
    pub pgw_fteid:Option<Fteid>,
    pub rnc_fteid:Option<Fteid>,
    pub epdg_fteid:Option<Fteid>,
    pub twan_fteid:Option<Fteid>,
    pub mme_fteid:Option<Fteid>,
    pub bearer_qos:Option<BearerQos>,
    pub charging_id:Option<ChargingId>,
    pub bearer_flags:Option<BearerFlags>,
    pub pco:Option<Pco>,
    pub apco: Option<Apco>,
    pub epco:Option<Epco>,
    pub max_packet_loss:Option<MaxPacketLossRate>,
    // pub ran_nas_cause:Option<RanNasCause>,
}

impl Default for BearerContext {
    fn default() -> Self {
        BearerContext { t: BEARER_CTX, 
                        length: 5,
                        ins:0,
                        ebi:Ebi::default(),
                        cause:None,
                        tft:None,
                        enb_fteid:None,
                        sgsn_fteid:None,
                        sgw_fteid:None,
                        pgw_fteid:None,
                        rnc_fteid:None,
                        epdg_fteid:None,
                        twan_fteid:None,
                        mme_fteid:None,
                        bearer_qos:None,
                        charging_id:None,
                        bearer_flags:None,
                        pco:None,
                        apco:None,
                        epco:None,
                        max_packet_loss:None,
                        //ran_nas_cause:None,    
                    }        
    }
}

impl IEs for BearerContext {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.indication);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= ACTION_IND_LENGTH + MIN_IE_SIZE {
            let mut data = ActionIndication::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.indication = buffer[4] & 0x07; 
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(ACTION_IND))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn action_indication_ie_unmarshal_test () {
    let encoded:[u8;5]=[0xa8, 0x00, 0x01, 0x00, 0x02];
    let decoded = ActionIndication { t:ACTION_IND, length: ACTION_IND_LENGTH as u16, ins:0, indication:0x02 };
    let i = ActionIndication::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn action_indication_ie_marshal_test () {
    let encoded:[u8;5]=[0xa8, 0x00, 0x01, 0x00, 0x02];
    let decoded = ActionIndication { t:ACTION_IND, length: ACTION_IND_LENGTH as u16, ins:0, indication:0x02 };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
