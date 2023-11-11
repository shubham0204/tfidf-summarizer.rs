# Using the static library with C

In this example, we will load the static library generated with `cargo build` and use it with C code. **Make sure you have generated a header and a static library as shown in [Building the Debian package](https://github.com/shubham0204/tfidf-summarizer-rs/blob/main/README.md).** 

### Building an executable with `gcc`

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