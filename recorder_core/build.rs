// ABOUTME: Build script for linking Swift capture library with Rust
// ABOUTME: Handles platform-specific compilation and linking requirements

fn main() {
    #[cfg(target_os = "macos")]
    {
        // Link with the Swift capture library
        println!("cargo:rustc-link-lib=dylib=AppleCapture");
        println!("cargo:rustc-link-search=native=../apple_capture/.build/release");
        
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