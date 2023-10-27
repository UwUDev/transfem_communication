use std::collections::HashMap;

const CHARSET: [&str; 8] = [":3", "ε:", ">:3", "ε:<", ":<", ">:", ">~<", ">-<"];

pub fn encode(bytes: Vec<u8>) -> String {
    let mut result = String::new();

    let mut buffer: u16 = 0;
    let mut buffer_size = 0;

    for byte in bytes {
        buffer = (buffer << 8) | byte as u16;
        buffer_size += 8;

        while buffer_size >= 3 {
            buffer_size -= 3;
            let index = ((buffer >> buffer_size) & 0b111) as usize;
            buffer &= (1 << buffer_size) - 1;

            result.push_str(CHARSET[index]);
            result.push(' ');
        }
    }

    if buffer_size > 0 {
        let index = (buffer << (3 - buffer_size)) as usize;
        result.push_str(CHARSET[index]);
    }

    result
}


pub fn encode_str(message: &str) -> String {
    return encode(message.as_bytes().to_vec());
}

pub fn decode(message: &str) -> Vec<u8> {
    let mut result = Vec::new();

    let map: HashMap<_, _> = CHARSET.iter().enumerate().map(|(i, &s)| (s, i)).collect();

    let characters = message.split(' ');

    let mut buffer: u16 = 0;
    let mut buffer_size = 0;

    for char in characters {
        if char.is_empty() {
            continue;
        }

        let value = *map.get(char).expect("Invalid character in input");
        buffer = (buffer << 3) | value as u16;
        buffer_size += 3;

        while buffer_size >= 8 {
            buffer_size -= 8;
            let byte = ((buffer >> buffer_size) & 0xFF) as u8;
            buffer &= (1 << buffer_size) - 1;

            result.push(byte);
        }
    }

    if buffer_size == 3 {
        let byte = buffer as u8;
        result.push(byte);
    }

    result
}

pub fn decode_str(message: &str) -> String {
    String::from_utf8(decode(message)).expect("Decoded bytes were not valid UTF-8")
}