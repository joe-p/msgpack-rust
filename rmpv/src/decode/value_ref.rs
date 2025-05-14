#[cfg(feature = "std")]
use std::io::Cursor;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::str;
#[cfg(not(feature = "std"))]
use core::str;

use rmp::Marker;

use super::Error;
use crate::{Utf8StringRef, ValueRef};

/// A `BorrowRead` is a type of Reader which has an internal buffer.
///
/// This magic trait acts like a standard `BufRead` but unlike the standard this has an explicit
/// internal buffer lifetime, which allows to borrow from underlying buffer while consuming bytes.
pub trait BorrowRead<'a> {
    /// Returns the buffer contents.
    ///
    /// This function is a lower-level call. It needs to be paired with the consume method to
    /// function properly. When calling this method, none of the contents will be "read" in the
    /// sense that later calling read may return the same contents. As such, consume must be called
    /// with the number of bytes that are consumed from this buffer to ensure that the bytes are
    /// never returned twice.
    ///
    /// An empty buffer returned indicates that the stream has reached EOF.
    fn fill_buf(&self) -> &'a [u8];

    /// Tells this buffer that len bytes have been consumed from the buffer, so they should no
    /// longer be returned in calls to read.
    fn consume(&mut self, len: usize);
}

impl<'a> BorrowRead<'a> for &'a [u8] {
    fn fill_buf(&self) -> &'a [u8] {
        self
    }

    fn consume(&mut self, len: usize) {
        *self = &(*self)[len..];
    }
}

#[cfg(feature = "std")]
impl<'a> BorrowRead<'a> for Cursor<&'a [u8]> {
    fn fill_buf(&self) -> &'a [u8] {
        let len = std::cmp::min(self.position(), self.get_ref().len() as u64);
        &self.get_ref()[len as usize..]
    }

    fn consume(&mut self, len: usize) {
        let pos = self.position();
        self.set_position(pos + len as u64);
    }
}

fn read_str_data<'a, R>(rd: &mut R, len: usize, depth: u16) -> Result<Utf8StringRef<'a>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    let buf = read_bin_data(rd, len, depth)?;
    match str::from_utf8(buf) {
        Ok(s) => Ok(Utf8StringRef::from(s)),
        Err(err) => {
            let s = Utf8StringRef {
                s: Err((buf, err)),
            };
            Ok(s)
        }
    }
}

fn read_bin_data<'a, R>(rd: &mut R, len: usize, depth: u16) -> Result<&'a [u8], Error>
    where R: BorrowRead<'a>
{
    let _depth = super::decrement_depth(depth)?;
    let buf = rd.fill_buf();

    if len > buf.len() {
        return Err(Error::DepthLimitExceeded);
    }

    // Take a slice.
    let buf = &buf[..len];
    rd.consume(len);

    Ok(buf)
}

fn read_array_data<'a, R>(rd: &mut R, mut len: usize, depth: u16) -> Result<Vec<ValueRef<'a>>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    // Note: Do not preallocate a Vec of size `len`.
    // See https://github.com/3Hren/msgpack-rust/issues/151
    let mut vec = Vec::new();

    while len > 0 {
        vec.push(read_value_ref_inner(rd, depth)?);
        len -= 1;
    }

    Ok(vec)
}

fn read_map_data<'a, R>(rd: &mut R, mut len: usize, depth: u16) -> Result<Vec<(ValueRef<'a>, ValueRef<'a>)>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    // Note: Do not preallocate a Vec of size `len`.
    // See https://github.com/3Hren/msgpack-rust/issues/151
    let mut vec = Vec::new();

    while len > 0 {
        vec.push((
            read_value_ref_inner(rd, depth)?,
            read_value_ref_inner(rd, depth)?,
        ));
        len -= 1;
    }

    Ok(vec)
}

fn read_ext_data<'a, R>(rd: &mut R, len: usize, depth: u16) -> Result<(i8, &'a [u8]), Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;
    
    // Read type byte
    let buf = rd.fill_buf();
    if buf.is_empty() {
        return Err(Error::DepthLimitExceeded);
    }
    let ty = buf[0] as i8;
    rd.consume(1);
    
    let data = read_bin_data(rd, len, depth)?;
    
    Ok((ty, data))
}

