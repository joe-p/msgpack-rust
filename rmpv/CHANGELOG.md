# Change Log

All user-facing changes to this project are documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/).

Versions marked with "YANKED" (`~~~`) are considered unsuitable for public use.

## Unreleased

## 1.4.0 - TBD

### Added
- Add `no_std` support by making the `std` feature optional
- New default features: `std` (can be disabled for `no_std` environments)
- Implement `RmpRead` and `RmpWrite` trait usage from rmp crate
- Updated examples to demonstrate usage in both std and no_std environments

### Changed
- The `Error` type in decode module is now parameterized by the reader's error type

## 1.3.0 - 2023-08-14

### Changed
- Make `Integer` `Copy` and add convenience methods for extracting the value
- Make generic over more integral types
- Update dependencies

### Fixed
- Fix clippy warnings

## 1.2.0 - 2023-03-04

### Changed
- Bump `rmp` to `0.8.12`

## 1.1.0 - 2022-11-13

### Changed
- Bump `rmp-serde` to `1.1.0`
- Bump MSRV to `1.58.0`

## 1.0.0 - 2022-06-24

### Changed
- Update all deps to 1.0

### Removed
- Remove deprecated `MSGPACK_EXT_STRING`

## 0.5.1 - 2022-05-15

### Fixed
- Mitigate overflow from overallocating memory in valueref functions

## 0.5.0 - 2020-10-27

### Changed
- Update deps

### Removed
- Deprecated functions removed:
  - `rmpv::ext::from_value`
  - `rmpv::ext::to_value`

## 0.4.7 - 2020-10-05

### Fixed
- More consistent MessagePack ext (de)serialization round-trip

## 0.4.6 - 2020-08-13

### Added
- Added `PartialEq` implementation between `Value` and `ValueRef`

## 0.4.5 - 2020-05-22

### Fixed
- Fixed exponentially large memory use in array & map deserialization

## 0.4.4 - 2020-01-02

### Added
- Added `#[inline]` to `read_value` and `read_value_ref`

## 0.4.3 - 2019-09-30

### Added
- Added `Option<T>` conversions for `Value`

## 0.4.2 - 2019-05-16

### Changed
- Improved performance when checking types in `ValueRef::as_*` methods

## 0.4.1 - 2019-01-24

### Added
- Add `Display` implementation for `ValueRef`

## 0.4.0 - 2019-01-20

### Added
- Added error types
- Added `From<f32>` and `TryFrom<Value>` for numeric types
- Added depth checking to limit deeply nested structure parsing

## 0.3.2 - 2017-09-06

### Changed
- Removed generic args from `as_xxx` methods

## 0.3.1 - 2017-08-13

### Changed
- Improved performance for `as_xxx` methods

## 0.3.0 - 2017-04-26

### Added
- Implemented `FromIterator` for `Value`
- Added support for deserializing large binary/string/array/map with `read_value_ref`

## 0.2.0 - 2017-01-05

### Changed
- Refactored the API (finally)

## 0.1.0 - 2016-07-31

### Added
- Initial release