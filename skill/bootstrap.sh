#!/bin/bash
# Bootstrap script for x402 tools
# Downloads platform-specific binaries on first use

set -e

REPO="second-state/x402-agent-tools"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="${SCRIPT_DIR}/scripts"

# Detect platform
detect_platform() {
    local os arch platform

    # Detect OS
    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="darwin" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *)
            echo "Error: Unsupported operating system: $(uname -s)" >&2
            exit 1
            ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)
            echo "Error: Unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac

    echo "${os}-${arch}"
}

# Get the download URL for the latest release
get_download_url() {
    local platform="$1"
    local artifact_name="x402-${platform}.zip"

    # Try to get latest release URL using GitHub API
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    local download_url

    if command -v curl &>/dev/null; then
        download_url=$(curl -sL "$api_url" | grep -o "https://github.com/${REPO}/releases/download/[^\"]*${artifact_name}" | head -1)
    elif command -v wget &>/dev/null; then
        download_url=$(wget -qO- "$api_url" | grep -o "https://github.com/${REPO}/releases/download/[^\"]*${artifact_name}" | head -1)
    else
        echo "Error: Neither curl nor wget found. Please install one of them." >&2
        exit 1
    fi

    if [ -z "$download_url" ]; then
        echo "Error: Could not find release for platform ${platform}" >&2
        echo "Please check https://github.com/${REPO}/releases for available downloads." >&2
        exit 1
    fi

    echo "$download_url"
}

# Download and extract binaries
download_binaries() {
    local platform="$1"
    local url="$2"
    local temp_dir
    local zip_file

    echo "Downloading x402 tools for ${platform}..." >&2

    # Create scripts directory if it doesn't exist
    mkdir -p "${SCRIPTS_DIR}"

    # Create temp directory
    temp_dir=$(mktemp -d)
    zip_file="${temp_dir}/x402-${platform}.zip"

    # Download
    if command -v curl &>/dev/null; then
        curl -sL -o "$zip_file" "$url"
    else
        wget -q -O "$zip_file" "$url"
    fi

    # Extract
    if command -v unzip &>/dev/null; then
        unzip -q -o "$zip_file" -d "${SCRIPTS_DIR}"
    else
        echo "Error: unzip not found. Please install unzip." >&2
        rm -rf "$temp_dir"
        exit 1
    fi

    # Make binaries executable (not needed on Windows)
    if [ "$(uname -s)" != "MINGW"* ] && [ "$(uname -s)" != "MSYS"* ] && [ "$(uname -s)" != "CYGWIN"* ]; then
        chmod +x "${SCRIPTS_DIR}"/*
    fi

    # Cleanup
    rm -rf "$temp_dir"

    echo "x402 tools installed successfully." >&2
}

# Main bootstrap function
# Usage: bootstrap_and_run <tool_name> [args...]
bootstrap_and_run() {
    local tool_name="$1"
    shift

    local platform
    local binary_path
    local binary_name

    platform=$(detect_platform)

    # Determine binary name (add .exe for Windows)
    if [[ "$platform" == windows-* ]]; then
        binary_name="${tool_name}.exe"
    else
        binary_name="${tool_name}"
    fi

    binary_path="${SCRIPTS_DIR}/${binary_name}"

    # Check if binary exists, download if not
    if [ ! -x "$binary_path" ]; then
        local download_url
        download_url=$(get_download_url "$platform")
        download_binaries "$platform" "$download_url"
    fi

    # Verify binary exists after download
    if [ ! -x "$binary_path" ]; then
        echo "Error: Binary ${binary_name} not found after download." >&2
        exit 1
    fi

    # Execute the tool
    exec "$binary_path" "$@"
}
