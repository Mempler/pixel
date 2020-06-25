# Pixel
Pixel is a tiny little game engine which is supposed to run on major platforms

## Compilation

### Linux
##### Dependencies
* [Rust](https://www.rust-lang.org/)
* [FMOD](https://www.fmod.com/) `--no-default-features --features audio_fmod`
* [llvm](https://releases.llvm.org/download.html)
* [SDL2](https://www.libsdl.org/download-2.0.php)

### Windows
##### Dependencies
* [Rust](https://www.rust-lang.org/)
* [FMOD](https://www.fmod.com/) `--no-default-features --features audio_fmod`
* [llvm](https://releases.llvm.org/download.html)
* [SDL2](https://www.libsdl.org/download-2.0.php)

For both platforms, the compilation is very similar for this purpose, we're using Linux shell for simplicity.
```shell script
~$ cargo build --release 
```
