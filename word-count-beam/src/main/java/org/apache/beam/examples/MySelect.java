package org.apache.beam.examples;

// Codes from <https://github.com/apache/beam/blob/master/sdks/java/core/src/test/java/org/apache/beam/sdk/schemas/transforms/SelectTest.java>

import com.google.auto.value.AutoValue;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import org.apache.beam.sdk.schemas.AutoValueSchema;
import org.apache.beam.sdk.schemas.JavaFieldSchema;
import org.apache.beam.sdk.schemas.Schema;
import org.apache.beam.sdk.schemas.Schema.FieldType;
import org.apache.beam.sdk.schemas.annotations.DefaultSchema;
import org.apache.beam.sdk.schemas.transforms.Convert;
import org.apache.beam.sdk.schemas.transforms.Select;
import org.apache.beam.sdk.transforms.Create;
import org.apache.beam.sdk.values.PCollection;
import org.apache.beam.sdk.values.Row;
import org.apache.beam.vendor.guava.v26_0_jre.com.google.common.collect.ImmutableList;
import org.apache.beam.vendor.guava.v26_0_jre.com.google.common.collect.ImmutableMap;
import org.apache.beam.sdk.Pipeline;

import org.apache.beam.sdk.options.PipelineOptions;
import org.apache.beam.sdk.options.PipelineOptionsFactory;
import org.apache.beam.runners.flink.FlinkPipelineOptions;

public class MySelect {

  /** flat schema to select from. */
  @DefaultSchema(JavaFieldSchema.class)
  public static class Schema1 {
    public String field1;
    public Integer field2;
    public Double field3;

    Schema1(String f1, Integer f2, Double f3) {
      field1 = f1;
      field2 = f2;
      field3 = f3;
    }

    public static Schema1 create() {
      return new Schema1("field1", 42, 3.14);
    }
  }

  /** A class matching the schema resulting from selection field1, field3. */
  @DefaultSchema(JavaFieldSchema.class)
  public static class Schema1Selected {
    public String field1;
    public Double field3;

    Schema1Selected(String f1, Double f3) {
      field1 = f1;
      field3 = f3;
    }

    public static Schema1Selected create() {
      return new Schema1Selected("field1", 3.14);
    }
  }

  Pipeline createPipeline(String[] args) {
    FlinkPipelineOptions options = PipelineOptionsFactory.fromArgs(args).withValidation()
        .as(FlinkPipelineOptions.class);
    Pipeline p = Pipeline.create(options);
    return p;
  }

  // Added to be called from rust
  public void simpleSelect() {
    String[] args = new String[10];
    args[0] = "--runner=FlinkRunner";
    // args[1] = "--flinkMaster=localhost:8081";
    // args[2] =
    // "--filesToStage=/Users/sho.nakatani/.ghq/src/github.com/laysakura/beam-rust-jni/word-count-beam/target/original-word-count-beam-bundled-0.1.jar";

    Pipeline pipeline = createPipeline(args);

    PCollection<Schema1Selected> rows = pipeline
        .apply(Create.of(Schema1.create()))
        .apply(Select.fieldNames("field1", "field3"))
        .apply(Convert.to(Schema1Selected.class));

    pipeline.run().waitUntilFinish();
  }

}
