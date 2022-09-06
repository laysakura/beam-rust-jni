# Running Rust entrypoint

## Prerequisites

```bash
java -version
# 8.X
```

## Build example with custom Beam SDK (Java)

```bash
cd beam
./gradlew -p examples/java/ shadowJar
ls -lh examples/java/build/libs/beam-examples-java-2.42.0-SNAPSHOT-all.jar
```

## Run the pipeline from Rust

Run beam-flink backend:

```bash
docker run --net=host apache/beam_flink1.14_job_server:latest
```

And run the pipeline with beam-flink.

```bash
cd ..
DYLD_LIBRARY_PATH=$JAVA_HOME/lib cargo run
```
