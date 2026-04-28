#!/bin/sh
# Installer for maxwell, the spinning ASCII cat for your terminal.
# Usage:  curl -fsSL https://raw.githubusercontent.com/ka1kqi/maxwell/main/install.sh | sh

set -e

REPO="ka1kqi/maxwell"
URL="https://github.com/$REPO/releases/latest/download/maxwell"
DEST="/usr/local/bin/maxwell"
TMP="/tmp/maxwell.$$"

if [ "$(uname -s)" != "Darwin" ]; then
    echo "This installer currently supports macOS only."
    echo "On Linux, build from source:  cargo install --git https://github.com/$REPO"
    exit 1
fi

echo "Downloading maxwell..."
if ! curl -fSL "$URL" -o "$TMP"; then
    echo "Download failed. Check your internet connection or try again later."
    exit 1
fi
chmod +x "$TMP"

echo "Installing to $DEST (you'll be asked for your Mac password)..."
sudo mv "$TMP" "$DEST"
sudo xattr -d com.apple.quarantine "$DEST" 2>/dev/null || true

echo ""
echo "Installed. Type 'maxwell' in any Terminal window."
echo "Press Ctrl+C to make him leave."
