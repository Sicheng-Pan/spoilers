#include "spoilers/src/ctranslate2/mod.rs.h"
#include "spoilers/src/ctranslate2/wrapper.h"

namespace ctranslate2 {

TranslatorWrapper::TranslatorWrapper(const std::string &model_path,
                                     const Device device,
                                     const ComputeType compute_type,
                                     const std::vector<int> &device_indices,
                                     const ReplicaPoolConfig &config)
    : translator(std::make_unique<Translator>(model_path, device, compute_type,
                                              device_indices, config)){};

rust::Vec<TokenVec>
TranslatorWrapper::translate(rust::Vec<TokenVec> source_batches,
                             rust::Vec<TokenVec> target_prefixes) const {
  std::vector<std::vector<std::string>> source, target_prefix;
  for (auto batch : source_batches) {
    std::vector<std::string> source_tokens;
    for (auto token : batch.tokens) {
      source_tokens.push_back(std::string(token));
    }
    source.push_back(source_tokens);
  }
  for (auto prefix : target_prefixes) {
    std::vector<std::string> prefix_tokens;
    for (auto token : prefix.tokens) {
      prefix_tokens.push_back(std::string(token));
    }
    target_prefix.push_back(prefix_tokens);
  }
  auto translation_results = translator->translate_batch(source, target_prefix);
  rust::Vec<TokenVec> target_batches;
  for (auto result : translation_results) {
    rust::Vec<rust::String> target_tokens;
    for (auto token : result.output()) {
      target_tokens.push_back(rust::String(token));
    }
    target_batches.push_back({target_tokens});
  }
  return target_batches;
}

Device device_auto() { return str_to_device("auto"); }

std::unique_ptr<TranslatorWrapper>
new_translator(rust::String model_path, Device device, ComputeType compute_type,
               rust::Vec<int> device_indices, ReplicaPoolConfig config) {
  std::vector<int> cindices;
  for (auto &ri : device_indices) {
    cindices.push_back(ri);
  }
  return std::make_unique<TranslatorWrapper>(std::string(model_path), device,
                                             compute_type, cindices, config);
}

} // namespace ctranslate2
