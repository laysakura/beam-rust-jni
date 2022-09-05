# Running Rust entrypoint

```bash
java -version
# 17.X
```

You need to change Java option at: `org.apache.beam.examples.WordCount.entrypoint`

```bash
cd word-count-beam
mvn package -Dmaven.test.skip=true -Pportable-runner  # to create fat jar for WordCount Java app
```

Run beam-flink backend:

```bash
docker run --net=host apache/beam_flink1.14_job_server:latest --flink-master=localhost:8081
```

And run the pipeline with beam-flink.

```bash
DYLD_LIBRARY_PATH=$JAVA_HOME/lib LOG_LEVEL=debug cargo run

ls ../counts*
```
