# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

---

## Reporting a Vulnerability

We take the security of TUF Control Center and hardware control on Linux seriously.

If you believe you have discovered a security vulnerability in `tuf-daemon`, `tuf-gui`, `tuf-cli`, or our PolicyKit configuration:

1. **Do not open a public GitHub issue.**
2. Report the vulnerability privately via [GitHub Security Advisory](https://github.com/TharinduRashan/TUF-Control-Center/security/advisories/new).
3. Alternatively, contact the maintainers directly through GitHub.

Please include:
- A description of the issue and potential impact
- Steps or a Proof-of-Concept to reproduce the behavior
- Your environment details (Linux kernel version, Ubuntu release, laptop model)

We will acknowledge receipt within 48 hours and work with you on a patch prior to any public disclosure.

---

## Security Architecture Overview

TUF Control Center implements a strict least-privilege model:
- **Unprivileged GUI & CLI**: Desktop UI and CLI commands run entirely as an unprivileged user without root permissions.
- **D-Bus Daemon**: Hardware modifications are brokered by `tuf-daemon` via the system D-Bus socket.
- **PolicyKit Verification**: Every state-changing method verifies caller authorization using PolicyKit (`pkcheck` bound to D-Bus sender bus names, preventing PID-reuse attacks).
- **Fail-Closed Authorization**: If authorization checks cannot be completed or pkcheck fails, all requests are strictly denied.
- **Disruptive Operations Protected**: While non-disruptive tweaks (e.g. battery limits) allow active desktop session users, disruptive operations (such as GPU switching) require administrator authorization (`auth_admin_keep`).
- **No Arbitrary Execution**: No user strings are passed to shells. Sysfs writes target strictly predefined canonical kernel nodes.

For full architectural details, see [docs/SECURITY.md](docs/SECURITY.md).
