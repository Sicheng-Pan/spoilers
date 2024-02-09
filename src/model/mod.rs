use crate::translator::{Adapter, Language};
use anyhow::{Error, Result};
use std::path::Path;
use tokenizers::Tokenizer;

pub struct NLLBAdapter {
    tokenizer: Tokenizer,
}

impl NLLBAdapter {
    /// Load from local file
    pub fn new_from_file(file: impl AsRef<Path>) -> Result<Self> {
        Ok(NLLBAdapter {
            tokenizer: Tokenizer::from_file(file).map_err(|e| Error::msg(e.to_string()))?,
        })
    }

    /// Download from Hugging Face Hub
    pub fn new_from_web(identifier: impl AsRef<str>) -> Result<Self> {
        Ok(NLLBAdapter {
            tokenizer: Tokenizer::from_pretrained(identifier, None)
                .map_err(|e| Error::msg(e.to_string()))?,
        })
    }
}

impl Adapter for NLLBAdapter {
    fn encode(&self, source: String, language: Language) -> Result<Vec<String>> {
        let raw = self
            .tokenizer
            .encode(source, false)
            .map_err(|e| Error::msg(e.to_string()))?;
        let mut tokens = self.target_prefix(language);
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
            .map_err(|e| Error::msg(e.to_string()))?)
    }

    /// The NLLB models uses FLORES-200 language codes:
    /// <https://github.com/facebookresearch/flores/blob/main/flores200/README.md#languages-in-flores-200>
    fn target_prefix(&self, language: Language) -> Vec<String> {
        vec![match language {
            Language::Chinese => "zho_Hans",
            Language::English => "eng_Latn",
            Language::Japanese => "jpn_Jpan",
        }
        .into()]
    }
}
