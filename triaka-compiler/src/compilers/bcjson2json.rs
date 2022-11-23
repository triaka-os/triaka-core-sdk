//! A compiler to convert Bincode JSON to JSON.

use std::io::Read;

/// A BCJSON-to-JSON compiler instance.
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
            return Err("bcjson2json: this compiler doesn't receive flags".into());
        }
        self.input = Some(input);
        Ok(())
    }

    fn compile(mut self: Box<Self>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = Vec::with_capacity(4 * 1024);
        self.input.take().unwrap().read_to_end(&mut buf)?;
        let value: bincode_json::Value = bincode_json::from_slice(&buf[..])?;
        let json = value.to_json();
        Ok(serde_json::to_vec(&json)?)
    }
}
