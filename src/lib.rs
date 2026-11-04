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
//! See [`vmaSysSetHandlers`] to route failed checks and leak reports somewhere other than stderr.
//! Checks are compiled in when this crate is built with debug assertions. Leak reports are always
//! on. A pool still alive when its allocator is destroyed is a check, not a leak report.
//!
//! The library is built for Vulkan 1.4 with `VMA_EXTERNAL_MEMORY` on and
//! `VMA_EXTERNAL_MEMORY_WIN32` off, and for 64-bit targets only.

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

include!("bindings.rs");
