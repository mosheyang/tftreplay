// ABOUTME: Test to verify the binary has correct rpath settings for dylib loading
// ABOUTME: Prevents shipping binaries that can't find their dynamic libraries

#[test]
#[cfg(target_os = "macos")]
fn binary_has_frameworks_rpath() {
    use std::process::Command;
    
    // First, ensure the binary exists
    let binary_path = "target/release/recorder";
    if !std::path::Path::new(binary_path).exists() {
        eprintln!("Binary not found at {}, building it first...", binary_path);
        
        // Build the release binary
        let build_status = Command::new("cargo")
            .args(&["build", "--bin", "recorder", "--release"])
            .status()
            .expect("Failed to run cargo build");
            
        assert!(build_status.success(), "Failed to build release binary");
    }
    
    // Run otool to check for rpath entries
    let output = Command::new("otool")
        .args(&["-l", binary_path])
        .output()
        .expect("Failed to run otool");
    
    let text = String::from_utf8_lossy(&output.stdout);
    
    // Check for the required rpath
    assert!(
        text.contains("@executable_path/../Frameworks"),
        "Binary missing @executable_path/../Frameworks rpath. Output:\n{}",
        text
    );
}

#[test]
#[cfg(target_os = "macos")]
#[ignore] // Run with: cargo test -- --ignored
fn bundle_binary_has_correct_dylib_references() {
    use std::process::Command;
    use std::path::Path;
    
    // Run packaging script first
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
    
    let bundle_binary = workspace_root.join("target/release/bundle/osx/TFT Recorder.app/Contents/MacOS/recorder");
    assert!(bundle_binary.exists(), "Bundle binary not found at {:?}", bundle_binary);
    
    // Check that the binary references libAppleCapture.dylib via @rpath
    let output = Command::new("otool")
        .args(&["-L", bundle_binary.to_str().unwrap()])
        .output()
        .expect("Failed to run otool -L");
    
    let text = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        text.contains("@rpath/libAppleCapture.dylib"),
        "Binary not referencing libAppleCapture.dylib via @rpath. Output:\n{}",
        text
    );
}