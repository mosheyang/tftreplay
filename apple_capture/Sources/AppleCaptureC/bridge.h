#ifndef APPLE_CAPTURE_BRIDGE_H
#define APPLE_CAPTURE_BRIDGE_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void* swift_capture_create(void);
bool swift_capture_start(void* cap,
                        const char* window_title,
                        uint32_t width,
                        uint32_t height,
                        uint32_t bitrate,
                        const char* output_path);
void swift_capture_stop(void* cap);
void swift_capture_destroy(void* cap);

#ifdef __cplusplus
}
#endif

#endif /* APPLE_CAPTURE_BRIDGE_H */