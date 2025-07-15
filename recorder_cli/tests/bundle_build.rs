// ABOUTME: Integration test to verify macOS app bundle can be generated
// ABOUTME: Ensures CI will catch any bundle configuration issues

#![cfg(target_os = "macos")]

#[test]
#[ignore] // Run with: cargo test -- --ignored
fn tft_recorder_app_builds() {
    use std::process::Command;
    
    // Check if cargo-bundle is installed
    let check = Command::new("cargo")
        .args(&["bundle", "--version"])
        .output()
        .expect("Failed to check cargo-bundle");
        
    if !check.status.success() {
        eprintln!("cargo-bundle not installed. Install with: cargo install cargo-bundle");
        panic!("cargo-bundle is required for this test");
    }
    
    // Try to build the bundle
    let status = Command::new("cargo")
        .args(&["bundle", "--bin", "recorder", "--release"])
        .status()
        .expect("Failed to spawn cargo-bundle");
        
    assert!(
        status.success(),
        "cargo bundle failed; see stdout/stderr for details"
    );

    // Verify the app bundle was created
    let app = std::path::Path::new("target/release/bundle/osx/TFT Recorder.app");
    assert!(app.exists(), "Bundle path missing: {:?}", app);
    
    // Check that the main executable exists
    let exe = app.join("Contents/MacOS/recorder");
    assert!(exe.exists(), "Bundle executable missing: {:?}", exe);
    
    // Check that Info.plist was created
    let info_plist = app.join("Contents/Info.plist");
    assert!(info_plist.exists(), "Info.plist missing: {:?}", info_plist);
}