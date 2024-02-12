use self::wrapper::{ReplicaPoolConfig, TokenVec};

#[cxx::bridge(namespace = "ctranslate2")]
pub mod wrapper {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[repr(u32)]
    pub enum ComputeType {
        DEFAULT,
        AUTO,
        FLOAT32,
        INT8,
        INT8_FLOAT32,
        INT8_FLOAT16,
        INT8_BFLOAT16,
        INT16,
        FLOAT16,
        BFLOAT16,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[repr(u32)]
    pub enum Device {
        CPU,
        CUDA,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ReplicaPoolConfig {
        pub num_threads_per_replica: usize,
        pub max_queued_batches: i64,
        pub cpu_core_offset: i32,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct TokenVec {
        pub tokens: Vec<String>,
    }

    unsafe extern "C++" {
        include!("ctranslate2/types.h");
        type ComputeType;
    }

    unsafe extern "C++" {
        include!("ctranslate2/devices.h");
        type Device;
    }

    unsafe extern "C++" {
        include!("ctranslate2/replica_pool.h");
        type ReplicaPoolConfig;
    }

    unsafe extern "C++" {
        include!("spoilers/src/ctranslate2/wrapper.h");

        pub type TranslatorWrapper;

        pub fn translate(
            &self,
            source_batches: Vec<TokenVec>,
            target_prefixes: Vec<TokenVec>,
        ) -> Result<Vec<TokenVec>>;

        pub fn device_auto() -> Device;
        pub fn new_translator(
            model_path: String,
            device: Device,
            compute_type: ComputeType,
            device_indices: Vec<i32>,
            config: ReplicaPoolConfig,
        ) -> Result<UniquePtr<TranslatorWrapper>>;

    }
}

impl PartialEq for ReplicaPoolConfig {
    fn eq(&self, other: &Self) -> bool {
        return self.num_threads_per_replica == other.num_threads_per_replica
            && self.max_queued_batches == other.max_queued_batches
            && self.cpu_core_offset == other.cpu_core_offset;
    }
}

impl Eq for ReplicaPoolConfig {}

impl TokenVec {
    pub fn new(tokens: impl Iterator<Item = impl AsRef<str>>) -> Self {
        Self {
            tokens: tokens.map(|token| token.as_ref().into()).collect(),
        }
    }
}
