//! Each test runs again as a child process, which has no logger yet, puts VMA's stderr where the
//! parent can read it, and can abort.

use std::env;
use std::process::{Command, Output};

mod common;

const CHILD: &str = "VMA_SYS_TEST_CHILD";

/// Runs `body` if this is the child. Otherwise runs `test` in a child and returns its output.
fn in_child(test: &str, body: fn()) -> Option<Output> {
    if env::var_os(CHILD).is_some() {
        body();
        return None;
    }
    let output = Command::new(env::current_exe().unwrap())
        .args(["--exact", test])
        .env(CHILD, "1")
        .output()
        .unwrap();
    Some(output)
}

#[test]
fn a_leak_with_no_logger_reaches_stderr() {
    let Some(output) = in_child("a_leak_with_no_logger_reaches_stderr", common::leak) else {
        return;
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("VMA leak: UNFREED VIRTUAL ALLOCATION; Offset: 0; Size: 64;"),
        "{stderr}"
    );
}

// Checks are compiled in only with debug assertions.
#[cfg(debug_assertions)]
#[test]
fn a_failed_check_reaches_stderr_past_a_logger_that_drops_it_and_aborts() {
    use std::ptr;

    use vulkan_memory_allocator_sys::*;

    struct Drops;

    impl log::Log for Drops {
        fn enabled(&self, _: &log::Metadata<'_>) -> bool {
            true
        }

        fn log(&self, _: &log::Record<'_>) {}

        fn flush(&self) {}
    }

    fn fail_a_check() {
        log::set_logger(&Drops).unwrap();
        log::set_max_level(log::LevelFilter::Error);
        let info = VmaVirtualBlockCreateInfo::default();
        let mut block = ptr::null_mut();
        // SAFETY: the size of 0 fails a check, which aborts before anything else happens.
        unsafe { vmaCreateVirtualBlock(&info, &mut block) };
    }

    let test = "a_failed_check_reaches_stderr_past_a_logger_that_drops_it_and_aborts";
    let Some(output) = in_child(test, fail_a_check) else {
        return;
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("VMA check failed at "), "{stderr}");
}
