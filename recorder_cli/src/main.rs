// ABOUTME: CLI entry point for TFT recorder with subcommands for recording and extension host
// ABOUTME: Provides user-friendly interface for screen capture and plugin management

use anyhow::Result;
use clap::{Parser, Subcommand};
use recorder_core::Recorder;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Parser)]
#[command(name = "recorder")]
#[command(about = "Ultra-light screen recorder for Team Fight Tactics", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Record TFT gameplay
    Record {
        /// Window title to capture
        #[arg(long, default_value = "Teamfight Tactics")]
        window: String,
        
        /// Video width in pixels
        #[arg(long, default_value = "1280")]
        width: u32,
        
        /// Video height in pixels
        #[arg(long, default_value = "720")]
        height: u32,
        
        /// Video bitrate in bits per second
        #[arg(long, default_value = "4000000")]
        bitrate: u32,
        
        /// Output file path
        #[arg(long, default_value = "tft_recording.mp4")]
        out: String,
        
        /// Duration in seconds (0 for manual stop)
        #[arg(long, default_value = "0")]
        duration: u32,
    },
    
    /// Start the extension host (internal use)
    Host {
        /// Port for gRPC communication
        #[arg(long, default_value = "0")]
        port: u16,
    },
    
    /// Run as daemon for background recording
    Daemon {
        /// Unix socket path for IPC
        #[arg(long, default_value = "/tmp/tft-recorder.sock")]
        socket: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Record { window, width, height, bitrate, out, duration } => {
            record_command(window, width, height, bitrate, out, duration)
        }
        Commands::Host { port } => {
            host_command(port)
        }
        Commands::Daemon { socket } => {
            daemon_command(socket)
        }
    }
}

fn record_command(
    window: String,
    width: u32,
    height: u32,
    bitrate: u32,
    out: String,
    duration: u32,
) -> Result<()> {
    println!("Starting recording...");
    println!("Window: {}", window);
    println!("Resolution: {}x{}", width, height);
    println!("Bitrate: {} bps", bitrate);
    println!("Output: {}", out);
    
    let mut recorder = Recorder::new();
    
    // Set up graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        println!("\nStopping recording...");
        r.store(false, Ordering::SeqCst);
    })?;
    
    // Start recording
    if let Err(e) = recorder.start(&window, width, height, bitrate, &out) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    println!("Recording started. Press Ctrl+C to stop.");
    
    // Wait for duration or interrupt
    if duration > 0 {
        println!("Recording for {} seconds...", duration);
        let start = std::time::Instant::now();
        
        while running.load(Ordering::SeqCst) && start.elapsed().as_secs() < duration as u64 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    } else {
        // Wait for Ctrl+C
        while running.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    // Stop recording
    recorder.stop();
    println!("Recording saved to: {}", out);
    
    Ok(())
}

fn host_command(port: u16) -> Result<()> {
    println!("Starting extension host on port {}...", port);
    
    // In a real implementation, this would launch the Node.js process
    // For now, we'll just spawn it as a subprocess
    let status = std::process::Command::new("node")
        .arg("extension-host/dist/index.js")
        .arg("--port")
        .arg(port.to_string())
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Extension host failed to start");
    }
    
    Ok(())
}

fn daemon_command(socket: String) -> Result<()> {
    println!("Starting recorder daemon on socket: {}", socket);
    
    // Set up the async runtime for gRPC
    let runtime = tokio::runtime::Runtime::new()?;
    
    runtime.block_on(async {
        // In a real implementation, this would start the gRPC server
        println!("Daemon started. Listening for commands...");
        
        // Keep running until interrupted
        tokio::signal::ctrl_c().await?;
        println!("Daemon shutting down...");
        
        Ok::<(), anyhow::Error>(())
    })?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
    
    #[test]
    fn test_default_args() {
        let cli = Cli::parse_from(vec!["recorder", "record"]);
        match cli.command {
            Commands::Record { window, width, height, bitrate, out, duration } => {
                assert_eq!(window, "Teamfight Tactics");
                assert_eq!(width, 1280);
                assert_eq!(height, 720);
                assert_eq!(bitrate, 4000000);
                assert_eq!(out, "tft_recording.mp4");
                assert_eq!(duration, 0);
            }
            _ => panic!("Expected Record command"),
        }
    }
}