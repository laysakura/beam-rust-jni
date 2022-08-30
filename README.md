# Running Rust entrypoint

```bash
mvn package -Dmaven.test.skip=true  # to create fat jar for WordCount Java app

DYLD_LIBRARY_PATH=/Library/Java/JavaVirtualMachines/jdk-18.0.1.1.jdk/Contents/Home/lib cargo run
```

# (temp) Running Java entrypoint

```bash
java -version
# 17.X
```

```bash
cd word-count-beam

mvn compile exec:java -Dexec.mainClass=org.apache.beam.examples.WordCount -Dexec.args="--inputFile=/Users/sho.nakatani/.ghq/src/github.com/laysakura/beam-rust-jni/sample.txt --output=/Users/sho.nakatani/.ghq/src/github.com/laysakura/beam-rust-jni/counts"

ls ../counts*
```
