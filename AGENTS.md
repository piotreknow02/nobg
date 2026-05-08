# **nobg** - Rust CLI/WebUI for background removal using u2net models and ONNX Runtime.

## Build
```bash
cargo build                     # Debug
cargo build --release           # Release
```

## Tests
```bash
cargo test
cargo test -- --nocapture
```

## Lint and format
```bash
cargo clippy # lint
cargo fmt # format
```

## Code Style
- Imports: `use crate::module::Item` for local, absolute for external
- Alphabetize imports within groups, no wildcards
- Prefer `Result<T, E>` over `Option` where applicable
- Use `thiserror::Error` for generic error, but prefer module-specific custom error types like `crate::inference::error::Error`
- Propagate errors with `?`, `match` exhaustiveness required, never omit them
- `tokio::runtime::Runtime` for blocking, `async` functions preferred

## Module Structure
- `src/main.rs` - CLI routing
- `src/cli.rs` - Command definitions
- `src/model/` - Model management
- `src/inference/` - Inference pipeline
- `src/webui.rs` - Axum server
- `src/webui_assets.rs` - Embedded static files for web ui
- `webui/` - web ui web app code

## Common Tasks
- Add model: `src/model/registry.rs`, `MODELS` array
- Add command: `src/cli.rs` variant, impl in `commands.rs`, update `main.rs`
