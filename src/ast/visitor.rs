use ast::*;

pub trait Visitor<'v> : Sized {
  fn visit_block(&mut self, b: &'v Node<Block>) {
    walk_block(self, b)
  }

  fn visit_statement(&mut self, s: &'v Node<Statement>) {
    walk_statement(self, s)
  }

  fn visit_expression(&mut self, e: &'v Node<Expression>) {
    walk_expression(self, e)
  }

  fn visit_function_definition(&mut self, f: &'v Node<FunctionDefinition>) {
    walk_function_definition(self, f)
  }

  fn visit_variable_declaration(&mut self, v: &'v Node<VariableDeclaration>) {
    walk_variable_declaration(self, v)
  }

  fn visit_assignment(&mut self, a: &'v Node<Assignment>) {
    walk_assignment(self, a)
  }

  fn visit_switch(&mut self, s: &'v Node<Switch>) {
    walk_switch(self, s)
  }

  fn visit_case(&mut self, c: &'v Node<Case>) {
    walk_case(self, c)
  }

  fn visit_for_loop(&mut self, f: &'v Node<ForLoop>) {
    walk_for_loop(self, f)
  }

  fn visit_control_op(&mut self, o: &'v Node<ControlOp>) {
    // leaf node
  }

  fn visit_sub_assembly(&mut self, a: &'v Node<SubAssembly>) {
    walk_sub_assembly(self, a)
  }

  fn visit_function_call(&mut self, c: &'v Node<FunctionCall>) {
    walk_function_call(self, c)
  }

  fn visit_identifier(&mut self, i: &'v Node<Identifier>) {
    // leaf node
  }

  fn visit_literal(&mut self, l: &'v Node<Literal>) {
    walk_literal(self, l)
  }

  fn visit_string_literal(&mut self, s: &'v Node<StringLiteral>) {
    // leaf node
  }

  fn visit_hex_literal(&mut self, x: &'v Node<HexLiteral>) {
    // leaf node
  }

  fn visit_hex_number(&mut self, x: &'v Node<HexNumber>) {
    // leaf node
  }

  fn visit_dec_number(&mut self, n: &'v Node<DecNumber>) {
    // leaf node
  }
}


pub fn walk_block<'v, V>(visitor: &mut V, block: &'v Node<Block>)
  where V: Visitor<'v>
{
  for s in &block.statements {
    visitor.visit_statement(s);
  }
}

pub fn walk_statement<'v, V>(visitor: &mut V, statement: &'v Node<Statement>)
  where V: Visitor<'v>
{
  match **statement {
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
}

pub fn walk_expression<'v, V>(visitor: &mut V, expression: &'v Node<Expression>)
  where V: Visitor<'v>
{
  match **expression {
    Expression::Identifier(ref node) => visitor.visit_identifier(node),
    Expression::Literal(ref node) => visitor.visit_literal(node),
    Expression::FunctionCall(ref node) => visitor.visit_function_call(node),
  }
}

pub fn walk_function_definition<'v, V>(visitor: &mut V, definition: &'v Node<FunctionDefinition>)
  where V: Visitor<'v>
{
  match **definition {
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
        },
        None => {}
      }

      visitor.visit_block(body);
    }
  }
}

pub fn walk_variable_declaration<'v, V>(visitor: &mut V, declaration: &'v Node<VariableDeclaration>)
  where V: Visitor<'v>
{
  match **declaration {
    VariableDeclaration { ref identifiers, ref expression } => {
      for ident in identifiers {
        visitor.visit_identifier(ident);
      }

      visitor.visit_expression(expression);
    }
  }
}

pub fn walk_assignment<'v, V>(visitor: &mut V, assignment: &'v Node<Assignment>)
  where V: Visitor<'v>
{
  match **assignment {
    Assignment { ref identifiers, ref expression } => {
      for ident in identifiers {
        visitor.visit_identifier(ident);
      }

      visitor.visit_expression(expression);
    }
  }
}

pub fn walk_switch<'v, V>(visitor: &mut V, switch: &'v Node<Switch>)
  where V: Visitor<'v>
{
  match **switch {
    Switch { ref expression, ref cases, ref default } => {
      visitor.visit_expression(expression);

      for case in cases {
        visitor.visit_case(case);
      }

      match *default {
        Some(ref block) => {
          visitor.visit_block(block);
        },
        None => {}
      }
    }
  }
}

pub fn walk_case<'v, V>(visitor: &mut V, case: &'v Node<Case>)
  where V: Visitor<'v>
{
  match **case {
    Case { ref expression, ref block } => {
      visitor.visit_expression(expression);
      visitor.visit_block(block);
    }
  }
}

pub fn walk_for_loop<'v, V>(visitor: &mut V, for_loop: &'v Node<ForLoop>)
  where V: Visitor<'v>
{
  match **for_loop {
    ForLoop { ref init, ref condition, ref post, ref body } => {
      visitor.visit_block(init);
      visitor.visit_expression(condition);
      visitor.visit_block(post);
      visitor.visit_block(body);
    }
  }
}

pub fn walk_sub_assembly<'v, V>(visitor: &mut V, sub: &'v Node<SubAssembly>)
  where V: Visitor<'v>
{
  match **sub {
    SubAssembly { ref identifier, ref block } => {
      visitor.visit_identifier(identifier);
      visitor.visit_block(block);
    }
  }
}

pub fn walk_function_call<'v, V>(visitor: &mut V, call: &'v Node<FunctionCall>)
  where V: Visitor<'v>
{
  match **call {
    FunctionCall { ref identifier, ref arguments } => {
      visitor.visit_identifier(identifier);
      for expression in arguments {
        visitor.visit_expression(expression);
      }
    }
  }
}

pub fn walk_literal<'v, V>(visitor: &mut V, literal: &'v Node<Literal>)
  where V: Visitor<'v>
{
  match **literal {
    Literal::HexNumber(ref node) => visitor.visit_hex_number(node),
    Literal::DecNumber(ref node) => visitor.visit_dec_number(node),
    Literal::StringLiteral(ref node) => visitor.visit_string_literal(node),
    Literal::HexLiteral(ref node) => visitor.visit_hex_literal(node),
  }
}

