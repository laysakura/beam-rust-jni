# Running Rust entrypoint

```bash
java -version
# 17.X
```

You need to change Java option at: `org.apache.beam.examples.WordCount.entrypoint`

```bash
cd word-count-beam
mvn package -Dmaven.test.skip=true -Pflink-runner  # to create fat jar for WordCount Java app
```

And run the pipeline with Flink.

```bash
docker run \
    --rm \
    --name=jobmanager \
    --network flink-network \
    --publish 8081:8081 \
    --env FLINK_PROPERTIES="jobmanager.rpc.address: jobmanager" \
    flink:1.14 jobmanager

docker run \
    --rm \
    --name=taskmanager \
    --network flink-network \
    --env FLINK_PROPERTIES="jobmanager.rpc.address: jobmanager" \
    flink:1.14 taskmanager
```

```bash
DYLD_LIBRARY_PATH=$JAVA_HOME/lib LOG_LEVEL=debug cargo run

ls ../counts*
```
