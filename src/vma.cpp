// VMA's implementation. Failed checks and leaks go to `log` and stderr, so no C runtime dialog
// opens.
static void vma_sys_check_failed(const char* expr, const char* file, int line);
static void vma_sys_leak(int flush, const char* format, ...);

#ifdef VMA_SYS_CHECKS
#define VMA_ASSERT(expr) ((expr) ? (void)0 : vma_sys_check_failed(#expr, __FILE__, __LINE__))
#else
#define VMA_ASSERT(expr)
#endif
// A block's listing comes first, so the summary is where a report ends.
#define VMA_ASSERT_LEAK(expr) ((expr) ? (void)0 : vma_sys_leak(1, "VMA leak: %s", #expr))
#define VMA_LEAK_LOG_FORMAT(format, ...) vma_sys_leak(0, "VMA leak: " format, __VA_ARGS__)

#define VMA_IMPLEMENTATION
#include "vma.h"

#include <cstdarg>
#include <cstdio>
#include <cstdlib>

static_assert(VMA_VERSION == VK_MAKE_VERSION(3, 5, 0), "src/bindings.rs is generated from VMA 3.5.0");

// Defined in src/report.rs. Hidden so that a Rust cdylib doesn't export it on ELF.
extern "C"
#ifdef __GNUC__
__attribute__((visibility("hidden")))
#endif
int vma_sys_log(const char* message, int flush);

// A logger can drop or hold a message, so stderr comes first.
static void vma_sys_check_failed(const char* expr, const char* file, int line) {
    char message[1024];
    std::snprintf(message, sizeof message, "VMA check failed at %s:%d: %s", file, line, expr);
    std::fprintf(stderr, "%s\n", message);
    std::fflush(stderr);
    vma_sys_log(message, 1);
    std::abort();
}

static void vma_sys_leak(int flush, const char* format, ...) {
    char message[1024];
    std::va_list args;
    va_start(args, format);
    std::vsnprintf(message, sizeof message, format, args);
    va_end(args);
    if (!vma_sys_log(message, flush)) {
        std::fprintf(stderr, "%s\n", message);
    }
}
