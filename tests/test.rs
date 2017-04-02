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
