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
  pub statements: Vec<Statement>,
}

impl Block {
  pub fn new(statements: Vec<Statement>) -> Block {
    Block { statements: statements }
  }
}

/*
 * Statement
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Statement {
  Block(Block),
  FunctionDefinition(FunctionDefinition),
  VariableDeclaration(VariableDeclaration),
  Assignment(Assignment),
  Expression(Expression),
  LabelDefinition(LabelDefinition),
  Switch(Switch),
  ForLoop(ForLoop),
  ControlOp(ControlOp),
  SubAssembly(SubAssembly),
  DataSize(DataSize),
  LinkerSymbol(LinkerSymbol),
}

/*
 * Expression
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
  Identifier(Identifier),
  Literal(Literal),
  FunctionCall(FunctionCall),
}

/*
 * Label Definition
 */
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
 * Function Definition
 */
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
             block: Block,
             ) -> FunctionDefinition {
    FunctionDefinition {
      identifier: i,
      arguments: args,
      returns: returns,
      block: block,
    }
  }
}

/*
 * Variable Declaration
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct VariableDeclaration {
    pub identifiers: Vec<Identifier>,
    pub expression: Expression,
}

impl VariableDeclaration {
    pub fn new(is: Vec<Identifier>, e: Expression) -> VariableDeclaration {
        VariableDeclaration {
            identifiers: is,
            expression: e,
        }
    }
}

/*
 * Assignment
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Assignment {
    pub identifiers: Vec<Identifier>,
    pub expression: Expression,
}

impl Assignment {
    pub fn new(is: Vec<Identifier>, e: Expression) -> Assignment {
        Assignment {
            identifiers: is,
            expression: e,
        }
    }
}

/*
 * Switch
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Switch {
    pub expression: Expression,
    pub cases: Vec<Case>,
    pub default: Option<Block>,
}

impl Switch {
    pub fn new(e: Expression, cs: Vec<Case>, d: Option<Block>) -> Switch {
        Switch {
            expression: e,
            cases: cs,
            default: d,
        }
    }
}

/*
 * Case
 */
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

/*
 * For Loop
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ForLoop {
    pub init: Block,
    pub condition: Expression,
    pub post: Block,
    pub body: Block,
}

impl ForLoop {
    pub fn new(i: Block, c: Expression, p: Block, b: Block) -> ForLoop {
        ForLoop {
            init: i,
            condition: c,
            post: p,
            body: b,
        }
    }
}

/*
 * Control Operation
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ControlOp {
  Break(),
  Continue(),
  BytecodeSize(),
}

/*
 * Data
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataSize {
    pub identifier: Identifier,
}

impl DataSize {
    pub fn new(i: Identifier) -> DataSize {
        DataSize { identifier: i }
    }
}

/*
 * Sub-Assembly
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

/*
 * Function Call
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub args: Vec<Statement>,
}

impl FunctionCall {
    pub fn new(i: Identifier, args: Vec<Statement>) -> FunctionCall {
        FunctionCall {
            identifier: i,
            args: args,
        }
    }
}

/*
 * Linker Symbol
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LinkerSymbol {
    pub symbol: StringLiteral,
}

impl LinkerSymbol {
    pub fn new(s: StringLiteral) -> LinkerSymbol {
        LinkerSymbol { symbol: s }
    }
}

/*
 * Identifier
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
 * Literal
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Literal {
  HexNumber(HexNumber),
  DecNumber(DecNumber),
  StringLiteral(StringLiteral),
  HexLiteral(HexLiteral),
}


/*
 * String Literal
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
