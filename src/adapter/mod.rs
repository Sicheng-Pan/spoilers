use crate::{adapter::nllb::NLLBTokenizerAdapter, ctranslate2::wrapper::TokenVec, emsg};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

pub mod nllb;

#[derive(Clone, Debug, Default, Deserialize, Display, EnumIter, Eq, PartialEq, Serialize)]
pub enum AdapterKind {
    #[default]
    None,
    NLLBTokenizerHub,
    NLLBTokenizerLocal,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterConfig {
    pub chunk_size: usize,
    pub kind: AdapterKind,
    pub source: String,
}

impl AdapterConfig {
    pub fn initialize(&self) -> Result<Box<dyn Adapter>> {
        match self.kind {
            AdapterKind::None => Err(emsg("Missing adapter configuration")),
            AdapterKind::NLLBTokenizerHub => {
                Ok(Box::new(NLLBTokenizerAdapter::new_from_hub(&self.source)?))
            }
            AdapterKind::NLLBTokenizerLocal => {
                Ok(Box::new(NLLBTokenizerAdapter::new_from_file(&self.source)?))
            }
        }
    }
}

pub trait Adapter {
    fn available_languages(&self) -> Vec<String>;
    fn encode(&self, content: String, language: String) -> Result<Vec<TokenVec>>;
    fn decode(&self, tokens: Vec<TokenVec>) -> Result<String>;
    fn target_prefix(&self, language: String) -> Result<TokenVec>;
}

impl<T> Adapter for &Box<T>
where
    T: Adapter + ?Sized,
{
    fn available_languages(&self) -> Vec<String> {
        (**self).available_languages()
    }

    fn encode(&self, content: String, language: String) -> Result<Vec<TokenVec>> {
        (**self).encode(content, language)
    }

    fn decode(&self, tokens: Vec<TokenVec>) -> Result<String> {
        (**self).decode(tokens)
    }

    fn target_prefix(&self, language: String) -> Result<TokenVec> {
        (**self).target_prefix(language)
    }
}
