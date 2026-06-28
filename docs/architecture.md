# Nyx OS Architecture

## System Boot Sequence

```
Power On
  └→ UEFI / systemd-boot
      └→ Linux kernel (LTS)
          └→ systemd (PID 1)
              ├→ nyx-hw          (hardware detection)
              ├→ NetworkManager   (networking)
              ├→ PipeWire         (audio/video)
              ├→ nyx-settingsd    (configuration daemon)
              └→ nyx-session      (session manager)
                  └→ Greeter (login screen)
                      └→ Authentication
                          └→ nyx-compositor  (Wayland compositor)
                              ├→ nyx-shell   (desktop shell)
                              ├→ nyx-portal  (XDG portals)
                              ├→ nyx-auth    (polkit agent)
                              └→ User apps
```

## Component Communication

All components communicate via **D-Bus** (system bus + session bus).

| Service | Bus | Interface |
|---------|-----|-----------|
| Settings | Session | `org.nyx.Settings` |
| Hardware | System | `org.nyx.Hardware` |
| Updates | System | `org.nyx.Updater` |
| Portal | Session | `org.freedesktop.portal.*` |
| Auth | Session | `org.nyx.AuthAgent` |
| Session | Session | `org.nyx.Session` |

## Design Decisions

### Why Smithay over wlroots?
- Pure Rust — no C FFI overhead, no unsafe boundary to audit
- Better integration with Rust async ecosystem
- Proven by COSMIC desktop (System76)

### Why Iced over GTK/Qt?
- GPU-rendered via wgpu — every frame goes through the GPU
- Full control over rendering — no theme engine inconsistencies
- Same language as compositor — entire stack is Rust
- No system library dependencies to break on updates

### Why btrfs?
- Copy-on-write snapshots enable atomic rollbacks
- Transparent compression saves disk space
- Subvolumes provide clean separation of system/home/snapshots
- Proven at scale (Facebook/Meta uses it on all servers)

### Why Flatpak (not Snap/AppImage)?
- True sandboxing via bubblewrap
- Portal-based permission system integrates with our desktop
- No daemon requirement (unlike Snap's snapd)
- Flathub has the largest app catalog
