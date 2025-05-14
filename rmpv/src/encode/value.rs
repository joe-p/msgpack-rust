#[cfg(feature = "std")]
#[cfg(feature = "std")]
use rmp::encode::{
    write_array_len, write_bin, write_bool, write_ext_meta, write_f32, write_f64, write_map_len,
    write_nil, write_sint, write_str, write_uint, RmpWrite, ValueWriteError,
};

#[cfg(not(feature = "std"))]
use rmp::encode::{
    write_array_len, write_bin, write_bool, write_ext_meta, write_f32, write_f64, write_map_len,
    write_nil, write_sint, write_str, write_uint, RmpWrite, ValueWriteError,
};
use crate::{IntPriv, Integer, Utf8String, Value};

/// Encodes and attempts to write the most efficient representation of the given Value.
///
/// # Note
#[cfg(feature = "std")]
/// All instances of `ErrorKind::Interrupted` are handled by this function and the underlying
/// operation is retried.
#[cfg(feature = "std")]
pub fn write_value<W>(wr: &mut W, val: &Value) -> Result<(), ValueWriteError<W::Error>>
    where W: RmpWrite
{
    match *val {
        Value::Nil => {
            write_nil(wr).map_err(ValueWriteError::InvalidMarkerWrite)?;
        }
        Value::Boolean(val) => {
            write_bool(wr, val).map_err(ValueWriteError::InvalidMarkerWrite)?;
        }
        Value::Integer(Integer { n }) => match n {
            IntPriv::PosInt(n) => {
                write_uint(wr, n)?;
            }
            IntPriv::NegInt(n) => {
                write_sint(wr, n)?;
            }
        },
        Value::F32(val) => {
            write_f32(wr, val)?;
        }
        Value::F64(val) => {
            write_f64(wr, val)?;
        }
        Value::String(Utf8String { ref s }) => match *s {
            Ok(ref val) => write_str(wr, val)?,
            Err(ref err) => write_bin(wr, &err.0)?,
        },
        Value::Binary(ref val) => {
            write_bin(wr, val)?;
        }
        Value::Array(ref vec) => {
            write_array_len(wr, vec.len() as u32)?;
            for v in vec {
                write_value(wr, v)?;
            }
        }
        Value::Map(ref map) => {
            write_map_len(wr, map.len() as u32)?;
            for (key, val) in map {
                write_value(wr, key)?;
                write_value(wr, val)?;
            }
        }
        Value::Ext(ty, ref data) => {
            write_ext_meta(wr, data.len() as u32, ty)?;
            wr.write_bytes(data).map_err(ValueWriteError::InvalidDataWrite)?;
        }
    }

    Ok(())
}

/// No-std version of write_value
#[cfg(not(feature = "std"))]
pub fn write_value<W>(wr: &mut W, val: &Value) -> Result<(), ValueWriteError<W::Error>>
    where W: RmpWrite
{
    match *val {
        Value::Nil => {
            write_nil(wr).map_err(ValueWriteError::InvalidMarkerWrite)?;
        }
        Value::Boolean(val) => {
            write_bool(wr, val).map_err(ValueWriteError::InvalidMarkerWrite)?;
        }
        Value::Integer(Integer { n }) => match n {
            IntPriv::PosInt(n) => {
                write_uint(wr, n)?;
            }
            IntPriv::NegInt(n) => {
                write_sint(wr, n)?;
            }
        },
        Value::F32(val) => {
            write_f32(wr, val)?;
        }
        Value::F64(val) => {
            write_f64(wr, val)?;
        }
        Value::String(Utf8String { ref s }) => match *s {
            Ok(ref val) => write_str(wr, val)?,
            Err(ref err) => write_bin(wr, &err.0)?,
        },
        Value::Binary(ref val) => {
            write_bin(wr, val)?;
        }
        Value::Array(ref vec) => {
            write_array_len(wr, vec.len() as u32)?;
            for v in vec {
                write_value(wr, v)?;
            }
        }
        Value::Map(ref map) => {
            write_map_len(wr, map.len() as u32)?;
            for (key, val) in map {
                write_value(wr, key)?;
                write_value(wr, val)?;
            }
        }
        Value::Ext(ty, ref data) => {
            write_ext_meta(wr, data.len() as u32, ty)?;
            wr.write_bytes(data).map_err(ValueWriteError::InvalidDataWrite)?;
        }
    }

    Ok(())
}
