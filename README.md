# Sollua

A Simple, fast, beautiful Lua 5.4 parsing Rust crate.

## Why another lua parsing crate?

Other options are extremely outdated, slow, generate _ugly_ AST's, or just
refuse to compile for release builds. Luckily **Sollua** is a modern, extremely fast, beautifly crafted parsing
crate anyone and their grandmothers could use.
**Sollua** is the perfect option for developers who want to create powerful compilers, language servers,
semantic analyzers, etc.

## What features does it support?

Sollua supports every feature present in Lua 5.4 **EXCEPT!!!** the following:

- Long Strings with Deliminers: `[=[content]=]`
- Hex Float numbers: `0xff.ff` (who uses these??)

### Benchmarks against other crates

Each result is the average after running 5 tests.
The files used in each test are in the `lua` directory of the repository.

| Crate      | 1000 Funcs (7337 Lines) | 2500 Funcs (18337 Lines) | 5000 Funcs (36669 Lines) |
| :--------- | :---------------------: | :----------------------: | :----------------------: |
| **sollua** |          9 ms           |         19.2 ms          |         33.2 ms          |
| luaparse   |         44.6 ms         |          125 ms          |          246 ms          |
| full_moon  |          48 ms          |          137 ms          |           246            |

## Usage

### Installation

```bash
cargo add sollua
```

### Implementation Example

```rust
use sollua::lexer::Lexer;
use sollua::parser::Parser;

fn main() {
    let source = "function add(a, b) return a + b end";

    let mut tokens = Lexer::new(source).collect();
    let mut parser = parser::new(source, &tokens);
    let ast = parser.parse();

    println!("AST: \n{:?}", ast);

    if parser.errors.len() > 0 {
        for error in &parser.errors {
            println!("Parser error: {:?}", error);
        }
    }
}

```

**Expected Output**:

```
AST:
[
    Statement(
        FunctionDeclaration {
            name_path: [
                "add",
            ],
            is_method: false,
            parameters: [
                "a",
                "b",
            ],
            body: Statement(
                Block(
                    [
                        Statement(
                            Return(
                                [
                                    BinaryOp {
                                        left: Variable(
                                            "a",
                                        ),
                                        operator: Plus,
                                        right: Variable(
                                            "b",
                                        ),
                                    },
                                ],
                            ),
                        ),
                    ],
                ),
            ),
        },
    ),
]
```
