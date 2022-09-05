package org.apache.beam.examples;

// Codes from <https://github.com/apache/beam/blob/master/sdks/java/core/src/test/java/org/apache/beam/sdk/schemas/transforms/SelectTest.java>

import org.apache.beam.sdk.schemas.Schema;
import org.apache.beam.sdk.schemas.transforms.Select;
import org.apache.beam.sdk.transforms.Create;
import org.apache.beam.sdk.values.PCollection;
import org.apache.beam.sdk.values.Row;
import org.apache.beam.sdk.Pipeline;

import org.apache.beam.sdk.options.PortablePipelineOptions;
import org.apache.beam.sdk.options.PipelineOptionsFactory;

public class MySelect {

  Pipeline createPipeline(String[] args) {
    PortablePipelineOptions options = PipelineOptionsFactory.fromArgs(args).withValidation()
        .as(PortablePipelineOptions.class);
    Pipeline p = Pipeline.create(options);
    return p;
  }

  // Added to be called from rust
  public void simpleSelect() {
    String[] args = new String[10];
    args[0] = "--runner=PortableRunner";
    args[1] = "--jobEndpoint=localhost:8099";
    args[2] = "--defaultEnvironmentType=LOOPBACK";

    Pipeline pipeline = createPipeline(args);

    Schema inputSchema = Schema.of(
        Schema.Field.of("f0", Schema.FieldType.INT16),
        Schema.Field.of("f1", Schema.FieldType.INT32),
        Schema.Field.of("f2", Schema.FieldType.STRING));

    Row input = Row.withSchema(inputSchema).addValues((short) 1, 2, "3").build();

    PCollection<Row> rows = pipeline
        .apply(Create.of(input).withRowSchema(inputSchema))
        .apply(Select.fieldNames("f2", "f1"));

    pipeline.run().waitUntilFinish();
  }

}
