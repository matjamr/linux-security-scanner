/// confrisk-gradle — skaner zaleznosci Gradle
use confrisk::cli::{run, ToolInfo};
use confrisk::gradle::GradleScanner;

fn main() {
    run(ToolInfo::GRADLE, GradleScanner::new);
}
