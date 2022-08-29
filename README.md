# Running Rust entrypoint

```bash
DYLD_LIBRARY_PATH=/Library/Java/JavaVirtualMachines/jdk-18.0.1.1.jdk/Contents/Home/lib cargo run
```

# (temp) Running Java entrypoint

```bash
./gradlew run --args="--inputFile=/Users/sho.nakatani/.ghq/src/github.com/laysakura/beam-rust-jni/sample.txt --output=/Users/sho.nakatani/.ghq/src/github.com/laysakura/beam-rust-jni/counts"

ls counts*
```
