pub use snafu::{prelude::*, Whatever, ErrorCompat};
#[derive(Debug, Snafu(annotations))]
pub enum Errors{
    #[snafu(display("Input ID is invalid"))]
    InvalidIdError,
    #[snafu(whatever, display("Could not connect to database {message}"))]
    CouldNotConnectDBError{
        message:String,
    },
    InvalidInputError,
    #[snafu(whatever, display("Un expected error: {message}"))]
    WhateverError{
        message:String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source:Option<Box<dyn std::error::Error>>
    },
}
pub fn is_valid_id(id: u16) -> Result<(), Errors> {
    if id < 10 {
        whatever!("ID may not be less than 10, but it was {id}");
    }
    Ok(())
}
fn read_config_file(path: &str) -> Result<String, Whatever> {
    std::fs::read_to_string(path)
        .with_whatever_context(|_| format!("Could not read file {path}"))
}
pub fn returns_an_error() -> Result<String,Whatever >{
    whatever!("here is error");
}