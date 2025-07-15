// ABOUTME: Basic smoke tests for FFI bridge functionality
// ABOUTME: Ensures Rust-Swift integration symbols are available

#[cfg(test)]
mod tests {
    use crate::Recorder;

    #[test]
    fn test_recorder_new() {
        let recorder = Recorder::new();
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_recorder_lifecycle() {
        let mut recorder = Recorder::new();
        
        // Should not be recording initially
        assert!(!recorder.is_recording());
        
        // Stop should be safe even when not recording
        recorder.stop();
        assert!(!recorder.is_recording());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_recorder_start_invalid_window() {
        let mut recorder = Recorder::new();
        
        // Starting with non-existent window should fail
        let result = recorder.start(
            "NonExistentWindow12345",
            1280,
            720,
            4000000,
            "/tmp/test_invalid.mp4"
        );
        
        assert!(result.is_err());
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_concurrent_recorders() {
        let recorder1 = Recorder::new();
        let recorder2 = Recorder::new();
        
        // Both should be independent
        assert!(!recorder1.is_recording());
        assert!(!recorder2.is_recording());
    }
}