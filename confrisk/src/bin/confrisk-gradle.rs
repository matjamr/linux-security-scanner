/// confrisk-gradle — Gradle dependency security scanner
///
/// Scans Gradle projects for vulnerable dependencies and security issues.
/// Can be integrated into CI/CD and build processes.
///
/// All argument parsing, config resolution, rendering and exit-code logic is
/// shared via `confrisk::cli`; this binary only wires the Gradle scanner in.
use confrisk::cli::{run, ToolInfo};
use confrisk::gradle::GradleScanner;

fn main() {
    run(ToolInfo::GRADLE, GradleScanner::new);
}
