// ABOUTME: Build script for recorder_cli to set proper rpath for dylib loading
// ABOUTME: Ensures the binary can find libAppleCapture.dylib both in bundle and development

#[cfg(target_os = "macos")]
fn main() {
    // Add rpaths that work both inside app bundles and in development
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../Frameworks");
}

#[cfg(not(target_os = "macos"))]
fn main() {}