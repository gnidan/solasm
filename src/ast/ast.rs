use std::fmt::{Display, Formatter, Error};
use std::str::FromStr;
use std::vec::Vec;

extern crate bigint;
use self::bigint::{U256, Uint};

extern crate rustc_serialize;
use self::rustc_serialize::hex::FromHex;


/*
 * String Literals
 */

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StringLiteral {
    pub string: String,
}

impl StringLiteral {
    pub fn new(s: String) -> StringLiteral {
        StringLiteral { string: s }
    }
}


/*
 * Hex Literals (Raw Hex Bytestring)
 */

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HexLiteral {
    pub bytes: Vec<u8>,
}

impl HexLiteral {
    pub fn new(bytes: &str) -> HexLiteral {
        HexLiteral { bytes: bytes.from_hex().unwrap() }
    }
}


/*
 * Number Literals
 */

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HexNumber {
    pub uint: U256,
}

impl HexNumber {
    pub fn new(uint: &str) -> HexNumber {
        HexNumber { uint: U256::from_str(uint).unwrap() }
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DecNumber {
    pub uint: U256,
}

impl DecNumber {
    pub fn new(uint: &str) -> DecNumber {
        DecNumber { uint: U256::from_dec_str(uint).unwrap() }
    }
}
