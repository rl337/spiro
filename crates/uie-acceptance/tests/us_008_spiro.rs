//! US-008
#[test]
fn us_008_spiro_crate_has_main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/uie-spiro/src/main.rs");
    assert!(root.is_file());
}