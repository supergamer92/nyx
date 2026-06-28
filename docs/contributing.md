# Contributing to Nyx OS

Thank you for your interest in contributing to Nyx OS!

## Getting Started

### Prerequisites

- **Rust 1.85+** — `rustup install stable`
- **Linux** (for compositor development) — Arch Linux recommended
- **Windows/macOS** (for widget/shell UI development) — works cross-platform

### Building

```bash
# Clone the repo
git clone https://github.com/nyx-os/nyx.git
cd nyx

# Build everything
cargo build --workspace

# Build specific crate
cargo build -p nyx-widgets
cargo build -p nyx-shell

# Run the shell in development mode
cargo run -p nyx-shell

# Run tests
cargo test --workspace
```

### Development Workflow

1. **Widget/Shell development** — works on any OS. Run `cargo run -p nyx-shell` to see the desktop shell in a window.
2. **Compositor development** — requires Linux with Wayland. Run in nested mode inside an existing desktop.
3. **System services** — require Linux with systemd for full testing.

## Code Style

- Follow `rustfmt` defaults
- Use `clippy` — `cargo clippy --workspace`
- All public APIs must have doc comments
- Prefer `thiserror` for error types
- Use `tracing` for logging (not `log` or `println!`)

## Architecture

See [architecture.md](architecture.md) for system design details.

## Pull Request Process

1. Fork the repo and create a feature branch
2. Make your changes
3. Run `cargo fmt` and `cargo clippy`
4. Run `cargo test --workspace`
5. Submit a PR with a clear description

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0-or-later.
