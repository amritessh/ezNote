#!/usr/bin/env bash
set -e

# ezNote Universal Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/eznote/main/install.sh | bash

REPO="YOUR_USERNAME/eznote" # UPDATE THIS!
BINARY_NAME="ezn"
INSTALL_DIR="/usr/local/bin"

echo "🚀 Installing ezNote..."

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
Linux*) OS_TYPE="linux" ;;
Darwin*) OS_TYPE="macos" ;;
MINGW* | MSYS* | CYGWIN*) OS_TYPE="windows" ;;
*)
  echo "❌ Unsupported OS: $OS"
  exit 1
  ;;
esac

case "$ARCH" in
x86_64 | amd64) ARCH_TYPE="x86_64" ;;
arm64 | aarch64) ARCH_TYPE="aarch64" ;;
*)
  echo "❌ Unsupported architecture: $ARCH"
  exit 1
  ;;
esac

echo "📦 Detected: $OS_TYPE-$ARCH_TYPE"

# Get latest release version
echo "🔍 Fetching latest version..."
LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
  echo "❌ Could not fetch latest version. Make sure you have a release published."
  exit 1
fi

echo "📥 Downloading ezNote $LATEST_VERSION..."

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION/ezn-${OS_TYPE}-${ARCH_TYPE}"

if [ "$OS_TYPE" = "windows" ]; then
  DOWNLOAD_URL="${DOWNLOAD_URL}.exe"
fi

# Download binary
TMP_FILE=$(mktemp)
if ! curl -fsSL "$DOWNLOAD_URL" -o "$TMP_FILE"; then
  echo "❌ Download failed. Binary might not exist for your platform."
  echo "Available at: https://github.com/$REPO/releases"
  exit 1
fi

# Make executable
chmod +x "$TMP_FILE"

# Install
echo "📝 Installing to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
else
  echo "   (requires sudo)"
  sudo mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
fi

echo ""
echo "✅ ezNote installed successfully!"
echo ""
echo "Quick start:"
echo "  ezn add \"My first note\""
echo "  ezn list"
echo "  ezn today"
echo ""
echo "Run 'ezn --help' for all commands"
echo ""
echo "⭐ Star the repo: https://github.com/$REPO"
