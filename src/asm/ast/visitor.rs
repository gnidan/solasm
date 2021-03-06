use super::ast::*;

pub trait Visitor<'v>: Sized {
  fn push(&mut self, _: Nid) {}
  fn pop(&mut self) {}

  fn visit_block(&mut self, b: &'v Node<Block>) {
    &b.walk(self);
  }

  fn visit_statement(&mut self, s: &'v Node<Statement>) {
    &s.walk(self);
  }

  fn visit_expression(&mut self, e: &'v Node<Expression>) {
    &e.walk(self);
  }

  fn visit_function_definition(&mut self, f: &'v Node<FunctionDefinition>) {
    &f.walk(self);
  }

  fn visit_variable_declaration(&mut self, v: &'v Node<VariableDeclaration>) {
    &v.walk(self);
  }

  fn visit_assignment(&mut self, a: &'v Node<Assignment>) {
    &a.walk(self);
  }

  fn visit_switch(&mut self, s: &'v Node<Switch>) {
    &s.walk(self);
  }

  fn visit_case(&mut self, c: &'v Node<Case>) {
    &c.walk(self);
  }

  fn visit_for_loop(&mut self, f: &'v Node<ForLoop>) {
    &f.walk(self);
  }

  fn visit_control_op(&mut self, o: &'v Node<ControlOp>) {
    &o.walk(self);
  }

  fn visit_sub_assembly(&mut self, a: &'v Node<SubAssembly>) {
    &a.walk(self);
  }

  fn visit_function_call(&mut self, c: &'v Node<FunctionCall>) {
    &c.walk(self);
  }

  fn visit_identifier(&mut self, i: &'v Node<Identifier>) {
    &i.walk(self);
  }

  fn visit_literal(&mut self, l: &'v Node<Literal>) {
    &l.walk(self);
  }

  fn visit_string_literal(&mut self, s: &'v Node<StringLiteral>) {
    &s.walk(self);
  }

  fn visit_hex_literal(&mut self, x: &'v Node<HexLiteral>) {
    &x.walk(self);
  }

  fn visit_hex_number(&mut self, x: &'v Node<HexNumber>) {
    &x.walk(self);
  }

  fn visit_dec_number(&mut self, n: &'v Node<DecNumber>) {
    &n.walk(self);
  }
}

pub trait Walkable<'w> {
  fn walk<V: Visitor<'w>>(&'w self, _: &mut V) {}
}

impl<'w> Walkable<'w> for Node<Block> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    for s in &self.statements {
      visitor.visit_statement(s);
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Statement> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      Statement::Block(ref node) => visitor.visit_block(node),
      Statement::FunctionDefinition(ref node) => visitor.visit_function_definition(node),
      Statement::VariableDeclaration(ref node) => visitor.visit_variable_declaration(node),
      Statement::Assignment(ref node) => visitor.visit_assignment(node),
      Statement::Expression(ref node) => visitor.visit_expression(node),
      Statement::Switch(ref node) => visitor.visit_switch(node),
      Statement::ForLoop(ref node) => visitor.visit_for_loop(node),
      Statement::ControlOp(ref node) => visitor.visit_control_op(node),
      Statement::SubAssembly(ref node) => visitor.visit_sub_assembly(node),
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Expression> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      Expression::Identifier(ref node) => visitor.visit_identifier(node),
      Expression::Literal(ref node) => visitor.visit_literal(node),
      Expression::FunctionCall(ref node) => visitor.visit_function_call(node),
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<FunctionDefinition> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      FunctionDefinition { ref identifier, ref arguments, ref returns, ref body } => {
        visitor.visit_identifier(identifier);

        for arg in arguments {
          visitor.visit_identifier(arg);
        }

        match *returns {
          Some(ref idents) => {
            for ident in idents {
              visitor.visit_identifier(ident);
            }
          }
          None => {}
        }

        visitor.visit_block(body);
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<VariableDeclaration> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      VariableDeclaration { ref identifiers, ref expression } => {
        for ident in identifiers {
          visitor.visit_identifier(ident);
        }

        visitor.visit_expression(expression);
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Assignment> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      Assignment { ref identifiers, ref expression } => {
        for ident in identifiers {
          visitor.visit_identifier(ident);
        }

        visitor.visit_expression(expression);
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Switch> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      Switch { ref expression, ref cases, ref default } => {
        visitor.visit_expression(expression);

        for case in cases {
          visitor.visit_case(case);
        }

        match *default {
          Some(ref block) => {
            visitor.visit_block(block);
          }
          None => {}
        }
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Case> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      Case { ref expression, ref block } => {
        visitor.visit_expression(expression);
        visitor.visit_block(block);
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<ForLoop> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      ForLoop { ref init, ref condition, ref post, ref body } => {
        visitor.visit_block(init);
        visitor.visit_expression(condition);
        visitor.visit_block(post);
        visitor.visit_block(body);
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<SubAssembly> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      SubAssembly { ref identifier, ref block } => {
        visitor.visit_identifier(identifier);
        visitor.visit_block(block);
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<FunctionCall> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      FunctionCall { ref identifier, ref arguments } => {
        visitor.visit_identifier(identifier);
        for expression in arguments {
          visitor.visit_expression(expression);
        }
      }
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Literal> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    match **self {
      Literal::HexNumber(ref node) => visitor.visit_hex_number(node),
      Literal::DecNumber(ref node) => visitor.visit_dec_number(node),
      Literal::StringLiteral(ref node) => visitor.visit_string_literal(node),
      Literal::HexLiteral(ref node) => visitor.visit_hex_literal(node),
    }
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<DecNumber> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<HexNumber> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<HexLiteral> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<StringLiteral> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<Identifier> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    visitor.pop();
  }
}

impl<'w> Walkable<'w> for Node<ControlOp> {
  fn walk<V: Visitor<'w>>(&'w self, visitor: &mut V) {
    visitor.push(self.id);
    visitor.pop();
  }
}
