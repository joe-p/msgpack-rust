//! Example of using rmpv in a no_std environment
#![no_std]
#![cfg_attr(not(test), no_main)] // Only use no_main when not testing

extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;

use rmp::decode::Bytes;
use rmp::encode::ByteBuf;
use rmpv::Value;

// Simple no_std compatible helper function to demonstrate usage
pub fn encode_and_decode_msgpack() -> Value {
    // Example 1: Create a complex value
    let mut map_vec = Vec::new();
    map_vec.push((Value::from("compact"), Value::from(true)));
    map_vec.push((Value::from("schema"), Value::from(0)));
    
    let array = vec![
        Value::from(1),
        Value::from(2),
        Value::from(3),
    ];
    
    map_vec.push((Value::from("array"), Value::Array(array)));
    
    let complex = Value::Map(map_vec);
    
    // Example 2: Encode a Value
    let mut writer = ByteBuf::new();
    
    // Encode the value
    rmpv::encode::write_value(&mut writer, &complex).unwrap();
    let encoded = writer.as_slice();
    
    // Example 3: Decode a Value 
    let mut bytes = Bytes::new(encoded);
    let decoded: Value = rmpv::decode::read_value(&mut bytes).unwrap();
    
    // Example 4: Use ValueRef for zero-copy decoding
    let _value_ref = rmpv::decode::read_value_ref(&mut &*encoded).unwrap();
    
    // Return the decoded value
    decoded
}

// Entry point for examples (when not testing)
#[cfg_attr(not(test), no_mangle)]
pub fn main() {
    let _ = encode_and_decode_msgpack();
}

// Add a simple test to run in no_std environments
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_decode() {
        let value = encode_and_decode_msgpack();
        
        // Verify the structure
        if let Value::Map(map) = value {
            assert_eq!(map.len(), 3);
        } else {
            panic!("Expected map value");
        }
    }
}