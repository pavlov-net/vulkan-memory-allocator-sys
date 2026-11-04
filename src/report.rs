//! The side of `src/vma.cpp`'s reports that needs `log`.

use core::ffi::{CStr, c_char, c_int};
use core::fmt;

use log::Level;

use crate::LOG_TARGET;

/// Shows bytes as UTF-8. VMA names can hold any bytes, and truncation can split a character.
struct Lossy<'a>(&'a [u8]);

impl fmt::Display for Lossy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.0.utf8_chunks() {
            f.write_str(chunk.valid())?;
            if !chunk.invalid().is_empty() {
                f.write_str("\u{FFFD}")?;
            }
        }
        Ok(())
    }
}

/// Logs `message`, and flushes the logger if `fatal` isn't 0. Returns 0 if the logger doesn't
/// want it.
///
/// # Safety
///
/// `message` is NUL-terminated.
#[unsafe(no_mangle)]
unsafe extern "C" fn vma_sys_log(message: *const c_char, fatal: c_int) -> c_int {
    if !log::log_enabled!(target: LOG_TARGET, Level::Error) {
        return 0;
    }
    // SAFETY: the caller passes a NUL-terminated string.
    let message = unsafe { CStr::from_ptr(message) }.to_bytes();
    log::error!(target: LOG_TARGET, "{}", Lossy(message));
    if fatal != 0 {
        log::logger().flush();
    }
    1
}
