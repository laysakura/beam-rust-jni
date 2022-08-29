use jni::{objects::JValue, sys::jint, InitArgsBuilder, JNIVersion, JavaVM};

fn main() {
    // Build the VM properties
    let jvm_args = InitArgsBuilder::new()
        // Pass the JNI API version (default is 8)
        .version(JNIVersion::V8)
        // You can additionally pass any JVM options (standard, like a system property,
        // or VM-specific).
        // Here we enable some extra JNI checks useful during development
        .option("-Xcheck:jni")
        .build()
        .unwrap();

    // Create a new VM
    let jvm = JavaVM::new(jvm_args).unwrap();

    // Attach the current thread to call into Java — see extra options in
    // "Attaching Native Threads" section.
    //
    // This method returns the guard that will detach the current thread when dropped,
    // also freeing any local references created in it
    let env = jvm.attach_current_thread().unwrap();

    // Call Java Math#abs(-10)
    let input = -10;
    let x = JValue::from(input);
    let val: jint = env
        .call_static_method("java/lang/Math", "abs", "(I)I", &[x])
        .unwrap()
        .i()
        .unwrap();

    println!("abs of {} = {} (calculated in Java)", input, val)
}
