extern crate solasm;
extern crate bigint;
use solasm::grammar::*;
use bigint::U256;

#[test]
fn it_works() {
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
}

#[test]
fn it_ignores_comments() {
    assert_eq!(hex_number("/* hex number */ 0xFF").unwrap().uint,
               U256::from(255));
    assert_eq!(hex_number("// hex number\n 0xFF").unwrap().uint,
               U256::from(255));
}
