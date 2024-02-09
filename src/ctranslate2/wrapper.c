#include "spoilers/src/ctranslate2/wrapper.h"

namespace ctranslate2 {
  
  TranslatorWrapper::TranslatorWrapper(
    const std::string& model_path,
    const Device device,
    const ComputeType compute_type,
    const std::vector<int>& device_indices,
    const ReplicaPoolConfig& config
  ): translator(std::make_unique<Translator>(
    model_path, device, compute_type, device_indices, config
  )) {};

  rust::Vec<rust::String> TranslatorWrapper::translate(
    rust::Vec<rust::String> source_tokens,
    rust::Vec<rust::String> target_prefix
  ) const {
    std::vector<std::string> localized;
    for(auto &rs : source_tokens) {
      localized.push_back(std::string(rs));
    }
    std::vector<std::vector<std::string>> container{ localized };
    std::vector<std::string> initial;
    for(auto &rp : target_prefix) {
      initial.push_back(std::string(rp));    
    }
    std::vector<std::vector<std::string>> prefix{ initial };
    auto product = translator->translate_batch(container, prefix).at(0).output();
    rust::Vec<rust::String> delivery;
    for (auto &cs : product) {
      delivery.push_back(rust::String(cs));
    }
    return delivery;
  }

  Device device_auto() { 
    return str_to_device("auto");
  }
    
  std::unique_ptr<TranslatorWrapper> new_translator(
    rust::String model_path,
    Device device,
    ComputeType compute_type,
    rust::Vec<int> device_indices,
    ReplicaPoolConfig config
  ) {
    std::vector<int> cindices;
    for (auto &ri : device_indices) {
      cindices.push_back(ri);    
    }
    return std::make_unique<TranslatorWrapper>(
      std::string(model_path), device, compute_type, cindices, config
    );
  }

}

