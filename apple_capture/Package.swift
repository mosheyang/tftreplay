// swift-tools-version: 5.10
// ABOUTME: Swift package for hardware-accelerated screen capture on macOS
// ABOUTME: Uses AVFoundation and VideoToolbox for minimal CPU overhead

import PackageDescription

let package = Package(
    name: "AppleCapture",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "AppleCapture",
            type: .dynamic,
            targets: ["AppleCapture", "AppleCaptureC"]),
    ],
    targets: [
        .target(
            name: "AppleCapture",
            dependencies: [],
            path: "Sources/AppleCapture",
            swiftSettings: [
                .unsafeFlags(["-enable-library-evolution"])
            ]
        ),
        // Header-only C target so Cargo can dlopen symbols
        .target(
            name: "AppleCaptureC",
            dependencies: ["AppleCapture"],
            path: "Sources/AppleCaptureC",
            publicHeadersPath: "."
        ),
        .testTarget(
            name: "AppleCaptureTests",
            dependencies: ["AppleCapture"],
            path: "Sources/AppleCaptureTests"
        ),
    ]
)