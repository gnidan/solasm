use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cell::Cell;

use asm::ast;
use asm::ast::visitor::Visitor;
use asm::ast::visitor::Walkable;

type Sid = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolTableError {
  Scope(Sid),
  Redeclare(ast::Identifier),
  Unknown(ast::Identifier, Sid),
}

#[derive(Debug, Clone)]
struct Scope {
  parent_sid: Option<Sid>,
  variables: HashMap<ast::Identifier, Variable>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  expression: ast::Expression,
}

impl Variable {
  pub fn new(expression: ast::Expression) -> Variable {
    Variable { expression: expression }
  }
}


#[derive(Debug, Clone)]
pub struct SymbolTable {
  scopes: HashMap<Sid, Scope>,
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    let sid = SymbolTable::next_sid();
    let globals = Scope {
      parent_sid: None,
      variables: HashMap::new(),
    };
    let mut t = SymbolTable { scopes: HashMap::new() };
    t.scopes.insert(sid, globals);
    t
  }

  pub fn next_sid() -> Sid {
    thread_local!{
      static CURRENT_ID: Cell<Sid> = Cell::new(0)
    };

    CURRENT_ID.with(|c| {
                      let id = c.get();
                      c.set(id + 1);
                      id
                    })
  }

  pub fn subscope(&mut self, parent_sid: Sid) -> Result<Sid, SymbolTableError> {
    if !self.scopes.contains_key(&parent_sid) {
      return Err(SymbolTableError::Scope(parent_sid));
    }

    let sid = SymbolTable::next_sid();
    let scope = Scope {
      parent_sid: Some(parent_sid),
      variables: HashMap::new(),
    };

    self.scopes.insert(sid, scope);
    Ok(sid)
  }

  pub fn declare(&mut self,
                 sid: Sid,
                 identifier: &ast::Identifier,
                 expression: &ast::Expression)
                 -> Result<&Variable, SymbolTableError> {
    let variable = Variable::new(expression.clone());

    self.scopes
      .get_mut(&sid)
      .ok_or(SymbolTableError::Scope(sid))
      .and_then(|scope| {
        let entry = scope.variables.entry(identifier.clone());

        match entry {
          Entry::Occupied(_) => Err(SymbolTableError::Redeclare(identifier.clone())),
          Entry::Vacant(v) => Ok(&*v.insert(variable))
        }
      })
  }

  pub fn get(&self, sid: Sid, identifier: &ast::Identifier) -> Result<&Variable, SymbolTableError> {
    if !self.scopes.contains_key(&sid) {
      return Err(SymbolTableError::Scope(sid));
    }

    let mut current_scope = self.scopes.get(&sid).unwrap();
    loop {
      if current_scope.variables.contains_key(identifier) {
        return Ok(current_scope.variables.get(identifier).unwrap());
      }

      if let Some(parent_sid) = current_scope.parent_sid {
        current_scope = self.scopes.get(&parent_sid).unwrap();
      } else {
        return Err(SymbolTableError::Unknown(identifier.clone(), sid));
      }
    }

  }
}

#[test]
fn it_allows_variable_declaration_in_scope() {
  let identifier = ast::Identifier::new("foo").unwrap();
  let zero = ast::Node::new(ast::Literal::DecNumber(ast::DecNumber::new("0")));
  let expression = ast::Expression::Literal(zero);

  let mut t = SymbolTable::new();
  {
    let scope_err = t.declare(10, &identifier, &expression);
    assert_eq!(scope_err, Err(SymbolTableError::Scope(10)));
  }

  {
    let first = t.declare(0, &identifier, &expression);
    assert!(first.is_ok());
  }

  {
    let second = t.declare(0, &identifier, &expression);
    assert_eq!(second, Err(SymbolTableError::Redeclare(identifier)));
  }
}

#[test]
fn it_allows_variable_lookup_from_nonstrictly_ancestral_scopes() {
  let mut t = SymbolTable::new();
  let declaration_sid = t.subscope(0).unwrap();
  let child_sid = t.subscope(declaration_sid).unwrap();

  let identifier = ast::Identifier::new("foo").unwrap();
  let zero = ast::Node::new(ast::Literal::DecNumber(ast::DecNumber::new("0")));
  let expression = ast::Expression::Literal(zero);

  assert!(t.get(0, &identifier).is_err());

  t.declare(declaration_sid, &identifier, &expression).ok();

  let from_declaration_scope = t.get(declaration_sid, &identifier);
  assert!(from_declaration_scope.is_ok());

  let from_child_scope = t.get(child_sid, &identifier);
  assert!(from_child_scope.is_ok());

  assert_eq!(from_declaration_scope, from_child_scope);
}

