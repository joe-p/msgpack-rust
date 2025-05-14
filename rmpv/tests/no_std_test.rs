//! Testing no_std compatibility
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec;

use rmp::decode::Bytes;
use rmp::encode::ByteBuf;
use rmpv::Value;

#[test]
fn test_value_encode_decode() {
    // Create a sample value
    let value = Value::from("hello no_std");

    // Encode the value
    let mut writer = ByteBuf::new();
    rmpv::encode::write_value(&mut writer, &value).unwrap();
    let encoded = writer.as_slice();

    // Decode the value
    let mut reader = Bytes::new(encoded);
    let decoded = rmpv::decode::read_value(&mut reader).unwrap();

    // Verify the result
    assert_eq!(value, decoded);
}

#[test]
fn test_value_ref() {
    // Create a sample value
    let value = Value::from("zero copy");

    // Encode the value
    let mut writer = ByteBuf::new();
    rmpv::encode::write_value(&mut writer, &value).unwrap();
    let encoded = writer.as_slice();

    // Decode as ValueRef
    let value_ref = rmpv::decode::read_value_ref(&mut &*encoded).unwrap();

    // Convert to owned and compare
    let decoded = value_ref.to_owned();
    assert_eq!(value, decoded);
}

#[test]
fn test_complex_value() {
    // Create a complex value with array
    let array = vec![
        Value::from(1),
        Value::from(2),
        Value::from(3),
    ];
    
    let original = Value::Array(array);

    // Encode the value
    let mut writer = ByteBuf::new();
    rmpv::encode::write_value(&mut writer, &original).unwrap();
    let encoded = writer.as_slice();

    // Decode the value
    let mut reader = Bytes::new(encoded);
    let decoded = rmpv::decode::read_value(&mut reader).unwrap();

    // Verify the result
    assert_eq!(original, decoded);
}