// ABOUTME: GUI module for TFT Recorder using egui/eframe
// ABOUTME: Provides a minimalist interface for recording and managing recordings

use eframe::{egui, NativeOptions};
use recorder_core::Recorder;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};

const RECORDINGS_DIR: &str = "~/Movies/TFT Recorder";

pub fn launch() -> anyhow::Result<()> {
    // Create the recordings directory if it doesn't exist
    let recordings_path = expand_home(RECORDINGS_DIR);
    fs::create_dir_all(&recordings_path)?;

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "TFT Recorder",
        options,
        Box::new(|_cc| Box::<RecorderApp>::default()),
    )
    .map_err(|e| anyhow::anyhow!("eframe failed: {e}"))
}

#[derive(Default)]
struct RecorderApp {
    recorder: Arc<Mutex<Recorder>>,
    is_recording: bool,
    started_at: Option<DateTime<Local>>,
    error_message: Option<String>,
}

impl eframe::App for RecorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ---------- global style ----------
        ctx.set_style(egui::Style {
            visuals: egui::Visuals::dark(),
            ..Default::default()
        });

        // ---------- top toolbar ----------
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(10.0);
                
                if self.is_recording {
                    // Stop button
                    let stop_button = egui::Button::new("⏹ Stop")
                        .min_size(egui::vec2(80.0, 30.0));
                    if ui.add(stop_button).clicked() {
                        if let Ok(mut rec) = self.recorder.lock() {
                            rec.stop();
                        }
                        self.is_recording = false;
                        self.started_at = None;
                    }

                    // Live timer
                    if let Some(start) = self.started_at {
                        let secs = (Local::now() - start).num_seconds();
                        ui.add_space(20.0);
                        ui.strong(format!("{:02}:{:02}", secs / 60, secs % 60));
                    }
                } else {
                    // Record button
                    let rec_button = egui::Button::new("● Rec")
                        .min_size(egui::vec2(80.0, 30.0));
                    if ui.add(rec_button).clicked() {
                        self.start_recording();
                    }
                }
            });
        });

        // ---------- left panel ----------
        egui::SidePanel::left("recordings_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Recordings");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let recordings = list_recordings();
                    
                    if recordings.is_empty() {
                        ui.label("No recordings yet");
                    } else {
                        for recording in recordings {
                            let file_name = recording
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown");
                            
                            if ui.button(file_name).clicked() {
                                // Reveal in Finder
                                let _ = std::process::Command::new("open")
                                    .arg("-R")
                                    .arg(&recording)
                                    .spawn();
                            }
                        }
                    }
                });
            });

        // ---------- central panel (info / errors) ----------
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if !self.is_recording {
                    ui.add_space(40.0);
                    ui.label("Ready to capture your gameplay.");
                    ui.small("Files land in ~/Movies/TFT Recorder");
                }

                if let Some(err) = &self.error_message {
                    ui.add_space(20.0);
                    ui.colored_label(egui::Color32::LIGHT_RED, err);
                    
                    if ui.button("Dismiss").clicked() {
                        self.error_message = None;
                    }
                }
            });
        });
        
        // Request repaint if recording (to update timer)
        if self.is_recording {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }
}

impl RecorderApp {
    fn start_recording(&mut self) {
        let output_path = next_file_name();
        
        if let Ok(mut recorder) = self.recorder.lock() {
            // Try to record the desktop (entire screen)
            // We'll use "Desktop" as the window name, though this might need adjustment
            match recorder.start("Desktop", 1920, 1080, 6_000_000, &output_path) {
                Ok(_) => {
                    self.is_recording = true;
                    self.started_at = Some(Local::now());
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to start recording: {}", e));
                }
            }
        }
    }
}

// Helper functions

pub fn expand_home(path: &str) -> PathBuf {
    PathBuf::from(shellexpand::tilde(path).as_ref())
}

fn list_recordings() -> Vec<PathBuf> {
    let dir = expand_home(RECORDINGS_DIR);
    
    if let Ok(entries) = fs::read_dir(&dir) {
        let mut recordings: Vec<PathBuf> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                
                // Only include .mp4 files
                if path.extension()?.to_str()? == "mp4" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by modification time (newest first)
        recordings.sort_by(|a, b| {
            let a_time = fs::metadata(a).and_then(|m| m.modified()).ok();
            let b_time = fs::metadata(b).and_then(|m| m.modified()).ok();
            b_time.cmp(&a_time)
        });
        
        recordings
    } else {
        Vec::new()
    }
}

pub fn next_file_name() -> String {
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let dir = expand_home(RECORDINGS_DIR);
    format!("{}/TFT-{}.mp4", dir.display(), timestamp)
}

// Re-export for use in main.rs
pub use self::next_file_name as get_default_output_path;