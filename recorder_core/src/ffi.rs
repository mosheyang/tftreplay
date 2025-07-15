// ABOUTME: FFI bridge between Rust and Swift using manual C bindings
// ABOUTME: Provides low-level interface for cross-language communication

use std::ffi::{c_char, c_void, CString};

#[repr(transparent)]
pub struct SwiftCapture {
    ptr: *mut c_void,
}

// Ensure SwiftCapture is Send + Sync for thread safety
unsafe impl Send for SwiftCapture {}
unsafe impl Sync for SwiftCapture {}

#[cfg(target_os = "macos")]
extern "C" {
    fn swift_capture_create() -> *mut c_void;
    fn swift_capture_destroy(ptr: *mut c_void);
    fn swift_capture_start(
        ptr: *mut c_void,
        window_title: *const c_char,
        width: u32,
        height: u32,
        bitrate: u32,
        output_path: *const c_char,
    ) -> bool;
    fn swift_capture_stop(ptr: *mut c_void);
}

#[cfg(target_os = "macos")]
pub fn create_capture_session() -> SwiftCapture {
    let ptr = unsafe { swift_capture_create() };
    assert!(!ptr.is_null(), "Failed to create Swift capture session");
    SwiftCapture { ptr }
}

#[cfg(target_os = "macos")]
pub fn start_capture(
    cap: &mut SwiftCapture,
    window_title: &str,
    width: u32,
    height: u32,
    bitrate: u32,
    output_path: &str,
) -> bool {
    let c_title = CString::new(window_title).expect("Invalid window title");
    let c_path = CString::new(output_path).expect("Invalid output path");
    
    unsafe {
        swift_capture_start(
            cap.ptr,
            c_title.as_ptr(),
            width,
            height,
            bitrate,
            c_path.as_ptr(),
        )
    }
}

#[cfg(target_os = "macos")]
pub fn stop_capture(cap: &mut SwiftCapture) {
    unsafe { swift_capture_stop(cap.ptr) }
}

#[cfg(target_os = "macos")]
impl Drop for SwiftCapture {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { swift_capture_destroy(self.ptr) }
        }
    }
}

// Non-macOS stubs
#[cfg(not(target_os = "macos"))]
pub fn create_capture_session() -> SwiftCapture {
    SwiftCapture { ptr: std::ptr::null_mut() }
}

#[cfg(not(target_os = "macos"))]
pub fn start_capture(
    _cap: &mut SwiftCapture,
    _window_title: &str,
    _width: u32,
    _height: u32,
    _bitrate: u32,
    _output_path: &str,
) -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn stop_capture(_cap: &mut SwiftCapture) {}

#[cfg(not(target_os = "macos"))]
impl Drop for SwiftCapture {
    fn drop(&mut self) {}
}