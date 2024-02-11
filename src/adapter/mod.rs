use crate::{adapter::nllb::NLLBAdapter, emsg};
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
    pub kind: AdapterKind,
    pub source: String,
}

impl AdapterConfig {
    pub fn initialize(&self) -> Result<Box<dyn Adapter>> {
        match self.kind {
            AdapterKind::None => Err(emsg("Missing adapter configuration")),
            AdapterKind::NLLBTokenizerHub => Ok(Box::new(NLLBAdapter::new_from_hub(&self.source)?)),
            AdapterKind::NLLBTokenizerLocal => {
                Ok(Box::new(NLLBAdapter::new_from_file(&self.source)?))
            }
        }
    }
}

pub trait Adapter {
    fn available_languages(&self) -> Vec<String>;
    fn encode(&self, content: String, language: String) -> Result<Vec<String>>;
    fn decode(&self, tokens: Vec<String>) -> Result<String>;
    fn target_prefix(&self, language: String) -> Result<Vec<String>>;
}

impl<T> Adapter for &Box<T>
where
    T: Adapter + ?Sized,
{
    fn available_languages(&self) -> Vec<String> {
        (**self).available_languages()
    }

    fn encode(&self, content: String, language: String) -> Result<Vec<String>> {
        (**self).encode(content, language)
    }

    fn decode(&self, tokens: Vec<String>) -> Result<String> {
        (**self).decode(tokens)
    }

    fn target_prefix(&self, language: String) -> Result<Vec<String>> {
        (**self).target_prefix(language)
    }
}
