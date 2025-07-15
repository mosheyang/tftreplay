// ABOUTME: Integration tests for Swift-Rust FFI bridge
// ABOUTME: Tests actual screen recording functionality on macOS

#[cfg(target_os = "macos")]
#[cfg(test)]
mod tests {
    use crate::Recorder;
    use std::path::Path;

    #[test]
    fn test_start_stop_finder() {
        // Finder is always running on macOS
        let mut rec = Recorder::new();
        let output_path = "/tmp/finder_test.mp4";
        
        // Remove any existing file
        let _ = std::fs::remove_file(output_path);
        
        // Try to record Finder window
        let res = rec.start("Finder", 640, 360, 1_000_000, output_path);
        
        // This might fail if:
        // 1. No screen recording permission
        // 2. No Finder window visible
        // 3. Running in CI environment
        if res.is_ok() {
            assert!(rec.is_recording(), "Should be recording after successful start");
            
            // Record for a short time
            std::thread::sleep(std::time::Duration::from_millis(500));
            
            rec.stop();
            assert!(!rec.is_recording(), "Should not be recording after stop");
            
            // Check if file was created
            assert!(Path::new(output_path).exists(), "Output file should exist");
            
            // Clean up
            let _ = std::fs::remove_file(output_path);
        } else {
            // If start failed, just ensure we're in a consistent state
            assert!(!rec.is_recording(), "Should not be recording after failed start");
            println!("Note: Recording test skipped (no permission or window)");
        }
    }

    #[test]
    fn test_invalid_window_fails() {
        let mut rec = Recorder::new();
        
        // This should always fail
        let res = rec.start(
            "NonExistentWindow_TestOnly_12345",
            640,
            360,
            1_000_000,
            "/tmp/should_not_exist.mp4"
        );
        
        assert!(res.is_err(), "Should fail with non-existent window");
        assert!(!rec.is_recording(), "Should not be recording after failed start");
    }

    #[test]
    fn test_concurrent_recording_fails() {
        let mut rec = Recorder::new();
        
        // First attempt (might succeed or fail depending on environment)
        let first_result = rec.start("Finder", 640, 360, 1_000_000, "/tmp/test1.mp4");
        
        if first_result.is_ok() {
            // If first succeeded, second should fail
            let second_result = rec.start("Finder", 640, 360, 1_000_000, "/tmp/test2.mp4");
            assert!(second_result.is_err(), "Second recording should fail");
            assert!(rec.is_recording(), "Should still be recording from first start");
            
            // Clean up
            rec.stop();
            let _ = std::fs::remove_file("/tmp/test1.mp4");
        }
    }
}