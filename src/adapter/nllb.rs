use super::Adapter;
use crate::emsg;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::path::Path;
use tokenizers::Tokenizer;
use toml::Table;

const FLORES_200: Lazy<Table> =
    Lazy::new(|| include_str!("flores-200.toml").parse::<Table>().unwrap());

#[derive(Clone, Debug)]
pub struct NLLBAdapter {
    tokenizer: Tokenizer,
}

impl NLLBAdapter {
    /// Load from local file
    pub fn new_from_file(file: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            tokenizer: Tokenizer::from_file(file).map_err(emsg)?,
        })
    }

    /// Download from Hugging Face Hub
    pub fn new_from_hub(identifier: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            tokenizer: Tokenizer::from_pretrained(identifier, None).map_err(emsg)?,
        })
    }
}

impl Adapter for NLLBAdapter {
    fn available_languages(&self) -> Vec<String> {
        FLORES_200.keys().into_iter().cloned().collect()
    }

    fn encode(&self, source: String, language: String) -> Result<Vec<String>> {
        let raw = self.tokenizer.encode(source, false).map_err(emsg)?;
        let mut tokens = self.target_prefix(language)?;
        tokens.extend_from_slice(raw.get_tokens());
        tokens.push(String::from("</s>"));
        Ok(tokens)
    }

    fn decode(&self, tokens: Vec<String>) -> Result<String> {
        Ok(self
            .tokenizer
            .decode(
                tokens
                    .into_iter()
                    .flat_map(|t| self.tokenizer.token_to_id(t.as_str()))
                    .collect::<Vec<u32>>()
                    .as_slice(),
                true,
            )
            .map_err(emsg)?)
    }

    /// The NLLB models uses FLORES-200 language codes:
    /// <https://github.com/facebookresearch/flores/blob/main/flores200/README.md#languages-in-flores-200>
    fn target_prefix(&self, language: String) -> Result<Vec<String>> {
        Ok(vec![FLORES_200
            .get(&language)
            .context("Language not supported")?
            .as_str()
            .context("Invalid language code")?
            .into()])
    }
}
