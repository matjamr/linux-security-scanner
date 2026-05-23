/// Linux configuration security checks

use crate::model::{Exposure, Finding, Severity};
use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Helper: read file content, return None if not accessible
fn read(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

/// Helper: extract active (non-commented) directive value from config file
fn directive_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 && parts[0].eq_ignore_ascii_case(key) {
            return Some(parts[1].to_string());
        }
    }
    None
}

/// Helper: check file permissions
#[cfg(unix)]
fn check_permissions(path: &str, max_mode: u32) -> (bool, u32, String) {
    match fs::metadata(path) {
        Ok(meta) => {
            let mode = meta.permissions().mode() & 0o777;
            let passed = mode <= max_mode;
            let evidence = format!("File {} has mode {:o}", path, mode);
            (passed, mode, evidence)
        }
        Err(e) => {
            let evidence = format!("Cannot read {}: {}", path, e);
            (true, 0, evidence) // Low confidence, treat as passed
        }
    }
}

#[cfg(not(unix))]
fn check_permissions(_path: &str, _max_mode: u32) -> (bool, u32, String) {
    (true, 0, "Not on Unix system".to_string())
}

/// SSH-001: PermitRootLogin should be 'no'
pub fn check_ssh_root_login() -> Finding {
    let path = "/etc/ssh/sshd_config";

    match read(path) {
        Some(content) => {
            let value = directive_value(&content, "PermitRootLogin");
            let passed = value.as_deref() == Some("no");
            let has_value = value.is_some();

            let evidence = if let Some(ref v) = value {
                format!("PermitRootLogin is set to: {}", v)
            } else {
                "PermitRootLogin directive not found (defaults to 'yes' on many systems)".to_string()
            };

            Finding {
                id: "SSH-001".to_string(),
                title: "SSH root login permitted".to_string(),
                description: "SSH allows direct root login, increasing attack surface. Root access should require initial login as unprivileged user followed by privilege escalation (sudo).".to_string(),
                severity: Severity::High,
                exposure: Exposure::InternetFacing,
                confidence: if has_value { 0.95 } else { 0.7 },
                effort: 1.0,
                remediation: "Set 'PermitRootLogin no' in /etc/ssh/sshd_config and restart sshd".to_string(),
                evidence,
                passed,
            }
        }
        None => Finding {
            id: "SSH-001".to_string(),
            title: "SSH root login permitted".to_string(),
            description: "Cannot read SSH configuration".to_string(),
            severity: Severity::High,
            exposure: Exposure::InternetFacing,
            confidence: 0.3,
            effort: 1.0,
            remediation: "Ensure SSH is configured and /etc/ssh/sshd_config is readable".to_string(),
            evidence: format!("Cannot read {}", path),
            passed: true, // Low confidence
        },
    }
}

/// SSH-002: PasswordAuthentication should be 'no'
pub fn check_ssh_password_auth() -> Finding {
    let path = "/etc/ssh/sshd_config";

    match read(path) {
        Some(content) => {
            let value = directive_value(&content, "PasswordAuthentication");
            let passed = value.as_deref() == Some("no");
            let has_value = value.is_some();

            let evidence = if let Some(ref v) = value {
                format!("PasswordAuthentication is set to: {}", v)
            } else {
                "PasswordAuthentication directive not found (defaults vary)".to_string()
            };

            Finding {
                id: "SSH-002".to_string(),
                title: "SSH password authentication enabled".to_string(),
                description: "SSH accepts password authentication, vulnerable to brute-force attacks. Key-based authentication is more secure.".to_string(),
                severity: Severity::Medium,
                exposure: Exposure::InternetFacing,
                confidence: if has_value { 0.95 } else { 0.6 },
                effort: 2.0,
                remediation: "Set 'PasswordAuthentication no' in /etc/ssh/sshd_config, ensure key-based auth is configured, restart sshd".to_string(),
                evidence,
                passed,
            }
        }
        None => Finding {
            id: "SSH-002".to_string(),
            title: "SSH password authentication enabled".to_string(),
            description: "Cannot read SSH configuration".to_string(),
            severity: Severity::Medium,
            exposure: Exposure::InternetFacing,
            confidence: 0.3,
            effort: 2.0,
            remediation: "Ensure SSH is configured and /etc/ssh/sshd_config is readable".to_string(),
            evidence: format!("Cannot read {}", path),
            passed: true,
        },
    }
}

/// FILE-001: /etc/passwd permissions should be 644 or less
pub fn check_passwd_permissions() -> Finding {
    let path = "/etc/passwd";
    let (passed, _mode, evidence) = check_permissions(path, 0o644);

    Finding {
        id: "FILE-001".to_string(),
        title: "/etc/passwd permissions too permissive".to_string(),
        description: "The /etc/passwd file should be readable by all but writable only by root (644 or less).".to_string(),
        severity: Severity::Medium,
        exposure: Exposure::Local,
        confidence: if passed { 1.0 } else { 0.95 },
        effort: 1.0,
        remediation: format!("Run: chmod 644 {}", path),
        evidence,
        passed,
    }
}

