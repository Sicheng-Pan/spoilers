use crate::{
    adapter::Adapter,
    ctranslate2::wrapper::{
        device_auto, new_translator, ComputeType, Device, ReplicaPoolConfig, TranslatorWrapper,
    },
};
use anyhow::Result;
use cxx::UniquePtr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TranslatorConfig {
    pub model_path: String,
    pub device: Device,
    pub compute_type: ComputeType,
    pub device_indices: Vec<i32>,
    pub config: ReplicaPoolConfig,
}

impl TranslatorConfig {
    pub fn initialize(&self) -> Result<Translator> {
        Translator::new(
            &self.model_path,
            self.device,
            self.compute_type,
            self.device_indices.clone(),
            self.config.clone(),
        )
    }
}

impl Default for TranslatorConfig {
    fn default() -> Self {
        Self {
            model_path: String::new(),
            device: device_auto(),
            compute_type: ComputeType::DEFAULT,
            device_indices: vec![0],
            config: ReplicaPoolConfig {
                num_threads_per_replica: 0,
                max_queued_batches: 0,
                cpu_core_offset: -1,
            },
        }
    }
}

pub struct Translator {
    ctranslate2: UniquePtr<TranslatorWrapper>,
}

impl Translator {
    pub fn new(
        model_path: impl AsRef<str>,
        device: Device,
        compute_type: ComputeType,
        device_indices: Vec<i32>,
        config: ReplicaPoolConfig,
    ) -> Result<Self> {
        Ok(Self {
            ctranslate2: new_translator(
                model_path.as_ref().into(),
                device,
                compute_type,
                device_indices,
                config,
            )?,
        })
    }

    pub fn translate(
        &self,
        adapter: impl Adapter,
        source_content: impl AsRef<str>,
        source_language: impl AsRef<str>,
        target_language: impl AsRef<str>,
    ) -> Result<String> {
        let source_batches = adapter.encode(
            source_content.as_ref().into(),
            source_language.as_ref().into(),
        )?;
        let prefix = adapter.target_prefix(target_language.as_ref().into())?;
        let target_prefixes = source_batches.iter().map(|_| prefix.clone()).collect();
        let target_tokens = self
            .ctranslate2
            .translate(source_batches, target_prefixes)?;
        adapter.decode(target_tokens)
    }
}
