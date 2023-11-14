# Using the static library with C/C++

`src/lib.rs` defines a FFI (Foreign Function Interface) which accepts a string (`const* uint8_t` in C and `*const u8` in Rust) and returns the summarized version of the string, given the `reduction_factor` and `length` of the string. The length is expressed with `usize` in Rust and with `const uintptr_t` in C. Our goal is generate two resources for building a C library for our Rust project:

1. A static library which provides the implementation of the `summarize` function in `src/lib.rs`
2. A header file which provides the declaration/prototype of the `summarize` function in `src/lib.rs`

### 1. Generating C headers for FFI

We use `cbindgen`, a crate provided by Mozilla, to generate C headers for functions defined in Rust source code. To install `cbindgen`, use,

```
$> cargo install --force cbindgen
```

Once installed, we use `cbindgen` to generate headers for `src/lib.rs` with target `lang` as `C`. The header file is produced in our Debian package's directory by specifying the `--output` option.

```
$> cbindgen --lang C --output examples/c/summarizer.h
```

The contents of the header `summarizer.h` are,

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

const uint8_t *summarize(const uint8_t *text, uintptr_t length, float reduction_factor);

const uint8_t *par_summarize(const uint8_t *text, uintptr_t length, float reduction_factor);
```

Notice, how `const* u8` and `usize` were transformed to `const uint8_t` and `uintptr_t` respectively with ease. Without `cbindgen` performing this task, we would have to determine C equivalent data-types by ourselves which may lead to inconsistency errors.

### 2. Generating a static library for our Rust code

The header file produced in step 1 will only provide the prototype of the `summarize` function to the calling C program. Upon compilation, we need to provide a library to the linker that will contain the implementation of the `summarize` function, else an `Unresolved reference` error is evident.

We generate a static library instead of a dynamic library in order to eliminate most external dependencies. However static libraries are heavier than dynamic libraries, the latter being loaded at run-time by operating system.

To start building a static library, we first need to set `crate_type` to `staticlib` in `Cargo.toml`,

```toml
[lib]
name = "summarizer"
crate_type = [ "staticlib" ]
```

Next, we build the project with `cargo build`, specifying the `target` architecture and type of build required (in our case, we produce an optimized `release` build),

```
$> cargo build --target=x86_64-unknown-linux-gnu --release
```

### 3. Building an executable with `gcc`

`main.c` contains the source code which reads the `wiki.txt` text file (a long text with ~26 words) and prints the summary along with wall time in milliseconds. `summarizer.h` is the header file generated with `cbindgen` for `src/lib.rs`.

```
$> examples/c> gcc main.c -o main -lsummarizer -lpthread -lm -ldl
$> examples/c> ./main wiki.txt
Do you personally know ANYONE who 
...
when I borrowed the book entitled ‘How to sharpen a pencil for dummies’ (Don’t blame me!. goodbye. 
786 milliseconds elapsed
$> examples/c>
```

Libraries like `libm`, `libpthread` and `libdl` have to be linked separately as they are not a part of the static library we generated from our Rust code. Here's an excellent [Reddit comment](https://www.reddit.com/r/C_Programming/comments/t0eh5f/comment/hybahx9/?utm_source=share&utm_medium=web2x&context=3) which explains why these libraries have to be linked separately.