use super::Adapter;
use crate::{ctranslate2::wrapper::TokenVec, emsg};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::{iter::once, path::Path};
use text_splitter::{ChunkCapacity, ChunkSize, ChunkSizer, TextSplitter};
use tokenizers::Tokenizer;
use toml::Table;

const CHUNK_SIZE: usize = 128;
const END_TOKEN: &str = "</s>";
const FLORES_200: Lazy<Table> =
    Lazy::new(|| include_str!("flores-200.toml").parse::<Table>().unwrap());

#[derive(Debug)]
pub struct NLLBTokenizerAdapter {
    tokenizer: Tokenizer,
}

impl NLLBTokenizerAdapter {
    pub fn new(file: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            tokenizer: Tokenizer::from_file(file).map_err(emsg)?,
        })
    }
}

impl Adapter for NLLBTokenizerAdapter {
    fn available_languages(&self) -> Vec<String> {
        FLORES_200.keys().into_iter().cloned().collect()
    }

    fn encode(&self, content: String, language: String) -> Result<Vec<TokenVec>> {
        let splitter = TextSplitter::new(&self.tokenizer).with_trim_chunks(true);
        let prefix = self.target_prefix(language)?.tokens;
        Ok(splitter
            .chunks(content.as_str(), CHUNK_SIZE)
            .into_iter()
            .map(|batch| self.tokenizer.encode(batch, false).map_err(emsg))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|encoding| {
                TokenVec::new(
                    prefix
                        .clone()
                        .iter()
                        .chain(encoding.get_tokens().into_iter())
                        .chain(once(&END_TOKEN.into())),
                )
            })
            .collect())
    }

    fn decode(&self, tokens: Vec<TokenVec>) -> Result<String> {
        Ok(self
            .tokenizer
            .decode(
                tokens
                    .into_iter()
                    .flat_map(|batch| batch.tokens.into_iter())
                    .flat_map(|token| self.tokenizer.token_to_id(token.as_str()))
                    .collect::<Vec<u32>>()
                    .as_slice(),
                true,
            )
            .map_err(emsg)?)
    }

    /// The NLLB models uses FLORES-200 language codes:
    /// <https://github.com/facebookresearch/flores/blob/main/flores200/README.md#languages-in-flores-200>
    fn target_prefix(&self, language: String) -> Result<TokenVec> {
        Ok(TokenVec::new(once(
            FLORES_200
                .get(&language)
                .context("Language not supported")?
                .as_str()
                .context("Invalid language code")?,
        )))
    }
}

impl ChunkSizer for NLLBTokenizerAdapter {
    fn chunk_size(&self, chunk: &str, capacity: &impl ChunkCapacity) -> ChunkSize {
        self.tokenizer.chunk_size(chunk, capacity)
    }
}
