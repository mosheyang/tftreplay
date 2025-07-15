// ABOUTME: Ring buffer for storing recent frames to enable rewind/replay features
// ABOUTME: Currently a placeholder for future instant-replay functionality

import Foundation
import CoreVideo

public final class FrameRingBuffer {
    private let capacity: Int
    private var buffer: [CVPixelBuffer?]
    private var writeIndex = 0
    private let lock = NSLock()
    
    public init(capacity: Int = 300) { // 5 seconds at 60fps
        self.capacity = capacity
        self.buffer = Array(repeating: nil, count: capacity)
    }
    
    public func append(_ pixelBuffer: CVPixelBuffer) {
        lock.lock()
        defer { lock.unlock() }
        
        // Release old buffer if present
        buffer[writeIndex] = nil
        
        // Store new buffer with retained reference
        buffer[writeIndex] = pixelBuffer
        writeIndex = (writeIndex + 1) % capacity
    }
    
    public func getFrames(last seconds: TimeInterval) -> [CVPixelBuffer] {
        lock.lock()
        defer { lock.unlock() }
        
        let frameCount = min(Int(seconds * 60), capacity)
        var frames: [CVPixelBuffer] = []
        
        for i in 0..<frameCount {
            let index = (writeIndex - 1 - i + capacity) % capacity
            if let frame = buffer[index] {
                frames.append(frame)
            }
        }
        
        return frames.reversed()
    }
    
    public func clear() {
        lock.lock()
        defer { lock.unlock() }
        
        buffer = Array(repeating: nil, count: capacity)
        writeIndex = 0
    }
}