fn read_value_ref_inner<'a, R>(rd: &mut R, depth: u16) -> Result<ValueRef<'a>, Error>
    where R: BorrowRead<'a>
{
    let depth = super::decrement_depth(depth)?;

    // Read marker
    let buf = rd.fill_buf();
    if buf.is_empty() {
        return Err(Error::DepthLimitExceeded);
    }
    let marker = Marker::from_u8(buf[0]);
    rd.consume(1);
    
    match marker {
        Marker::Null => Ok(ValueRef::Nil),
        Marker::True => Ok(ValueRef::Boolean(true)),
        Marker::False => Ok(ValueRef::Boolean(false)),
        Marker::FixPos(val) => Ok(ValueRef::from(val)),
        Marker::FixNeg(val) => Ok(ValueRef::from(val)),
        Marker::U8 => {
            let buf = rd.fill_buf();
            if buf.is_empty() { return Err(Error::DepthLimitExceeded); }
            let val = buf[0];
            rd.consume(1);
            Ok(ValueRef::from(val))
        }
        Marker::U16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let val = u16::from_be_bytes([buf[0], buf[1]]);
            rd.consume(2);
            Ok(ValueRef::from(val))
        }
        Marker::U32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let val = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
            rd.consume(4);
            Ok(ValueRef::from(val))
        }
        Marker::U64 => {
            let buf = rd.fill_buf();
            if buf.len() < 8 { return Err(Error::DepthLimitExceeded); }
            let val = u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]);
            rd.consume(8);
            Ok(ValueRef::from(val))
        }
        Marker::I8 => {
            let buf = rd.fill_buf();
            if buf.is_empty() { return Err(Error::DepthLimitExceeded); }
            let val = buf[0] as i8;
            rd.consume(1);
            Ok(ValueRef::from(val))
        }
        Marker::I16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let val = i16::from_be_bytes([buf[0], buf[1]]);
            rd.consume(2);
            Ok(ValueRef::from(val))
        }
        Marker::I32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let val = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
            rd.consume(4);
            Ok(ValueRef::from(val))
        }
        Marker::I64 => {
            let buf = rd.fill_buf();
            if buf.len() < 8 { return Err(Error::DepthLimitExceeded); }
            let val = i64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]);
            rd.consume(8);
            Ok(ValueRef::from(val))
        }
        Marker::F32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let bits = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
            rd.consume(4);
            Ok(ValueRef::F32(f32::from_bits(bits)))
        }
        Marker::F64 => {
            let buf = rd.fill_buf();
            if buf.len() < 8 { return Err(Error::DepthLimitExceeded); }
            let bits = u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]);
            rd.consume(8);
            Ok(ValueRef::F64(f64::from_bits(bits)))
        }
        Marker::FixStr(len) => {
            read_str_data(rd, len as usize, depth).map(ValueRef::String)
        }
        Marker::Str8 => {
            let buf = rd.fill_buf();
            if buf.is_empty() { return Err(Error::DepthLimitExceeded); }
            let len = buf[0] as usize;
            rd.consume(1);
            read_str_data(rd, len, depth).map(ValueRef::String)
        }
        Marker::Str16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
            rd.consume(2);
            read_str_data(rd, len, depth).map(ValueRef::String)
        }
        Marker::Str32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
            rd.consume(4);
            read_str_data(rd, len, depth).map(ValueRef::String)
        }
        Marker::Bin8 => {
            let buf = rd.fill_buf();
            if buf.is_empty() { return Err(Error::DepthLimitExceeded); }
            let len = buf[0] as usize;
            rd.consume(1);
            read_bin_data(rd, len, depth).map(ValueRef::Binary)
        }
        Marker::Bin16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
            rd.consume(2);
            read_bin_data(rd, len, depth).map(ValueRef::Binary)
        }
        Marker::Bin32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
            rd.consume(4);
            read_bin_data(rd, len, depth).map(ValueRef::Binary)
        }
        Marker::FixArray(len) => {
            read_array_data(rd, len as usize, depth).map(ValueRef::Array)
        }
        Marker::Array16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
            rd.consume(2);
            read_array_data(rd, len, depth).map(ValueRef::Array)
        }
        Marker::Array32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
            rd.consume(4);
            read_array_data(rd, len, depth).map(ValueRef::Array)
        }
        Marker::FixMap(len) => {
            read_map_data(rd, len as usize, depth).map(ValueRef::Map)
        }
        Marker::Map16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
            rd.consume(2);
            read_map_data(rd, len, depth).map(ValueRef::Map)
        }
        Marker::Map32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
            rd.consume(4);
            read_map_data(rd, len, depth).map(ValueRef::Map)
        }
        Marker::FixExt1 => {
            read_ext_data(rd, 1, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::FixExt2 => {
            read_ext_data(rd, 2, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::FixExt4 => {
            read_ext_data(rd, 4, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::FixExt8 => {
            read_ext_data(rd, 8, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::FixExt16 => {
            read_ext_data(rd, 16, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::Ext8 => {
            let buf = rd.fill_buf();
            if buf.is_empty() { return Err(Error::DepthLimitExceeded); }
            let len = buf[0] as usize;
            rd.consume(1);
            read_ext_data(rd, len, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::Ext16 => {
            let buf = rd.fill_buf();
            if buf.len() < 2 { return Err(Error::DepthLimitExceeded); }
            let len = u16::from_be_bytes([buf[0], buf[1]]) as usize;
            rd.consume(2);
            read_ext_data(rd, len, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::Ext32 => {
            let buf = rd.fill_buf();
            if buf.len() < 4 { return Err(Error::DepthLimitExceeded); }
            let len = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
            rd.consume(4);
            read_ext_data(rd, len, depth).map(|(ty, data)| ValueRef::Ext(ty, data))
        }
        Marker::Reserved => Ok(ValueRef::Nil),
    }
}

/// Attempts to read the data from the given reader until either a complete MessagePack value
/// decoded or an error detected.
///
/// Returns either a non-owning `ValueRef`, which borrows the buffer from the given reader or an
/// error.
///
/// The reader should meet the requirement of a special `BorrowRead` trait, which allows to mutate
/// itself but permits to mutate the buffer it contains. It allows to perform a completely
/// zero-copy reading without a data loss fear in case of an error.
///
/// Currently only two types fit in this requirement: `&[u8]` and `Cursor<&[u8]>`. Using Cursor is
/// helpful, when you need to know how exactly many bytes the decoded `ValueRef` consumes. A `Vec<u8>`
/// type doesn't fit in the `BorrowRead` requirement, because its mut reference can mutate the
/// underlying buffer - use `Vec::as_slice()` if you need to decode a value from the vector.
///
/// # Errors
///
/// Returns an `Error` value if unable to continue the decoding operation either because of read
/// failure or any other circumstances. See `Error` documentation for more information.
///
/// This function enforces a maximum recursion depth of [`MAX_DEPTH`](super::MAX_DEPTH) and returns
/// [`Error::DepthLimitExceeded`] if the maximum is hit. If you run into stack overflows despite
/// this, use [`read_value_ref_with_max_depth`] with a custom maximum depth.
///
/// # Examples
/// ```
/// use rmpv::ValueRef;
/// use rmpv::decode::read_value_ref;
///
/// let buf = [0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
/// let mut rd = &buf[..];
///
/// assert_eq!(ValueRef::from("le message"), read_value_ref(&mut rd).unwrap());
/// ```
#[inline(never)]
pub fn read_value_ref<'a, R>(rd: &mut R) -> Result<ValueRef<'a>, Error>
    where R: BorrowRead<'a>
{
    read_value_ref_inner(rd, super::MAX_DEPTH as _)
}

/// Attempts to read the data from the given reader until either a complete MessagePack value
/// decoded or an error detected.
///
/// Returns either a non-owning `ValueRef`, which borrows the buffer from the given reader or an
/// error.
///
/// See [`read_value_ref`] for more information on how to use this function. This variant allows
/// you to specify the maximum recursion depth, if [`MAX_DEPTH`](super::MAX_DEPTH) is too high.
///
/// # Errors
///
/// Same as [`read_value_ref`], using the `max_depth` parameter in place of
/// [`MAX_DEPTH`](super::MAX_DEPTH).
#[inline(never)]
pub fn read_value_ref_with_max_depth<'a, R>(rd: &mut R, max_depth: usize) -> Result<ValueRef<'a>, Error>
    where R: BorrowRead<'a>
{
    read_value_ref_inner(rd, max_depth.min(u16::MAX as _) as u16)
}