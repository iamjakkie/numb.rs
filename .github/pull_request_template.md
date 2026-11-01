Closes #

## Why

<!-- What problem does this solve? Link the issue above and add one or two sentences of context. -->

## What changed

<!-- Short list of the changes. Mention any public API additions/removals explicitly. -->

## Math identities / tests added

<!-- Which identities or properties are now verified by tests (e.g. `A * A.inverse() == I`, `det(AB) == det(A) * det(B)`), and in which test file. -->

## Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` and `cargo test --no-default-features` pass
- [ ] `cargo build --no-default-features --target thumbv7em-none-eabihf` passes (no_std)
- [ ] `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features` passes
- [ ] No panics in library code (no `unwrap`/`expect`/`panic!`/indexing that can go out of bounds); errors are returned, not thrown
- [ ] Every new `pub` item has a doc comment
