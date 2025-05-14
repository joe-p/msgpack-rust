#[cfg(feature = "std")]
pub use rmp::encode::ValueWriteError as Error;
#[cfg(not(feature = "std"))]
pub use rmp::encode::ValueWriteError;

mod value;
mod value_ref;

pub use self::value::write_value;
pub use self::value_ref::write_value_ref;
