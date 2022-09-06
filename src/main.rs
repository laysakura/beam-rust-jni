use j4rs::{errors::J4RsError, ClasspathEntry, JvmBuilder};

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

    // ここで "entrypoint" を呼ぶのではなく、 create_proto(); run_proto(); みたいにする

    // The instances returned from invocations and instantiations can be viewed as pointers to Java Objects.
    // They can be used for further Java calls.
    // For example, the following invokes the `isEmpty` method of the created java.lang.String instance
    let boolean_instance = jvm.invoke(&select_class_instance, "simpleSelect", &[])?;

    // If we need to transform an `Instance` to Rust value, the `to_rust` should be called
    let rust_boolean: bool = jvm.to_rust(boolean_instance)?;
    println!("The simpleSelect() method of the org.apache.beam.examples.MySelect instance finished with {}", rust_boolean);

    // Static invocation
    let _static_invocation_result = jvm.invoke_static(
        "java.lang.System",  // The Java class to invoke
        "currentTimeMillis", // The static method of the Java class to invoke
        &Vec::new(), // The `InvocationArg`s to use for the invocation - empty for this example
    )?;

    Ok(())
}
