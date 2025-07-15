// ABOUTME: C bridge for Swift FFI - placeholder implementation
// ABOUTME: In production, this would call into Swift via @objc exports

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct SwiftCapture {
    void* internal;
} SwiftCapture;

SwiftCapture* swift_capture_create(void) {
    // Placeholder - would create Swift CaptureSession
    SwiftCapture* capture = (SwiftCapture*)malloc(sizeof(SwiftCapture));
    capture->internal = NULL;
    return capture;
}

void swift_capture_destroy(SwiftCapture* capture) {
    if (capture) {
        free(capture);
    }
}

bool swift_capture_start(SwiftCapture* capture,
                        const char* window_title,
                        uint32_t width,
                        uint32_t height,
                        uint32_t bitrate,
                        const char* output_path) {
    // Placeholder - would call Swift implementation
    return false; // Always fail in test environment
}

void swift_capture_stop(SwiftCapture* capture) {
    // Placeholder - would stop Swift capture
}