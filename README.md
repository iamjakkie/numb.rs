# numb.rs

A `no_std`-compatible Rust crate for numerical computation and linear algebra, built from scratch using const generics.

Every operation in this crate was implemented by hand and verified against manual calculations. No AI-generated code. No wrappers around existing libraries. Built to understand the math, not just call it.

## What it does

`numb.rs` provides generic matrix and vector types with compile-time dimensions using Rust's const generics. All data is stack-allocated (`[[T; N]; M]`) — no heap, no `Vec`, no `Box`.

```rust
use numbrs::{matrix, vector};

// Create matrices with compile-time dimensions
let a = matrix![[1.0, 2.0], [3.0, 4.0]];
let b = matrix![[5.0, 6.0], [7.0, 8.0]];
let c = a * b;

// Vectors
let v = vector![[1.0, 2.0, 3.0]];
let dot = v.dot(&v);
```

## Features

### Vectors
- Row and column vectors with const-generic dimensions
- Addition, subtraction, scalar multiplication
- Dot product
- Macro-based construction (`vector!`)

### Matrices
- Generic `Matrix<T, M, N>` with stack-allocated storage
- Addition, subtraction, scalar multiplication
- Matrix multiplication (dimension-checked at compile time)
- Transposition
- Identity matrix construction
- Macro-based construction (`matrix!`)

### Complex numbers
- Complex arithmetic (add, subtract, multiply, divide)
- Conjugation
- Compatible with vector and matrix operations
- Uses `num::Complex` internally

## Constraints

- `no_std` compatible (with `default-features = false` on the `num` dependency)
- Zero heap allocation — all storage is `[[T; N]; M]` on the stack
- Generic over numeric types via `num::Num` trait bounds

## What's not here (yet)

Determinant, inverse, eigenvalue decomposition, LU/QR/Cholesky factorization, SVD, and geometric transformations are not implemented. This crate covers the fundamentals — if you need production-grade linear algebra, use [nalgebra](https://nalgebra.org) or [faer](https://github.com/sarah-ek/faer-rs).

## Used in

- [`swarm-loc`](https://github.com/iamjakkie/swarm-loc) — cooperative localization library for GPS-denied embedded swarm systems (in development)

## License

MIT