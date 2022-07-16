// Common utils

// IETF Protocol Numbers

pub const IPV4:u8 = 0x21;
pub const IPV6:u8 = 0x57;
pub const IPV4V6:u8 = 0x8d;

// Encode string into TBCD format, return slice of bytes

pub fn tbcd_encode (imsi:&str) -> Vec<u8> {
    let mut chr:Vec<u8>=imsi.chars().flat_map( |c| c.to_digit(10)).map( |x| x as u8).collect();
    if chr.len() % 2 != 0 {
        chr.push(0x0f);
    }
    let tbcd:Vec<u8>=chr.iter().step_by(2).zip(chr.iter().skip(1).step_by(2)).map( |(a,b)| (b<<4)|a ).collect();
    tbcd
}

#[test]
fn tbcd_encode_test () {
    let test_imsi:&str="987432101314063";
    let encoded_imsi:[u8;8]=[137, 71, 35, 1, 49, 65, 96, 243];
    assert_eq!(tbcd_encode(test_imsi),encoded_imsi);
}

// Decode slice of bytes from TBCD to string

pub fn tbcd_decode (buffer:&[u8]) -> String {
    let mut data:Vec<u8>=vec!();
    for x in buffer.iter() {
       data.push(x & 0b00001111);
       data.push(x >> 4);
    }
    let result:String=data.iter().flat_map( |c| char::from_digit(*c as u32,10)).into_iter().collect();
    result
}

#[test]
fn tbcd_decode_test () {
    let test_imsi:String="987432101314063".to_string();
    let encoded_imsi:[u8;8]=[137, 71, 35, 1, 49, 65, 96, 243];
    assert_eq!(tbcd_decode(&encoded_imsi), test_imsi);
}

// Convert unsigned int to vector of digits

pub fn to_digits <T:ToString> (i:T) -> Vec<u8> {
    i.to_string().chars().flat_map(|c| c.to_digit(10)).map( |x| x as u8 ).collect()
}

// Set the right size of GTP message based on buffer size

pub fn set_length (buffer: &mut Vec<u8>) {
    let size = ((buffer.len()-8) as u16).to_be_bytes();
    buffer[2]=size[0];
    buffer[3]=size[1];             
} 

// Set the right size of IE based on buffer size

pub fn set_tlv_ie_length (buffer: &mut Vec<u8>) {
    let size = ((buffer.len()-3) as u16).to_be_bytes();
    buffer[1]=size[0];
    buffer[2]=size[1]; 
}

// Check TLV IE length vs buffer size

pub fn check_tlv_ie_buffer (length:u16, buffer:&[u8]) -> bool {
    if (length+3) as usize <= buffer.len() {
        true
    } else {
        false
    }
}