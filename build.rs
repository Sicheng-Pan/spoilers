fn main() {
    println!("cargo:rerun-if-changed=src/ctranslate2/mod.rs");
    println!("cargo:rerun-if-changed=src/ctranslate2/wrapper.c");
    println!("cargo:rerun-if-changed=src/ctranslate2/wrapper.h");
    let mut wrapper = cxx_build::bridge("src/ctranslate2/mod.rs");
    wrapper.file("src/ctranslate2/wrapper.c").std("c++20");
    #[cfg(feature = "static")]
    {
        let onednn_static = cmake::Config::new(env!("ONEDNN_SRC"))
            .define("CMAKE_DISABLE_FIND_PACKAGE_GIT", "TRUE")
            .define("ONEDNN_LIBRARY_TYPE", "STATIC")
            .define("ONEDNN_BUILD_EXAMPLES", "OFF")
            .define("ONEDNN_BUILD_TESTS", "OFF")
            .define("ONEDNN_EXPERIMENTAL", "ON")
            .build();
        let ctranslate2_static = cmake::Config::new(env!("CTRANSLATE2_SRC"))
            .define("BUILD_CLI", "OFF")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("OPENMP_RUNTIME", "COMP")
            .define("WITH_DNNL", "ON")
            .define("WITH_MKL", "OFF")
            .build();
        std::fs::copy(
            ctranslate2_static.join("build/third_party/cpu_features/libcpu_features.a"),
            ctranslate2_static.join("lib64/libcpu_features.a"),
        )
        .expect("Unable to find cpu_features static library");
        println!(
            "cargo:rustc-link-search=native={}",
            onednn_static.join("lib64").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            ctranslate2_static.join("lib64").display()
        );
        println!("cargo:rustc-link-arg=-fopenmp");
        println!("cargo:rustc-link-lib=static=dnnl");
        println!("cargo:rustc-link-lib=static=cpu_features");
        println!("cargo:rustc-link-lib=static=ctranslate2");
        wrapper
            .include(ctranslate2_static.join("include"))
            .include(onednn_static.join("include"));
    }
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=dylib=ctranslate2");
    wrapper.compile("ctranslate2-spoilers");
}
