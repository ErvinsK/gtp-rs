// Signalling Priority Indication (SPI) IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// SPI IE TV

pub const SPI: u8 = 157;
pub const SPI_LENGTH: usize = 1;

// SPI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spi {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub lapi: bool, // LAPI - Low Access Priority Indication
}

impl Default for Spi {
    fn default() -> Spi {
        Spi {
            t: SPI,
            length: SPI_LENGTH as u16,
            ins: 0,
            lapi: false,
        }
    }
}

impl From<Spi> for InformationElement {
    fn from(i: Spi) -> Self {
        InformationElement::Spi(i)
    }
}

impl IEs for Spi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SPI);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.lapi {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= SPI_LENGTH + MIN_IE_SIZE {
            let data = Spi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                lapi: match buffer[4] {
                    0 => false,
                    1 => true,
                    _ => return Err(GTPV2Error::IEIncorrect(SPI)),
                },
                ..Spi::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SPI))
        }
    }

    fn len(&self) -> usize {
        SPI_LENGTH + MIN_IE_SIZE
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

#[test]
fn spi_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x9d, 0x00, 0x01, 0x00, 0x01];
    let decoded = Spi {
        t: SPI,
        length: SPI_LENGTH as u16,
        ins: 0,
        lapi: true,
    };
    let i = Spi::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn spi_ie_marshal_test() {
    let encoded: [u8; 5] = [0x9d, 0x00, 0x01, 0x00, 0x01];
    let decoded = Spi {
        t: SPI,
        length: SPI_LENGTH as u16,
        ins: 0,
        lapi: true,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
