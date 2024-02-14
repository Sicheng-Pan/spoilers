# Spoilers

## What is this

Spoilers is a high-level Rust binding to [CTranslate2](https://github.com/OpenNMT/CTranslate2), a fast inference engine for transformer models. It also contains a barebone GUI based on the binding, which can translate texts offline using compatible models and tokenizer configurations.

Note that this project is not yet stable, and it is likely that things will change.

## Dependencies

If you want to dynamically link CTranslate2, Spoilers should be compiled against the corresponding headers and shared libraries in your system.

To compile the code statically, use the feature flag `static`. You also need the source code for CTranslate2 and oneDNN, and set the corresponding environment variables (`$CTRANSLATE2_SRC` and `$ONEDNN_SRC`) so that Rust can find them during compilation.

To build the GUI, use the feature flag `app`. The environment variable `$CJK_PATH` should be set to a font file (e.g. `Sarasa-Regular.ttc` from [Sarasa Gothic](https://github.com/be5invis/Sarasa-Gothic)) for CJK support.

Take a look at `build.rs` and `flake.nix` in the [repository](https://github.com/Sicheng-Pan/spoilers) for more details.

## Model data and adapters

Spoilers should be able to run CTranslate2 compatible models, given the appropriate model weights.

Take a look at [CTranslate2 documentation](https://opennmt.net/CTranslate2) for how to convert models into compatible formats.

The GUI needs adapters to tokenize raw inputs for the model and parse output tokens from the model. Adapaters are wrappers of tokenizers, which can perform additional processing, like model-specific formatting, after tokenization.

You can find many models and their tokenizers on [Hugging Face](https://huggingface.co/).

To get started, [here](https://archive.org/details/ctranslate2-nllb-1.3b) is a collection of tokenizer and model weights for the [NLLB-1.3B](https://huggingface.co/facebook/nllb-200-distilled-1.3B) model that can be used with the GUI.

## Credit

I came to realize that other people have already built similar wrappers (e.g. [ctranslate2-rs](https://github.com/jquesnelle/ctranslate2-rs)) after I prototyped this project. Feel free to take a look if you want alternative implementations!
