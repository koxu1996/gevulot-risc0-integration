mod input_builder;

pub use input_builder::InputBuilder;

fn main() {
    let mut builder = InputBuilder::new();

    // Push example guest arguments.
    builder.push_arg(&1787569u32);
    builder.push_arg(&1337u32);

    // Print output for Gevulot prover.
    let json_output = builder.to_json();
    println!("{}", json_output);
}
