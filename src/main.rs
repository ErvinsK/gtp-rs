//use std::io;
//use std::io::prelude::*;
use std::fs::File;
use pcap_parser::*;
use pcap_parser::traits::PcapReaderIterator;
//use ::gtpu::header;
use ::gtpu::messages;

fn main() {

    /*let pkt88:[u8;56] = [
        0x96, 0x76, 0x91, 0xa0, 0xaf, 0xc0, 0x02, 0xfe, /* .v...... */
        0x22, 0x09, 0x62, 0xba, 0x08, 0x00, 0x45, 0x68, /* ".b...Eh */
        0x00, 0x2a, 0x23, 0x16, 0x00, 0x00, 0x40, 0x11, /* .*#...@. */
        0x8c, 0xa9, 0x3e, 0x99, 0x89, 0x4d, 0x42, 0xc9, /* ..>..MB. */
        0xbf, 0xec, 0x08, 0x68, 0x8c, 0x20, 0x00, 0x16, /* ...h. .. */
        0x60, 0x94, 0x32, 0x02, 0x00, 0x06, 0x00, 0x00, /* `.2..... */
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x0e, 0x00  /* ........ */
    ];
    (let _packet:[u8;100] = [0x30, 0xff, 0x00, 0x5c, 0x34, 0xfe, /* ..0..\4. */
    0x1b, 0x27, 0x45, 0x00, 0x00, 0x5c, 0x00, 0xf2, /* .'E..\.. */
    0x00, 0x00, 0xff, 0x11, 0xbf, 0x0a, 0x64, 0x58, /* ......dX */
    0xbc, 0x9a, 0x0a, 0x40, 0xd0, 0x61, 0x07, 0x5d, /* ...@.a.] */
    0x07, 0x5c, 0x00, 0x48, 0xdd, 0xcd, 0x49, 0x1d, /* .\.H..I. */
    0xb0, 0xb5, 0x67, 0x4e, 0x62, 0xbe, 0x5b, 0x68, /* ..gNb.[h */
    0xa5, 0xbc, 0xab, 0xb6, 0x9e, 0x51, 0x15, 0xd2, /* .....Q.. */
    0xee, 0xcc, 0x2c, 0x6f, 0x5d, 0xf1, 0x76, 0xff, /* ..,o].v. */
    0x42, 0xeb, 0x48, 0x11, 0x83, 0x47, 0xaf, 0xa2, /* B.H..G.. */
    0xc4, 0x81, 0x28, 0x4f, 0x95, 0x05, 0x6e, 0x6e, /* ..(O..nn */
    0x84, 0xe3, 0xc2, 0x59, 0x40, 0x5a, 0xc1, 0xf7, /* ...Y@Z.. */
    0xc9, 0x91, 0x29, 0x73, 0x47, 0x5e, 0xab, 0x85, /* ..)sG^.. */
    0x34, 0x69, 0xa0, 0x0c, 0x50, 0x1d];
    
    let _echo_request:[u8;12]=[0x32, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x49, 0xca, 0x00, 0x00];

    let gpdu_sqn:Vec<u8> = vec![  0x30, 0xff, 0x00, 0x1e, 0x1b, 0x2f, 0xef, 0x7f, 0x45, 0x00, 0x00, 0x1e, 0x02, 0xc6, 0x00, 0x00, 0xff, 0x11, 0x3f, 0xdc, 
                            0x64, 0x72, 0xc3, 0x0d, 0x0a, 0x40, 0x47, 0x6d, 0x49, 0xc3, 0xc4, 0x89, 0x00, 0x0a, 0x44, 0x1e, 0x34, 0x42];

        */
    //let echo = vec![0x32, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00];

    //let echo_err = vec![0x32, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];

    //let pdu_with_seq = vec![0x32, 0xff, 0x00, 0x2c, 0x43, 0x80, 0x22, 0xb0, 0x0f, 0x3e, 0x00, 0x00];

    /*
    if let Ok(m)=header::GtpuHeader::unmarshal(&packet) {
    println! ("GTP-U version {} and protocol type {}", m.version, m.protocol_type);
    println! ("Extended header flag {}, sequence number flag {}, n-pdu flag {}", m.extension_header_flag, m.sequence_number_flag, m.npdu_number_flag);
    println! ("Message type {}", m.msgtype);
    println! ("Message length {}", m.length);
    println! ("TEID {}", m.teid);
    if let Some(i) = m.sequence_number { println! ("Sequence number {}", i); } else { println!("No sequence number!"); }
    if m.extension_headers.len()==0 {
        println! ("No extension headers");
    } else {
        println! ("Something else");
    }
    }
    */
    let file_name = "echo_response1.pcap";
    let file = File::open(file_name).expect("No file in the directory!");
    let mut reader = LegacyPcapReader::new(3000, file).expect("Pcap Reader did not initialized!");
    if let Ok((offset, _))=reader.next() {
        reader.consume(offset);
    }
    if let Ok((_, block)) = reader.next() {
        if let PcapBlockOwned::Legacy(b) = block {    
            match messages::GTPUMessage::unmarshal(&b.data[42..]) {
                Ok(messages::GTPUMessage::Gpdu(i)) => println! ("{:?}", i),
                Ok(messages::GTPUMessage::EchoRequest(i)) => println! ("{:?}", i),
                Ok(messages::GTPUMessage::EchoResponse(i)) => println! ("{:?}", i),
                Ok(messages::GTPUMessage::EndMarker(i)) => println! ("{:?}", i),
                Ok(messages::GTPUMessage::ErrorIndication(i)) => println! ("{:?}", i),
                Ok(messages::GTPUMessage::SupportedExtensionHeadersNotification(i)) => println!("{:?}", i),
                Err(i) => println! ("Error: {}", i),
            }
        }
    }
    /*if let messages::GTPUMessage::Gpdu(gpdu) = messages::GTPUMessage::unmarshal(&echo_request).unwrap() {
        println!("{:?}", gpdu);
    }
   */
    
    
/*
    if let Ok(m)=header::GtpuHeader::unmarshal(&echo) {
        println! ("GTP-U version {} and protocol type {}", m.version, m.protocol_type);
        println! ("Extended header flag {}, sequence number flag {}, n-pdu flag {}", m.extension_header_flag, m.sequence_number_flag, m.npdu_number_flag);
        println! ("Message type {}", m.msgtype);
        println! ("Message length {}", m.length);
        println! ("TEID {}", m.teid);
        if let Some(i) = m.sequence_number { println! ("Sequence number {}", i); }
    } 

    if let Ok(m)=header::GtpuHeader::unmarshal(&pdu_with_seq) {
        println! ("GTP-U version {} and protocol type {}", m.version, m.protocol_type);
        println! ("Extended header flag {}, sequence number flag {}, n-pdu flag {}", m.extension_header_flag, m.sequence_number_flag, m.npdu_number_flag);
        println! ("Message type {}", m.msgtype);
        println! ("Message length {}", m.length);
        println! ("TEID {:#x}", m.teid);
        if let Some(i) = m.sequence_number { println! ("Sequence number {:#x}", i);}
    } 
    */
}
