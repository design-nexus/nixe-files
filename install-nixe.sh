#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
if [[ -f Cargo.toml ]]; then cargo build --release --locked; binary=target/release/nixe-files; helper=target/release/nixe-files-rar; else binary=./nixe-files; helper=./nixe-files-rar; fi
bin_dir="$HOME/.local/bin"
data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
install -Dm755 "$helper" "$bin_dir/nixe-files-rar.new"
mv -f "$bin_dir/nixe-files-rar.new" "$bin_dir/nixe-files-rar"
install -Dm755 "$binary" "$bin_dir/nixe-files.new"
mv -f "$bin_dir/nixe-files.new" "$bin_dir/nixe-files"
install -Dm644 data/icons/scalable/apps/io.github.design_nexus.nixe.Files.svg "$data_home/icons/hicolor/scalable/apps/io.github.design_nexus.nixe.Files.svg"
install -d "$data_home/applications" "$data_home/dbus-1/services"
sed "s|^Exec=nixe-files|Exec=$bin_dir/nixe-files|; s|^TryExec=.*|TryExec=$bin_dir/nixe-files|" data/io.github.design_nexus.nixe.Files.desktop > "$data_home/applications/io.github.design_nexus.nixe.Files.desktop"
for service in "$data_home"/dbus-1/services/*.service; do
  if [[ "$service" != "$data_home/dbus-1/services/io.github.design_nexus.nixe.Files.FileManager1.service" ]] && grep -qx 'Name=org.freedesktop.FileManager1' "$service"; then
    mv -- "$service" "$service.nixe-files-backup"
  fi
done
cat > "$data_home/dbus-1/services/io.github.design_nexus.nixe.Files.FileManager1.service" <<SERVICE
[D-BUS Service]
Name=org.freedesktop.FileManager1
Exec=$bin_dir/nixe-files --gapplication-service
SERVICE
update-desktop-database "$data_home/applications"
xdg-mime default io.github.design_nexus.nixe.Files.desktop inode/directory
