use std::fmt::Write;


use super::super::grammar;
use ast::*;
use self::visitor::*;

pub struct PrettyPrinter<'a, W: 'a> {
  out: &'a mut W,

  indent: u32,
  statement_newlines: bool,
}


impl<'a, W: Write> PrettyPrinter<'a, W> {
  pub fn print(block: &'a Node<Block>, out: &mut W) {
    PrettyPrinter {
      indent: 0,
      statement_newlines: false,
      out: out,
    }.visit_block(block);
  }

  pub fn newline(&mut self) {
    write!(&mut self.out, "\n").ok();
    self.print_indent();
  }

  pub fn print_indent(&mut self) {
    for _ in 0..(self.indent * 2) {
      write!(&mut self.out, " ").ok();
    }
  }

  fn print_before_list(&mut self) {
    write!(&mut self.out, "(").ok();
  }

  fn print_between_list_items(&mut self) {
    write!(&mut self.out, ", ").ok();
  }

  fn print_after_list(&mut self) {
    write!(&mut self.out, ")").ok();
  }

  fn print_list<T, F>(&mut self, items: &'a Vec<Node<T>>, f: F)
    where T: 'a, F: Fn(&'a Node<T>)
  {
    write!(&mut self.out, "(").ok();
    for ref item in items {
      f(&item);
      write!(&mut self.out, ", ").ok();
    }
  }

}

impl<'v, W:Write> Visitor<'v> for PrettyPrinter<'v, W> {
  fn visit_block(&mut self, b: &'v Node<Block>) {
    let number_of_statements = (*b).statements.len();

    write!(&mut self.out, "{{").ok();
    match number_of_statements {
      0 => { write!(&mut self.out, " ").ok(); },
      1 => {
        write!(&mut self.out, " ").ok();
        b.walk(self);
        write!(&mut self.out, " ").ok();
      },
      _ => {
        let old_statement_newlines = self.statement_newlines;
        self.statement_newlines = true;
        self.indent += 1;
        b.walk(self);
        self.indent -= 1;
        self.statement_newlines = old_statement_newlines;
        self.newline();
      }
    }
    write!(&mut self.out, "}}").ok();
  }

  fn visit_statement(&mut self, s: &'v Node<Statement>) {
    if self.statement_newlines { self.newline(); }
    &s.walk(self);
  }

  fn visit_function_definition(&mut self, f: &'v Node<FunctionDefinition>) {
    write!(&mut self.out, "function ");
    match **f {
      FunctionDefinition { ref identifier, ref arguments, ref returns, ref body } => {
        self.visit_identifier(identifier);
        self.print_before_list();
        for (i, identifier) in arguments.iter().enumerate() {
          if i != 0 {
            self.print_between_list_items();
          }

          self.visit_identifier(identifier);
        }
        self.print_after_list();

        match *returns {
          Some(ref identifiers) => {
            write!(&mut self.out, " ->").ok();
            self.print_before_list();
            for (i, identifier) in identifiers.iter().enumerate() {
              if i != 0 {
                self.print_between_list_items();
              }

              self.visit_identifier(identifier);
            }
            self.print_after_list();
          },
          None => {}
        }

        write!(&mut self.out, " ").ok();
        self.visit_block(body);
      }
    }
  }

  fn visit_variable_declaration(&mut self, v: &'v Node<VariableDeclaration>) {
    write!(&mut self.out, "let ");
    match **v {
      VariableDeclaration { ref identifiers, .. } => {
        if identifiers.len() == 1 {
          self.visit_identifier(&identifiers[0]);
        } else {
          self.print_before_list();
          for (i, identifier) in identifiers.iter().enumerate() {
            if i != 0 {
              self.print_between_list_items();
            }

            self.visit_identifier(identifier);
          }
          self.print_after_list();
        }
      }
    }

    write!(&mut self.out, " := ");
    match **v {
      VariableDeclaration { ref expression, .. } => {
        self.visit_expression(expression);
      }
    }
  }

  fn visit_function_call(&mut self, c: &'v Node<FunctionCall>) {
    match **c {
      FunctionCall { ref identifier, ref arguments } => {
        self.visit_identifier(identifier);
        self.print_before_list();
        for (i, expression) in arguments.iter().enumerate() {
          if i != 0 {
            self.print_between_list_items();
          }

          self.visit_expression(expression);
        }
        self.print_after_list();
      }
    }
  }

  fn visit_identifier(&mut self, i: &'v Node<Identifier>) {
    write!(&mut self.out, "{}", (*i).symbol).ok();
  }

  fn visit_string_literal(&mut self, s: &'v Node<StringLiteral>) {
    write!(&mut self.out, "\"{}\"", (*s).string);
  }

  fn visit_hex_literal(&mut self, x: &'v Node<HexLiteral>) {
    write!(&mut self.out, "hex\"{:?}\"", (*x).bytes);
  }

  fn visit_hex_number(&mut self, x: &'v Node<HexNumber>) {
    write!(&mut self.out, "{:x}", (*x).uint);
  }

  fn visit_dec_number(&mut self, n: &'v Node<DecNumber>) {
    write!(&mut self.out, "{}", (*n).uint);
  }

}

fn assert_print_quine(program: &str) {
  let block = grammar::block(program).unwrap();
  let mut s = String::new();
  PrettyPrinter::print(&block, &mut s);

  if s != program {
    println!("{}", s);
    panic!("source doesn't line up with output");
  }
}

#[test]
fn it_writes_braces() {
  let block = grammar::block("{ let (i, j) := 0 j k }").unwrap();

  let mut s = String::new();
  PrettyPrinter::print(&block, &mut s);

  let expected =
r#"{
  let (i, j) := 0
  j
  k
}"#;

  assert_eq!(s, expected);
}

#[test]
fn it_writes_functions() {
  let mut program;

  program = r#"{ function frobinate(i, j) { } }"#;
  assert_print_quine(program);

  program = r#"{
  function frobinate(i, j) {
    i
    j
    k
  }
  let frobbed := frobinate(x, y)
}"#;
  assert_print_quine(program);

  program = r#"{
  function frobinate(i, j) {
    i
    frobinate(j, k)
    k
  }
  let frobbed := frobinate(x, y)
}"#;
  assert_print_quine(program);
}