/// FILE-002: /etc/shadow should not be readable by 'other'
pub fn check_shadow_permissions() -> Finding {
    let path = "/etc/shadow";

    #[cfg(unix)]
    {
        match fs::metadata(path) {
            Ok(meta) => {
                let mode = meta.permissions().mode() & 0o777;
                let other_readable = (mode & 0o004) != 0;
                let passed = !other_readable;

                let evidence = format!(
                    "File {} has mode {:o}{}",
                    path,
                    mode,
                    if other_readable { " — readable by 'other'" } else { "" }
                );

                Finding {
                    id: "FILE-002".to_string(),
                    title: "/etc/shadow readable by unprivileged users".to_string(),
                    description: "The shadow file contains password hashes and must not be world-readable (CRITICAL security issue).".to_string(),
                    severity: Severity::Critical,
                    exposure: Exposure::Local,
                    confidence: 0.99,
                    effort: 1.0,
                    remediation: format!("Run: chmod 640 {} or chmod 600 {}", path, path),
                    evidence,
                    passed,
                }
            }
            Err(e) => Finding {
                id: "FILE-002".to_string(),
                title: "/etc/shadow readable by unprivileged users".to_string(),
                description: "Cannot check shadow file permissions".to_string(),
                severity: Severity::Critical,
                exposure: Exposure::Local,
                confidence: 0.3,
                effort: 1.0,
                remediation: "Run as root to check shadow permissions".to_string(),
                evidence: format!("Cannot read metadata for {}: {}", path, e),
                passed: true, // Low confidence
            },
        }
    }

    #[cfg(not(unix))]
    {
        Finding {
            id: "FILE-002".to_string(),
            title: "/etc/shadow readable by unprivileged users".to_string(),
            description: "Not on Unix system".to_string(),
            severity: Severity::Critical,
            exposure: Exposure::Local,
            confidence: 0.0,
            effort: 1.0,
            remediation: "N/A".to_string(),
            evidence: "Not running on Unix".to_string(),
            passed: true,
        }
    }
}

/// FILE-003: No world-writable files in /etc
pub fn check_world_writable_etc() -> Finding {
    let etc_path = "/etc";

    #[cfg(unix)]
    {
        let mut world_writable = Vec::new();

        if let Ok(entries) = fs::read_dir(etc_path) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() {
                        let mode = meta.permissions().mode() & 0o777;
                        if (mode & 0o002) != 0 {
                            if let Some(name) = entry.file_name().to_str() {
                                world_writable.push(format!("{} ({:o})", name, mode));
                            }
                        }
                    }
                }
            }
        }

        let passed = world_writable.is_empty();
        let evidence = if passed {
            format!("No world-writable files found in {}", etc_path)
        } else {
            format!("Found {} world-writable file(s) in {}: {}",
                world_writable.len(), etc_path, world_writable.join(", "))
        };

        Finding {
            id: "FILE-003".to_string(),
            title: "World-writable files in /etc".to_string(),
            description: "Configuration files in /etc should not be writable by all users.".to_string(),
            severity: Severity::High,
            exposure: Exposure::Local,
            confidence: 0.95,
            effort: 1.0,
            remediation: "Remove write permission for 'other': chmod o-w <file>".to_string(),
            evidence,
            passed,
        }
    }

    #[cfg(not(unix))]
    {
        Finding {
            id: "FILE-003".to_string(),
            title: "World-writable files in /etc".to_string(),
            description: "Not on Unix system".to_string(),
            severity: Severity::High,
            exposure: Exposure::Local,
            confidence: 0.0,
            effort: 1.0,
            remediation: "N/A".to_string(),
            evidence: "Not running on Unix".to_string(),
            passed: true,
        }
    }
}

/// KRNL-001: ASLR should be fully enabled (value 2)
pub fn check_aslr() -> Finding {
    let path = "/proc/sys/kernel/randomize_va_space";

    match read(path) {
        Some(content) => {
            let value = content.trim();
            let passed = value == "2";

            let evidence = format!("ASLR setting: {} (expected: 2)", value);

            Finding {
                id: "KRNL-001".to_string(),
                title: "ASLR not fully enabled".to_string(),
                description: "Address Space Layout Randomization (ASLR) provides memory protection against exploitation. Value should be 2 for full randomization.".to_string(),
                severity: Severity::Medium,
                exposure: Exposure::Local,
                confidence: 0.95,
                effort: 1.0,
                remediation: "Run: echo 2 | sudo tee /proc/sys/kernel/randomize_va_space; add 'kernel.randomize_va_space=2' to /etc/sysctl.conf".to_string(),
                evidence,
                passed,
            }
        }
        None => Finding {
            id: "KRNL-001".to_string(),
            title: "ASLR not fully enabled".to_string(),
            description: "Cannot read ASLR setting".to_string(),
            severity: Severity::Medium,
            exposure: Exposure::Local,
            confidence: 0.3,
            effort: 1.0,
            remediation: format!("Ensure {} is readable", path),
            evidence: format!("Cannot read {}", path),
            passed: true,
        },
    }
}

