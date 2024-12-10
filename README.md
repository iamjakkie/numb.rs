# numb.rs

`numb.rs` is a cutting-edge Rust crate for numerical computation, linear algebra, and geometry. Designed to be modular and efficient, it supports vectors, matrices, complex numbers, and geometric transformations, making it ideal for both general-purpose and scientific computing.

## Features

### Status
- ✅ Feature A (Working)
- ❌ Feature B (Missing)
- ⚠️ Feature C (Partially Implemented)

### Vectors
- **Creation**:
  - ✅ Define vectors using the `vector!` macro for row and column vectors.
  - Supports repeated values, square roots, and explicit element definitions.
- **Operations**:
  - ✅ Addition
  - ✅ Scalar multiplication
  - ✅ Dot product
  - ❌ Tensor product.
  - ❌ Basis transformations and orthogonality checks.
  - ❌ Supports complex-valued vectors.
- **Theorems**:
  - ❌ Gram-Schmidt orthogonalization.
  - ❌ Cauchy-Schwarz inequality.

### Matrices
- **Creation**:
  - ✅ Use the `matrix!` macro for defining matrices with rows and columns.
  - ❌ Generate identity matrices with `identity_matrix!`.
- **Operations**:
  - ✅ Addition
  - ✅ Scalar multiplication
  - ✅ Matrix multiplication
  - ❌ Transposition.
  - ❌ Conjugate transpose (Hermitian)
  - ❌ Inverse
  - ❌ Determinant.
  - ❌ Eigenvalue and eigenvector calculations.
- **Properties**:
  - ❌ Square matrix detection and trace calculation.
  - ❌ Complex-valued matrices supported.

### Complex Numbers
- **Creation**:
  - ⚠️ Define complex numbers using the `complex!` macro with intuitive syntax:
    - `complex!(1 + 2i)` for `1 + 2i`.
    - `complex!(3 - 4i)` for `3 - 4i`.
    - `complex!(5i)` for pure imaginary numbers.
- **Operations**:
  - ✅ Addition
  - ✅ Subtraction
  - ✅ Multiplication
  - ✅ Division
  - ✅ Conjugation
  - ❌ Modulus calculation.
  - ❌ Compatible with vector and matrix computations.

### Geometry
- **Transformations**:
  - ❌ Reflect
  - ❌ Rotate
  - ❌ Project
  - ❌ Translate points in 2D/3D space.
  - ❌ Use `rotation_2d!` and `rotation_3d!` for generating rotation matrices.
- **Advanced Features**:
  - ❌ Projections onto lines or planes.
  - ❌ Scaling and shearing transformations.

### Decompositions
- ❌ Spectral decomposition
- ❌ Singular value decomposition (SVD)
- ❌ Polar decomposition.
- ❌ QR decomposition.
- ❌ LU decomposition.
- ❌ Cholesky decomposition.

### Additional Features
- ❌ Solve systems of linear equations using matrix operations.
- ❌ Support for trigonometric functions
- ❌ Eigen decompositions,
- ❌ and more.

### Development
- ❌ Error handling and robustness.
- ⚠️ Unit tests and benchmarks.
- ❌ Documentation and examples.
- ❌ Parallel processing and GPU acceleration.

---

## Macros

### `vector!`
1. **Row Vector**:
   - `vector![[1.0, 2.0, 3.0]]` → Row vector `[1.0, 2.0, 3.0]`.
2. **Column Vector**:
   - `vector![[1.0]; 3]` → Column vector `[[1.0], [1.0], [1.0]]`.
3. **Repeated Values**:
   - `vector![[0.0; 3]]` → Row vector `[0.0, 0.0, 0.0]`.
   - `vector![[0.0]; 3]` → Column vector `[[0.0], [0.0], [0.0]]`.

### `matrix!`
1. **Explicit Values**:
   - `matrix![[1, 2], [3, 4]]` → 2x2 matrix.
2. **Repeated Values**:
   - `matrix![0; 2, 2]` → 2x2 zero matrix.
3. **Empty Matrix**:
   - `matrix![[]]` → Empty matrix.

### `identity_matrix!`
- Generate identity matrices:
  - `identity_matrix!(f64, 3)` → 3x3 identity matrix.

### `sqrt_vector!`
- Create vectors with square roots:
  - `sqrt_vector!(4.0, 9.0)` → `[2.0, 3.0]`.

### `complex!`
- Intuitive syntax for complex numbers:
  - `complex!(1 + 2i)` → `1 + 2i`.
  - `complex!(3, -4)` → `3 - 4i`.

### `rotation_2d!` and `rotation_3d!`
- Generate rotation matrices:
  - `rotation_2d!(angle)` → 2D rotation matrix.
  - `rotation_3d!(angle, axis)` → 3D rotation matrix along a specific axis.

---

## Roadmap

### Near-Term Goals
- **Complex Number Enhancements**:
  - Modularize the complex number implementation for reuse.
  - Add support for exponential and logarithmic operations.

- **Advanced Matrix Features**:
  - LU, QR, and Cholesky decompositions.
  - Block matrix operations.
  - Advanced linear system solvers.

- **Optimizations**:
  - GPU offloading for large-scale computations.
  - Parallel processing for matrix multiplications and decompositions.

### Long-Term Goals
- **Quantum Computing Extensions**:
  - Introduce `qbit.rs` for quantum-specific operations.
  - Support for Dirac notation and quantum gates.

- **Visualization Tools**:
  - Integrate with `wgpu` or other Rust libraries for rendering geometric transformations.

---

## Contributing
We welcome contributions! Here’s how you can help:
1. Fork the repository.
2. Open issues for bugs, improvements, or feature requests.
3. Submit pull requests with clear descriptions and tests.

---

## License
This project is licensed under the MIT License.