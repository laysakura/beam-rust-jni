# Running Rust entrypoint

```bash
java -version
# 17.X
```

You need to change Java option at: `org.apache.beam.examples.WordCount.entrypoint`

```bash
cd word-count-beam
mvn package -Dmaven.test.skip=true -Pflink-runner  # to create fat jar for WordCount Java app

DYLD_LIBRARY_PATH=$JAVA_HOME/lib LOG_LEVEL=debug cargo run

ls ../counts*
```