/// NET-001: IP forwarding should be disabled on non-routers (value 0)
pub fn check_ip_forwarding() -> Finding {
    let path = "/proc/sys/net/ipv4/ip_forward";

    match read(path) {
        Some(content) => {
            let value = content.trim();
            let passed = value == "0";

            let evidence = format!("IP forwarding: {} (expected: 0 for non-router)", value);

            Finding {
                id: "NET-001".to_string(),
                title: "IP forwarding enabled".to_string(),
                description: "IP forwarding should be disabled on systems that don't act as routers to prevent potential routing-based attacks.".to_string(),
                severity: Severity::Low,
                exposure: Exposure::AdjacentNetwork,
                confidence: 0.9,
                effort: 1.0,
                remediation: "Run: echo 0 | sudo tee /proc/sys/net/ipv4/ip_forward; add 'net.ipv4.ip_forward=0' to /etc/sysctl.conf".to_string(),
                evidence,
                passed,
            }
        }
        None => Finding {
            id: "NET-001".to_string(),
            title: "IP forwarding enabled".to_string(),
            description: "Cannot read IP forwarding setting".to_string(),
            severity: Severity::Low,
            exposure: Exposure::AdjacentNetwork,
            confidence: 0.3,
            effort: 1.0,
            remediation: format!("Ensure {} is readable", path),
            evidence: format!("Cannot read {}", path),
            passed: true,
        },
    }
}

/// CRON-001: /etc/crontab should not be world-writable
pub fn check_crontab_permissions() -> Finding {
    let path = "/etc/crontab";

    #[cfg(unix)]
    {
        if !Path::new(path).exists() {
            return Finding {
                id: "CRON-001".to_string(),
                title: "Crontab file has insecure permissions".to_string(),
                description: format!("{} does not exist", path),
                severity: Severity::High,
                exposure: Exposure::Local,
                confidence: 0.5,
                effort: 1.0,
                remediation: "N/A — file does not exist".to_string(),
                evidence: format!("{} not found", path),
                passed: true,
            };
        }

        match fs::metadata(path) {
            Ok(meta) => {
                let mode = meta.permissions().mode() & 0o777;
                let writable_by_other = (mode & 0o002) != 0;
                let writable_by_group = (mode & 0o020) != 0;
                let passed = !writable_by_other && !writable_by_group;

                let issues = if writable_by_other && writable_by_group {
                    "writable by group and other"
                } else if writable_by_other {
                    "writable by other"
                } else if writable_by_group {
                    "writable by group"
                } else {
                    ""
                };

                let issue_suffix = if issues.is_empty() {
                    String::new()
                } else {
                    format!(" — {}", issues)
                };

                let evidence = format!(
                    "File {} has mode {:o}{}",
                    path,
                    mode,
                    issue_suffix
                );

                Finding {
                    id: "CRON-001".to_string(),
                    title: "Crontab file has insecure permissions".to_string(),
                    description: "The crontab file should only be writable by root to prevent privilege escalation.".to_string(),
                    severity: Severity::High,
                    exposure: Exposure::Local,
                    confidence: 0.95,
                    effort: 1.0,
                    remediation: format!("Run: chmod 644 {}", path),
                    evidence,
                    passed,
                }
            }
            Err(e) => Finding {
                id: "CRON-001".to_string(),
                title: "Crontab file has insecure permissions".to_string(),
                description: "Cannot check crontab permissions".to_string(),
                severity: Severity::High,
                exposure: Exposure::Local,
                confidence: 0.3,
                effort: 1.0,
                remediation: "Check file permissions manually".to_string(),
                evidence: format!("Cannot read metadata for {}: {}", path, e),
                passed: true,
            },
        }
    }

    #[cfg(not(unix))]
    {
        Finding {
            id: "CRON-001".to_string(),
            title: "Crontab file has insecure permissions".to_string(),
            description: "Not on Unix system".to_string(),
            severity: Severity::High,
            exposure: Exposure::Local,
            confidence: 0.0,
            effort: 1.0,
            remediation: "N/A".to_string(),
            evidence: "Not running on Unix".to_string(),
            passed: true,
        }
    }
}

/// Run all checks and return findings
pub fn run_all() -> Vec<Finding> {
    vec![
        check_ssh_root_login(),
        check_ssh_password_auth(),
        check_passwd_permissions(),
        check_shadow_permissions(),
        check_world_writable_etc(),
        check_aslr(),
        check_ip_forwarding(),
        check_crontab_permissions(),
    ]
}
