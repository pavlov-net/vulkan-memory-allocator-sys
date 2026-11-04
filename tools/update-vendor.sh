#!/usr/bin/env bash
# Replaces vendor/ with the files this crate needs from the pinned upstream commits. Bump the
# commits here, then run tools/gen-bindings.sh. A new VMA version also goes in the static_assert
# in src/vma.cpp, the Cargo.toml description and README.md, and new commits in
# THIRD_PARTY_LICENSES.md.
set -euo pipefail

# v3.5.0
vma_commit=0f1a757da579831283413295cc7f3b47eb521c77
# vulkan-sdk-1.4.363.0
headers_commit=6802bb4733b63ed5efd3adb308a6c885ef180ea1

root="$(cd "$(dirname "$0")/.." && pwd)"
# vendor/ is replaced only after every fetch succeeds.
vendor="$root/target/vendor.new"

fetch() { # repository, commit, path under the repository, directory under vendor/
  mkdir -p "$vendor/$4/$(dirname "$3")"
  curl -fsSL --retry 3 -o "$vendor/$4/$3" \
    "https://raw.githubusercontent.com/$1/$2/$3"
}

rm -rf "$vendor"

vma=GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator
fetch "$vma" "$vma_commit" include/vk_mem_alloc.h VulkanMemoryAllocator
fetch "$vma" "$vma_commit" LICENSE.txt VulkanMemoryAllocator

headers=KhronosGroup/Vulkan-Headers
for file in \
  LICENSE.md \
  LICENSES/Apache-2.0.txt \
  LICENSES/MIT.txt \
  include/vulkan/vk_platform.h \
  include/vulkan/vulkan.h \
  include/vulkan/vulkan_core.h; do
  fetch "$headers" "$headers_commit" "$file" Vulkan-Headers
done
# vulkan_core.h includes the video codec headers.
grep -o 'vk_video/[a-z0-9_]*\.h' "$vendor/Vulkan-Headers/include/vulkan/vulkan_core.h" | sort -u |
  while read -r file; do
    fetch "$headers" "$headers_commit" "include/$file" Vulkan-Headers
  done
# Those include each other.
while :; do
  missing=$(grep -rhoE 'vulkan_video_[a-z0-9_]*\.h' "$vendor/Vulkan-Headers/include/vk_video" | sort -u |
    while read -r file; do
      [ -f "$vendor/Vulkan-Headers/include/vk_video/$file" ] || echo "$file"
    done)
  [ -n "$missing" ] || break
  for file in $missing; do
    fetch "$headers" "$headers_commit" "include/vk_video/$file" Vulkan-Headers
  done
done

rm -rf "$root/vendor"
mv "$vendor" "$root/vendor"

echo "vendor/ updated: VMA $vma_commit, Vulkan-Headers $headers_commit"
