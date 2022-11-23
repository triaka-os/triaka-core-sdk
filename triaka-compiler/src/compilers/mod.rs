//! This module contains compiler modules of `triaka-compiler`, and utilities to implement or use
//! compiler modules, e.g. `get_compiler_by_name`.

mod bcjson2json;
mod json2bcjson;

use crate::Compiler;

/// Gets a `Compiler` implementation by its name.
pub fn get_compiler_by_name(name: &str) -> Option<Box<dyn Compiler>> {
    match name {
        "json2bcjson" => Some(Box::new(json2bcjson::Compiler::default())),
        "bcjson2json" => Some(Box::new(bcjson2json::Compiler::default())),
        _ => None,
    }
}
