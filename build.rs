//! Compiles Vulkan Memory Allocator's implementation from the headers in `vendor/`.

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/vma.h");
    println!("cargo::rerun-if-changed=src/vma.cpp");
    println!("cargo::rerun-if-changed=vendor");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .file("src/vma.cpp")
        .include("src")
        .include("vendor/VulkanMemoryAllocator/include")
        .include("vendor/Vulkan-Headers/include")
        .warnings(false);
    // Cargo at this crate's minimum Rust version leaves the variable unset, so this script's own
    // profile covers it there.
    if cfg!(debug_assertions) || std::env::var_os("CARGO_CFG_DEBUG_ASSERTIONS").is_some() {
        build.define("VMA_SYS_CHECKS", None);
    }
    build.compile("vulkan_memory_allocator");
}
