cargo build --target aarch64-linux-android --features=android --release
cargo build --target armv7-linux-androideabi --features=android --release
cargo build --target i686-linux-android --features=android --release
cargo build --target x86_64-linux-android --features=android --release

mkdir -p examples/android/jniLibs/arm64-v8a
mkdir -p examples/android/jniLibs/armeabi-v7a
mkdir -p examples/android/jniLibs/x86
mkdir -p examples/android/jniLibs/x86_64

cp target/aarch64-linux-android/release/libsummarizer.so examples/android/jniLibs/arm64-v8a
cp target/armv7-linux-androideabi/release/libsummarizer.so examples/android/jniLibs/armeabi-v7a
cp target/i686-linux-android/release/libsummarizer.so examples/android/jniLibs/x86
cp target/x86_64-linux-android/release/libsummarizer.so examples/android/jniLibs/x86_64