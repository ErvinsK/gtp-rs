use ::gtp_rs::gtpv1::gtpu::*;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv6Addr};

fn main() {
    let mut buffer: Vec<u8> = vec![];
    let message = ErrorIndication {
        header: Gtpv1Header {
            msgtype: ERROR_INDICATION,
            sequence_number: Some(2000),
            teid: 4001,
            extension_headers: Some(vec![ExtensionHeader::UDPPort(UDPPort {
                udp_port: 6511,
                ..UDPPort::default()
            })]),
            ..Gtpv1Header::default()
        },
        teid_data: Teid {
            teid: 5000,
            ..Teid::default()
        },
        peer_addr: GsnAddress {
            ip: IpAddr::V6(Ipv6Addr::new(
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            )),
            ..GsnAddress::default()
        },
        ..ErrorIndication::default()
    };
    println!("Message to be sent {:?}", message);
    message.marshal(&mut buffer);
    println!("Marshaled buffer {:?}", buffer);
    let socket = UdpSocket::bind("127.0.1.1:33000").expect("failed to bind to address");
    socket
        .send_to(&buffer, "127.0.0.1:2152")
        .expect("couldn't send data");
}
