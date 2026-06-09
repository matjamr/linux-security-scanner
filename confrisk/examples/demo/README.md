# confrisk Demo — Vulnerable Container

This demo showcases **confrisk** scanning a deliberately misconfigured Ubuntu container with multiple security vulnerabilities.

## Prerequisites

- Docker and Docker Compose installed and running
- Rust toolchain (for building the binary)

## Quick Start

### Option 1: Using docker-compose (Recommended)

1. **Start Docker Desktop** (if not already running)

2. **Build the static Linux binary:**

   ```bash
   cd ..  # Go back to confrisk root directory

   # Build using Docker (no need for local Rust toolchain)
   docker run --rm \
     -v "$(pwd)":/workspace \
     -w /workspace \
     rust:1.75-alpine \
     sh -c "apk add --no-cache musl-dev && cargo build --release --target x86_64-unknown-linux-musl"

   # Copy binary to demo directory
   cp target/x86_64-unknown-linux-musl/release/confrisk demo/confrisk

   cd demo
   ```

   Or if you have Rust installed locally with musl target:

   ```bash
   cd ..
   rustup target add x86_64-unknown-linux-musl
   cargo build --release --target x86_64-unknown-linux-musl
   cp target/x86_64-unknown-linux-musl/release/confrisk demo/confrisk
   cd demo
   ```

3. **Run the scan** with different asset profiles:

   ```bash
   # Scan as crown-jewel asset (highest criticality)
   docker compose run --rm confrisk-demo

   # Scan as dev asset (lowest criticality)
   docker compose run --rm confrisk-demo-dev

   # Scan as production asset (medium criticality)
   docker compose run --rm confrisk-demo-production
   ```

4. **View the reports:**

   ```bash
   # Open in browser (macOS)
   open out/report.html
   open out/dev.html
   open out/production.html

   # Or on Linux
   xdg-open out/report.html
   ```

### Option 2: Manual Docker Build

```bash
# Build the image
docker build -f Dockerfile.vulnerable -t confrisk-demo .

# Run with different profiles
docker run --rm \
  --sysctl net.ipv4.ip_forward=1 \
  -v "$(pwd)/out:/out" \
  confrisk-demo --asset crown-jewel --out /out/report.html

docker run --rm \
  --sysctl net.ipv4.ip_forward=1 \
  -v "$(pwd)/out:/out" \
  confrisk-demo --asset dev --out /out/dev.html
```

## What Gets Scanned

The vulnerable container has the following intentional misconfigurations:

| Finding ID | Vulnerability | Severity |
|------------|---------------|----------|
| SSH-001 | Root login via SSH enabled | High |
| SSH-002 | Password authentication enabled | Medium |
| FILE-001 | /etc/passwd permissions too permissive | Medium |
| FILE-002 | /etc/shadow readable by all (CRITICAL!) | Critical |
| FILE-003 | World-writable files in /etc | High |
| KRNL-001 | ASLR not fully enabled | Medium |
| NET-001 | IP forwarding enabled | Low |
| CRON-001 | /etc/crontab writable by non-root | High |

## Comparing Asset Profiles

The key demonstration is how the **same vulnerabilities** get **different priority scores** based on the asset criticality:

For example, **SSH-001** (root login enabled):
- On `dev` profile (×0.5): **lower priority**
- On `production` profile (×1.1): **medium priority**
- On `crown-jewel` profile (×1.3): **HIGH priority**

This shows that confrisk performs **contextual risk assessment**, not just vulnerability listing.

## Understanding the Reports

Each HTML report includes:

1. **Posture Banner** — Overall security status and cumulative risk
2. **Statistics** — Count of findings by severity
3. **Risk Model** — Explanation of how risk and priority are calculated
4. **Findings List** — Sorted by priority (highest first)

Each finding shows:
- **Evidence** — What was actually detected on the system
- **Score Breakdown** — How the risk score was calculated (explainability)
- **Remediation Effort** — Estimated effort to fix (1.0-5.0)
- **Recommendation** — How to fix the issue

## Files

- `Dockerfile.vulnerable` — The intentionally vulnerable container
- `docker-compose.yml` — Configuration for different scan profiles
- `build.sh` — Helper script to build the static binary
- `out/` — Output directory for HTML reports

## Cleanup

```bash
# Remove the demo image
docker rmi confrisk-demo

# Remove generated reports
rm -rf out/*.html
```
