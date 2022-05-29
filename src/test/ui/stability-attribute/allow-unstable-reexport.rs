// Allow an unstable re-export without requiring a feature gate. https://github.com/rust-lang/rust/issues/94972

// aux-build:lint-stability.rs
// aux-build:lint-stability-reexport.rs
#![feature(staged_api)]
#![stable(feature = "lint_stability", since = "1.0.0")]

extern crate lint_stability;
extern crate lint_stability_reexport;

use lint_stability::unstable;
// We want to confirm that using a re-export through another crate behaves the same way as using an item directly
use lint_stability_reexport::unstable_text;

fn main() {
    // Since we didn't enable the feature in this crate, we still can't use these items, even though they're in scope from the `use`s which are now allowed.
    unstable(); //~ ERROR use of unstable library feature 'unstable_test_feature'
    unstable_text(); //~ ERROR use of unstable library feature 'unstable_test_feature'
}