/// The shared abstraction implemented by every ecosystem scanner.
///
/// `confrisk-npm`, `confrisk-gradle`, and any future generic scanner all produce
/// the same [`Finding`] values from a [`Config`]. Implementing this trait is the
/// single contract a new ecosystem needs to satisfy in order to plug into the
/// shared CLI driver in [`crate::cli`].
///
/// ```ignore
/// use confrisk::scanner::Scanner;
///
/// struct CargoScanner { /* ... */ }
/// impl Scanner for CargoScanner {
///     fn scan(&self) -> Vec<confrisk::model::Finding> { /* ... */ vec![] }
/// }
/// ```
use crate::model::Finding;

/// A security scanner for one ecosystem (npm, gradle, ...).
pub trait Scanner {
    /// Run every check this scanner knows about and return the raw findings.
    ///
    /// Scoring/prioritisation is applied later by the CLI driver, so
    /// implementations only need to describe *what* they found, not rank it.
    fn scan(&self) -> Vec<Finding>;
}
