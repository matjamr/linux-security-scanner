/// confrisk-npm — skaner zaleznosci npm
use confrisk::cli::{run, ToolInfo};
use confrisk::npm::NpmScanner;

fn main() {
    run(ToolInfo::NPM, NpmScanner::new);
}
