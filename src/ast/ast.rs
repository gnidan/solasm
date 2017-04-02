use std::fmt::{Display, Formatter, Error};
use std::str::FromStr;
use std::vec::Vec;

extern crate bigint;
use self::bigint::{U256, Uint};

extern crate rustc_serialize;
use self::rustc_serialize::hex::FromHex;

/*
 * Assignments/Labels
 */

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Assignment {
    pub identifier: Identifier,
}

impl Assignment {
    pub fn new(i: Identifier) -> Assignment {
        Assignment { identifier: i }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LabelDefinition {
    pub identifier: Identifier,
}

impl LabelDefinition {
    pub fn new(i: Identifier) -> LabelDefinition {
        LabelDefinition { identifier: i }
    }
}



/*
 * Identifiers
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier {
    pub symbol: String,
}

impl Identifier {
    pub fn new(s: &str) -> Identifier {
        Identifier { symbol: s.to_string() }
    }
}


/*
 * Linker Symbols
 */

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LinkerSymbol {
    pub symbol: String,
}

impl LinkerSymbol {
    pub fn new(s: StringLiteral) -> LinkerSymbol {
        LinkerSymbol { symbol: s.string }
    }
}


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
