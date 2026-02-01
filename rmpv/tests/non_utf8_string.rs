use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct NonUtf8String(pub Vec<u8>);

impl NonUtf8String {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl<'de> Deserialize<'de> for NonUtf8String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NonUtf8StringVisitor;

        impl<'de> serde::de::Visitor<'de> for NonUtf8StringVisitor {
            type Value = NonUtf8String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string (possibly non-UTF8)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonUtf8String(v.as_bytes().to_vec()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonUtf8String(v.into_bytes()))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonUtf8String(v.to_vec()))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NonUtf8String(v))
            }
        }

        deserializer.deserialize_any(NonUtf8StringVisitor)
    }
}

impl Serialize for NonUtf8String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Use the new MSGPACK_RAW_STR_STRUCT_NAME mechanism to serialize
        // raw bytes as MessagePack string without UTF-8 validation
        serializer.serialize_newtype_struct(
            rmpv::MSGPACK_RAW_STR_STRUCT_NAME,
            serde_bytes::Bytes::new(&self.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::NonUtf8String;

    #[test]
    fn test_rmpv_to_value_non_utf8_string() {
        let original = NonUtf8String::new(vec![0xff, 0xfe, 0xfd]);

        // Test that rmpv::ext::to_value works correctly
        let value = rmpv::ext::to_value(&original).unwrap();

        // Verify it's a Value::String, not Value::Binary
        assert!(matches!(value, rmpv::Value::String(_)));

        // Verify the bytes match
        if let rmpv::Value::String(s) = value {
            assert_eq!(s.as_bytes(), &[0xff, 0xfe, 0xfd]);
        } else {
            panic!("Expected Value::String");
        }
    }

    #[test]
    fn test_rmpv_round_trip_non_utf8_string() {
        let original = NonUtf8String::new(vec![0xff, 0xfe, 0xfd]);

        // Serialize to Value
        let value = rmpv::ext::to_value(&original).unwrap();

        // Verify it's a String type
        assert!(matches!(value, rmpv::Value::String(_)));

        // Encode to MessagePack bytes
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &value).unwrap();

        // Verify the bytes are correct (fixstr with length 3)
        assert_eq!(buf, vec![0xa3, 0xff, 0xfe, 0xfd]);

        // Decode back to Value
        let decoded_value = rmpv::decode::read_value(&mut &buf[..]).unwrap();

        // Should still be a String
        assert!(matches!(decoded_value, rmpv::Value::String(_)));

        // Deserialize back to NonUtf8String
        let deserialized: NonUtf8String = rmpv::ext::from_value(decoded_value).unwrap();

        // Should match the original
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_rmpv_value_string_non_utf8_encode_decode() {
        // Start with raw MessagePack bytes representing a non-UTF8 string
        // 0xa3 = fixstr with length 3, followed by non-UTF8 bytes
        let msgpack_bytes = vec![0xa3, 0xff, 0xfe, 0xfd];
        
        // Decode to Value - should be a String (not Binary)
        let value = rmpv::decode::read_value(&mut &msgpack_bytes[..]).unwrap();
        
        // Verify it's a String type
        assert!(matches!(value, rmpv::Value::String(_)));
        
        // Verify the bytes match
        if let rmpv::Value::String(ref s) = value {
            assert_eq!(s.as_bytes(), &[0xff, 0xfe, 0xfd]);
        } else {
            panic!("Expected Value::String");
        }

        // Now re-encode it back to MessagePack
        let mut encoded_buf = Vec::new();
        rmpv::encode::write_value(&mut encoded_buf, &value).unwrap();

        // Should produce identical bytes (string format, NOT binary format)
        // This is the critical test - it should remain as fixstr (0xa3), not become bin8 (0xc4)
        assert_eq!(encoded_buf, msgpack_bytes);
        
        // Decode again to verify round-trip
        let decoded_again = rmpv::decode::read_value(&mut &encoded_buf[..]).unwrap();
        assert!(matches!(decoded_again, rmpv::Value::String(_)));
        
        if let rmpv::Value::String(s) = decoded_again {
            assert_eq!(s.as_bytes(), &[0xff, 0xfe, 0xfd]);
        }
    }
}
