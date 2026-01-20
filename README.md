# LPC176x5x PAC

Peripheral Access Crate for LPC176x5x MCUs.

## Generation

This codebase was produced by patching the [LPC176x5x_v0.2.svd](LPC176x5x_v0.2.svd) file ([source](https://github.com/cmsis-svd/cmsis-svd/blob/aa4721af946a253d18c8737b01d23e9c88a42e84/data/NXP/LPC176x5x_v0.2.svd)) and using [svd2rust](https://github.com/rust-embedded/svd2rust).

The python script modifies hundreds of enums with the same name, `ENUM`. Unfortunately, this was not possible with `svdtools`, and an [issue](https://github.com/rust-embedded/svdtools/issues/294) has been opened.

The [patch.yaml](patch.yaml) provides a few fixes for duplicate enum variants and other miscellaneous fixes.

```bash
uv run patch.py
svdtools patch patch.yaml
svd2rust --edition 2024 -i LPC176x5x_v0.2-python.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt
```
