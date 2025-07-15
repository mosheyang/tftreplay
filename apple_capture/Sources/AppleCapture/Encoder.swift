// ABOUTME: Hardware H.264 encoder using AVAssetWriter and VideoToolbox
// ABOUTME: Handles real-time encoding with minimal CPU usage for TFT recording

import AVFoundation
import VideoToolbox
import CoreMedia

final class Encoder: NSObject {
    private let writer: AVAssetWriter
    private let input: AVAssetWriterInput
    private let adaptor: AVAssetWriterInputPixelBufferAdaptor
    private let queue = DispatchQueue(label: "encoder", qos: .userInitiated)
    private var isWriting = false
    
    init(outputURL: URL, width: Int, height: Int, bitrate: Int) throws {
        // Remove existing file if present
        try? FileManager.default.removeItem(at: outputURL)
        
        // Create writer
        writer = try AVAssetWriter(outputURL: outputURL, fileType: .mp4)
        
        // Configure H.264 settings
        let settings: [String: Any] = [
            AVVideoCodecKey: AVVideoCodecType.h264,
            AVVideoWidthKey: width,
            AVVideoHeightKey: height,
            AVVideoCompressionPropertiesKey: [
                AVVideoAverageBitRateKey: bitrate,
                AVVideoMaxKeyFrameIntervalKey: 60, // Keyframe every second at 60fps
                AVVideoProfileLevelKey: AVVideoProfileLevelH264HighAutoLevel,
                AVVideoH264EntropyModeKey: AVVideoH264EntropyModeCABAC
            ] as [String: Any]
        ]
        
        // Create input
        input = AVAssetWriterInput(mediaType: .video, outputSettings: settings)
        input.expectsMediaDataInRealTime = true
        
        // Create pixel buffer adaptor
        let sourcePixelBufferAttributes: [String: Any] = [
            kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_32BGRA,
            kCVPixelBufferWidthKey as String: width,
            kCVPixelBufferHeightKey as String: height,
            kCVPixelBufferIOSurfacePropertiesKey as String: [:] // Enable IOSurface for better performance
        ]
        
        adaptor = AVAssetWriterInputPixelBufferAdaptor(
            assetWriterInput: input,
            sourcePixelBufferAttributes: sourcePixelBufferAttributes
        )
        
        // Add input to writer
        if writer.canAdd(input) {
            writer.add(input)
        } else {
            throw CaptureError.encoderSetupFailed
        }
        
        super.init()
    }
    
    func attach(to session: AVCaptureSession) throws {
        let output = AVCaptureVideoDataOutput()
        output.setSampleBufferDelegate(self, queue: queue)
        
        // Configure output
        output.videoSettings = [
            kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_32BGRA
        ]
        
        // Disable frame dropping for consistent recording
        output.alwaysDiscardsLateVideoFrames = false
        
        if session.canAddOutput(output) {
            session.addOutput(output)
        } else {
            throw CaptureError.encoderSetupFailed
        }
    }
    
    func finalizeRecording() {
        queue.async { [weak self] in
            guard let self = self, self.isWriting else { return }
            
            self.input.markAsFinished()
            self.writer.finishWriting {
                print("Recording finished: \(self.writer.status == .completed ? "Success" : "Failed")")
            }
        }
    }
}

// MARK: - AVCaptureVideoDataOutputSampleBufferDelegate
extension Encoder: AVCaptureVideoDataOutputSampleBufferDelegate {
    func captureOutput(_ output: AVCaptureOutput,
                       didOutput sampleBuffer: CMSampleBuffer,
                       from connection: AVCaptureConnection) {
        
        guard CMSampleBufferDataIsReady(sampleBuffer) else { return }
        
        let presentationTime = CMSampleBufferGetPresentationTimeStamp(sampleBuffer)
        
        // Start writing on first frame
        if !isWriting {
            guard writer.status == .unknown else { return }
            
            writer.startWriting()
            writer.startSession(atSourceTime: presentationTime)
            isWriting = true
        }
        
        // Write frame
        guard writer.status == .writing,
              input.isReadyForMoreMediaData,
              let imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            return
        }
        
        adaptor.append(imageBuffer, withPresentationTime: presentationTime)
    }
    
    func captureOutput(_ output: AVCaptureOutput,
                       didDrop sampleBuffer: CMSampleBuffer,
                       from connection: AVCaptureConnection) {
        print("Dropped frame at: \(CMSampleBufferGetPresentationTimeStamp(sampleBuffer).seconds)")
    }
}