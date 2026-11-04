#ifndef VMA_SYS_H
#define VMA_SYS_H

#define VK_NO_PROTOTYPES
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 1
// The macros that choose struct members are fixed, so the committed bindings match the
// compiled library on every platform.
#define VMA_VULKAN_VERSION 1004000
#define VMA_EXTERNAL_MEMORY 1
#define VMA_EXTERNAL_MEMORY_WIN32 0
#include "vk_mem_alloc.h"

#ifdef __cplusplus
extern "C" {
#endif

// Called when one of VMA's internal checks fails. The process aborts when it returns.
typedef void (*VmaSysCheckFailedHandler)(const char* expr, const char* file, int line);
// Called when an allocator, pool or virtual block is destroyed with allocations left, or memory is
// freed while still mapped.
typedef void (*VmaSysLeakHandler)(const char* expr);

// Replaces the handlers, which print to stderr by default. Null restores a default. Safe to call
// from any thread. Internal checks are compiled only when the crate is built with debug
// assertions; the leak handler is always active.
void vmaSysSetHandlers(VmaSysCheckFailedHandler check_failed, VmaSysLeakHandler leak);

#ifdef __cplusplus
}
#endif

#endif
