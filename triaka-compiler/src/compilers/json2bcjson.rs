//! A compiler to convert JSON to Bincode JSON.

use std::io::Read;

/// A JSON-to-BCJSON compiler instance.
#[derive(Default)]
pub struct Compiler {
    input: Option<Box<dyn Read>>,
}
impl crate::Compiler for Compiler {
    fn prepare(
        &mut self,
        input: Box<dyn Read>,
        flags: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !flags.is_empty() {
            return Err("json2bcjson: this compiler doesn't receive flags".into());
        }
        self.input = Some(input);
        Ok(())
    }

    fn compile(mut self: Box<Self>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let json: serde_json::Value = serde_json::from_reader(self.input.take().unwrap())?;
        Ok(bincode_json::to_vec::<bincode_json::Value>(&json.into())?)
    }
}
