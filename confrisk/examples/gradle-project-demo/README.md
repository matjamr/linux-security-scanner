# Gradle Project Security Demo

Demo Gradle project with intentionally vulnerable dependencies for testing `confrisk-gradle`.

## Vulnerable Dependencies

This project includes several known vulnerable libraries:

- **log4j-core 2.14.1** - Log4Shell vulnerability (CVE-2021-44228)
- **spring-core 5.3.5** - Spring4Shell vulnerability (CVE-2022-22965)
- **jackson-databind 2.12.0** - Deserialization vulnerabilities
- **commons-collections 3.2.1** - Remote code execution via deserialization

## Running Security Scan

```bash
# From confrisk root directory
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --config config

# With fail-on-high for CI/CD
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --fail-on high --exit-code

# JSON output
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --format json
```

## Expected Results

The scanner should detect:
- 4+ CRITICAL vulnerabilities
- Multiple security issues with detailed remediation

## Fixing Vulnerabilities

Update `build.gradle` with safe versions:

```gradle
dependencies {
    implementation 'org.apache.logging.log4j:log4j-core:2.17.1'
    implementation 'org.springframework:spring-core:5.3.18'
    implementation 'com.fasterxml.jackson.core:jackson-databind:2.13.4'
    implementation 'org.apache.commons:commons-collections4:4.4'
}
```
