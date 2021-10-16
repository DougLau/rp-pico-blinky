This is a stripped-down version of the rp2040 project [template]

[template]: https://github.com/rp-rs/rp2040-project-template

## Dependencies (Fedora 34)

```
dnf install libusb-devel
dnf install arm-none-eabi-gcc-cs
dnf install arm-none-eabi-newlib
rustup target install thumbv6m-none-eabi
cargo install elf2uf2-rs
```

## Running

Reset the Pico while pressing the BOOTSEL button, then:

```
cargo run --release
```
