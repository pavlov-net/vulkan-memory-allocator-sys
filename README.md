# vulkan-memory-allocator-sys

Raw Rust bindings to AMD's [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) 3.5.0.

VMA and the Vulkan headers are included, so building needs only a C++17 compiler. The crate doesn't depend on ash or any other Vulkan bindings. There is no safe wrapper, and everything is named as in VMA's C API.

Failed checks go to stderr and the [`log`](https://crates.io/crates/log) crate. Leak reports go to `log`, or to stderr if no logger takes them. The crate docs have the details.

## License

MIT. See [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md) for VMA and the Vulkan headers.
