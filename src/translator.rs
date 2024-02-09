use crate::ctranslate2::wrapper::{
    device_auto, new_translator, ComputeType, Device, ReplicaPoolConfig, TranslatorWrapper,
};
use anyhow::Result;
use cxx::UniquePtr;

#[derive(Debug)]
pub enum Language {
    Chinese,
    English,
    Japanese,
}

pub trait Adapter {
    fn encode(&self, source: String, language: Language) -> Result<Vec<String>>;
    fn decode(&self, tokens: Vec<String>) -> Result<String>;
    fn target_prefix(&self, language: Language) -> Vec<String>;
}

pub struct Translator {
    adapter: Box<dyn Adapter>,
    ctranslate2: UniquePtr<TranslatorWrapper>,
}

impl Translator {
    pub fn new(
        adapter: Box<dyn Adapter>,
        model_path: impl AsRef<str>,
        device: Device,
        compute_type: ComputeType,
        device_indices: Vec<i32>,
        config: ReplicaPoolConfig,
    ) -> Result<Self> {
        Ok(Translator {
            adapter,
            ctranslate2: new_translator(
                model_path.as_ref().into(),
                device,
                compute_type,
                device_indices,
                config,
            )?,
        })
    }

    pub fn new_default(adapter: Box<dyn Adapter>, model_path: impl AsRef<str>) -> Result<Self> {
        Self::new(
            adapter,
            model_path,
            device_auto(),
            ComputeType::DEFAULT,
            vec![0],
            ReplicaPoolConfig {
                num_threads_per_replica: 0,
                max_queued_batches: 0,
                cpu_core_offset: -1,
            },
        )
    }

    pub fn translate(
        &self,
        source: impl AsRef<str>,
        from_language: Language,
        to_language: Language,
    ) -> Result<String> {
        let from_tokens = self.adapter.encode(source.as_ref().into(), from_language)?;
        let to_tokens = self
            .ctranslate2
            .translate(from_tokens, self.adapter.target_prefix(to_language))?;
        Ok(self.adapter.decode(to_tokens)?)
    }
}
