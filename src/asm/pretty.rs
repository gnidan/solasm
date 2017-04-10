use std::io::Write;

use asm::ast::*;
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
}

impl<'v, W:Write> Visitor<'v> for PrettyPrinter<'v, W> {
  fn visit_block(&mut self, b: &'v Node<Block>) {
    let number_of_statements = (*b).statements.len();

    write!(&mut self.out, "{{").ok();
    match number_of_statements {
      0 => { write!(&mut self.out, " ").ok(); },
      1 => {
        write!(&mut self.out, " ").ok();
        let old_statement_newlines = self.statement_newlines;
        self.statement_newlines = false;
        b.walk(self);
        self.statement_newlines = old_statement_newlines;
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
    write!(&mut self.out, "function ").ok();
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
            write!(&mut self.out, " -> ").ok();
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
    write!(&mut self.out, "let ").ok();
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

    write!(&mut self.out, " := ").ok();
    match **v {
      VariableDeclaration { ref expression, .. } => {
        self.visit_expression(expression);
      }
    }
  }

  fn visit_assignment(&mut self, v: &'v Node<Assignment>) {
    match **v {
      Assignment { ref identifiers, .. } => {
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

    write!(&mut self.out, " := ").ok();
    match **v {
      Assignment { ref expression, .. } => {
        self.visit_expression(expression);
      }
    }
  }

  fn visit_switch(&mut self, f: &'v Node<Switch>) {
    write!(&mut self.out, "switch ").ok();
    match **f {
      Switch { ref expression, ref cases, ref default } => {
        self.visit_expression(expression);

        for case in cases {
          if self.statement_newlines {
            self.newline();
          } else {
            write!(&mut self.out, " ").ok();
          }
          self.visit_case(case);
        }

        match *default {
          Some(ref block) => {
            if self.statement_newlines {
              self.newline();
            } else {
              write!(&mut self.out, " ").ok();
            }
            write!(&mut self.out, "default: ").ok();
            self.visit_block(block);
          },
          None => {}
        }
      }
    }
  }

  fn visit_case(&mut self, f: &'v Node<Case>) {
    write!(&mut self.out, "case ").ok();
    match **f {
      Case { ref expression, ref block } => {
        self.visit_expression(expression);
        write!(&mut self.out, ": ").ok();
        self.visit_block(block);
      }
    }
  }

  fn visit_for_loop(&mut self, f: &'v Node<ForLoop>) {
    write!(&mut self.out, "for ").ok();
    match **f {
      ForLoop { ref init, ref condition, ref post, ref body } => {
        self.visit_block(init);
        write!(&mut self.out, " ").ok();
        self.visit_expression(condition);
        write!(&mut self.out, " ").ok();
        self.visit_block(post);
        write!(&mut self.out, " ").ok();
        self.visit_block(body);
      }
    }
  }

  fn visit_control_op(&mut self, o: &'v Node<ControlOp>) {
    match **o {
      ControlOp::Break() => { write!(&mut self.out, "break").ok(); },
      ControlOp::Continue() => { write!(&mut self.out, "continue").ok(); },
    }
  }

  fn visit_sub_assembly(&mut self, a: &'v Node<SubAssembly>) {
    match **a {
      SubAssembly { ref identifier, ref block } => {
        write!(&mut self.out, "assembly ").ok();
        self.visit_identifier(identifier);
        write!(&mut self.out, " ").ok();
        self.visit_block(block);
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
    write!(&mut self.out, "\"{}\"", (*s).string).ok();
  }

  fn visit_hex_literal(&mut self, x: &'v Node<HexLiteral>) {
    write!(&mut self.out, "hex\"{:?}\"", (*x).bytes).ok();
  }

  fn visit_hex_number(&mut self, x: &'v Node<HexNumber>) {
    write!(&mut self.out, "{:x}", (*x).uint).ok();
  }

  fn visit_dec_number(&mut self, n: &'v Node<DecNumber>) {
    write!(&mut self.out, "{}", (*n).uint).ok();
  }

}

#[cfg(test)]
use asm::grammar;

#[cfg(test)]
use std::io::BufWriter;

#[cfg(test)]
use std::str::from_utf8;

#[cfg(test)]
fn assert_print_quine(program: &str) {
  let block = grammar::block(program).unwrap();
  let mut buf = vec![];
  {
    let mut out : BufWriter<_> = BufWriter::new(&mut buf);
    PrettyPrinter::print(&block, &mut out);
  }

  let s: &str = from_utf8(&mut buf).unwrap();
  if s != program {
    println!("{}", s);
    panic!("source doesn't line up with output");
  }
}

#[test]
fn it_writes_braces() {
  let program =
r#"{
  let (i, j) := 0
  j
  k
}"#;

  assert_print_quine(program);
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

#[test]
fn it_writes_switches() {
  let mut program;
  program = r#"{ switch i case 0: {
  i
  j
  k
} case 1: { foo } default: { } }"#;
  assert_print_quine(program);

  program = r#"{
  switch i
  case 0: {
    i
    j
    k
  }
  case 1: { foo }
  default: { }
  i
}"#;
  assert_print_quine(program);
}

#[test]
fn it_writes_for_loops() {
  let program;
  program = r#"{ for { let i := 0 } lt(i, 5) { i := add(i, 1) } {
  i
  j
  k
} }"#;
  assert_print_quine(program);
}

#[test]
fn it_writes_sub_assemblies() {
  let program;
  program = r#"{ assembly fnord { for { let i := 0 } lt(i, 5) { i := add(i, 1) } {
  i
  j
  k
} } }"#;
  assert_print_quine(program);
}
