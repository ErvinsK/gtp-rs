// Common utils

// IETF Protocol Numbers

pub const IPV4: u8 = 0x21;
pub const IPV6: u8 = 0x57;
pub const IPV4V6: u8 = 0x8d;

// Telephony numbering plans

pub const E164: u8 = 0x01;
pub const E212: u8 = 0x06;

// Encode string into TBCD format, return slice of bytes

pub fn tbcd_encode(number: &str) -> Vec<u8> {
    let mut chr: Vec<u8> = number
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|x| x as u8)
        .collect();
    if chr.len() % 2 != 0 {
        chr.push(0x0f);
    }
    let tbcd: Vec<u8> = chr
        .iter()
        .step_by(2)
        .zip(chr.iter().skip(1).step_by(2))
        .map(|(a, b)| (b << 4) | a)
        .collect();
    tbcd
}

#[test]
fn tbcd_encode_test() {
    let test_number: &str = "987432101314063";
    let encoded_number: [u8; 8] = [137, 71, 35, 1, 49, 65, 96, 243];
    assert_eq!(tbcd_encode(test_number), encoded_number);
}

// Decode slice of bytes from TBCD to string

pub fn tbcd_decode(buffer: &[u8]) -> String {
    let mut data: Vec<u8> = vec![];
    for x in buffer.iter() {
        data.push(x & 0b00001111);
        data.push(x >> 4);
    }
    let result: String = data
        .iter()
        .flat_map(|c| char::from_digit(*c as u32, 10))
        .collect();
    result
}

#[test]
fn tbcd_decode_test() {
    let test_number: String = "987432101314063".to_string();
    let encoded_number: [u8; 8] = [137, 71, 35, 1, 49, 65, 96, 243];
    assert_eq!(tbcd_decode(&encoded_number), test_number);
}

// Encode MCC and MNC

pub fn mcc_mnc_encode(mcc: u16, mnc: u16, mnc_is_three_digits: bool) -> Vec<u8> {
    let mut mcc_digits: Vec<u8> = to_digits(mcc);
    while mcc_digits.len() < 3 {
        mcc_digits.insert(0, 0);
    }

    let mut mnc_digits: Vec<u8> = to_digits(mnc);
    let encode_three_digit_mnc = mnc_is_three_digits || mnc >= 100;
    if encode_three_digit_mnc && mnc_digits.len() < 3 {
        mnc_digits.insert(0, 0);
    }

    let mut result: Vec<u8> = vec![];
    result.push(mcc_digits[1] << 4 | mcc_digits[0]);
    if encode_three_digit_mnc {
        result.push(mnc_digits[2] << 4 | mcc_digits[2]);
    } else {
        result.push(0b1111 << 4 | mcc_digits[2]);
    }
    result.push(mnc_digits[1] << 4 | mnc_digits[0]);
    result
}

#[test]
fn mcc_mnc_encode_test() {
    // 2-digit MNC (MCC=262, MNC=01)
    let test_mcc_262: u16 = 262;
    let test_mnc_01: u16 = 1;
    let encoded_number_262_01: [u8; 3] = [0x62, 0xf2, 0x10];
    assert_eq!(
        mcc_mnc_encode(test_mcc_262, test_mnc_01, false),
        encoded_number_262_01
    );

    // 3-digit MNC naturally (MCC=310, MNC=123)
    let test_mcc_310: u16 = 310;
    let test_mnc_123: u16 = 123;
    let encoded_number_310_123: [u8; 3] = [0x13, 0x30, 0x21];
    assert_eq!(
        mcc_mnc_encode(test_mcc_310, test_mnc_123, false),
        encoded_number_310_123
    );

    // Forced 3-digit MNC (MCC=310, MNC=012)
    let test_mnc_012: u16 = 12;
    let encoded_number_310_012: [u8; 3] = [0x13, 0x00, 0x21];
    assert_eq!(
        mcc_mnc_encode(test_mcc_310, test_mnc_012, true),
        encoded_number_310_012
    );
}

// Decode MCC and MNC

pub fn mcc_mnc_decode(buffer: &[u8]) -> (u16, u16, bool) {
    let mut mcc_digits: Vec<u8> = vec![];
    let mut mnc_digits: Vec<u8> = vec![];
    mcc_digits.push(buffer[0] & 0b1111);
    mcc_digits.push(buffer[0] >> 4);
    mcc_digits.push(buffer[1] & 0b00001111);
    mnc_digits.push(buffer[2] & 0b1111);
    mnc_digits.push(buffer[2] >> 4);
    let third_digit = buffer[1] >> 4;
    let mnc_is_three_digits = third_digit != 0b1111;
    if mnc_is_three_digits {
        mnc_digits.push(third_digit);
    }
    let (mut mcc, mut mnc) = (0, 0);
    if let Ok(i) = mcc_digits
        .iter()
        .flat_map(|c| char::from_digit(*c as u32, 10))
        .collect::<String>()
        .parse::<u16>()
    {
        mcc = i;
    }
    if let Ok(i) = mnc_digits
        .iter()
        .flat_map(|c| char::from_digit(*c as u32, 10))
        .collect::<String>()
        .parse::<u16>()
    {
        mnc = i;
    }
    (mcc, mnc, mnc_is_three_digits)
}

#[test]
fn mcc_mnc_decode_test() {
    // 2-digit MNC (MCC=262, MNC=01)
    let test_mcc_262: u16 = 262;
    let test_mnc_01: u16 = 1;
    let encoded_number_262_01: [u8; 3] = [0x62, 0xf2, 0x10];
    assert_eq!(
        mcc_mnc_decode(&encoded_number_262_01),
        (test_mcc_262, test_mnc_01, false)
    );

    // 3-digit MNC naturally (MCC=310, MNC=123)
    let test_mcc_310: u16 = 310;
    let test_mnc_123: u16 = 123;
    let encoded_number_310_123: [u8; 3] = [0x13, 0x30, 0x21];
    assert_eq!(
        mcc_mnc_decode(&encoded_number_310_123),
        (test_mcc_310, test_mnc_123, true)
    );

    // Forced 3-digit MNC (MCC=310, MNC=012)
    let test_mnc_012: u16 = 12;
    let encoded_number_310_012: [u8; 3] = [0x13, 0x00, 0x21];
    assert_eq!(
        mcc_mnc_decode(&encoded_number_310_012),
        (test_mcc_310, test_mnc_012, true)
    );
}

// Convert unsigned int to vector of digits

pub fn to_digits<T: ToString>(i: T) -> Vec<u8> {
    let mut result: Vec<u8> = i
        .to_string()
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|x| x as u8)
        .collect();
    if result.len() == 1 {
        result.insert(0, 0);
    }
    result
}

// Set the right size of GTP message based on buffer size

pub fn set_length(buffer: &mut [u8]) {
    let size = ((buffer.len() - 8) as u16).to_be_bytes();
    buffer[2] = size[0];
    buffer[3] = size[1];
}

// Set the right size of IE based on buffer size

pub fn set_tlv_ie_length(buffer: &mut [u8]) {
    let size = ((buffer.len() - 3) as u16).to_be_bytes();
    buffer[1] = size[0];
    buffer[2] = size[1];
}

// Check TLV IE length vs buffer size

pub fn check_tlv_ie_buffer(length: u16, buffer: &[u8]) -> bool {
    match length {
        0 => false,
        _ => (length + 3) as usize <= buffer.len(),
    }
}
