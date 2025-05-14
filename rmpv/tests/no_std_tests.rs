#![cfg_attr(not(feature = "std"), no_std)]

// Only run these tests when built without std feature
#[cfg(not(feature = "std"))]
mod tests {
    extern crate alloc;
    
    use alloc::vec;
    
    use rmp::decode::Bytes;
    use rmp::encode::ByteBuf;
    use rmpv::Value;
    
    #[test]
    fn test_encode_decode_string() {
        let original = Value::from("Hello World");
        
        // Encode
        let mut writer = ByteBuf::new();
        rmpv::encode::write_value(&mut writer, &original).unwrap();
        let encoded = writer.as_slice();
        
        // Decode
        let mut reader = Bytes::new(encoded);
        let decoded = rmpv::decode::read_value(&mut reader).unwrap();
        
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_encode_decode_numbers() {
        let values = vec![
            Value::from(0),
            Value::from(-1),
            Value::from(42),
            Value::from(127),
            Value::from(128),
            Value::from(255),
            Value::from(256),
            Value::from(65535),
            Value::from(65536),
            Value::from(-128),
            Value::from(-129),
            Value::from(-32768),
            Value::from(-32769),
        ];
        
        for original in values {
            // Encode
            let mut writer = ByteBuf::new();
            rmpv::encode::write_value(&mut writer, &original).unwrap();
            let encoded = writer.as_slice();
            
            // Decode
            let mut reader = Bytes::new(encoded);
            let decoded = rmpv::decode::read_value(&mut reader).unwrap();
            
            assert_eq!(original, decoded);
        }
    }
    
    #[test]
    fn test_encode_decode_array() {
        let original = Value::Array(vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from("test"),
        ]);
        
        // Encode
        let mut writer = ByteBuf::new();
        rmpv::encode::write_value(&mut writer, &original).unwrap();
        let encoded = writer.as_slice();
        
        // Decode
        let mut reader = Bytes::new(encoded);
        let decoded = rmpv::decode::read_value(&mut reader).unwrap();
        
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_zero_copy_value_ref() {
        let original = Value::from("zero copy");
        
        // Encode
        let mut writer = ByteBuf::new();
        rmpv::encode::write_value(&mut writer, &original).unwrap();
        let encoded = writer.as_slice();
        
        // Decode with ValueRef (zero-copy)
        let value_ref = rmpv::decode::read_value_ref(&mut &*encoded).unwrap();
        
        // Convert to owned Value and compare
        assert_eq!(original, value_ref.to_owned());
    }
}