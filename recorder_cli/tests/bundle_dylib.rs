// ABOUTME: End-to-end test to verify Swift dylib is properly included in app bundle
// ABOUTME: Catches dyld issues that would cause silent app exits

#![cfg(target_os = "macos")]

#[test]
#[ignore] // Run with: cargo test -- --ignored
fn dylib_in_bundle() {
    use std::process::Command;
    use std::path::Path;

    // Build via the helper script
    // Find the workspace root by looking for Cargo.toml
    let workspace_root = std::env::current_dir()
        .expect("Failed to get current dir")
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists() && p.join("scripts/package_app.sh").exists())
        .expect("Could not find workspace root")
        .to_path_buf();
    
    let status = Command::new("bash")
        .arg("scripts/package_app.sh")
        .current_dir(&workspace_root)
        .status()
        .expect("Failed to run package_app.sh");
    
    assert!(status.success(), "package_app.sh failed");

    let app = workspace_root.join("target/release/bundle/osx/TFT Recorder.app");
    assert!(app.exists(), "App bundle not found at {:?}", app);
    
    let dylib = app.join("Contents/Frameworks/libAppleCapture.dylib");
    assert!(dylib.exists(), "Swift dylib missing in bundle at {:?}", dylib);
    
    // Also verify the icon is present
    let icon = app.join("Contents/Resources/tft.icns");
    assert!(icon.exists(), "Icon missing in bundle at {:?}", icon);
    
    // Verify the executable exists
    let exe = app.join("Contents/MacOS/recorder");
    assert!(exe.exists(), "Executable missing in bundle at {:?}", exe);
}