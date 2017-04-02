use std::str::FromStr;
use std::vec::Vec;

extern crate bigint;
use self::bigint::{U256, Uint};

extern crate rustc_serialize;
use self::rustc_serialize::hex::FromHex;


/*
 * Block
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Block {
    pub items: Vec<Item>,
}

impl Block {
    pub fn new(items: Vec<Item>) -> Block {
        Block { items: items }
    }
}

/*
 * Item
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Item {
    Identifier(Identifier),
    Block(Block),
    FunctionalExpression(FunctionalExpression),
    LocalDefinition(LocalDefinition),
    FunctionalAssignment(FunctionalAssignment),
    Assignment(Assignment),
    LabelDefinition(LabelDefinition),
    Switch(Switch),
    FunctionDefinition(FunctionDefinition),
    For(For),
    Break(),
    Continue(),
    SubAssembly(SubAssembly),
    DataSize(Identifier),
    LinkerSymbol(LinkerSymbol),
    ErrorLabel(),
    BytecodeSize(),
    HexLiteral(HexLiteral),
    StringLiteral(StringLiteral),
    HexNumber(HexNumber),
    DecNumber(DecNumber),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    FunctionalExpression(FunctionalExpression),
    HexNumber(HexNumber),
    DecNumber(DecNumber),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
}

/*
 * Functional Expressions!
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionalExpression {
    pub identifier: Identifier,
    pub items: Vec<Item>,
}

impl FunctionalExpression {
    pub fn new(i: Identifier, items: Vec<Item>) -> FunctionalExpression {
        FunctionalExpression {
            identifier: i,
            items: items,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LocalDefinition {
    pub identifiers: Vec<Identifier>,
    pub expression: Expression,
}

impl LocalDefinition {
    pub fn new(is: Vec<Identifier>, e: Expression) -> LocalDefinition {
        LocalDefinition {
            identifiers: is,
            expression: e,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionalAssignment {
    pub identifiers: Vec<Identifier>,
    pub expression: Expression,
}

impl FunctionalAssignment {
    pub fn new(is: Vec<Identifier>, e: Expression) -> FunctionalAssignment {
        FunctionalAssignment {
            identifiers: is,
            expression: e,
        }
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionDefinition {
    pub identifier: Identifier,
    pub arguments: Vec<Identifier>,
    pub returns: Option<Vec<Identifier>>,
    pub block: Block,
}

impl FunctionDefinition {
    pub fn new(i: Identifier,
               args: Vec<Identifier>,
               returns: Option<Vec<Identifier>>,
               block: Block)
               -> FunctionDefinition {
        FunctionDefinition {
            identifier: i,
            arguments: args,
            returns: returns,
            block: block,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct For {
    pub init: ForOp,
    pub check: FunctionalExpression,
    pub each: ForOp,
    pub block: Block,
}

impl For {
    pub fn new(init: ForOp, check: FunctionalExpression, each: ForOp, block: Block) -> For {
        For {
            init: init,
            check: check,
            each: each,
            block: block,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ForOp {
    Block(Block),
    FunctionalExpression(FunctionalExpression),
}


/*
 * Control Structures
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubAssembly {
    pub identifier: Identifier,
    pub block: Block,
}

impl SubAssembly {
    pub fn new(i: Identifier, b: Block) -> SubAssembly {
        SubAssembly {
            identifier: i,
            block: b,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Switch {
    pub expression: Expression,
    pub cases: Vec<Case>,
    pub default: Option<DefaultCase>,
}

impl Switch {
    pub fn new(e: Expression, cs: Vec<Case>, d: Option<DefaultCase>) -> Switch {
        Switch {
            expression: e,
            cases: cs,
            default: d,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Case {
    pub expression: Expression,
    pub block: Block,
}

impl Case {
    pub fn new(e: Expression, b: Block) -> Case {
        Case {
            expression: e,
            block: b,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DefaultCase {
    pub block: Block,
}

impl DefaultCase {
    pub fn new(b: Block) -> DefaultCase {
        DefaultCase { block: b }
    }
}



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
