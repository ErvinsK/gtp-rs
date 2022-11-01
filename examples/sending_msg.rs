use std::net::UdpSocket;
use ::gtp_rs::gtpv1::gtpu::{*};
use std::net::{IpAddr,Ipv6Addr};

fn main() {
    let mut send_header=Gtpv1Header::default();
    send_header.msgtype=ERROR_INDICATION;
    send_header.sequence_number=Some(2000);
    send_header.teid=4000;
    let mut port = UDPPort::default();
    port.udp_port=6511;
    send_header.extension_headers = Some(vec!(ExtensionHeader::UDPPort(port)));
    let mut buffer:Vec<u8> = vec!();
    let mut message = ErrorIndication::default();
    message.header=send_header;
    message.teid_data=Teid { t:TEID_DATA, teid:5000};
    message.peer_addr= GsnAddress { t:GSN_ADDRESS, length: 16, ip: IpAddr::V6(Ipv6Addr::new(0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff))};
    println!("Message to be sent {:?}", message);
    message.marshal(&mut buffer);
    println!("Marshaled buffer {:?}", buffer);
    let socket = UdpSocket::bind("127.0.1.1:33000").expect("failed to bind to address");
    socket.send_to(&buffer, "127.0.0.1:2152").expect("couldn't send data");
}
