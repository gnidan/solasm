use std::str::FromStr;
use std::vec::Vec;
use std::ops::{Deref};

extern crate bigint;
use self::bigint::{U256, Uint};

extern crate rustc_serialize;
use self::rustc_serialize::hex::FromHex;

/*
 * Generic Node
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Node<T> {
  node: T,
}

impl<T> Node<T> {
  pub fn new(t: T) -> Node<T> {
    Node { node: t }
  }

  pub fn unwrap(self) -> T {
    self.node
  }
}

impl<T> Deref for Node<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.node
  }
}

/*
 * Block
 */

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Block {
  pub statements: Vec<Node<Statement>>,
}

impl Block {
  pub fn new(statements: Vec<Node<Statement>>) -> Node<Block> {
    Node::new(Block { statements: statements })
  }
}

/*
 * Statement
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Statement {
  Block(Node<Block>),
  FunctionDefinition(Node<FunctionDefinition>),
  VariableDeclaration(Node<VariableDeclaration>),
  Assignment(Node<Assignment>),
  Expression(Node<Expression>),
  Switch(Node<Switch>),
  ForLoop(Node<ForLoop>),
  ControlOp(Node<ControlOp>),
  SubAssembly(Node<SubAssembly>),
}

/*
 * Expression
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
  Identifier(Node<Identifier>),
  Literal(Node<Literal>),
  FunctionCall(Node<FunctionCall>),
}

/*
 * Function Definition
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionDefinition {
  pub identifier: Node<Identifier>,
  pub arguments: Vec<Node<Identifier>>,
  pub returns: Option<Vec<Node<Identifier>>>,
  pub body: Node<Block>,
}

impl FunctionDefinition {
  pub fn new(i: Node<Identifier>,
             args: Vec<Node<Identifier>>,
             returns: Option<Vec<Node<Identifier>>>,
             body: Node<Block>,
             ) -> Node<FunctionDefinition> {
    Node::new(FunctionDefinition {
      identifier: i,
      arguments: args,
      returns: returns,
      body: body,
    })
  }
}

/*
 * Variable Declaration
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct VariableDeclaration {
  pub identifiers: Vec<Node<Identifier>>,
  pub expression: Node<Expression>,
}

impl VariableDeclaration {
  pub fn new(is: Vec<Node<Identifier>>, e: Node<Expression>) -> Node<VariableDeclaration> {
    Node::new(VariableDeclaration {
      identifiers: is,
      expression: e,
    })
  }
}

/*
 * Assignment
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Assignment {
  pub identifiers: Vec<Node<Identifier>>,
  pub expression: Node<Expression>,
}

impl Assignment {
  pub fn new(is: Vec<Node<Identifier>>, e: Node<Expression>) -> Node<Assignment> {
    Node::new(Assignment {
      identifiers: is,
      expression: e,
    })
  }
}

/*
 * Switch
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Switch {
  pub expression: Node<Expression>,
  pub cases: Vec<Node<Case>>,
  pub default: Option<Node<Block>>,
}

impl Switch {
  pub fn new(e: Node<Expression>, cs: Vec<Node<Case>>, d: Option<Node<Block>>) -> Node<Switch> {
    Node::new(Switch {
      expression: e,
      cases: cs,
      default: d,
    })
  }
}

/*
 * Case
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Case {
  pub expression: Node<Expression>,
  pub block: Node<Block>,
}

impl Case {
  pub fn new(e: Node<Expression>, b: Node<Block>) -> Node<Case> {
    Node::new(Case {
      expression: e,
      block: b,
    })
  }
}

/*
 * For Loop
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ForLoop {
  pub init: Node<Block>,
  pub condition: Node<Expression>,
  pub post: Node<Block>,
  pub body: Node<Block>,
}

impl ForLoop {
  pub fn new(i: Node<Block>, c: Node<Expression>, p: Node<Block>, b: Node<Block>) -> Node<ForLoop> {
    Node::new(ForLoop {
      init: i,
      condition: c,
      post: p,
      body: b,
    })
  }
}

/*
 * Control Operation
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ControlOp {
  Break(),
  Continue(),
}

/*
 * Sub-Assembly
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubAssembly {
  pub identifier: Node<Identifier>,
  pub block: Node<Block>,
}

impl SubAssembly {
  pub fn new(i: Node<Identifier>, b: Node<Block>) -> Node<SubAssembly> {
    Node::new(SubAssembly {
      identifier: i,
      block: b,
    })
  }
}

/*
 * Function Call
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionCall {
  pub identifier: Node<Identifier>,
  pub arguments: Vec<Node<Expression>>,
}

impl FunctionCall {
  pub fn new(i: Node<Identifier>, args: Vec<Node<Expression>>) -> Node<FunctionCall> {
    Node::new(FunctionCall {
      identifier: i,
      arguments: args,
    })
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
  pub fn new(s: &str) -> Node<Identifier> {
    Node::new(Identifier { symbol: s.to_string() })
  }
}

/*
 * Literal
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Literal {
  HexNumber(Node<HexNumber>),
  DecNumber(Node<DecNumber>),
  StringLiteral(Node<StringLiteral>),
  HexLiteral(Node<HexLiteral>),
}

/*
 * String Literal
 */
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StringLiteral {
  pub string: String,
}

impl StringLiteral {
  pub fn new(s: String) -> Node<StringLiteral> {
    Node::new(StringLiteral { string: s })
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
  pub fn new(bytes: &str) -> Node<HexLiteral> {
    Node::new(HexLiteral { bytes: bytes.from_hex().unwrap() })
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
  pub fn new(uint: &str) -> Node<HexNumber> {
    Node::new(HexNumber { uint: U256::from_str(uint).unwrap() })
  }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DecNumber {
  pub uint: U256,
}

impl DecNumber {
  pub fn new(uint: &str) -> Node<DecNumber> {
    Node::new(DecNumber { uint: U256::from_dec_str(uint).unwrap() })
  }
}
