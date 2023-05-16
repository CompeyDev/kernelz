# kernelz

A simple kernel written in Rust, which prints the classic "Hello, world!" onto the VGA buffer.

## Running & Development
In order to actually run the kernel, first make sure you have the following prerequisites installed:
- qemu
- virt-manager
- virt-view
- dnsmasq
- vde2
- bridge-utils
- openbsd-netcat

On an arch-based system, they can be installed using pacman as such:
``` 
sudo pacman -S qemu virt-manager virt-viewer dnsmasq vde2 bridge-utils openbsd-netcat 
```
 ---
 
 Once installed, simply run the kernel using `cargo run`. To build the kernel, run `cargo build`, which will produce a compiled kernel
 that can be located `./target/target/$RELEASE_TYPE/bootimage-kernelz.bin`, where `RELEASE_TYPE` is either debug or release based on the 
 type compiled using cargo.
