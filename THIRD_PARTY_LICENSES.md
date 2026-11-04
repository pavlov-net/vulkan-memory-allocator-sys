# Third-party licenses

## Vulkan Memory Allocator

`vendor/VulkanMemoryAllocator/` holds files from
<https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator> v3.5.0, at commit
`0f1a757da579831283413295cc7f3b47eb521c77`, unchanged. `src/bindings.rs` is generated from them.
Their license is in `vendor/VulkanMemoryAllocator/LICENSE.txt`:

> Copyright (c) 2017-2026 Advanced Micro Devices, Inc. All rights reserved.
>
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included in
> all copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
> THE SOFTWARE.

## Vulkan-Headers

`vendor/Vulkan-Headers/` holds files from <https://github.com/KhronosGroup/Vulkan-Headers> at
tag `vulkan-sdk-1.4.363.0`, commit `6802bb4733b63ed5efd3adb308a6c885ef180ea1`, unchanged. The
Vulkan types in `src/bindings.rs` are generated from them. They're
Copyright 2015-2026 The Khronos Group Inc. and marked `SPDX-License-Identifier: Apache-2.0 OR MIT`;
this crate uses them under MIT. Both license texts are in `vendor/Vulkan-Headers/LICENSES/`.
