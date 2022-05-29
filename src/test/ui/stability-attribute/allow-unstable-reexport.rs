// Demonstrating that intermediary unstable attributes on re-exports appear to be ignored. This is a modified version of a test from another PR

// aux-build:lint-stability.rs
// aux-build:lint-stability-reexport.rs
#![feature(staged_api)]

// Enable the feature the root item requires.
#![feature(unstable_test_feature)]
#![stable(feature = "lint_stability", since = "1.0.0")]

extern crate lint_stability;
extern crate lint_stability_reexport;

// the `#[unstable(feature = "unstable_reexport")` attribute placed on this re-export hasn't been enabled here, but it seems to work anyway.
use lint_stability_reexport::unstable_text;

fn main() {
    unstable_text(); //~ ERROR use of unstable library feature 'unstable_reexport'
}
