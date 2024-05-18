use risc0_zkvm::serde::to_vec;

pub struct InputBuilder {
    args: Vec<String>,
}

impl InputBuilder {
    /// Creates a new `InputBuilder`.
    pub fn new() -> Self {
        InputBuilder { args: Vec::new() }
    }

    /// Adds an argument to the collector, serializing and hex-encoding it.
    pub fn push_arg<T: serde::ser::Serialize>(&mut self, data: &T) {
        let data: Vec<u32> = to_vec(data).unwrap();
        let bytes: &[u8] = bytemuck::cast_slice(&data);
        let hex_string = hex::encode(bytes);
        self.args.push(hex_string);
    }

    /// Returns the JSON representation of the arguments as a string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.args).unwrap()
    }
}
