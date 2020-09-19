extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::Read;
use std::io::{Result as IoResult, Error as IoError, ErrorKind};
use std::string::String;
use std::option::Option;
use base64::DecodeError;

/// Returns a file's extension based on the its hexidecimal representation.
/// 
/// Note: TIF files will be considered as TIFF.
fn get_file_type(hex: &str) -> Option<&str> {
    if Regex::new(r"^ffd8ffe").ok()?.is_match(hex) {
        return Some("jpeg");
    } else if 
        Regex::new(r"^49492a00").ok()?.is_match(hex) || 
        Regex::new(r"^4d4d002a").ok()?.is_match(hex) {
        return Some("tiff");
    } else if Regex::new(r"^424d").ok()?.is_match(hex) {
        return Some("bmp");
    } else if Regex::new(r"^89504e47").ok()?.is_match(hex) {
        return Some("png");
    } else if Regex::new(r"^47494638").ok()?.is_match(hex) {
        return Some("gif");
    } else if Regex::new(r"^00000100").ok()?.is_match(hex) {
        return Some("ico");
    } else if Regex::new(r"^52494646").ok()?.is_match(hex) {
        return Some("webp");
    } else {
        None
    }
}

/// Converts an image file to a base64 encoded string.
pub fn to_base64(path: &str) -> IoResult<String> {
    let mut file = File::open(path)?;
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);

    return to_base64_from_memory(&vec).ok_or(IoError::from(ErrorKind::InvalidInput));
}

/// Converts an image buffer to a base64 encoded string.
pub fn to_base64_from_memory(vec: &[u8]) -> Option<String> {
    get_file_type(&vec.to_hex()).map(| file_type | {
        to_base64_from_memory_with_extension(vec, file_type)
    })
}

/// Converts an image file to a base64 encoded string with a specified extension.
pub fn to_base64_with_extension(path: &str, extension: &str) -> IoResult<String> {
    let mut file = File::open(path)?;
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);

    return Ok(to_base64_from_memory_with_extension(&vec, extension));
}

/// Converts an image buffer to a base64 encoded string with a specified extension.
pub fn to_base64_from_memory_with_extension(vec: &[u8], extension: &str) -> String {
    let base64 = vec.to_base64(MIME);
    return format!(
        "data:image/{};base64,{}",
        extension,
        base64.replace("\r\n", "")
    );
}

/// Converts a base64 encoded string to an image buffer.
pub fn from_base64(base64: String) -> Result<Vec<u8>, DecodeError> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;

    value.drain(..offset);

    return base64::decode(&value);
}
