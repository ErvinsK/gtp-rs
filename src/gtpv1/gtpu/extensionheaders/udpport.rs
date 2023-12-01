use crate::gtpv1::{errors::GTPV1Error, gtpu::extensionheaders::commons::*};

pub const UDP_PORT: u8 = 0x40;
pub const UDP_PORT_LENGTH: u8 = 1;

// Struct for UDP Port Extension Header

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UDPPort {
    pub extension_header_type: u8,
    pub length: u8,
    pub udp_port: u16,
}

impl Default for UDPPort {
    fn default() -> UDPPort {
        UDPPort {
            extension_header_type: UDP_PORT,
            length: UDP_PORT_LENGTH,
            udp_port: 0,
        }
    }
}

impl ExtensionHeaders for UDPPort {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.extension_header_type);
        buffer.push(self.length);
        buffer.extend_from_slice(&self.udp_port.to_be_bytes());
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut data = UDPPort {
            length: match buffer[1] {
                0 => return Err(GTPV1Error::ExtHeaderInvalidLength),
                _ => buffer[1],
            },
            ..Default::default()
        };
        if (data.length * 4) as usize <= buffer.len() {
            data.udp_port = u16::from_be_bytes([buffer[2], buffer[3]]);
            Ok(data)
        } else {
            Err(GTPV1Error::ExtHeaderInvalidLength)
        }
    }

    fn len(&self) -> usize {
        (self.length * 4) as usize
    }

    fn is_empty(&self) -> bool {
        self.udp_port == 0
    }
}

#[test]
fn udp_port_exthdr_unmarshal_test() {
    let encoded_ie: [u8; 4] = [0x40, 0x01, 0x10, 0x00];
    let test_struct = UDPPort {
        extension_header_type: UDP_PORT,
        length: UDP_PORT_LENGTH,
        udp_port: 4096,
    };
    let i = UDPPort::unmarshal(&encoded_ie);
    assert_eq!(i.unwrap(), test_struct);
}

#[test]
fn udp_port_exthdr_marshal_test() {
    let encoded_ie: [u8; 4] = [0x40, 0x01, 0x10, 0x00];
    let test_struct = UDPPort {
        extension_header_type: UDP_PORT,
        length: UDP_PORT_LENGTH,
        udp_port: 4096,
    };
    let mut buffer: Vec<u8> = vec![];
    test_struct.marshal(&mut buffer);
    assert_eq!(buffer, encoded_ie);
}
