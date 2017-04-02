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
}
