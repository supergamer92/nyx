# Nyx OS

A clean, modern operating system built on Arch Linux. No baggage. No compromises.

## Vision

Nyx is an opinionated operating system that delivers a unified, consumer-grade experience rivaling Windows and macOS — while keeping the power of Linux underneath for those who want it.

## Architecture

```
┌─────────────────────────────────────────┐
│  Nyx Shell (Iced/wgpu)                  │
│  ┌─────┐ ┌──────┐ ┌────────┐ ┌───────┐ │
│  │ Dock│ │TopBar│ │Launcher│ │ Apps  │ │
│  └─────┘ └──────┘ └────────┘ └───────┘ │
├─────────────────────────────────────────┤
│  Nyx Compositor (Smithay + wgpu)        │
├─────────────────────────────────────────┤
│  Nyx Platform Services                  │
│  Settings · Updates · Hardware · Portal │
├─────────────────────────────────────────┤
│  Arch Linux Base                        │
│  systemd · PipeWire · NetworkManager    │
├─────────────────────────────────────────┤
│  Linux Kernel                           │
└─────────────────────────────────────────┘
```

## Crates

| Crate | Description |
|-------|-------------|
| `nyx-compositor` | Wayland compositor built on Smithay |
| `nyx-shell` | Desktop shell UI (top bar, dock, launcher) |
| `nyx-widgets` | Custom Iced widget library (Nyx Lux design) |
| `nyx-settings` | System settings daemon + UI |
| `nyx-updater` | Atomic update orchestrator |
| `nyx-hw` | Hardware detection & driver management |
| `nyx-portal` | XDG desktop portal implementation |
| `nyx-session` | Session manager (login, lock, sleep) |
| `nyx-auth` | Authentication & polkit agent |

## Building

```bash
# Build all crates
cargo build --workspace

# Run the shell in development mode (nested window)
cargo run -p nyx-shell

# Build release
cargo build --workspace --release
```

## Requirements

- Rust 1.85+
- Linux (for compositor — shell/widgets can develop on any OS)
- Wayland-compatible GPU drivers

## License

GPL-3.0-or-later
