//! Basic integration tests for template-rs

use template_rs::{miette, Result};

#[test]
fn test_crate_compiles() {
    // Basic smoke test to ensure the crate compiles and can be imported
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_miette_integration() -> Result<()> {
    // Test that miette error handling works
    let _error = miette!("Test error");
    Ok(())
}
