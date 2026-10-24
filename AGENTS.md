# Agent Working Guidelines & Standards

This document outlines coding standards, engineering principles, and operational rules for any AI agent or contributor developing **TUF Control Center**.

---

## 1. Safety First for Hardware Control
- **Never guess sysfs nodes or WMI parameters**. All writes must correspond to documented ASUS kernel interfaces (`asus-nb-wmi`) or standard Linux subsystems (`power_supply`, `hwmon`).
- **Never bypass input validation**. The privileged daemon must reject any out-of-range value before writing to the kernel.
- **Fail-safe defaults**: If custom control fails or is terminated, revert to embedded controller (EC) automatic firmware control.
- **Preserve Battery Life**: Do not wake the NVIDIA discrete GPU when in runtime D3cold suspended state simply to read temperature or clock speeds.

---

## 2. Code Quality & Modularity
- **Rust Edition**: Rust 2021 edition.
- **Architecture**: Strict multi-crate Cargo workspace:
  - `crates/tuf-core`: Core data structures, telemetry parsing, HAL traits, mock backend, config.
  - `crates/tuf-daemon`: Privileged D-Bus daemon with Polkit authorization.
  - `crates/tuf-cli`: Command-line interface.
  - `crates/tuf-gui`: GTK4 / Libadwaita desktop application.
- **Error Handling**: Use `thiserror` for typed library errors in `tuf-core` and `anyhow` for top-level binaries.
- **Asynchronous Execution**: Polling and I/O must never block the GTK main UI thread. Use `glib::MainContext` channel receivers or async worker tasks.

---

## 3. UI/UX Guidelines
- Emulate the clean, compact, non-intrusive aesthetic of G-Helper.
- Seamless GNOME 4x+ desktop integration using Libadwaita standard widgets (`AdwPreferencesPage`, `AdwActionRow`, `AdwSwitchRow`, `AdwComboRow`).
- Dark and light theme auto-adaptation.
- Always provide clear badges and tooltips when hardware features are unsupported on a specific model (e.g. MUX switch or Fn-Lock on FA506NC).

---

## 4. Verification & Testing
- Every crate must compile cleanly without warnings.
- Run `cargo test` to verify unit tests for telemetry parsing, input validation, and configuration migration.
- Always test `--simulate` mode to ensure the GUI works reliably without physical root or ASUS hardware.
