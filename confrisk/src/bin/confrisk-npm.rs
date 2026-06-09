/// confrisk-npm — NPM dependency security scanner
///
/// Scans npm projects for vulnerable dependencies, blocked packages,
/// and security issues. Can be integrated into CI/CD and git hooks.
///
/// All argument parsing, config resolution, rendering and exit-code logic is
/// shared via `confrisk::cli`; this binary only wires the npm scanner in.
use confrisk::cli::{run, ToolInfo};
use confrisk::npm::NpmScanner;

fn main() {
    run(ToolInfo::NPM, NpmScanner::new);
}
