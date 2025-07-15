// ABOUTME: Build script for linking Swift capture library with Rust
// ABOUTME: Handles platform-specific compilation and linking requirements

fn main() {
    #[cfg(target_os = "macos")]
    {
        use std::path::PathBuf;
        
        // Get the absolute path to the Swift library
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let swift_lib_path = PathBuf::from(manifest_dir)
            .parent()
            .unwrap()
            .join("apple_capture")
            .join(".build")
            .join("arm64-apple-macosx")
            .join("release");
        
        // For now, always use the C stub until we implement proper Swift-C bridging
        let bridge_file = PathBuf::from(manifest_dir)
            .parent()
            .unwrap()
            .join("apple_capture")
            .join("Sources")
            .join("AppleCaptureC")
            .join("bridge.c");
            
        cc::Build::new()
            .file(bridge_file)
            .compile("applecapture_stub");
            
        // Also link the Swift library if it exists (for future use)
        if swift_lib_path.exists() {
            println!("cargo:rustc-link-search=native={}", swift_lib_path.display());
        }
        
        // Link with system frameworks
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=CoreMedia");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=VideoToolbox");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        
        // Rerun if Swift package changes
        println!("cargo:rerun-if-changed=../apple_capture/Sources");
    }
}