#[test]
fn it_retrieves_correct_variable_upon_overriding() {
  let mut t = SymbolTable::new();
  let declaration_sid = t.subscope(0).unwrap();
  let child_sid = t.subscope(declaration_sid).unwrap();

  let identifier = ast::Identifier::new("foo").unwrap();
  let zero = ast::Node::new(ast::Literal::DecNumber(ast::DecNumber::new("0")));
  let first_expression = ast::Expression::Literal(zero);
  let one = ast::Node::new(ast::Literal::DecNumber(ast::DecNumber::new("1")));
  let second_expression = ast::Expression::Literal(one);

  assert!(t.get(0, &identifier).is_err());

  t.declare(declaration_sid, &identifier, &first_expression).ok();
  t.declare(child_sid, &identifier, &second_expression).ok();

  let from_declaration_scope = t.get(declaration_sid, &identifier);
  let from_child_scope = t.get(child_sid, &identifier);
  assert_ne!(from_declaration_scope, from_child_scope);
}



struct ScopeVisitor<'v> {
  symbols: SymbolTable,
  current_sid: Sid,
  root_node: &'v ast::Node<ast::Block>,
  sids: HashMap<ast::Nid, Sid>,
  origins: HashMap<Sid, Option<ast::Nid>>,
}

impl<'v> ScopeVisitor<'v> {
  pub fn new(node: &'v ast::Node<ast::Block>) -> ScopeVisitor<'v> {
    ScopeVisitor {
      symbols: SymbolTable::new(),
      current_sid: 0,
      root_node: node,
      sids: HashMap::new(),
      origins: HashMap::new(),
    }
  }

  pub fn visit(&mut self) -> &SymbolTable {
    {
      self.visit_block(self.root_node);
    }

    &self.symbols
  }
}

impl<'v> Visitor<'v> for ScopeVisitor<'v> {
  fn push(&mut self, nid: ast::Nid) {
    self.sids.insert(nid, self.current_sid);
  }

  fn visit_block(&mut self, b: &'v ast::Node<ast::Block>) {
    let sid = self.symbols.subscope(self.current_sid).unwrap();
    self.origins.insert(sid, Some(b.id));
    self.current_sid = sid;
    &b.walk(self);
  }
}

#[test]
fn it_gives_root_node_its_own_scope_with_global_scope_parent() {
  let node = ast::Block::new(vec![]);
  let mut visitor = ScopeVisitor::new(&node);

  assert_eq!(visitor.sids.get(&node.id), None);

  visitor.visit();

  let global_sid = 0;
  let expected_sid = 1;

  assert_eq!(visitor.sids.get(&node.id), Some(&expected_sid));

  let block_scope = visitor.symbols
    .scopes
    .get(&expected_sid)
    .unwrap();
  assert_eq!(block_scope.parent_sid, Some(global_sid));
}

#[test]
fn it_creates_a_scope_for_each_block() {
  let node = ast::Block::new(vec![ast::Node::new(ast::Statement::Block(ast::Block::new(vec![])))]);
  let mut visitor = ScopeVisitor::new(&node);

  // just the global scope
  assert_eq!(visitor.symbols.scopes.len(), 1);

  visitor.visit();

  // global scope, root block node, first statement block
  assert_eq!(visitor.symbols.scopes.len(), 3);
}

#[test]
fn it_stores_nodes_by_scope() {
  let identifier = ast::Identifier::new("i");
  let identifier_nid = identifier.id;

  let zero = ast::Node::new(ast::Literal::DecNumber(ast::DecNumber::new("0")));
  let zero_nid = zero.id;

  let expression = ast::Node::new(ast::Expression::Literal(zero));
  let expression_nid = expression.id;

  let declaration = ast::VariableDeclaration::new(vec![identifier], expression);
  let declaration_nid = declaration.id;

  let statement = ast::Node::new(ast::Statement::VariableDeclaration(declaration));
  let statement_nid = statement.id;

  let block = ast::Block::new(vec![statement]);
  let block_nid = block.id;

  let outer_statement = ast::Node::new(ast::Statement::Block(block));
  let outer_statement_nid = outer_statement.id;

  let root = ast::Block::new(vec![outer_statement]);
  let root_nid = root.id;

  let mut visitor = ScopeVisitor::new(&root);

  visitor.visit();

  let root_sid = 1;
  let block_sid = 2;

  assert_eq!(visitor.sids.get(&root_nid), Some(&root_sid));
  assert_eq!(visitor.sids.get(&outer_statement_nid), Some(&root_sid));
  assert_eq!(visitor.sids.get(&block_nid), Some(&block_sid));
  assert_eq!(visitor.sids.get(&statement_nid), Some(&block_sid));
  assert_eq!(visitor.sids.get(&declaration_nid), Some(&block_sid));
  assert_eq!(visitor.sids.get(&expression_nid), Some(&block_sid));
  assert_eq!(visitor.sids.get(&zero_nid), Some(&block_sid));
  assert_eq!(visitor.sids.get(&identifier_nid), Some(&block_sid));
}
