# confrisk Configuration System

## Overview

confrisk v0.2+ introduces a **config-driven architecture** that makes it generic, extensible, and plug-and-play. Instead of hardcoded checks, everything is defined in JSON configuration files.

## Architecture

```
config/
├── categories.json           # Issue category definitions
├── scoring.json              # Risk model weights (configurable!)
├── checks/                   # Individual check definitions
│   ├── ssh-root-login.json
│   ├── shadow-permissions.json
│   └── ...
├── plugins/                  # External scanner integrations
│   ├── trivy.json
│   ├── lynis.json
│   ├── gitleaks.json
│   └── osv-scanner.json
└── rules/                    # User-defined security rules
    ├── dependencies.json     # Vulnerable package blocklist
    └── ports.json            # Dangerous port rules
```

## Issue Categories

12 predefined categories covering all aspects of system security:

| Category | ID | Scope |
|----------|-----|-------|
| Privilege & Access Control | `PRIVILEGES` | sudo, PAM, user permissions |
| Software Dependencies | `DEPENDENCIES` | Vulnerable packages, outdated libraries |
| Network Ports | `OPEN_PORTS` | Exposed services, listening ports |
| Logging & Auditing | `LOGS` | Audit config, log retention |
| Secrets & Credentials | `SECRETS` | API keys, passwords, certificates |
| Network Configuration | `NETWORK` | Firewall, routing, DNS |
| Kernel Hardening | `KERNEL` | ASLR, stack protection, sysctl |
| Container Security | `CONTAINERS` | Docker, Kubernetes config |
| File System | `FILES` | Permissions, mount options |
| Process & Services | `PROCESSES` | Systemd, cron, running services |
| Compliance & Policy | `COMPLIANCE` | CIS, NIST, PCI-DSS |
| Encryption & TLS | `ENCRYPTION` | Disk encryption, cipher suites |

See `config/categories.json` for full definitions.

## Configurable Risk Model

The scoring model is now **fully configurable** via `config/scoring.json`:

```json
{
  "severity": {
    "critical": 10.0,
    "high": 8.0,
    "medium": 5.5,
    "low": 3.0,
    "info": 1.0
  },
  "asset_criticality": {
    "crown_jewel": 1.3,
    "production": 1.1,
    "internal": 0.8,
    "dev": 0.5
  },
  "exposure": {
    "public": 1.5,
    "internet_facing": 1.25,
    "adjacent_network": 0.95,
    "local": 0.7
  }
}
```

**Customize these weights** to match your organization's risk appetite!

## Check Definitions

Each check is a standalone JSON file in `config/checks/`. Example:

```json
{
  "id": "SSH-001",
  "name": "SSH root login permitted",
  "category": "PRIVILEGES",
  "description": "SSH allows direct root login...",
  "severity": "high",
  "exposure": "internet_facing",
  "confidence": 0.95,
  "effort": "trivial",
  "enabled": true,

  "detection": {
    "type": "config_directive",
    "file": "/etc/ssh/sshd_config",
    "directive": "PermitRootLogin",
    "expected": "no"
  },

  "remediation": {
    "summary": "Disable direct root login via SSH",
    "steps": [
      "Edit /etc/ssh/sshd_config",
      "Set 'PermitRootLogin no'",
      "Restart SSH: systemctl restart sshd"
    ],
    "references": [
      "https://www.ssh.com/academy/ssh/sshd_config",
      "CIS Benchmark 5.2.10"
    ]
  },

  "tags": ["ssh", "authentication", "cis-benchmark"]
}
```

### Detection Types

#### 1. Config Directive

Check configuration file values:

```json
{
  "detection": {
    "type": "config_directive",
    "file": "/etc/ssh/sshd_config",
    "directive": "PasswordAuthentication",
    "expected": "no"
  }
}
```

#### 2. File Permission

Check file/directory permissions:

```json
{
  "detection": {
    "type": "file_permission",
    "file": "/etc/shadow",
    "check": "not_world_readable",
    "max_mode": "0640"
  }
}
```

#### 3. Command Output

Run command and check output:

```json
{
  "detection": {
    "type": "command_output",
    "command": "docker info --format '{{.SecurityOptions}}'",
    "pattern": "apparmor",
    "expected": "present"
  }
}
```

#### 4. File Exists

