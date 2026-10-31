#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"

echo "=== Building grit ==="
cargo ${GRIT_TOOLCHAIN:++$GRIT_TOOLCHAIN} build --release -p grit-cli

source_binary="target/release/grit"
install_directory="$HOME/.local/bin"
installed_binary="$install_directory/grit"
trash_directory="$HOME/.Trash"
candidate="$install_directory/.grit-candidate-$$"

mkdir -p "$install_directory" "$trash_directory"
install -m 755 "$source_binary" "$candidate"
xattr -c "$candidate" 2>/dev/null || true
"$candidate" --help >/dev/null

if [ -e "$installed_binary" ]; then
  mv "$installed_binary" "$trash_directory/grit.replaced-$(date +%s)-$$"
fi
mv "$candidate" "$installed_binary"
cmp "$source_binary" "$installed_binary"

echo "Installed: $installed_binary"
shasum -a 256 "$installed_binary"
