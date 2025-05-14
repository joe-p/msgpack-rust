#![cfg(feature = "std")]

use std::io::Cursor;
use rmpv::decode::{read_value_ref, Error};
use rmpv::ValueRef;

// A helper for tests that require std features
pub fn read_value_from_cursor<'a>(buf: &'a [u8]) -> Result<ValueRef<'a>, Error> {
    let mut cursor = Cursor::new(buf);
    read_value_ref(&mut cursor)
}

// A helper to read multiple values from a buffer
pub fn read_values_from_cursor<'a>(buf: &'a [u8], count: usize) -> Vec<Result<ValueRef<'a>, Error>> {
    let mut cursor = Cursor::new(buf);
    let mut results = Vec::with_capacity(count);
    
    for _ in 0..count {
        results.push(read_value_ref(&mut cursor));
    }
    
    results
}