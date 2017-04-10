#[cfg(test)]
mod tests {
    extern crate solasm;
    extern crate bigint;
    use solasm::asm::grammar;
    // use self::solasm::grammar::*;
    // use self::solasm::ast::{Statement, Expression, ControlOp};
    // use self::bigint::U256;

    // #[test]
    // fn it_ignores_comments() {
    //     assert_eq!(hex_number("/* hex number */ 0xFF").unwrap().uint,
    //                U256::from(255));
    //     assert_eq!(hex_number("// hex number\n 0xFF").unwrap().uint,
    //                U256::from(255));
    // }

    // #[test]
    // fn it_parses_building_blocks() {
    //     assert_eq!(hex_number("0xFF").unwrap().uint, U256::from(255));
    //     assert_eq!(dec_number("10").unwrap().uint, U256::from(10));
    //     assert_eq!(string_literal("\"10\"").unwrap().string, "10".to_string());
    //     assert_eq!(hex_literal("hex\"FF11FFFF\"").unwrap().bytes,
    //                vec![255, 17, 255, 255]);
    //     assert_eq!(hex_literal("hex'FF11FFFF'").unwrap().bytes,
    //                vec![255, 17, 255, 255]);

    //     assert_eq!(identifier("foo").unwrap().symbol, "foo".to_string());
    //     assert_eq!(identifier("$foo").unwrap().symbol, "$foo".to_string());
    //     assert_eq!(identifier("$foo_").unwrap().symbol, "$foo_".to_string());

    //     assert_eq!(label_definition("x:").unwrap().identifier.symbol,
    //                "x".to_string());
    // }

    // #[test]
    // fn it_parses_statements() {
    //     assert_eq!(statement("foo").unwrap(),
    //                Statement::Expression(
    //                    Expression::Identifier(identifier("foo").unwrap())
    //                ));
    //     assert_eq!(statement("break").unwrap(),
    //                 Statement::ControlOp(ControlOp::Break()));
    // }

    #[test]
    fn it_parses_switches_and_functions() {
        let assembly = r#"
        {
          mstore(0x40, 0x60) // store the "free memory pointer"
          // function dispatcher
          switch div(calldataload(0), exp(2, 226))
          case 0xb3de648b: {
            let (r) := f(calldataload(4))
            let ret := $allocate(0x20)
            mstore(ret, r)
            return(ret, 0x20)
          }
          default: { jump(invalidJumpLabel) }
          // memory allocator
          function $allocate(size) -> (pos) {
            pos := mload(0x40)
            mstore(0x40, add(pos, size))
          }
          // the contract function
          function f(x) -> (y) {
            y := 1
            for { let i := 0 } lt(i, x) { i := add(i, 1) } {
              y := mul(2, y)
            }
          }
        }"#;
        assert_parses_ok(assembly);

        let assembly2 = r#"{
          function power(base, exponent) -> (result) {
            switch exponent
            case 0: { result := 1 }
            case 1: { result := base }
            default: {
                result := power(mul(base, base), div(exponent, 2))
                switch mod(exponent, 2)
                    case 1: { result := mul(base, result) }
            }
          }
        }"#;
        assert_parses_ok(assembly2);
    }

    #[test]
    fn it_parses_for_loops() {
        let assembly = r#"{
            function power(base, exponent) -> (result)
            {
                result := 1
                for { let i := 0 } lt(i, exponent) { i := add(i, 1) }
                {
                    result := mul(result, base)
                }
            }
        }"#;
        assert_parses_ok(assembly);
    }

    fn assert_parses_ok(assembly: &str) {
        let result = grammar::block(assembly);
        match result {
            Ok(tree) => { println!("{:?}", tree) },
            Err(err) => { panic!("{:?}", err) },
        }
    }
}
