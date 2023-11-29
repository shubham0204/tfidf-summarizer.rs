# Compilation for Android and Writing FFI with JNI

Shared libraries for use in Android apps can be compiled by installing specific toolchains and by using linkers 
provided by Android NDK.

## Setup

Steps (1) and (2) below are covered more extensively on [Mozilla's blog](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html) here.

### 1. Installing the toolchains

We install Android-specific toolchains with `rustup`,

```
rustup toolchain install aarch64-linux-android
rustup toolchain install armv7-linux-androideabi
rustup toolchain install i686-linux-android
rustup toolchain install x86_64-linux-android
```

Thus, we will be compiling for both 32-bit and 64-bit, `arm` and `x86` targets. `arm` targets would run Android devices 
whereas `x86` targets can run on emulators and Chromebooks.

### 2. Configure Cargo

We need to install Android NDK and store the path of the resulting directory as an environment variable `NDK_HOME`.
Check the [official guides](https://developer.android.com/ndk/guides) on how to install Android NDK. 

Next, we edit the `~/.cargo/config` file to add linker paths for Android-specific builds we will initiate in the 
later steps. Use `nano ~/.cargo/config` and add the following contents to the file, replacing `NDK_HOME` with the 
directory path.

```
[target.aarch64-linux-android]
linker = "/home/shubham/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang"

[target.armv7-linux-androideabi]
linker = "/home/shubham/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi33-clang"

[target.i686-linux-android]
linker = "/home/shubham/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android33-clang"

[target.x86_64-linux-android]
linker = "/home/shubham/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android33-clang"
```

We add the newly configured toolchains to use them for compilation,

```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

### 3. Building the shared libraries

We use `cargo build --release` along with the `--target` argument providing the target for which we wish to 
build the shared libraries

```
cargo build --target aarch64-linux-android --feature=android --release
cargo build --target armv7-linux-androideabi --feature=android --release
cargo build --target i686-linux-android --feature=android --release
cargo build --target x86_64-linux-android --feature=android --release
```

Next, we move all `.so` files from the `target` directory to the `examples/android/jniLibs` directory which 
is the part of the Android app's project.

```
mkdir -p examples/android/jniLibs/arm64-v8a
mkdir -p examples/android/jniLibs/armeabi-v7a
mkdir -p examples/android/jniLibs/x86
mkdir -p examples/android/jniLibs/x86_64

cp target/aarch64-linux-android/release/libpredictor.so examples/android/jniLibs/arm64-v8a
cp target/armv7-linux-androideabi/release/libpredictor.so examples/android/jniLibs/armeabi-v7a
cp target/i686-linux-android/release/libpredictor.so examples/android/jniLibs/x86
cp target/x86_64-linux-android/release/libpredictor.so examples/android/jniLibs/x86_64
```