# Dendritic Calculus

**Dendritic Calculus** is an esoteric programming language based on manipulating one single tree (*dendron*) using simple arithmetic and one basic control instruction. 

Despite the limitations, it is Turing-Complete.

See the [language reference](https://cancrizans.github.io/dendritic-calculus) for details.

## Interpreter

This repository includes a compiler-interpreter for DC written in Rust.

This is how you run a program through CLI:

```bash
$ cargo run sample_programs/triang.dc 8
36

$ cargo run sample_programs/add_one.dc 5[[1]]+7
5[[1]]+8
```

