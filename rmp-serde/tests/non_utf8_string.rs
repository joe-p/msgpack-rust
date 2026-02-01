use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Debug)]
pub struct NonUtf8String(pub Vec<u8>);

impl NonUtf8String {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
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
            rmp_serde::MSGPACK_RAW_STR_STRUCT_NAME,
            serde_bytes::Bytes::new(&self.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::NonUtf8String;
    use serde::Serialize;

    #[test]
    fn test_serialize_non_utf8_string() {
        let original = NonUtf8String::new(vec![0xff, 0xfe, 0xfd]);

        let mut serde_serialized_buf = Vec::new();

        let mut serde_serializer = rmp_serde::Serializer::new(&mut serde_serialized_buf);
        original.serialize(&mut serde_serializer).unwrap();

        // Confirm that we serialized as a fixstr in MessagePack
        assert_eq!(
            serde_serialized_buf,
            vec![0xa3, 0xff, 0xfe, 0xfd] // fixstr with length 3 followed by bytes
        );

        let rmpv_decoded =
            rmpv::decode::read_value(&mut Cursor::new(&serde_serialized_buf)).unwrap();

        // Confirm that rmpv::decode reads it as a string value
        assert!(matches!(rmpv_decoded, rmpv::Value::String(_)));

        // Attempt to deserialize back to NonUtf8String
        let deserialized: NonUtf8String =
            rmp_serde::from_slice(&serde_serialized_buf).unwrap();

        assert_eq!(original, deserialized);
    }
}
