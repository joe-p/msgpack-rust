//! Example of using rmpv in a std environment
#[cfg(feature = "std")]
use std::io::Cursor;

#[cfg(feature = "std")]
use rmpv::Value;

#[cfg(not(feature = "std"))]
fn main() {
    // This example only works with std enabled
    // See no_std_example.rs for a no_std example
}

#[cfg(feature = "std")]
fn main() {
    // Example 1: Encode and decode a simple Value
    println!("Example 1: Simple Value encoding/decoding");
    let value = Value::from("hello std");
    
    // Encode the value to a Vec<u8>
    let mut buf = Vec::new();
    rmpv::encode::write_value(&mut buf, &value).unwrap();
    println!("Encoded bytes: {:?}", buf);
    
    // Decode the value
    let bytes = &buf[..];
    let decoded: Value = rmpv::decode::read_value(&mut &*bytes).unwrap();
    println!("Decoded value: {:?}", decoded);
    
    // Example 2: Working with complex structures
    println!("\nExample 2: Complex structure");
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
    println!("Original complex value: {:?}", complex);
    
    // Encode
    let mut buf = Vec::new();
    rmpv::encode::write_value(&mut buf, &complex).unwrap();
    println!("Encoded bytes: {:?}", buf);
    
    // Decode
    let bytes = &buf[..];
    let decoded: Value = rmpv::decode::read_value(&mut &*bytes).unwrap();
    println!("Decoded complex value: {:?}", decoded);
    
    // Example 3: Zero-copy value decoding with ValueRef
    println!("\nExample 3: Zero-copy decoding");
    let slice = &buf[..];
    let value_ref = rmpv::decode::read_value_ref(&mut &*slice).unwrap();
    println!("Zero-copy decoded value: {:?}", value_ref);
    
    // Convert from ValueRef to Value (involves copying)
    let owned_value = value_ref.to_owned();
    println!("After conversion to owned Value: {:?}", owned_value);
    
    // Example 4: Simple value access methods
    println!("\nExample 4: Value access methods");
    if let Value::Map(ref map) = decoded {
        for (key, value) in map {
            if let Value::String(ref s) = key {
                if let Some(key_str) = s.as_str() {
                    println!("Key: {}", key_str);
                    match key_str {
                        "compact" => {
                            if let Value::Boolean(b) = value {
                                println!("  compact = {}", b);
                            }
                        },
                        "schema" => {
                            if let Value::Integer(i) = value {
                                println!("  schema = {}", i);
                            }
                        },
                        "array" => {
                            if let Value::Array(ref arr) = value {
                                print!("  array = [");
                                for (i, item) in arr.iter().enumerate() {
                                    if i > 0 { print!(", "); }
                                    print!("{}", item);
                                }
                                println!("]");
                            }
                        },
                        _ => println!("  Unknown key")
                    }
                }
            }
        }
    }
}