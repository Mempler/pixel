# Pixel
Pixel is a tiny little game engine which is supposed to run on major platforms

## Compilation

### Linux
##### Dependencies
* [Rust](https://www.rust-lang.org/)
* [llvm](https://releases.llvm.org/download.html)

### Windows
##### Dependencies
* [Rust](https://www.rust-lang.org/)
* [llvm](https://releases.llvm.org/download.html)

For both platforms, the compilation is very similar for this purpose, we're using Linux shell for simplicity.
```shell script
~$ cargo build --release 
```

## Supported Platforms
|![Windows](./assets/windows_64x64.png)|![Linux](./assets/linux_64x64.png)|![osX](./assets/osx_64x64.png)|![android](./assets/android_64x64.png)|![ios](./assets/ios_64x64.png)|![web](./assets/wasm_64x64.png)|
|---|---|---|---|---|---|
|:heavy_check_mark:|:heavy_check_mark:|:heavy_check_mark:|:x:|:x:|:x:|
