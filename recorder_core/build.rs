// ABOUTME: Build script for linking Swift capture library with Rust
// ABOUTME: Handles platform-specific compilation and linking requirements

#[cfg(target_os = "macos")]
fn main() {
    use std::{path::PathBuf, process::Command};

    // 1. Build Swift package in release mode
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project_root = manifest_dir
        .parent()
        .expect("Failed to get project root");
    let swift_pkg = project_root.join("apple_capture");
    
    println!("cargo:warning=Building Swift package at {:?}", swift_pkg);
    
    let output = Command::new("swift")
        .args(["build", "-c", "release", "--package-path"])
        .arg(&swift_pkg)
        .output()
        .expect("Failed to run swift build");
        
    if !output.status.success() {
        eprintln!("Swift build failed!");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Swift build failed");
    }

    // 2. Tell Cargo where to find the compiled .dylib
    let lib_dir = swift_pkg.join(".build/arm64-apple-macosx/release");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=AppleCapture");
    
    // Add rpath so the binary can find the library at runtime
    // Use @executable_path/../Frameworks for app bundles
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
    // Also add the development path for non-bundled usage
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());

    // 3. Link system frameworks
    for fw in ["AVFoundation", "CoreMedia", "CoreVideo", "VideoToolbox", "CoreGraphics"] {
        println!("cargo:rustc-link-lib=framework={}", fw);
    }

    // 4. Rebuild if Swift code changed
    println!("cargo:rerun-if-changed=../apple_capture/Sources");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    // No-op on other platforms
}