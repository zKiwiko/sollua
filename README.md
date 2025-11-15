# Sollua

A Simple, fast, beautiful Lua 5.4 parsing Rust crate.
Sollua is the main parser in [llva](https://github.com/zkiwiko/llva) an Lua LLVM compiler.

## Why another lua parsing crate?

The rest are extremely slow and/or generate a useless "AST" filled with junk
that are _unusable_ when creating a lua compiler in Rust. Luckily **Sollua**
is designed to be extremely easy to implement with a beautifully generated AST
to walk manually or overriding the `ASTVisitor` trait.

## What features does it support?

Sollua supports every feature present in Lua 5.4 **EXCEPT!!!** the following:

- Long Strings with Deliminers: `[=[content]=]`
- Hex Float numbers: `0xff.ff` (who uses these??)

### Benchmarks against other crates

| Crate      | 1000 Lines | 2500 Lines | 5000 Lines |
| :--------- | :--------: | :--------: | :--------: |
| **sollua** |    N/A     |    N/A     |    N/A     |
| luaparse   |    N/A     |    N/A     |    N/A     |
| luaparser  |    N/A     |    N/A     |    N/A     |
| rslua      |    N/A     |    N/A     |    N/A     |
| full_moon  |    N/A     |    N/A     |    N/A     |
