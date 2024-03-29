use anyhow::Error;

pub mod adapter;
pub mod ctranslate2;
mod test;
pub mod translator;

pub fn emsg(e: impl ToString) -> Error {
    Error::msg(e.to_string())
}
