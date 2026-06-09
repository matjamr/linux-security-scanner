# confrisk Configuration System — Quick Start

## What Changed?

confrisk is now a **generic, config-driven security assessment framework**. Instead of hardcoded checks, everything is defined in JSON files.

## File Structure

```
confrisk/
├── config/
│   ├── categories.json              ← 12 issue categories
│   ├── scoring.json                 ← Configurable risk weights
│   ├── checks/
│   │   ├── ssh-root-login.json      ← Check: SSH root login
│   │   └── shadow-permissions.json  ← Check: /etc/shadow perms
│   ├── plugins/
│   │   ├── trivy.json               ← Trivy integration
│   │   ├── lynis.json               ← Lynis integration
│   │   ├── gitleaks.json            ← Secrets scanning
│   │   └── osv-scanner.json         ← Dependency vulnerabilities
│   └── rules/
│       ├── dependencies.json        ← Vulnerable packages blocklist
│       └── ports.json               ← Dangerous ports rules
├── src/
│   └── config.rs                    ← Config loader (NEW!)
└── CONFIG_SYSTEM.md                 ← Full documentation
```

## Quick Examples

### 1. Add Your Own Security Check

Create `config/checks/docker-socket.json`:

```json
{
  "id": "DOCKER-001",
  "name": "Docker socket exposed",
  "category": "CONTAINERS",
  "description": "Docker socket (/var/run/docker.sock) should not be world-accessible",
  "severity": "critical",
  "exposure": "local",
  "confidence": 0.99,
  "effort": "trivial",
  "enabled": true,

  "detection": {
    "type": "file_permission",
    "file": "/var/run/docker.sock",
    "check": "not_world_writable",
    "max_mode": "0660"
  },

  "remediation": {
    "summary": "Restrict Docker socket permissions",
    "steps": [
      "Run: chmod 660 /var/run/docker.sock",
      "Add users to docker group instead of 777 permissions"
    ],
    "references": ["https://docs.docker.com/engine/security/"]
  },

  "tags": ["docker", "privilege-escalation"]
}
```

**That's it!** confrisk will automatically load and run this check.

### 2. Block a Vulnerable Dependency

Edit `config/rules/dependencies.json`, add to `blocklist.packages`:

```json
{
  "name": "old-library",
  "ecosystem": "npm",
  "version_pattern": "^1\\..*",
  "reason": "Security vulnerability CVE-2024-XXXXX",
  "severity": "high",
  "alternative": "new-library >= 2.0"
}
```

### 3. Flag a Dangerous Port

Edit `config/rules/ports.json`, add to `dangerous_ports`:

```json
{
  "port": 8888,
  "protocol": "tcp",
  "name": "jupyter-notebook",
  "severity": "high",
  "reason": "Jupyter notebook often runs without authentication",
  "remediation": "Add authentication or bind to localhost",
  "exposure": "internet_facing"
}
```

### 4. Enable External Scanner (Trivy)

Edit `config/plugins/trivy.json`:

```json
{
  "enabled": true  ← Change from false to true
}
```

Then run:

```bash
confrisk --plugins trivy --asset production
```

### 5. Customize Risk Weights

Edit `config/scoring.json`:

```json
{
  "severity": {
    "critical": 15.0,    ← Increase critical weight
    "high": 10.0,        ← Increase high weight
    "medium": 5.5,
    "low": 2.0,
    "info": 0.5
  },
  "asset_criticality": {
    "crown_jewel": 2.0,  ← Double weight for critical assets
    "production": 1.3,
    "internal": 0.8,
    "dev": 0.3
  }
}
```

Now run a scan — your custom weights will be applied!

## Issue Categories

You can now organize checks into 12 categories:

| ID | Category | Examples |
|----|----------|----------|
| `PRIVILEGES` | Access Control | sudo, PAM, user perms |
| `DEPENDENCIES` | Software Packages | npm, pip, apt vulnerabilities |
| `OPEN_PORTS` | Network Services | Dangerous ports, exposed services |
| `LOGS` | Logging & Audit | rsyslog, auditd config |
| `SECRETS` | Credentials | API keys, passwords, certs |
| `NETWORK` | Network Config | Firewall, routing, DNS |
| `KERNEL` | Kernel Hardening | ASLR, stack canaries |
| `CONTAINERS` | Container Security | Docker, K8s misconfigs |
| `FILES` | File System | Permissions, mount options |
| `PROCESSES` | Services | systemd, cron, daemons |
| `COMPLIANCE` | Policy & Standards | CIS, NIST, PCI-DSS |
| `ENCRYPTION` | Crypto & TLS | Disk encryption, ciphers |

