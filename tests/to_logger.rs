//! The logger is set once per process, so this file holds the one test that sets it.

use std::sync::Mutex;

use log::{LevelFilter, Log, Metadata, Record};
use vulkan_memory_allocator_sys::LOG_TARGET;

mod common;

static LINES: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Capture;

impl Log for Capture {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.target() == LOG_TARGET
    }

    fn log(&self, record: &Record<'_>) {
        LINES.lock().unwrap().push(record.args().to_string());
    }

    fn flush(&self) {}
}

#[test]
fn a_leak_reaches_the_logger_with_its_offset_and_size() {
    log::set_logger(&Capture).unwrap();
    log::set_max_level(LevelFilter::Error);

    common::leak();

    let lines = LINES.lock().unwrap();
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(
        lines[0].starts_with("VMA leak: UNFREED VIRTUAL ALLOCATION; Offset: 0; Size: 64;"),
        "{lines:?}"
    );
    assert!(lines[1].starts_with("VMA leak: "), "{lines:?}");
}