Check if file/directory exists:

```json
{
  "detection": {
    "type": "file_exists",
    "file": "/etc/kubernetes/manifests",
    "should_exist": false
  }
}
```

#### 5. Custom Script

Run custom detection script:

```json
{
  "detection": {
    "type": "custom",
    "script": "/usr/local/bin/check-k8s-rbac.sh"
  }
}
```

### Adding Your Own Checks

1. Create a new JSON file in `config/checks/`
2. Follow the schema above
3. Set `"enabled": true`
4. Run confrisk — it will automatically pick up the new check!

## Plugin System

Integrate external scanners **without writing code**. Plugins are defined in `config/plugins/`.

### Supported Plugins

#### Trivy (Container & Vulnerability Scanner)

```bash
# Enable in config/plugins/trivy.json
{
  "enabled": true
}
```

Scans:
- Filesystem for vulnerabilities
- Container images
- IaC misconfigurations

#### Lynis (Security Auditing)

```bash
# Enable in config/plugins/lynis.json
{
  "enabled": true
}
```

Runs comprehensive system audit, integrates 200+ checks.

#### Gitleaks (Secrets Detection)

```bash
# Enable in config/plugins/gitleaks.json
{
  "enabled": true
}
```

Scans for API keys, passwords, tokens in files and git history.

#### OSV-Scanner (Dependency Vulnerabilities)

```bash
# Enable in config/plugins/osv-scanner.json
{
  "enabled": true
}
```

Checks dependencies against Google's OSV database.

### Plugin Configuration Structure

```json
{
  "name": "trivy",
  "description": "Comprehensive vulnerability scanner",
  "enabled": false,
  "required": false,

  "installation": {
    "check_command": "which trivy",
    "install_url": "https://aquasecurity.github.io/trivy/",
    "install_instructions": "curl -sfL ... | sh"
  },

  "scans": [
    {
      "name": "filesystem",
      "command": "trivy fs --format json /",
      "timeout_seconds": 300,
      "output_format": "json",
      "categories": ["DEPENDENCIES", "SECRETS"]
    }
  ],

  "mapping": {
    "severity": {
      "CRITICAL": "critical",
      "HIGH": "high"
    },
    "confidence": {"default": 0.9},
    "exposure": {"default": "local"}
  },

  "parser": {
    "type": "json",
    "results_path": "Results[].Vulnerabilities[]",
    "fields": {
      "id": "VulnerabilityID",
      "title": "Title",
      "severity": "Severity"
    }
  }
}
```

### Adding Your Own Plugin

1. Create `config/plugins/your-scanner.json`
2. Define the command to run
3. Map output fields to confrisk findings
4. Enable it!

Example for **semgrep**:

```json
{
  "name": "semgrep",
  "enabled": true,
  "scans": [{
    "name": "code_scan",
    "command": "semgrep --config=auto --json .",
    "output_format": "json",
    "categories": ["SECRETS", "COMPLIANCE"]
  }],
  "parser": {
    "type": "json",
    "results_path": "results[]",
    "fields": {
      "id": "check_id",
      "title": "extra.message",
      "severity": "extra.severity"
    }
  }
}
```

## Dependency Rules

Define which packages are forbidden or require attention.

### Blocklist (Forbidden Packages)

`config/rules/dependencies.json`:

```json
{
  "blocklist": {
    "packages": [
      {
        "name": "event-stream",
        "ecosystem": "npm",
        "reason": "Supply chain attack history (2018)",
        "severity": "critical",
        "alternative": "through2"
      },
      {
        "name": "log4j",
        "ecosystem": "maven",
        "version_pattern": "^2\\.(0|1[0-4])\\..*",
        "reason": "Log4Shell vulnerability",
        "severity": "critical",
        "alternative": "log4j >= 2.17.1"
      }
    ]
  }
}
```

**Add your own forbidden packages** here!

### Watchlist

Packages requiring extra scrutiny:

```json
{
  "watchlist": {
    "packages": [
      {
        "name": "axios",
        "ecosystem": "npm",
        "reason": "Popular supply chain target",
        "action": "verify_checksum"
      }
    ]
  }
}
```

## Port Rules

Define which ports are dangerous or unexpected.

`config/rules/ports.json`:

