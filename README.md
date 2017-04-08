# solasm
Solidity Assembler in Rust

[![Build Status](https://travis-ci.org/gnidan/solasm.svg?branch=master)](https://travis-ci.org/gnidan/solasm)
[![Coverage Status](https://coveralls.io/repos/github/gnidan/solasm/badge.svg?branch=master)](https://coveralls.io/github/gnidan/solasm?branch=master)

## Current Status

- [x] Parser
- [x] AST pretty-printing
- [ ] Semantics checking
- [ ] Desuraging phase
- [ ] Opcode generation
- [ ] Runtime interpreter?

## Installing

```bash
$ git clone git@github.com:gnidan/solasm.git
$ cd solasm
$ cargo build
```

## Running

After building, find executable at `./target/debug/solasm`.

### Outputting formatted ASM

Pass `--ast` option: `solasm --ast`:

File _test.asm:_
```
{function frobinate(x,y)->(z,t){z:=add(cos(x),sin(y))t:=sub(sin(x),cos(y))}for{let i:=0}lt(i,5){inc(i)}{mul(i,8)mstore(i)}switch i case underfrobbed(i):{frobinate(4,1)frobinate(6, 7)}case overfrobbed(i):{frobinate(0, 0)}}
```

Run:
```bash
cat test.asm | solasm --ast
```

Output:
```
{
  function frobinate(x, y) -> (z, t) {
    z := add(cos(x), sin(y))
    t := sub(sin(x), cos(y))
  }
  for { let i := 0 } lt(i, 5) { inc(i) } {
    mul(i, 8)
    mstore(i)
  }
  switch i
  case underfrobbed(i): {
    frobinate(4, 1)
    frobinate(6, 7)
  }
  case overfrobbed(i): { frobinate(0, 0) }
}
```
