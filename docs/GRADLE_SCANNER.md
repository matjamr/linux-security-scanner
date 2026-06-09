# confrisk-gradle — Gradle Dependency Security Scanner

A fast, config-driven security scanner for Gradle projects that detects vulnerable Java/Kotlin dependencies.

## Features

- 🔍 **Config-Driven** - All vulnerability rules loaded from `config/rules/dependencies.json`
- ⚡ **Fast** - Written in Rust, scans in milliseconds
- 📦 **Comprehensive** - 30+ critical Java library vulnerabilities detected
- 🎯 **CI/CD Ready** - Exit codes, JSON output, risk-based thresholds
- 🔧 **Zero Dependencies** - No Gradle installation required
- 📊 **Risk Scoring** - Context-aware risk assessment with asset criticality

## Quick Start

```bash
# Build the scanner
cargo build --release --bin confrisk-gradle

# Scan current Gradle project
./target/release/confrisk-gradle

# Scan specific project
./target/release/confrisk-gradle --path /path/to/gradle/project

# Fail CI/CD on high severity issues
./target/release/confrisk-gradle --fail-on high --exit-code
```

## Supported Gradle Formats

### Standard Groovy DSL (`build.gradle`)

```gradle
dependencies {
    implementation 'org.springframework:spring-core:5.3.5'
    implementation 'com.fasterxml.jackson.core:jackson-databind:2.12.0'
    compile group: 'org.apache.logging.log4j', name: 'log4j-core', version: '2.14.1'
}
```

### Kotlin DSL (`build.gradle.kts`)

```kotlin
dependencies {
    implementation("org.springframework:spring-core:5.3.5")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.12.0")
}
```

## Command-Line Options

```
USAGE:
    confrisk-gradle [OPTIONS]

OPTIONS:
    -p, --path <PATH>        Path to Gradle project (default: .)
    -a, --asset <PROFILE>    Asset criticality: dev, internal, production, crown-jewel
                             (default: production)
    -f, --format <FORMAT>    Output format: text, json (default: text)
    -c, --config <PATH>      Config directory path (default: config)
    --fail-on <LEVEL>        Fail build on: critical, high, medium, low
                             (default: high)
    --exit-code              Exit with non-zero code if vulnerabilities found
    -h, --help               Show this help message
```

## Detected Vulnerabilities

The scanner detects **30+ critical Java vulnerabilities** including:

### Critical RCE Vulnerabilities
- **Log4Shell** (CVE-2021-44228) - log4j-core < 2.17.1
- **Spring4Shell** (CVE-2022-22965) - spring-core, spring-web < 5.3.18
- **Struts RCE** - struts2-core < 2.5.26
- **XStream RCE** - xstream < 1.4.18
- **H2 RCE** (CVE-2021-42392) - h2 < 2.0.206
- **SnakeYAML RCE** (CVE-2022-1471) - snakeyaml < 1.31

### Deserialization Attacks
- **Jackson** - jackson-databind < 2.13.4
- **Commons Collections** - commons-collections < 4.0
- **Commons BeanUtils** - commons-beanutils < 1.9.4

### Template Injection
- **Velocity** - velocity < 2.3
- **FreeMarker** - freemarker < 2.3.30

### Other Critical Issues
- **Netty** - HTTP request smuggling
- **Tomcat** - Request smuggling
- **MySQL Connector** - MITM vulnerabilities
- **PostgreSQL** - SQL injection
- And many more...

## Usage Examples

### CI/CD Integration (GitHub Actions)

```yaml
name: Security Scan

on: [push, pull_request]

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install confrisk
        run: |
          cargo install --path . --bin confrisk-gradle

      - name: Scan Gradle dependencies
        run: |
          confrisk-gradle --fail-on high --exit-code
```

### JSON Output for Automation

```bash
confrisk-gradle --format json > gradle-security.json
```

Example JSON output:

```json
[
  {
    "id": "GRADLE-BLOCKED-LOG4J_CORE",
    "title": "Blocked dependency: log4j-core",
    "description": "Log4Shell and related vulnerabilities (CVE-2021-44228, CVE-2021-45046)",
    "severity": "critical",
    "risk": 10.0,
    "priority": 6.8,
    "risk_band": "critical",
    "evidence": "Found org.apache.logging.log4j:log4j-core:2.14.1 in build.gradle",
    "remediation": "Replace 'org.apache.logging.log4j:log4j-core:2.14.1' with 'log4j-core >= 2.17.1'",
    "passed": false,
    "confidence": 0.99,
    "effort": 2.0
  }
]
```

### Asset Criticality Contexts

Adjust risk scoring based on application criticality:

```bash
# Development environment (lower risk weights)
confrisk-gradle --asset dev

# Internal tools
confrisk-gradle --asset internal

# Production application (default)
confrisk-gradle --asset production

# Crown jewel systems (highest risk weights)
confrisk-gradle --asset crown-jewel
```

## Adding Custom Vulnerability Rules

Edit `config/rules/dependencies.json`:

```json
{
  "blocklist": {
    "packages": [
      {
        "name": "your-library",
        "ecosystem": "maven",
        "version_pattern": "^1\\.0\\..*",
        "reason": "Known security vulnerability (CVE-XXXX-XXXXX)",
        "severity": "critical",
        "alternative": "your-library >= 2.0.0"
      }
    ]
  }
}
```

Version patterns support regex:
- `^2\\.(0|1[0-4])\\..*` - Matches 2.0.x through 2.14.x
- `^[0-3]\\..*` - Matches versions 0.x, 1.x, 2.x, 3.x
- `^1\\.4\\.[0-9]$` - Matches exactly 1.4.0 through 1.4.9

## Exit Codes

- `0` - No vulnerabilities found or below threshold
- `1` - Vulnerabilities found above fail-on threshold

## Demo Project

See `examples/gradle-project-demo/` for a working example:

```bash
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --config config
```

## Comparison with Other Tools

| Feature | confrisk-gradle | Gradle's dependency-check | OWASP Dependency-Check |
|---------|----------------|---------------------------|------------------------|
| Speed | ⚡ Instant | Slow | Very Slow |
| Gradle Install Required | ❌ No | ✅ Yes | ✅ Yes |
| Config-Driven Rules | ✅ Yes | ❌ No | ❌ No |
| Risk Scoring | ✅ Yes | ❌ No | ⚠️ Limited |
| CI/CD Ready | ✅ Yes | ⚠️ Limited | ⚠️ Limited |
| Custom Rules | ✅ Easy JSON | ⚠️ Complex | ⚠️ Complex |

## Limitations

- Does not resolve transitive dependencies (scans build files only)
- Does not query online vulnerability databases
- Requires manual config updates for new CVEs
- Best used alongside `gradle dependencies` for full dependency tree analysis

## Related Tools

- `confrisk-npm` - NPM dependency scanner
- `confrisk` - Full Linux security assessment

## License

MIT License - See LICENSE file
