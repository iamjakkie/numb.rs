# AGENTS.md — numb.rs

`no_std`, const-generic, stack-only linear algebra in Rust. **A learning crate: the library code in `src/` is written by hand.** It is not a nalgebra replacement; nalgebra is the test oracle (dev-dependency only).

## Commands
- Build: `cargo build --all-targets`
- Test: `cargo test`
- Lint: `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`
- `no_std` check: `cargo build --no-default-features --target thumbv7em-none-eabihf` (install with `rustup target add thumbv7em-none-eabihf`)

## Layout
- `src/matrix.rs` matrices and vectors · `src/decomposition.rs` LU/Cholesky · `src/geometry.rs` rotations · `src/lib.rs` exports
- `tests/` integration tests; `tests/common/` comparison helpers against nalgebra

## Repo-specific rules
- Agents never write library logic in `src/`, whatever the task; Plumbing tasks here are CI, benches, test scaffolding and docs.
- No heap in the core: no `Vec`/`alloc` in `src/` outside `#[cfg(feature = "std")]`.
- Tolerances in tests are stated explicitly (relative error or ULPs), never `assert_eq!` on floats.

## How tasks work
- Planning lives in the private hq repo (`iamjakkie/hq`). Each task here is an issue with **Context**, **Acceptance oracles**, **Allowed files** and a **Kind**; its first line links the hq original (`HQ copy: iamjakkie/hq#N`).
- Start from `gh issue view <n>`. Restate goal, oracles and allowed files, propose a plan, and wait for approval before editing.
- Behave according to **Kind**:
  - **Plumbing (agent)** – implement it, inside Allowed files only.
  - **Core (by hand)** – the human writes the logic. Do not write implementation code for it. Explain, point to references, review diffs, suggest test cases.
  - **Thinking** – produce the requested document; no code.
- Branch: `task/<n>-<short-slug>`. PR description: `Closes #<n>, closes iamjakkie/hq#<N>`, then what you verified (command + result) and what you did **not** verify. Tick only the oracles you actually checked.

## Always
- Run the build, tests and linters below before calling anything done.
- Keep the diff minimal and inside Allowed files.
- Cite the source (book/paper, section or equation) in a comment for any algorithm or equation.

## Ask first
- Adding or upgrading dependencies.
- Changing public APIs, feature flags or CI.
- Anything that needs a file outside Allowed files.

## Never
- Weaken, skip or delete a failing test to make CI pass. Edit existing tests only if the task's Allowed files include them.
- Edit files marked `// HAND-CORE` or `# HAND-CORE`.
- Commit secrets. LLM access goes through OpenRouter with the key in an env var (`OPENROUTER_API_KEY`).
