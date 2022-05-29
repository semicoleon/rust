#![crate_type = "lib"]
#![feature(staged_api)]
#![feature(unstable_test_feature)]
#![stable(feature = "lint_stability", since = "1.0.0")]

extern crate lint_stability;

#[unstable(feature = "unstable_reexport", issue = "none")]
pub use lint_stability::unstable_text;
