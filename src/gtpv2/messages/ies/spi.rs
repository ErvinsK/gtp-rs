// Signalling Priority Indication (SPI) IE - according to 3GPP TS 29.274 V15.5.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// SPI IE TV

pub const SPI:u8 = 157;
pub const SPI_LENGTH:usize = 1;

// SPI IE implementation 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spi {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub lapi:bool,          // LAPI - Low Access Priority Indication
}

impl Default for Spi {
    fn default() -> Spi {
        Spi { t: SPI, length:SPI_LENGTH as u16, ins:0, lapi:false }        
    }
}

impl From<Spi> for InformationElement {
    fn from(i: Spi) -> Self {
        InformationElement::Spi(i)
    }
}

impl IEs for Spi {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.lapi {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= SPI_LENGTH + MIN_IE_SIZE {
            let mut data = Spi::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            match buffer[4] {
                0 => data.lapi = false,
                1 => data.lapi = true,
                _ => return Err(GTPV2Error::IEIncorrect(SPI)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SPI))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn spi_ie_unmarshal_test () {
    let encoded:[u8;5]=[0x9d, 0x00, 0x01, 0x00, 0x01];
    let decoded = Spi { t:SPI, length: SPI_LENGTH as u16, ins:0, lapi: true };
    let i = Spi::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn spi_ie_marshal_test () {
    let encoded:[u8;5]=[0x9d, 0x00, 0x01, 0x00, 0x01];
    let decoded = Spi { t:SPI, length: SPI_LENGTH as u16, ins:0, lapi: true };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
