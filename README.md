# rust_kernel

This is a minimalist Rust kernel that runs on a x86_64 architecture. 

# Download

The Build Artifact can be found in: Github Actions -> Artifacts.

# Run the Boot Image:

The .bin file can be executed via 

qemu-system-x86_64 -drive format=raw,file=bootimage-rust_kernel.bin.
