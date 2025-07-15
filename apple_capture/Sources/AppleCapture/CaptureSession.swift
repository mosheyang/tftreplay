// ABOUTME: Main capture session manager using AVFoundation for screen recording
// ABOUTME: Handles window targeting, frame capture, and session lifecycle

import AVFoundation
import CoreGraphics
import VideoToolbox

public final class CaptureSession: NSObject {
    private let session = AVCaptureSession()
    private let queue = DispatchQueue(label: "apple_capture", qos: .userInitiated)
    private var encoder: Encoder?
    
    public override init() {
        super.init()
    }
    
    public func start(windowTitle: String,
                      width: Int,
                      height: Int,
                      bitrate: Int,
                      outputURL: URL,
                      onError: @escaping (Error) -> Void) {
        
        queue.async { [weak self] in
            guard let self = self else { return }
            do {
                try self.configure(windowTitle: windowTitle,
                                   width: width, 
                                   height: height, 
                                   bitrate: bitrate,
                                   outputURL: outputURL)
                self.session.startRunning()
            } catch {
                onError(error)
            }
        }
    }
    
    public func stop() {
        queue.async { [weak self] in
            self?.session.stopRunning()
            self?.encoder?.finalize()
        }
    }
    
    // MARK: - Private helpers
    private func configure(windowTitle: String,
                           width: Int, 
                           height: Int,
                           bitrate: Int,
                           outputURL: URL) throws {
        session.beginConfiguration()
        defer { session.commitConfiguration() }
        
        session.sessionPreset = .high
        
        // Find window by title
        let windowList = CGWindowListCopyWindowInfo([.optionAll], kCGNullWindowID) as? [[String: Any]] ?? []
        
        guard let windowInfo = windowList.first(where: { window in
            (window[kCGWindowName as String] as? String) == windowTitle
        }) else {
            throw CaptureError.windowNotFound
        }
        
        guard let windowNumber = windowInfo[kCGWindowNumber as String] as? Int else {
            throw CaptureError.invalidWindowNumber
        }
        
        // Get main display ID
        let displayID = CGMainDisplayID()
        
        // Create screen input
        guard let input = AVCaptureScreenInput(displayID: displayID) else {
            throw CaptureError.cannotCreateInput
        }
        
        // Configure input
        input.minFrameDuration = CMTime(value: 1, timescale: 60) // 60 fps max
        input.capturesCursor = true
        input.capturesMouseClicks = true
        
        // Calculate scale factor
        let displayWidth = CGDisplayPixelsWide(displayID)
        if displayWidth > 0 && width > 0 {
            input.scaleFactor = CGFloat(width) / CGFloat(displayWidth)
        }
        
        // Add input to session
        if session.canAddInput(input) {
            session.addInput(input)
        } else {
            throw CaptureError.cannotAddInput
        }
        
        // Create and configure encoder
        encoder = try Encoder(outputURL: outputURL,
                              width: width,
                              height: height,
                              bitrate: bitrate)
        
        try encoder?.attach(to: session)
    }
}

public enum CaptureError: LocalizedError {
    case windowNotFound
    case invalidWindowNumber
    case cannotCreateInput
    case cannotAddInput
    case encoderSetupFailed
    
    public var errorDescription: String? {
        switch self {
        case .windowNotFound:
            return "Window with specified title not found"
        case .invalidWindowNumber:
            return "Invalid window number"
        case .cannotCreateInput:
            return "Cannot create screen capture input"
        case .cannotAddInput:
            return "Cannot add input to capture session"
        case .encoderSetupFailed:
            return "Failed to setup video encoder"
        }
    }
}