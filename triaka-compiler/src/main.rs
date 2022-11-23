//! # Triaka Compiler
//! Triaka Compiler is an universal CLI tool of all Triaka OS DSLs and data formats, including
//! Bincode JSON, etc.

pub mod compilers;

use clap::Parser;
use std::io::{Read, Write};

/// A Triaka compiler.
pub trait Compiler {
    /// Prepares the source code to compile.
    fn prepare(
        &mut self,
        input: Box<dyn Read>,
        flags: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Compiles data in the prepared compiler.
    ///
    /// # Panic
    /// If the compiler is not prepared, any call to this method will cause panic.
    fn compile(self: Box<Self>) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cmdline {
    /// The compiler module to use
    #[arg(short, long)]
    module: String,

    /// Path of the input file
    #[arg(short, long, default_value_t = String::from("-"))]
    input: String,

    /// Path of the output file
    #[arg(short, long, default_value_t = String::from("-"))]
    output: String,

    /// Compiler flags
    #[arg(short, long)]
    flags: Vec<String>,
}

/// The entrypoint of the program.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parses the command-line arguments
    let cmdline = Cmdline::parse();

    // Gets the compiler to use by name specified
    let mut compiler = compilers::get_compiler_by_name(&cmdline.module)
        .ok_or_else(|| "the specified compiler module was not found".to_owned())?;

    // Initializes input source
    let input: Box<dyn Read> = match &cmdline.input[..] {
        "-" => Box::new(std::io::stdin()),
        x => Box::new(std::fs::File::open(x)?),
    };

    // Prepares the compiler
    compiler.prepare(input, cmdline.flags)?;

    // Compiles the source code
    let output_data = compiler.compile()?;

    // Outputs the target code
    if cmdline.output == "-" {
        let mut stdout = std::io::stdout();
        stdout.write_all(&output_data)?;
        stdout.flush()?;
    } else {
        std::fs::write(&cmdline.output, &output_data)?;
    }
    Ok(())
}
