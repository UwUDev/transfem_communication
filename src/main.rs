use transfem_communication::encode_str;
use transfem_communication::decode_str;

fn main() {
    let message = "Also you can just encode bytes and decode them back and I think that every optimised protocol should have that.";
    let encoded = encode_str(message);
    println!("Encoded: {}", encoded);
    let decoded = decode_str(&encoded);
    println!("Decoded: {}", decoded);
}