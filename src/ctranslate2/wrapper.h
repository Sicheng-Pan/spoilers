#include "ctranslate2/translator.h"
#include "rust/cxx.h"
#pragma once

namespace ctranslate2 {

  struct TokenVec;

  class TranslatorWrapper {
    public:
      TranslatorWrapper(
        const std::string& model_path,
        const Device device,
        const ComputeType compute_type,
        const std::vector<int>& device_indices,
        const ReplicaPoolConfig& config
      );
      rust::Vec<TokenVec> translate(rust::Vec<TokenVec>, rust::Vec<TokenVec>) const;
    
    private:
      std::unique_ptr<Translator> translator;
  };

  Device device_auto();

  std::unique_ptr<TranslatorWrapper> new_translator(
    rust::String,
    Device,
    ComputeType,
    rust::Vec<int>,
    ReplicaPoolConfig
  );
  
}

