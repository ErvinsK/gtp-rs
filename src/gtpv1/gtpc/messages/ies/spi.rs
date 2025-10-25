// Signalling Priority Indication (SPI) IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::{errors::GTPV1Error, gtpc::messages::ies::commons::*, utils::*};

// SPI IE TV

pub const SPI: u8 = 203;
pub const SPI_LENGTH: u16 = 1;

// SPI IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spi {
    pub t: u8,
    pub length: u16,
    pub lapi: bool, // LAPI - Low Access Priority Indication
}

impl Default for Spi {
    fn default() -> Spi {
        Spi {
            t: SPI,
            length: SPI_LENGTH,
            lapi: false,
        }
    }
}

impl IEs for Spi {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        match self.lapi {
            false => buffer_ie.push(0x00),
            true => buffer_ie.push(0x01),
        }
        set_tlv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error>
    where
        Self: Sized,
    {
        if buffer.len() >= (SPI_LENGTH + 3) as usize {
            let data = Spi {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                lapi: match buffer[3] {
                    0 => false,
                    1 => true,
                    _ => return Err(GTPV1Error::IEIncorrect),
                },
                ..Default::default()
            };
            Ok(data)
        } else {
            Err(GTPV1Error::IEInvalidLength)
        }
    }

    fn len(&self) -> usize {
        SPI_LENGTH as usize + 3
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[test]
fn spi_ie_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0xcb, 0x00, 0x01, 0x01];
    let test_struct = Spi {
        t: SPI,
        length: SPI_LENGTH,
        lapi: true,
    };
    let i = Spi::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn spi_ie_marshal_test() {
    let encoded_ie: [u8; 4] = [0xcb, 0x00, 0x01, 0x01];
    let test_struct = Spi {
        t: SPI,
        length: SPI_LENGTH,
        lapi: true,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
