# Contributing to TUF Control Center

Thank you for your interest in contributing to TUF Control Center! We welcome bug reports, feature requests, documentation improvements, and code contributions.

---

## Development Setup

TUF Control Center is written in Rust (2021 Edition) using GTK4 and Libadwaita for GNOME desktop integration.

### Prerequisites (Ubuntu 24.04 / 26.04)

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    curl \
    pkg-config \
    libgtk-4-dev \
    libadwaita-1-dev \
    libdbus-1-dev \
    dpkg-dev \
    cargo-clippy \
    rustfmt
```

---

## Workspace Structure

The project is structured as a Cargo workspace:
- `crates/tuf-core`: Core data models, telemetry parser, capabilities detection, and mock backend.
- `crates/tuf-daemon`: Privileged D-Bus daemon running with PolicyKit authorization.
- `crates/tuf-cli`: Fast, scriptable command-line interface.
- `crates/tuf-gui`: Libadwaita desktop application.

---

## Testing & Quality Checks

Before submitting a pull request, ensure all checks pass:

```bash
# 1. Format verification
cargo fmt --check

# 2. Clippy lint verification (zero warnings)
cargo clippy --workspace -- -D warnings

# 3. Workspace unit tests
cargo test --workspace

# 4. Debian release packaging
./scripts/build-deb.sh
```

---

## Hardware Safety Guidelines

Hardware control code must adhere to safety standards:
1. **Never guess sysfs nodes**: All writes must target documented ASUS interfaces (`asus-nb-wmi`) or standard Linux subsystems (`power_supply`, `hwmon`).
2. **Never bypass input validation**: The daemon must validate all parameters before writing to kernel sysfs nodes.
3. **Preserve battery life**: Never wake the discrete NVIDIA GPU when in runtime D3cold suspended state simply to read telemetry.
4. **Fail-closed authorization**: PolicyKit checks must always fail closed on errors or absent dependencies.
