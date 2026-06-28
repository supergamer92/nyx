#!/usr/bin/env bash
#
# Nyx OS ISO Builder
#
# Builds a bootable ISO image using archiso.
# The ISO boots into a live Nyx desktop with an installer app.
#
# Usage:
#   sudo ./build.sh
#
# Requirements:
#   - archiso package installed
#   - Running on Arch Linux
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
WORK_DIR="/tmp/nyx-iso-build"
OUT_DIR="$PROJECT_ROOT/out"
PROFILE_DIR="$SCRIPT_DIR"

echo "╔══════════════════════════════════════╗"
echo "║       Nyx OS ISO Builder v0.1.0      ║"
echo "╚══════════════════════════════════════╝"
echo ""

# Check for root
if [[ $EUID -ne 0 ]]; then
    echo "Error: This script must be run as root (sudo)"
    exit 1
fi

# Check for archiso
if ! command -v mkarchiso &> /dev/null; then
    echo "Error: archiso is not installed"
    echo "Install it with: sudo pacman -S archiso"
    exit 1
fi

# Clean previous build
echo "[1/5] Cleaning previous build..."
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"
mkdir -p "$OUT_DIR"

# Copy archiso profile
echo "[2/5] Setting up ISO profile..."
cp -r /usr/share/archiso/configs/releng/* "$WORK_DIR/"

# Overlay our customizations
echo "[3/5] Applying Nyx customizations..."

# Add Nyx packages to the package list
cat "$PROJECT_ROOT/nyx-base/packages.txt" | grep -v '^#' | grep -v '^$' >> "$WORK_DIR/packages.x86_64"

# Copy airootfs overlay (live environment customizations)
if [[ -d "$SCRIPT_DIR/airootfs" ]]; then
    cp -r "$SCRIPT_DIR/airootfs/"* "$WORK_DIR/airootfs/"
fi

# Set up live environment auto-login and compositor start
mkdir -p "$WORK_DIR/airootfs/etc/systemd/system/getty@tty1.service.d/"
cat > "$WORK_DIR/airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf" << 'EOF'
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin nyx --noclear %I $TERM
EOF

# Build the ISO
echo "[4/5] Building ISO (this takes a while)..."
mkarchiso -v -w "$WORK_DIR/work" -o "$OUT_DIR" "$WORK_DIR"

echo "[5/5] Done!"
echo ""
echo "ISO written to: $OUT_DIR/"
ls -lh "$OUT_DIR/"*.iso 2>/dev/null || echo "  (no ISO found — check build output above)"
echo ""
echo "Flash to USB with:"
echo "  sudo dd bs=4M if=$OUT_DIR/nyx-*.iso of=/dev/sdX status=progress"
echo ""
echo "Or test in QEMU:"
echo "  qemu-system-x86_64 -enable-kvm -m 4G -cdrom $OUT_DIR/nyx-*.iso"
