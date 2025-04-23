//! Error types for Orbyte serialization and deserialization.

extern crate alloc;
use alloc::string::FromUtf8Error;

/// Errors that can occur during serialization or deserialization.
#[derive(Debug, PartialEq)]
pub enum OrbyteError {
    /// Input byte slice is too short or incorrectly sized for the type.
    InvalidLength,
    /// Invalid boolean value (neither 0 nor 1).
    InvalidBool,
    /// Invalid UTF-8 sequence when deserializing a string.
    InvalidUtf8(FromUtf8Error),
    /// Invalid Unicode code point when deserializing a char.
    InvalidChar,
}