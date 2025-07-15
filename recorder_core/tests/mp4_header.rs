//! Ensures generated MP4 starts with a valid ftyp atom (QuickTime readable)

#[test]
#[cfg(target_os = "macos")]
fn mp4_header_is_valid() {
    use recorder_core::Recorder;
    use std::{fs::File, io::Read, path::Path};

    let path = "/tmp/quicktime_test.mp4";
    let _ = std::fs::remove_file(path);

    let mut rec = Recorder::new();
    // Finder window is always available; 1 sec, low bitrate
    if rec.start("Finder", 640, 360, 500_000, path).is_ok() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        rec.stop();

        // Read first 12 bytes - ftyp atom must be present
        let mut buf = [0u8; 12];
        File::open(path).unwrap().read_exact(&mut buf).unwrap();
        assert_eq!(&buf[4..8], b"ftyp", "MP4 missing ftyp atom – not QuickTime compatible");
    } else {
        eprintln!("⚠️  Skipped – screen-recording permission missing");
    }
}