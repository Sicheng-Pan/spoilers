#[cxx::bridge(namespace = "ctranslate2")]
pub mod wrapper {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
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

    #[derive(Debug)]
    #[repr(u32)]
    pub enum Device {
        CPU,
        CUDA,
    }

    #[derive(Debug)]
    pub struct ReplicaPoolConfig {
        pub num_threads_per_replica: usize,
        pub max_queued_batches: i64,
        pub cpu_core_offset: i32,
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
            source_tokens: Vec<String>,
            target_prefix: Vec<String>,
        ) -> Result<Vec<String>>;

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
