use super::{write_marker, RmpWrite};
use crate::encode::ValueWriteError;
use crate::Marker;

/// Encodes and attempts to write the most efficient string length implementation to the given
/// write, returning the marker used.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_str_len<W: RmpWrite>(wr: &mut W, len: u32) -> Result<Marker, ValueWriteError<W::Error>> {
    let marker = if len < 32 {
        Marker::FixStr(len as u8)
    } else if len < 256 {
        Marker::Str8
    } else if u16::try_from(len).is_ok() {
        Marker::Str16
    } else {
        Marker::Str32
    };

    write_marker(wr, marker)?;
    if marker == Marker::Str8 {
        wr.write_data_u8(len as u8)?;
    }
    if marker == Marker::Str16 {
        wr.write_data_u16(len as u16)?;
    }
    if marker == Marker::Str32 {
        wr.write_data_u32(len)?;
    }
    Ok(marker)
}

/// Encodes and attempts to write the most efficient string binary representation to the
/// given `Write`.
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
// TODO: Docs, range check, example, visibility.
pub fn write_str<W: RmpWrite>(wr: &mut W, data: &str) -> Result<(), ValueWriteError<W::Error>> {
    write_str_len(wr, data.len() as u32)?;
    wr.write_bytes(data.as_bytes()).map_err(ValueWriteError::InvalidDataWrite)
}

/// Encodes and attempts to write raw bytes as a MessagePack string without UTF-8 validation.
///
/// This function writes the string marker and length, followed by the raw bytes, without
/// validating that the bytes form valid UTF-8. Use this only when you need to preserve
/// non-UTF-8 data as a MessagePack string type (rather than binary type).
///
/// # Errors
///
/// This function will return `ValueWriteError` on any I/O error occurred while writing either the
/// marker or the data.
pub fn write_str_bytes<W: RmpWrite>(wr: &mut W, bytes: &[u8]) -> Result<(), ValueWriteError<W::Error>> {
    write_str_len(wr, bytes.len() as u32)?;
    wr.write_bytes(bytes).map_err(ValueWriteError::InvalidDataWrite)
}
