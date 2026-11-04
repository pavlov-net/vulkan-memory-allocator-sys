use std::ptr;

use vulkan_memory_allocator_sys::*;

/// Destroys a virtual block with a 64-byte allocation left at offset 0.
pub fn leak() {
    let block_info = VmaVirtualBlockCreateInfo {
        size: 1024,
        ..Default::default()
    };
    let allocation_info = VmaVirtualAllocationCreateInfo {
        size: 64,
        alignment: 1,
        ..Default::default()
    };
    let mut block = ptr::null_mut();
    let mut allocation = ptr::null_mut();
    let mut offset = 0;
    // SAFETY: the arguments outlive each call, and `block` is live until destroyed.
    unsafe {
        assert_eq!(vmaCreateVirtualBlock(&block_info, &mut block), VK_SUCCESS);
        let result = vmaVirtualAllocate(block, &allocation_info, &mut allocation, &mut offset);
        assert_eq!(result, VK_SUCCESS);
        vmaDestroyVirtualBlock(block);
    }
}
