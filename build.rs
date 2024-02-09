fn main() {
    println!("cargo:rerun-if-changed=src/ctranslate2/mod.rs");
    println!("cargo:rerun-if-changed=src/ctranslate2/wrapper.c");
    println!("cargo:rerun-if-changed=src/ctranslate2/wrapper.h");
    println!("cargo:rustc-link-lib=ctranslate2");
    cxx_build::bridge("src/ctranslate2/mod.rs")
        .file("src/ctranslate2/wrapper.c")
        .opt_level_str("s")
        .std("c++20")
        .compile("ctranslate2-wrapper")
}
