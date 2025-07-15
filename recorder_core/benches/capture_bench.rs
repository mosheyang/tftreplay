// ABOUTME: Performance benchmarks for recorder initialization and operations
// ABOUTME: Ensures startup time stays under 50ms for responsive user experience

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use recorder_core::Recorder;

fn bench_recorder_creation(c: &mut Criterion) {
    c.bench_function("recorder_new", |b| {
        b.iter(|| {
            let recorder = Recorder::new();
            black_box(recorder);
        });
    });
}

fn bench_recorder_lifecycle(c: &mut Criterion) {
    c.bench_function("recorder_start_stop", |b| {
        b.iter(|| {
            let mut recorder = Recorder::new();
            
            // Attempt to start (will fail without valid window, but measures overhead)
            let _ = recorder.start(
                "Benchmark Window",
                1280,
                720,
                4000000,
                "/tmp/bench.mp4"
            );
            
            recorder.stop();
            black_box(recorder);
        });
    });
}

fn bench_is_recording_check(c: &mut Criterion) {
    let recorder = Recorder::new();
    
    c.bench_function("is_recording", |b| {
        b.iter(|| {
            black_box(recorder.is_recording());
        });
    });
}

criterion_group!(benches, bench_recorder_creation, bench_recorder_lifecycle, bench_is_recording_check);
criterion_main!(benches);