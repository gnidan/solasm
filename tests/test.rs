extern crate solasm;
extern crate bigint;
extern crate env_logger;
use solasm::grammar::*;
use solasm::ast::Item;
use bigint::U256;

#[test]
fn it_works() {
    let _ = env_logger::init();
    assert_eq!(hex_number("0xFF").unwrap().uint, U256::from(255));
    assert_eq!(dec_number("10").unwrap().uint, U256::from(10));
    assert_eq!(string("\"10\"").unwrap().string, "10".to_string());
    assert_eq!(hex_literal("hex\"FF11FFFF\"").unwrap().bytes,
               vec![255, 17, 255, 255]);
    assert_eq!(hex_literal("hex'FF11FFFF'").unwrap().bytes,
               vec![255, 17, 255, 255]);

    assert_eq!(identifier("foo").unwrap().symbol, "foo".to_string());
    assert_eq!(identifier("$foo").unwrap().symbol, "$foo".to_string());
    assert_eq!(identifier("$foo_").unwrap().symbol, "$foo_".to_string());

    assert_eq!(label_definition("x:").unwrap().identifier.symbol,
               "x".to_string());
    assert_eq!(assignment("=: x").unwrap().identifier.symbol,
               "x".to_string());

    assert_eq!(identifier_list("x, y, z").unwrap(),
               vec![identifier("x").unwrap(), identifier("y").unwrap(), identifier("z").unwrap()]);

    assert_eq!(identifier_or_list("(x, y, z)").unwrap(),
               identifier_list("x, y, z").unwrap());
    assert_eq!(identifier_or_list("x").unwrap(),
               identifier_list("x").unwrap());



}

#[test]
fn it_ignores_comments() {
    assert_eq!(hex_number("/* hex number */ 0xFF").unwrap().uint,
               U256::from(255));
    assert_eq!(hex_number("// hex number\n 0xFF").unwrap().uint,
               U256::from(255));
}

#[test]
fn it_parses_items() {
    assert_eq!(item("foo").unwrap(),
               Item::Identifier(identifier("foo").unwrap()));
    assert_eq!(item("break").unwrap(), Item::Break());

}

#[test]
fn it_parses_block() {
    let _ = env_logger::init();
    let assembly = r#"{
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


    let result = block(assembly);
    assert!(result.is_ok());

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

    let result2 = block(assembly2);
    assert!(result2.is_ok());


    let assembly3 = r#"{
        let n := calldataload(4)
        let a := 1
        let b := a
    loop:
        jumpi(loopend, eq(n, 0))
        a add swap1
        n := sub(n, 1)
        jump(loop)
    loopend:
        mstore(0, a)
        return(0, 0x20)
    }"#;

    let result3 = block(assembly3);

    assert!(result3.is_ok());
}
