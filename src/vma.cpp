// VMA's implementation. Failed checks and leaks go to replaceable handlers, so no C runtime
// dialog opens.
static void vma_sys_check_failed(const char* expr, const char* file, int line);
static void vma_sys_leak(const char* expr);

#ifdef VMA_SYS_CHECKS
#define VMA_ASSERT(expr) ((expr) ? (void)0 : vma_sys_check_failed(#expr, __FILE__, __LINE__))
#else
#define VMA_ASSERT(expr)
#endif
#define VMA_ASSERT_LEAK(expr) ((expr) ? (void)0 : vma_sys_leak(#expr))

#define VMA_IMPLEMENTATION
#include "vma.h"

#include <atomic>
#include <cstdio>
#include <cstdlib>

static_assert(VMA_VERSION == VK_MAKE_VERSION(3, 5, 0), "src/bindings.rs is generated from VMA 3.5.0");

static std::atomic<VmaSysCheckFailedHandler> check_failed_handler{nullptr};
static std::atomic<VmaSysLeakHandler> leak_handler{nullptr};

static void vma_sys_check_failed(const char* expr, const char* file, int line) {
    if (VmaSysCheckFailedHandler handler = check_failed_handler.load()) {
        handler(expr, file, line);
    } else {
        std::fprintf(stderr, "VMA check failed at %s:%d: %s\n", file, line, expr);
    }
    std::abort();
}

static void vma_sys_leak(const char* expr) {
    if (VmaSysLeakHandler handler = leak_handler.load()) {
        handler(expr);
    } else {
        std::fprintf(stderr, "VMA leak: %s\n", expr);
    }
}

extern "C" void vmaSysSetHandlers(VmaSysCheckFailedHandler check_failed, VmaSysLeakHandler leak) {
    check_failed_handler.store(check_failed);
    leak_handler.store(leak);
}
