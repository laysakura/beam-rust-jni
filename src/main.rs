mod beam_sdk;

use beam_proto_rs::v1::beam_runner_api::Pipeline as PipelineProto;
use j4rs::{errors::J4RsError, ClasspathEntry, InvocationArg, JvmBuilder};
use protobuf::Message;

use crate::beam_sdk::{
    options::PipelineOptions,
    runners::PipelineRunnerKind,
    schemas::{SchemaBuilder, SchemaFieldType},
    transforms::{Create, Select},
    values::RowBuilder,
    Pipeline,
};

fn create_pipeline_proto() -> PipelineProto {
    let options = PipelineOptions::new(PipelineRunnerKind::PortableRunner);
    let pipeline = Pipeline::new(options);

    let input_schema = SchemaBuilder::new()
        .add("f0", SchemaFieldType::Int16)
        .add("f1", SchemaFieldType::Int32)
        .add("f2", SchemaFieldType::String)
        .build();

    let input = RowBuilder::new(&input_schema)
        .add_value("f0", 1i16)
        .add_value("f1", 2)
        .add_value("f2", "3")
        .build()
        .unwrap();

    let rows = pipeline
        .apply(Create::from_row(input)) // Pipeline::apply -> PCollection
        .apply(Select::field_names(&["f2", "f0"])); // PCollection::apply -> PCollection

    let expected_schema = SchemaBuilder::new()
        .add("f2", SchemaFieldType::String)
        .add("f0", SchemaFieldType::Int16)
        .build();
    assert_eq!(rows.schema().unwrap(), &expected_schema); // determined before running the pipeline (PAssert not required)

    PipelineProto::from(pipeline)
}

fn main() -> Result<(), J4RsError> {
    let entry = ClasspathEntry::new(
        "beam/examples/java/build/libs/beam-examples-java-2.42.0-SNAPSHOT-all.jar",
    );

    // Create a JVM
    let jvm = JvmBuilder::new().classpath_entry(entry).build()?;

    // Create a java.lang.String instance
    let select_class_instance = jvm.create_instance(
        "org.apache.beam.examples.MySelect",
        &Vec::new(), // The `InvocationArg`s to use for the invocation - empty for this example
    )?;

    let pipeline = create_pipeline_proto();
    let mut pipeline_bin = Vec::<u8>::new();
    pipeline.write_to_vec(&mut pipeline_bin).unwrap();

    let pipeline_bin_java: Vec<InvocationArg> = pipeline_bin
        .into_iter()
        .map(|b| {
            InvocationArg::try_from(b as i8).expect("failed to convert rust byte into java byte")
        })
        .collect();
    let pipeline_bin_java = jvm.create_java_array("java.lang.Byte", &pipeline_bin_java)?;

    // The instances returned from invocations and instantiations can be viewed as pointers to Java Objects.
    // They can be used for further Java calls.
    // For example, the following invokes the `isEmpty` method of the created java.lang.String instance
    let boolean_instance = jvm.invoke(
        &select_class_instance,
        "simpleSelect",
        &[InvocationArg::from(pipeline_bin_java)],
    )?;

    // If we need to transform an `Instance` to Rust value, the `to_rust` should be called
    let rust_boolean: bool = jvm.to_rust(boolean_instance)?;
    println!("The simpleSelect() method of the org.apache.beam.examples.MySelect instance finished with {}", rust_boolean);

    Ok(())
}
