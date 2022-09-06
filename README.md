# Running Rust entrypoint

## Prerequisites

```bash
java -version
# 8.X
```

## Build custom Beam SDK (Java)

```bash
cd beam
./gradlew -p sdks/java/core assemble
ls -lh sdks/java/core/build/libs/beam-sdks-java-core-2.41.0-SNAPSHOT.jar
```

## Build example (Java)

```bash
cd word-count-beam
mvn package -Dmaven.test.skip=true -Pportable-runner
```

## Run the pipeline from Rust

Run beam-flink backend:

```bash
docker run --net=host apache/beam_flink1.14_job_server:latest
```

And run the pipeline with beam-flink.

```bash
DYLD_LIBRARY_PATH=$JAVA_HOME/lib LOG_LEVEL=debug cargo run

ls ../counts*
```
