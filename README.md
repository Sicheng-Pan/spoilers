# Spoilers

## What is this

Spoilers is a high-level Rust binding to [CTranslate2](https://github.com/OpenNMT/CTranslate2), a fast inference engine for transformer models. It also contains a barebone GUI based on the binding, which can translate texts offline using compatible models and tokenizer configurations.

Note that this project is not yet stable, and it is likely that things will change.

## Dependencies

Spoilers requires CTranslate2 at runtime, and it should be compiled against the corresponding header files in your system. The GUI may need a few more packages based on your platform, and the [Sarasa Gothic](https://github.com/be5invis/Sarasa-Gothic) font is packaged into the GUI for CJK font support.

Take a look at `flake.nix` in the [repository](https://github.com/Sicheng-Pan/spoilers) for more details. 

## Model data and adapters

Spoilers should be able to run CTranslate2 compatible models, given the appropriate model weights.

Take a look at [CTranslate2 documentation](https://opennmt.net/CTranslate2) for how to convert models into compatible formats.

For the GUI, we the adapter to tokenize raw inputs for the model and parse output tokens from the model. For example, compatible [NLLB-200](https://forum.opennmt.net/t/nllb-200-with-ctranslate2/5090) models can be used together with [tokenizers](https://docs.rs/tokenizers/latest/tokenizers).
