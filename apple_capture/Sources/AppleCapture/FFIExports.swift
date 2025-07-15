// ABOUTME: Swift FFI exports providing C-compatible functions for Rust integration
// ABOUTME: Uses @_cdecl to expose Swift functionality through C symbols

import Foundation

// Opaque pointer wrapper functions
private func retain(_ obj: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(obj).toOpaque()
}

private func fromOpaque(_ ptr: UnsafeMutableRawPointer) -> CaptureSession {
    Unmanaged<CaptureSession>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("swift_capture_create")
public func swift_capture_create() -> UnsafeMutableRawPointer? {
    retain(CaptureSession())
}

@_cdecl("swift_capture_start")
public func swift_capture_start(_ ptr: UnsafeMutableRawPointer?,
                                _ title: UnsafePointer<CChar>,
                                _ width: UInt32,
                                _ height: UInt32,
                                _ bitrate: UInt32,
                                _ outPath: UnsafePointer<CChar>) -> Bool {
    guard let ptr else { return false }
    let session = fromOpaque(ptr)
    let window = String(cString: title)
    let url = URL(fileURLWithPath: String(cString: outPath))
    
    var started = false
    let sema = DispatchSemaphore(value: 0)
    
    session.start(windowTitle: window,
                  width: Int(width),
                  height: Int(height),
                  bitrate: Int(bitrate),
                  outputURL: url) { error in
        // Error callback - signal failure
        print("Swift capture error: \(error.localizedDescription)")
        sema.signal()
    }
    
    // If no error arrives within 0.3s, assume success
    started = sema.wait(timeout: .now() + 0.3) == .timedOut
    return started
}

@_cdecl("swift_capture_stop")
public func swift_capture_stop(_ ptr: UnsafeMutableRawPointer?) {
    ptr.map { fromOpaque($0).stop() }
}

@_cdecl("swift_capture_destroy")
public func swift_capture_destroy(_ ptr: UnsafeMutableRawPointer?) {
    ptr.map { Unmanaged<CaptureSession>.fromOpaque($0).release() }
}