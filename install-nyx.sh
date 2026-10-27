#!/usr/bin/env bash
set -euo pipefail
# Piped from curl (`curl -fsSL …/install-nyx.sh | bash`): no checkout here, so fetch
# the source and run its installer from there.
if [[ -z ${BASH_SOURCE[0]:-} ]]; then
  command -v git >/dev/null || { echo "Git is needed to install nyx-files." >&2; exit 1; }
  work=$(mktemp -d)
  trap 'rm -rf "$work"' EXIT
  git clone -q --depth 1 https://github.com/design-nexus/nyx-files.git "$work/src"
  bash "$work/src/install-nyx.sh" "$@"
  exit
fi
cd -- "$(dirname -- "$0")"
# A release archive (.prebuilt) brings its binaries in target/release.
if [[ -f .prebuilt ]]; then binary=target/release/nyx-files; helper=target/release/nyx-files-rar
elif [[ -f Cargo.toml ]]; then cargo build --release --locked; binary=target/release/nyx-files; helper=target/release/nyx-files-rar; else binary=./nyx-files; helper=./nyx-files-rar; fi
bin_dir="$HOME/.local/bin"
data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
install -Dm755 "$helper" "$bin_dir/nyx-files-rar.new"
mv -f "$bin_dir/nyx-files-rar.new" "$bin_dir/nyx-files-rar"
install -Dm755 "$binary" "$bin_dir/nyx-files.new"
mv -f "$bin_dir/nyx-files.new" "$bin_dir/nyx-files"
install -Dm644 data/icons/scalable/apps/io.github.design_nexus.nyx.Files.svg "$data_home/icons/hicolor/scalable/apps/io.github.design_nexus.nyx.Files.svg"
install -d "$data_home/applications" "$data_home/dbus-1/services"
sed "s|^Exec=nyx-files|Exec=$bin_dir/nyx-files|; s|^TryExec=.*|TryExec=$bin_dir/nyx-files|" data/io.github.design_nexus.nyx.Files.desktop > "$data_home/applications/io.github.design_nexus.nyx.Files.desktop"
for service in "$data_home"/dbus-1/services/*.service; do
  if [[ "$service" != "$data_home/dbus-1/services/io.github.design_nexus.nyx.Files.FileManager1.service" ]] && grep -qx 'Name=org.freedesktop.FileManager1' "$service"; then
    mv -- "$service" "$service.nyx-files-backup"
  fi
done
cat > "$data_home/dbus-1/services/io.github.design_nexus.nyx.Files.FileManager1.service" <<SERVICE
[D-BUS Service]
Name=org.freedesktop.FileManager1
Exec=$bin_dir/nyx-files --gapplication-service
SERVICE
install -d "$data_home/licenses/nyx-files"
for notice in LICENSE LICENSE-MIT COPYING THIRD_PARTY_LICENSES.md; do
  install -m644 "$notice" "$data_home/licenses/nyx-files/$notice"
done
update-desktop-database "$data_home/applications"
xdg-mime default io.github.design_nexus.nyx.Files.desktop inode/directory
