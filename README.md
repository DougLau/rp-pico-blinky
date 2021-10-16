This is a stripped-down version of the rp2040 project [template]

[template]: https://github.com/rp-rs/rp2040-project-template

## Dependencies (Fedora 34)

```
dnf install libusb-devel
dnf install arm-none-eabi-gcc-cs
dnf install arm-none-eabi-newlib
rustup target install thumbv6m-none-eabi
cargo install --git https://github.com/rp-rs/probe-run --branch main
```

## Running

```
cargo build --release
elf2uf2-rs ./target/thumbv6m-none-eabi/release/rp-pico-blinky
```

Then, copy `rp-pico-blinky.uf2` to a mounted Pico.
