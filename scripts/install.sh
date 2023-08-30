#!/usr/bin/env bash

set -euo pipefail

# =============================================================================
# Define helper functions
# =============================================================================

text_bold() {
  echo -e "\033[1m$1\033[0m"
}
text_title() {
  echo ""
  text_bold "$1"
  if [ "$2" != "" ]; then echo "$2"; fi
}
text_title_error() {
    echo ""
    echo -e "\033[1;31m$1\033[00m"
}

# =============================================================================
# Define base variables
# =============================================================================

NAME="gitnr"
REPO="reemus-dev/$NAME"
VERSION="latest"
DOWNLOAD_BASE_URL="https://github.com/$REPO/releases/download/$VERSION"

if [ "$VERSION" == "latest" ]; then
  # The latest version is accessible from a slightly different URL
  DOWNLOAD_BASE_URL="https://github.com/$REPO/releases/latest/download"
fi

# =============================================================================
# Define binary list for supported OS & Arch
# =============================================================================

declare -A BINARIES=(
  ["Linux:x86_64"]="$NAME-linux-amd64"
  ["Darwin:x86_64"]="$NAME-macos-amd64"
  ["Darwin:arm64"]="$NAME-macos-arm64"
)

# =============================================================================
# Get the user's OS and Arch
# =============================================================================

OS="$(uname -s)"
ARCH="$(uname -m)"
SYSTEM="${OS}:${ARCH}"

# =============================================================================
# Match a binary to check if the system is supported
# =============================================================================

if [[ ! ${BINARIES["$SYSTEM"]+_} ]]; then
  text_title_error "Error"
  echo " Unsupported OS or arch: ${SYSTEM}"
  echo ""
  exit 1
fi

# =============================================================================
# Set the default installation variables
# =============================================================================

INSTALL_DIR="/usr/local/bin"
BINARY="${BINARIES["$SYSTEM"]}"
DOWNLOAD_URL="$DOWNLOAD_BASE_URL/$BINARY"

# =============================================================================
# Handle script arguments if passed
#  -u: install to user bin directory
#  -d <path>: specify installation directory
# =============================================================================

if [ $# -gt 0 ]; then
  while getopts ":ud:" opt; do
  case $opt in
    u)
      # Set default install dir based on OS
      [ "$OS" == "Darwin" ] && INSTALL_DIR="$HOME/bin" || INSTALL_DIR="$HOME/.local/bin"

      # Check that the user bin directory is in their PATH
      IFS=':' read -ra PATHS <<< "$PATH"
      INSTALL_DIR_IN_PATH="false"
      for P in "${PATHS[@]}"; do
        if [[ "$P" == "$INSTALL_DIR" ]]; then
          INSTALL_DIR_IN_PATH="true"
        fi
      done

      # If user bin directory doesn't exist or not in PATH, exit
      if [ ! -d "$INSTALL_DIR" ] || [ "$INSTALL_DIR_IN_PATH" == "false" ]; then
        text_title_error "Error"
        echo " The user bin directory '$INSTALL_DIR' does not exist or is not in your environment PATH variable"
        echo " To fix this error:"
        echo " - Omit the '-u' option and to install system-wide"
        echo " - Specify an installation directory with -d <path>"
        echo ""
        exit 1
      fi

      ;;
    d)
      # Get absolute path in case a relative path is provided
      INSTALL_DIR=$(cd "$OPTARG"; pwd)

      if [ ! -d "$INSTALL_DIR" ]; then
        text_title_error "Error"
        echo " The installation directory '$INSTALL_DIR' does not exist or is not a directory"
        echo ""
        exit 1
      fi

      ;;
    \?)
      text_title_error "Error"
      echo " Invalid option: -$OPTARG" >&2
      echo ""
      exit 1
      ;;
    :)
      text_title_error "Error"
      echo " Option -$OPTARG requires an argument." >&2
      echo ""
      exit 1
      ;;
  esac
done
fi

# =============================================================================
# Create and change to temp directory
# =============================================================================

cd "$(mktemp -d)"

# =============================================================================
# Download binary
# =============================================================================

text_title "Downloading Binary" " $DOWNLOAD_URL"
curl -LO --proto '=https' --tlsv1.2 -sSf "$DOWNLOAD_URL"

# =============================================================================
# Make binary executable and move to install directory with appropriate name
# =============================================================================

text_title "Installing Binary" " $INSTALL_DIR/$NAME"
chmod +x "$BINARY"
mv "$BINARY" "$INSTALL_DIR/$NAME"

# =============================================================================
# Display post install message
# =============================================================================

text_title "Installation Complete" " Run $NAME --help for more information"
echo ""
