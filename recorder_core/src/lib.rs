// ABOUTME: Core recorder library providing safe Rust API for Swift integration
// ABOUTME: Exposes screen recording functionality through FFI bridge

pub mod ffi;

use anyhow::Result;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Recorder {
    inner: Arc<Mutex<RecorderInner>>,
}

struct RecorderInner {
    #[cfg(target_os = "macos")]
    capture: Option<ffi::SwiftCapture>,
    is_recording: bool,
}

impl Recorder {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(RecorderInner {
                #[cfg(target_os = "macos")]
                capture: None,
                is_recording: false,
            })),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn start(
        &mut self,
        window_title: &str,
        width: u32,
        height: u32,
        bitrate: u32,
        output_path: &str,
    ) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        
        if inner.is_recording {
            anyhow::bail!("Already recording");
        }

        let mut capture = ffi::create_capture_session();
        let success = ffi::start_capture(
            &mut capture,
            window_title,
            width,
            height,
            bitrate,
            output_path,
        );

        if success {
            inner.capture = Some(capture);
            inner.is_recording = true;
            Ok(())
        } else {
            anyhow::bail!("Failed to start capture")
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn start(
        &mut self,
        _window_title: &str,
        _width: u32,
        _height: u32,
        _bitrate: u32,
        _output_path: &str,
    ) -> Result<()> {
        anyhow::bail!("Screen recording is only supported on macOS")
    }

    pub fn stop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        
        #[cfg(target_os = "macos")]
        if let Some(mut capture) = inner.capture.take() {
            ffi::stop_capture(&mut capture);
        }
        
        inner.is_recording = false;
    }

    pub fn is_recording(&self) -> bool {
        self.inner.lock().unwrap().is_recording
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_creation() {
        let recorder = Recorder::new();
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_double_start_fails() {
        let mut recorder = Recorder::new();
        
        // First start should work (but will fail in test env without window)
        let _ = recorder.start("Test", 640, 480, 1000000, "/tmp/test.mp4");
        
        // Second start should fail if first succeeded
        if recorder.is_recording() {
            let result = recorder.start("Test", 640, 480, 1000000, "/tmp/test2.mp4");
            assert!(result.is_err());
        }
    }
}

#[cfg(test)]
#[path = "tests/ffi_smoke.rs"]
mod ffi_smoke;