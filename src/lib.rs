//! Raw bindings to [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator).
//!
//! Vulkan types are generated here from the Khronos headers rather than taken from a Vulkan
//! binding crate, so this crate works with any of them. They have the Vulkan C layout, so cast
//! your binding's handles, structs and function pointers at the call.
//!
//! The library loads Vulkan functions dynamically: set `vkGetInstanceProcAddr` and
//! `vkGetDeviceProcAddr` in [`VmaVulkanFunctions`] and pass it in
//! [`VmaAllocatorCreateInfo::pVulkanFunctions`].
//!
//! Virtual blocks ([`vmaCreateVirtualBlock`]) need no Vulkan device.
//!
//! The library is built for Vulkan 1.4 with `VMA_EXTERNAL_MEMORY` on and
//! `VMA_EXTERNAL_MEMORY_WIN32` off, and for 64-bit targets only.
//!
//! # Failed checks and leaks
//!
//! VMA reports through macros with no allocator to hand, so this crate sends its reports to
//! [`log`] at error level with the target [`LOG_TARGET`].
//!
//! Checks are compiled in when this crate is built with debug assertions. A failed check goes to
//! stderr, then to `log`, then the logger is flushed and the process aborts.
//!
//! Leak reports are always on. Each one goes to `log`, or to stderr if `log_enabled!` is false for
//! [`LOG_TARGET`]. `log` can't tell a missing logger from a filter, so filtering out the target
//! moves leak reports to stderr rather than silencing them. A logger that accepts a report and
//! then drops it loses it.
//!
//! Each leak report is one line starting `VMA leak: `, cut off at 1023 bytes. They say:
//!
//! - For each allocation left in a memory block when it's destroyed: offset in the block, size,
//!   user data, name, type and usage. The name is the one set with [`vmaSetAllocationName`].
//! - For each allocation left in a virtual block: offset, size and user data.
//! - For each memory or virtual block destroyed with allocations left: one more line saying so.
//! - For dedicated allocations: only that some were left. They aren't listed.
//! - For memory still mapped when it's freed: only that it was mapped.
//!
//! No line says which allocator, pool or block it came from. A pool still alive when its allocator
//! is destroyed is a failed check, not a leak report.
//!
//! To act on a leak in code, call [`vmaIsVirtualBlockEmpty`] or [`vmaCalculateStatistics`] before
//! destroying, or match [`LOG_TARGET`] in a logger.
//!
//! The logger runs inside VMA's calls, sometimes with VMA's locks held, so it must not call VMA.
//! It must not panic either: a panic can't unwind through VMA, so it aborts the process.

#![no_std]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs,
    clippy::all,
    clippy::pedantic
)]

// The bindings are generated for a 64-bit target and hold pointer-sized handles.
#[cfg(not(target_pointer_width = "64"))]
compile_error!("vulkan-memory-allocator-sys supports only 64-bit targets");

mod report;

/// The `log` target of VMA's failed checks and leak reports.
pub const LOG_TARGET: &str = "vulkan_memory_allocator_sys";

include!("bindings.rs");
