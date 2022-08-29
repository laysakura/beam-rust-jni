package beam.rust.jni.common;

/**
 * The utility class that sets up and tears down external resources, and cancels
 * the streaming
c * pipelines once the program terminates.
 *
 * <p>
 * It is used to run Beam examples.
 */
public class ExampleUtils {

    public static final String TOKENIZER_PATTERN = "[^\\p{L}]+";

}
