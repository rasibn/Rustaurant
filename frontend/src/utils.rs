use serde::{Serialize, Deserialize};
use hex::{ToHex, FromHex}; // Import the FromHex trait
use serde_json;

pub fn struct_to_hex<T: Serialize>(data: &T) -> String {
    let json_string = serde_json::to_string(data).expect("Serialization failed");
    let bytes = json_string.as_bytes();
    bytes.encode_hex::<String>()
}

pub fn hex_to_struct<T: for<'a> Deserialize<'a>>(hex_string: &str) -> Option<T> {
    let bytes = <Vec<u8> as FromHex>::from_hex(hex_string).ok()?; // Use FromHex::from_hex
    let json_string = String::from_utf8_lossy(&bytes);
    serde_json::from_str(&json_string).ok()
}