## Plugin System

### Available Plugins

| Plugin | Purpose | Config File |
|--------|---------|-------------|
| **Trivy** | Container & vulnerability scanner | `plugins/trivy.json` |
| **Lynis** | System security auditing | `plugins/lynis.json` |
| **Gitleaks** | Secrets in code/git | `plugins/gitleaks.json` |
| **OSV-Scanner** | Dependency vulnerabilities | `plugins/osv-scanner.json` |

### Enable a Plugin

1. Edit the plugin JSON file
2. Set `"enabled": true`
3. Run: `confrisk --plugins <name>`

### Add Your Own Plugin

Create `config/plugins/custom-scanner.json`:

```json
{
  "name": "my-scanner",
  "enabled": true,
  "scans": [{
    "name": "full_scan",
    "command": "my-scanner --json /path/to/scan",
    "output_format": "json",
    "categories": ["SECRETS", "COMPLIANCE"]
  }],
  "mapping": {
    "severity": {
      "HIGH": "high",
      "LOW": "low"
    },
    "confidence": {"default": 0.9}
  },
  "parser": {
    "type": "json",
    "results_path": "findings[]",
    "fields": {
      "id": "finding_id",
      "title": "description",
      "severity": "level"
    }
  }
}
```

## Detection Types

Choose the right detection type for your check:

### 1. Config File Directive

For `/etc/ssh/sshd_config`, `/etc/nginx/nginx.conf`, etc.:

```json
{
  "detection": {
    "type": "config_directive",
    "file": "/etc/ssh/sshd_config",
    "directive": "PermitRootLogin",
    "expected": "no"
  }
}
```

### 2. File Permissions

For checking file/directory permissions:

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

### 3. Command Output

For running commands and checking output:

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

### 4. File Exists

For checking if files/dirs exist:

```json
{
  "detection": {
    "type": "file_exists",
    "file": "/etc/kubernetes/admin.conf",
    "should_exist": false
  }
}
```

### 5. Custom Script

For complex logic:

```json
{
  "detection": {
    "type": "custom",
    "script": "/usr/local/bin/my-check.sh"
  }
}
```

## Common Use Cases

### Block EOL Software

`config/rules/dependencies.json`:

```json
{
  "name": "python2",
  "ecosystem": "apt",
  "reason": "Python 2 EOL since 2020",
  "severity": "high",
  "alternative": "python3"
}
```

### Prevent Specific Packages

```json
{
  "name": "internal-old-sdk",
  "ecosystem": "npm",
  "version_pattern": "^1\\..*",
  "reason": "Company policy: use v2+ only",
  "severity": "medium",
  "alternative": "internal-sdk >= 2.0"
}
```

### Flag Development Ports in Production

`config/rules/ports.json`:

```json
{
  "port": 8080,
  "name": "http-alt",
  "severity": "medium",
  "reason": "Dev server port should not be in production",
  "exposure": "internet_facing"
}
```

### Check Kubernetes Security

`config/checks/k8s-rbac.json`:

```json
{
  "id": "K8S-002",
  "name": "Overly permissive RBAC",
  "category": "CONTAINERS",
  "detection": {
    "type": "command_output",
    "command": "kubectl get clusterrolebindings -o json | jq '.items[] | select(.subjects[].name==\"system:anonymous\")'",
    "pattern": "system:anonymous",
    "expected": "absent"
  }
}
```

## Migration from v0.1

The old hardcoded checks still work! You can:

1. **Keep using hardcoded checks** — they're still there
2. **Add new checks via JSON** — no code changes needed
3. **Gradually migrate** — move hardcoded checks to JSON over time

Both systems work side-by-side.

## What's Next?

1. **Add your company's security policies** to `config/rules/`
2. **Define custom checks** for your infrastructure
3. **Enable plugins** for your toolchain (Trivy, Lynis, etc.)
4. **Customize risk weights** to match your risk appetite
5. **Version control your configs** — treat them as code!

## Help

- Full documentation: `CONFIG_SYSTEM.md`
- Example configs: `config/` directory
- Schema reference: `src/config.rs`
- Original docs: `README.md`

## Summary

**Before (v0.1):**
```rust
// Add check → edit checks.rs → recompile → redeploy
```

**Now (v0.2+):**
```bash
# Add check → create JSON file → done!
vim config/checks/my-check.json
confrisk --asset production
```

**Config-driven = flexible, maintainable, and future-proof!**

---

**Quick Start Version:** 1.0
**Last Updated:** May 23, 2026