```json
{
  "dangerous_ports": [
    {
      "port": 23,
      "protocol": "tcp",
      "name": "telnet",
      "severity": "critical",
      "reason": "Plaintext credentials",
      "remediation": "Use SSH instead",
      "exposure": "internet_facing"
    },
    {
      "port": 27017,
      "protocol": "tcp",
      "name": "mongodb",
      "severity": "critical",
      "reason": "Often no authentication",
      "remediation": "Enable auth, bind to localhost"
    }
  ]
}
```

**Add your organization's port policies** here!

## Usage

### Basic Scan with Configs

```bash
# Load config from default location (./config)
confrisk --asset production --out report.html

# Use custom config directory
confrisk --config /etc/confrisk/config --asset crown-jewel --out scan.html
```

### Enable Plugins

```bash
# Run with Trivy integration
confrisk --plugins trivy --asset production

# Run with multiple plugins
confrisk --plugins trivy,lynis,gitleaks --asset crown-jewel
```

### Custom Scoring Weights

Edit `config/scoring.json` to adjust:

- Severity weights (how much each level counts)
- Asset criticality multipliers (dev vs production)
- Exposure multipliers (local vs internet-facing)
- Risk band thresholds

Then run normally — your custom weights will be applied!

## Config Validation

Validate your config files before running:

```bash
confrisk --validate-config
```

Checks:
- JSON syntax
- Required fields present
- Valid severity/exposure values
- Check IDs are unique
- Plugin commands exist

## Examples

### Example 1: Add Custom Check for Kubernetes

`config/checks/k8s-privileged-pods.json`:

```json
{
  "id": "K8S-001",
  "name": "Privileged pods detected",
  "category": "CONTAINERS",
  "description": "Kubernetes pods running in privileged mode can escape to the host",
  "severity": "high",
  "exposure": "adjacent_network",
  "confidence": 0.95,
  "effort": "moderate",

  "detection": {
    "type": "command_output",
    "command": "kubectl get pods --all-namespaces -o json | jq '.items[].spec.containers[].securityContext.privileged' | grep true",
    "pattern": "true",
    "expected": "absent"
  },

  "remediation": {
    "summary": "Remove privileged: true from pod specs",
    "steps": [
      "Audit pod security contexts",
      "Remove securityContext.privileged: true",
      "Use specific capabilities instead",
      "Consider Pod Security Standards"
    ]
  }
}
```

### Example 2: Block Company-Specific Dependency

`config/rules/dependencies.json` (add to blocklist):

```json
{
  "name": "internal-legacy-auth",
  "ecosystem": "npm",
  "reason": "Deprecated internal auth library, use SSO instead",
  "severity": "high",
  "alternative": "company-sso-client >= 2.0"
}
```

### Example 3: Custom Port Rule for Internal Service

`config/rules/ports.json` (add to dangerous_ports):

```json
{
  "port": 9999,
  "protocol": "tcp",
  "name": "legacy-admin-panel",
  "severity": "critical",
  "reason": "Old admin panel with known vulnerabilities",
  "remediation": "Migrate to new admin portal on port 8443",
  "exposure": "any"
}
```

## Config Schema Reference

See `src/config.rs` for complete Rust type definitions.

## Migration from Hardcoded Checks

Old (v0.1):
- Checks hardcoded in `checks.rs`
- No plugin support
- Fixed risk weights

New (v0.2+):
- Checks in JSON files
- Plugin system for external scanners
- Configurable risk model
- User-defined rules

To migrate:
1. Existing checks still work (backward compatible)
2. New checks can be added via JSON
3. Gradually move hardcoded checks to JSON config

## Roadmap

- [ ] Web UI for config management
- [ ] Config signing/verification
- [ ] Remote config loading from server
- [ ] Config versioning and rollback
- [ ] Automated config generation from compliance frameworks
- [ ] Plugin marketplace

## Benefits

✅ **Generic** — Works for any security domain
✅ **Extensible** — Add checks without code changes
✅ **Plug-and-Play** — Integrate external scanners easily
✅ **Customizable** — Adjust risk model to your needs
✅ **Maintainable** — Non-developers can add rules
✅ **Auditable** — Config is version-controlled
✅ **Future-Proof** — Config can be served from remote server

---

**Config System Version:** 1.0
**Last Updated:** May 23, 2026
