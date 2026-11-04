//! Virtual blocks run the compiled library with no Vulkan device.

use std::ptr;

use vulkan_memory_allocator_sys::*;

const SIZE: VkDeviceSize = 1 << 20;

fn create_block() -> VmaVirtualBlock {
    let info = VmaVirtualBlockCreateInfo {
        size: SIZE,
        ..Default::default()
    };
    let mut block = ptr::null_mut();
    // SAFETY: `info` and `block` outlive the call.
    let result = unsafe { vmaCreateVirtualBlock(&info, &mut block) };
    assert_eq!(result, VK_SUCCESS);
    block
}

fn allocate(
    block: VmaVirtualBlock,
    size: VkDeviceSize,
    alignment: VkDeviceSize,
) -> (VmaVirtualAllocation, VkDeviceSize) {
    let info = VmaVirtualAllocationCreateInfo {
        size,
        alignment,
        ..Default::default()
    };
    let mut allocation = ptr::null_mut();
    let mut offset = 0;
    // SAFETY: `block` is live, and the others outlive the call.
    let result = unsafe { vmaVirtualAllocate(block, &info, &mut allocation, &mut offset) };
    assert_eq!(result, VK_SUCCESS);
    (allocation, offset)
}

#[test]
fn allocations_are_aligned_apart_and_freeable() {
    let block = create_block();
    let (a, a_offset) = allocate(block, 4096, 256);
    let (b, b_offset) = allocate(block, 4096, 256);
    assert_eq!(a_offset % 256, 0);
    assert_eq!(b_offset % 256, 0);
    assert!(a_offset + 4096 <= b_offset || b_offset + 4096 <= a_offset);

    let info = VmaVirtualAllocationCreateInfo {
        size: SIZE + 1,
        ..Default::default()
    };
    let mut none = ptr::null_mut();
    let mut offset = 0;
    // SAFETY: `block` is live, and the others outlive the call.
    let result = unsafe { vmaVirtualAllocate(block, &info, &mut none, &mut offset) };
    assert_ne!(result, VK_SUCCESS, "more than the block holds");

    // SAFETY: both allocations are from `block`, which has nothing left when destroyed.
    unsafe {
        vmaVirtualFree(block, a);
        vmaVirtualFree(block, b);
        assert_ne!(vmaIsVirtualBlockEmpty(block), 0);
        vmaDestroyVirtualBlock(block);
    }
}
