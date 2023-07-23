# Rust UEFI Tetris

## Play

* `Left` `Right`: Move
* `Down`: Soft Drop
* `Z`: Hold
* `X`: Rotate Left
* `C`: Rotate Right
* `Space`: Hard Drop
* `R`: Reset
* SRS kick data

uefi macros depends on proc-macro2, which requires x86_64-pc-windows-msvc and MSVC Build Tool.

## Build

```bash
rustup target add x86_64-unknown-uefi
cargo build --target x86_64-unknown-uefi
```

Move `rust-uefi-tetris.efi` to a GPT disk with FAT partition's `EFT/BOOT/BOOTX64.EFI`
