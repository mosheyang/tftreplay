// ABOUTME: Unit tests for the AppleCapture screen recording functionality
// ABOUTME: Tests basic capture session creation and short recordings

import XCTest
@testable import AppleCapture

final class CaptureSessionTests: XCTestCase {
    
    func testCaptureSessionCreation() {
        let session = CaptureSession()
        XCTAssertNotNil(session, "Should create capture session")
    }
    
    func testFiveSecondCapture() throws {
        let expectation = self.expectation(description: "Capture completes")
        let url = URL(fileURLWithPath: "/tmp/test_capture.mp4")
        
        // Clean up any existing file
        try? FileManager.default.removeItem(at: url)
        
        let capture = CaptureSession()
        
        capture.start(windowTitle: "Finder", // Use Finder as it's always present
                      width: 640,
                      height: 360,
                      bitrate: 1_000_000,
                      outputURL: url) { error in
            XCTFail("Capture failed: \(error.localizedDescription)")
        }
        
        // Record for 5 seconds
        DispatchQueue.main.asyncAfter(deadline: .now() + 5) {
            capture.stop()
            
            // Wait a bit for file to be written
            DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
                expectation.fulfill()
            }
        }
        
        wait(for: [expectation], timeout: 10)
        
        // Verify file exists and size is reasonable
        XCTAssertTrue(FileManager.default.fileExists(atPath: url.path), "Output file should exist")
        
        let attributes = try FileManager.default.attributesOfItem(atPath: url.path)
        let fileSize = attributes[.size] as? Int64 ?? 0
        
        XCTAssertGreaterThan(fileSize, 10_000, "File should be at least 10KB")
        XCTAssertLessThan(fileSize, 5_000_000, "File should be less than 5MB for 5 second capture")
        
        // Clean up
        try? FileManager.default.removeItem(at: url)
    }
    
    func testInvalidWindowCapture() {
        let expectation = self.expectation(description: "Error callback triggered")
        let url = URL(fileURLWithPath: "/tmp/invalid_capture.mp4")
        let capture = CaptureSession()
        
        capture.start(windowTitle: "NonExistentWindow12345",
                      width: 640,
                      height: 360,
                      bitrate: 1_000_000,
                      outputURL: url) { error in
            XCTAssertNotNil(error, "Should receive error for invalid window")
            expectation.fulfill()
        }
        
        wait(for: [expectation], timeout: 5)
    }
    
    func testFrameRingBuffer() {
        let buffer = FrameRingBuffer(capacity: 10)
        
        // Create dummy pixel buffers
        var pixelBuffer: CVPixelBuffer?
        let attrs = [
            kCVPixelBufferCGImageCompatibilityKey: true,
            kCVPixelBufferCGBitmapContextCompatibilityKey: true
        ] as CFDictionary
        
        CVPixelBufferCreate(kCFAllocatorDefault, 100, 100,
                            kCVPixelFormatType_32BGRA, attrs, &pixelBuffer)
        
        guard let pb = pixelBuffer else {
            XCTFail("Failed to create pixel buffer")
            return
        }
        
        // Test appending
        for _ in 0..<15 {
            buffer.append(pb)
        }
        
        // Should only keep last 10 frames
        let frames = buffer.getFrames(last: 1.0)
        XCTAssertLessThanOrEqual(frames.count, 10, "Should not exceed buffer capacity")
        
        buffer.clear()
        let clearedFrames = buffer.getFrames(last: 1.0)
        XCTAssertEqual(clearedFrames.count, 0, "Buffer should be empty after clear")
    }
}