#!/bin/bash
# Bootstrap script for x402 tools
# Downloads and installs platform-specific binaries

set -e

REPO="second-state/payment-skill"
SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="${SKILL_DIR}/scripts"

# Detect platform
detect_platform() {
    local os arch

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
    local artifact_name="payment-${platform}.zip"

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
    zip_file="${temp_dir}/payment-${platform}.zip"

    # Download
    echo "Fetching from: ${url}" >&2
    if command -v curl &>/dev/null; then
        curl -sL -o "$zip_file" "$url"
    else
        wget -q -O "$zip_file" "$url"
    fi

    # Extract
    echo "Extracting binaries..." >&2
    if command -v unzip &>/dev/null; then
        unzip -q -o "$zip_file" -d "${SCRIPTS_DIR}"
    else
        echo "Error: unzip not found. Please install unzip." >&2
        rm -rf "$temp_dir"
        exit 1
    fi

    # Make binaries executable (not needed on Windows)
    if [[ "$(uname -s)" != MINGW* ]] && [[ "$(uname -s)" != MSYS* ]] && [[ "$(uname -s)" != CYGWIN* ]]; then
        find "${SCRIPTS_DIR}" -maxdepth 1 -type f ! -name ".*" -exec chmod +x {} \;
    fi

    # Cleanup
    rm -rf "$temp_dir"

    echo "x402 tools installed successfully to ${SCRIPTS_DIR}" >&2
}

# Ensure config file exists
ensure_config() {
    local config_dir="${HOME}/.payment"
    local config_file="${config_dir}/config.toml"
    local default_config="${SKILL_DIR}/config-default.toml"

    if [ ! -f "$config_file" ]; then
        echo "Creating default config at ${config_file}..." >&2

        # Create directory if it doesn't exist
        if [ ! -d "$config_dir" ]; then
            mkdir -p "$config_dir"
            chmod 700 "$config_dir"
        fi

        # Copy default config
        if [ -f "$default_config" ]; then
            cp "$default_config" "$config_file"
            chmod 600 "$config_file"
            echo "Default config created (base-mainnet with USDC)" >&2
        else
            echo "Warning: Default config not found at ${default_config}" >&2
        fi
    fi
}

# Main
main() {
    # Ensure config exists before anything else
    ensure_config

    local platform
    platform=$(detect_platform)
    echo "Detected platform: ${platform}" >&2

    local download_url
    download_url=$(get_download_url "$platform")

    download_binaries "$platform" "$download_url"

    # List installed binaries
    echo "" >&2
    echo "Installed tools:" >&2
    ls -1 "${SCRIPTS_DIR}" | grep -v '^\.' >&2
}

main "$@"
