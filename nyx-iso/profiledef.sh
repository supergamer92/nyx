#!/usr/bin/env bash
# archiso profile definition for Nyx OS

iso_name="nyx-os"
iso_label="NYX_$(date +%Y%m)"
iso_publisher="Nyx OS <https://nyx-os.org>"
iso_application="Nyx OS Live/Install"
iso_version="0.1.0"
install_dir="nyx"
buildmodes=('iso')
bootmodes=(
    'bios.syslinux.mbr'
    'bios.syslinux.eltorito'
    'uefi-ia32.systemd-boot.esp'
    'uefi-x64.systemd-boot.esp'
)
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'zstd' '-Xcompression-level' '15')
file_permissions=(
    ["/etc/shadow"]="0:0:400"
    ["/usr/bin/nyx-shell"]="0:0:755"
    ["/usr/bin/nyx-compositor"]="0:0:755"
    ["/usr/bin/nyx-settingsd"]="0:0:755"
    ["/usr/bin/nyx-hw"]="0:0:755"
    ["/usr/bin/nyx-updater"]="0:0:755"
    ["/usr/bin/nyx-start-desktop"]="0:0:755"
)
